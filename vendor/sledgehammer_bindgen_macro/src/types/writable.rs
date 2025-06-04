use quote::__private::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_quote, Ident, Type};

use crate::encoder::{CreateEncoder, Encode, EncodeTraitObject, Encoder, Encoders};

use super::numbers::{NumberEncoder, NumberEncoderFactory};

pub struct WritableEncoder<const S: u32> {
    size_type: EncodeTraitObject,
}

pub struct WritableEncoderFactory<const S: u32>;

impl<const S: u32> CreateEncoder for WritableEncoderFactory<S> {
    type Output = WritableEncoder<S>;

    fn create(&self, builder: &mut Encoders) -> Self::Output {
        WritableEncoder {
            size_type: builder
                .get_or_insert_with(NumberEncoderFactory::<S>)
                .clone(),
        }
    }

    fn rust_ident(&self) -> Ident {
        Ident::new(&format!("writable_{}", S * 8), Span::call_site())
    }
}

impl<const S: u32> WritableEncoder<S> {
    fn size_type(&self) -> &NumberEncoder<S> {
        self.size_type.downcast()
    }
}

impl<const S: u32> Encoder for WritableEncoder<S> {
    fn rust_type(&self) -> Type {
        parse_quote! {()}
    }

    fn rust_ident(&self) -> Ident {
        Ident::new(&format!("writable_{}", S * 8), Span::call_site())
    }
}

impl<const S: u32> Encode for WritableEncoder<S> {
    fn encode_js(&self) -> String {
        format!(
            "this.s.substring(this.sp,this.sp+={})",
            self.size_type.encode_js()
        )
    }

    fn encode_rust(&self, name: &Ident) -> TokenStream2 {
        let char_len = Ident::new("char_len", Span::call_site());
        let write_len = self.size_type.encode_rust(&char_len);
        let char_len_type = self.size_type().element_type();
        quote! {
            let prev_len = self.str_buffer.len();
            #name.write(&mut self.str_buffer);
            // the length of the string is the change in length of the string buffer
            let #char_len: usize = unsafe { std::str::from_utf8_unchecked(&self.str_buffer[prev_len..]).chars().map(|c| c.len_utf16()).sum() };
            let #char_len = {
                #char_len as #char_len_type
            };
            #write_len
        }
    }
}
