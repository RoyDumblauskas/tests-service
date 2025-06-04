use std::borrow::Borrow;
use std::fmt::{self, write, Arguments, Debug};
use std::hash::{BuildHasher, Hash};
use std::mem::MaybeUninit;

#[derive(Default)]
pub struct NonHashBuilder;

impl std::hash::BuildHasher for NonHashBuilder {
    type Hasher = NonHash;
    fn build_hasher(&self) -> Self::Hasher {
        NonHash(0)
    }
}

#[allow(unused)]
#[derive(Default)]
pub struct NonHash(u64);

impl std::hash::Hasher for NonHash {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }
    fn write_usize(&mut self, i: usize) {
        self.0 = i as u64;
    }
}

/// A pointer that is only used for hashing purposes
#[derive(Debug, Copy, Clone)]
pub struct StaticPtr(*const ());

impl std::hash::Hash for StaticPtr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.0, state);
    }
}

impl std::cmp::PartialEq for StaticPtr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ?Sized> From<&'static T> for StaticPtr {
    fn from(t: &'static T) -> Self {
        StaticPtr(t as *const T as *const ())
    }
}

pub type PtrConstLru<const N: usize, const N2: usize> = ConstLru<StaticPtr, NonHashBuilder, N, N2>;
pub type FxConstLru<T, const N: usize, const N2: usize> =
    ConstLru<T, std::hash::BuildHasherDefault<rustc_hash::FxHasher>, N, N2>;

struct RawTable<T, const N: usize> {
    buckets: [Option<T>; N],
}

impl<T: Copy, const N: usize> RawTable<T, N> {
    const fn new() -> Self {
        Self { buckets: [None; N] }
    }

    fn get(&self, hash: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&T> {
        let mut offset = 0;
        loop {
            let index = (hash as usize + offset) % N;
            let val = unsafe { self.buckets.get_unchecked(index).as_ref() };
            if let Some(val) = val {
                if eq(val) {
                    return Some(val);
                }
            } else {
                return None;
            }
            offset += 1;
        }
    }

    fn insert(&mut self, hash: u64, value: T) {
        let mut offset = 0;
        loop {
            let index = (hash as usize + offset) % N;
            let val = unsafe { self.buckets.get_unchecked_mut(index) };
            if val.is_none() {
                *val = Some(value);
                return;
            }
            offset += 1;
        }
    }
}

pub struct ConstLru<T, H, const N: usize, const N2: usize> {
    entries: [MaybeUninit<Node<T>>; N],
    free_after: u8,
    first: Option<u8>,
    last: Option<u8>,
    table: RawTable<u8, N2>,
    hasher: H,
}

impl<T: Hash + PartialEq, H: BuildHasher + Default, const N: usize, const N2: usize> Default
    for ConstLru<T, H, N, N2>
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Hash + PartialEq + Debug, H: BuildHasher, const N: usize, const N2: usize> Debug
    for ConstLru<T, H, N, N2>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstLru")
            .field("entries", &self.entries)
            .field("free_after", &self.free_after)
            .field("first", &self.first)
            .field("last", &self.last)
            .finish()
    }
}

impl<T, H: BuildHasher, const N: usize, const N2: usize> ConstLru<T, H, N, N2> {
    pub const fn new(hasher: H) -> Self {
        assert!(N < u8::MAX as usize);
        assert!(N > 0);
        Self {
            entries: [const { MaybeUninit::uninit() }; N],
            free_after: 0,
            first: None,
            last: None,
            table: RawTable::new(),
            hasher,
        }
    }

    pub fn push<I>(&mut self, borrowed: &I) -> (u8, bool)
    where
        T: Borrow<I>,
        I: ?Sized + ToOwned<Owned = T> + Hash + PartialEq,
    {
        let hash = self.hasher.hash_one(borrowed);
        if let Some(entry) = self.table.get(hash, |ptr| unsafe {
            let entry = self.entries.get_unchecked(*ptr as usize).assume_init_ref();
            entry.data.borrow() == borrowed
        }) {
            unsafe {
                let node = self
                    .entries
                    .get_unchecked_mut(*entry as usize)
                    .assume_init_mut();
                let next = node.next;
                let prev = node.prev;
                if let Some(next) = next {
                    let next = self
                        .entries
                        .get_unchecked_mut(next as usize)
                        .assume_init_mut();
                    next.prev = prev;
                }
                if let Some(prev) = prev {
                    let prev = self
                        .entries
                        .get_unchecked_mut(prev as usize)
                        .assume_init_mut();
                    prev.prev = next;
                }
                if let Some(old_first) = self.first {
                    self.entries
                        .get_unchecked_mut(old_first as usize)
                        .assume_init_mut()
                        .next = Some(*entry);
                }
                self.first = Some(*entry);
                (*entry, false)
            }
        } else {
            let idx = if self.free_after < N as u8 {
                unsafe {
                    let entry = self.entries.get_unchecked_mut(self.free_after as usize);
                    entry.write(Node {
                        data: borrowed.to_owned(),
                        next: None,
                        prev: self.first,
                    });
                }
                if let Some(first) = self.first {
                    unsafe {
                        let first = self
                            .entries
                            .get_unchecked_mut(first as usize)
                            .assume_init_mut();
                        first.next = Some(self.free_after);
                    }
                }
                self.first = Some(self.free_after);
                if self.last.is_none() {
                    self.last = self.first;
                }
                let idx = self.free_after;
                self.free_after += 1;
                self.table.insert(hash, idx);
                idx
            } else {
                let last = self.last.unwrap();
                let last_node = unsafe {
                    self.entries
                        .get_unchecked_mut(last as usize)
                        .assume_init_mut()
                };
                let new = Node {
                    data: borrowed.to_owned(),
                    next: None,
                    prev: self.first,
                };
                self.last = last_node.next;
                *last_node = new;
                if let Some(last) = self.last {
                    unsafe {
                        let last = self
                            .entries
                            .get_unchecked_mut(last as usize)
                            .assume_init_mut();
                        last.prev = None;
                    }
                }
                if let Some(first) = self.first {
                    unsafe {
                        let first = self
                            .entries
                            .get_unchecked_mut(first as usize)
                            .assume_init_mut();
                        first.next = Some(last);
                    }
                }
                self.first = Some(last);
                last
            };
            (idx, true)
        }
    }
}

impl<T, H, const N: usize, const N2: usize> Drop for ConstLru<T, H, N, N2> {
    fn drop(&mut self) {
        // Drop any entries that were allocated
        for entry in self.entries.iter_mut().take(self.free_after as usize) {
            unsafe {
                entry.assume_init_drop();
            }
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<u8>,
    prev: Option<u8>,
}

#[test]
fn push_2() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::*;
    let hash_builder: BuildHasherDefault<DefaultHasher> = BuildHasherDefault::default();
    let mut lru: ConstLru<i32, _, 2, 4> = ConstLru::new(hash_builder);
    assert_eq!(lru.push(&0), (0, true));
    assert_eq!(lru.push(&0), (0, false));
    assert_eq!(lru.push(&1), (1, true));
    assert_eq!(lru.push(&2), (0, true));
    assert_eq!(lru.push(&3), (1, true));
    assert_eq!(lru.push(&4), (0, true));
}

#[test]
fn push_100() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::*;
    let hash_builder: BuildHasherDefault<DefaultHasher> = BuildHasherDefault::default();
    let mut lru: ConstLru<i32, _, 100, 200> = ConstLru::new(hash_builder);
    for i in 0..100 {
        assert_eq!(lru.push(&i), (i as u8, true));
    }
    for i in 0..100 {
        assert_eq!(lru.push(&(i + 100)), (i as u8, true));
    }
}

#[test]
fn fuzzing() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::*;
    let hash_builder: BuildHasherDefault<DefaultHasher> = BuildHasherDefault::default();
    let mut lru: ConstLru<_, _, 100, 200> = ConstLru::new(hash_builder);
    #[cfg(miri)]
    let count = 1000;
    #[cfg(not(miri))]
    let count = 100_000;
    for _ in 0..count {
        let rand = rand::random::<u8>();
        _ = lru.push(&rand.to_string());
    }
}

#[test]
fn sizes() {
    use std::mem::size_of;

    dbg!(size_of::<RawTable<u8, 0>>());
    dbg!(size_of::<RawTable<u8, 10>>());
    dbg!(size_of::<RawTable<u8, 128>>());
    dbg!(size_of::<Node<()>>());
    dbg!(size_of::<ConstLru<(), (), 0, 0>>());
    dbg!(size_of::<ConstLru<(), (), 10, 20>>());
    dbg!(size_of::<ConstLru<u8, (), 128, 256>>());
}

pub trait Writable {
    fn write(self, into: &mut Vec<u8>);
}

impl Writable for &str {
    fn write(self, into: &mut Vec<u8>) {
        unsafe {
            copy(self, into);
        }
    }
}

impl Writable for char {
    fn write(self, into: &mut Vec<u8>) {
        into.push(self as u8);
    }
}

impl Writable for String {
    fn write(self, into: &mut Vec<u8>) {
        unsafe {
            copy(self.as_str(), into);
        }
    }
}

impl Writable for &String {
    fn write(self, into: &mut Vec<u8>) {
        unsafe {
            copy(self.as_str(), into);
        }
    }
}

impl Writable for Arguments<'_> {
    fn write(self, into: &mut Vec<u8>) {
        struct Wrapper<'a>(&'a mut Vec<u8>);

        impl<'a> fmt::Write for Wrapper<'a> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                unsafe {
                    copy(s, self.0);
                }
                Ok(())
            }
            fn write_char(&mut self, c: char) -> fmt::Result {
                self.0.push(c as u8);
                Ok(())
            }
        }

        let _ = write(&mut Wrapper(into), self);
    }
}

unsafe fn copy(s: &str, buf: &mut Vec<u8>) {
    let old_len = buf.len();
    buf.reserve(s.len());
    let ptr = buf.as_mut_ptr().add(old_len);
    let bytes = s.as_bytes();
    let str_ptr = bytes.as_ptr();
    for o in 0..s.len() {
        *ptr.add(o) = *str_ptr.add(o);
    }
    buf.set_len(old_len + s.len());
}

impl<F> Writable for F
where
    F: FnOnce(&mut Vec<u8>),
{
    fn write(self, to: &mut Vec<u8>) {
        self(to);
    }
}

macro_rules! write_unsized {
    ($t: ty) => {
        impl Writable for $t {
            fn write(self, to: &mut Vec<u8>) {
                let mut n = self;
                let mut n2 = n;
                let mut num_digits = 0;
                while n2 > 0 {
                    n2 /= 10;
                    num_digits += 1;
                }
                let len = num_digits.max(1);
                to.reserve(len);
                let ptr = to.as_mut_ptr().cast::<u8>();
                let old_len = to.len();
                let mut i = len - 1;
                loop {
                    unsafe { ptr.add(old_len + i).write((n % 10) as u8 + b'0') }
                    n /= 10;

                    if n == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }

                #[allow(clippy::uninit_vec)]
                unsafe {
                    to.set_len(old_len + (len - i));
                }
            }
        }
    };
}

macro_rules! write_sized {
    ($t: ty) => {
        impl Writable for $t {
            fn write(self, to: &mut Vec<u8>) {
                let neg = self < 0;
                let mut n = if neg {
                    match self.checked_abs() {
                        Some(n) => n,
                        None => <$t>::MAX / 2 + 1,
                    }
                } else {
                    self
                };
                let mut n2 = n;
                let mut num_digits = 0;
                while n2 > 0 {
                    n2 /= 10;
                    num_digits += 1;
                }
                num_digits = num_digits.max(1);
                let len = if neg { num_digits + 1 } else { num_digits };
                to.reserve(len);
                let ptr = to.as_mut_ptr().cast::<u8>();
                let old_len = to.len();
                let mut i = len - 1;
                loop {
                    unsafe { ptr.add(old_len + i).write((n % 10) as u8 + b'0') }
                    n /= 10;

                    if n == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }

                if neg {
                    i -= 1;
                    unsafe { ptr.add(old_len + i).write(b'-') }
                }

                #[allow(clippy::uninit_vec)]
                unsafe {
                    to.set_len(old_len + (len - i));
                }
            }
        }
    };
}

write_unsized!(u8);
write_unsized!(u16);
write_unsized!(u32);
write_unsized!(u64);
write_unsized!(u128);
write_unsized!(usize);

write_sized!(i8);
write_sized!(i16);
write_sized!(i32);
write_sized!(i64);
write_sized!(i128);
write_sized!(isize);

#[allow(unused)]
fn to_string_testing<T: Writable>(t: T) -> String {
    let mut buf = Vec::new();
    t.write(&mut buf);
    unsafe { String::from_utf8_unchecked(buf) }
}

#[test]
fn fmt_nums() {
    assert_eq!(to_string_testing(0u8), "0");
    assert_eq!(to_string_testing(100u8), "100");
    assert_eq!(to_string_testing(0u16), "0");
    assert_eq!(to_string_testing(100u16), "100");
    assert_eq!(to_string_testing(0u32), "0");
    assert_eq!(to_string_testing(100u32), "100");
    assert_eq!(to_string_testing(0i8), "0");
    assert_eq!(to_string_testing(-100i8), "-100");
    assert_eq!(to_string_testing(100i8), "100");
    assert_eq!(to_string_testing(0i16), "0");
    assert_eq!(to_string_testing(-100i16), "-100");
    assert_eq!(to_string_testing(100i16), "100");
    assert_eq!(to_string_testing(0i32), "0");
    assert_eq!(to_string_testing(-100i32), "-100");
    assert_eq!(to_string_testing(100i32), "100");
}

#[test]
fn fmt_str() {
    assert_eq!(to_string_testing("hello"), "hello");
    assert_eq!(to_string_testing("hello world"), "hello world");
    let mut buf = Vec::new();
    "hello".write(&mut buf);
    " world".write(&mut buf);
    assert_eq!(unsafe { String::from_utf8_unchecked(buf) }, "hello world");
}

#[test]
fn fmt_args() {
    assert_eq!(to_string_testing(format_args!("hello")), "hello");
    assert_eq!(
        to_string_testing(format_args!("hello world")),
        "hello world"
    );
    let mut buf = Vec::new();
    format_args!("hello").write(&mut buf);
    format_args!(" world").write(&mut buf);
    assert_eq!(unsafe { String::from_utf8_unchecked(buf) }, "hello world");
}
