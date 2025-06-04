use sledgehammer_bindgen::bindgen;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{console, Node};

#[wasm_bindgen(inline_js = r#"
class NodeInterpreter {
    constructor(){
        this.nodes = [document.getElementById("main")];
    }

    export function get_node(id){
        return this.nodes[id];
    }
}
"#)]
extern "C" {
    #[wasm_bindgen]
    pub type NodeInterpreter;

    #[wasm_bindgen(method)]
    fn get_node(this: &NodeInterpreter, id: u16) -> Node;
}

fn main() {
    #[bindgen]
    mod js {
        #[extends(NodeInterpreter)]
        struct Channel;

        fn create_element(id: u16, name: &'static str<u8, name_cache>) {
            "this.nodes[$id$]=document.createElement($name$);"
        }

        fn create_element_ns(
            id: u16,
            name: &'static str<u8, name_cache>,
            ns: &'static str<u8, ns_cache>,
        ) {
            "this.nodes[$id$]=document.createElementNS($ns$,$name$);"
        }

        fn set_attribute(id: u16, name: &'static str<u8, name_cache>, val: impl Writable<u8>) {
            "this.nodes[$id$].setAttribute($name$,$val$);"
        }

        fn remove_attribute(id: u16, name: &'static str<u8, name_cache>) {
            "this.nodes[$id$].removeAttribute($name$);"
        }

        fn append_child(id: u16, id2: u16) {
            "this.nodes[$id$].appendChild(nodes[$id2$]);"
        }

        fn insert_before(parent: u16, id: u16, id2: u16) {
            "this.nodes[$parent$].insertBefore(nodes[$id$],nodes[$id2$]);"
        }

        fn set_text(id: u16, text: impl Writable<u8>) {
            "this.nodes[$id$].textContent=$text$;"
        }

        fn remove(id: u16) {
            "this.nodes[$id$].remove();"
        }

        fn replace(id: u16, id2: u16) {
            "this.nodes[$id$].replaceWith(this.nodes[$id2$]);"
        }

        fn clone(id: u16, id2: u16) {
            "this.nodes[$id2$]=this.nodes[$id$].cloneNode(true);"
        }

        fn first_child(id: u16) {
            "this.node[id]=this.node[id].firstChild;"
        }

        fn next_sibling(id: u16) {
            "this.node[id]=this.node[id].nextSibling;"
        }
    }

    let mut channel1 = Channel::default();
    let main = 0;
    let node1 = 1;
    let node2 = 2;
    channel1.create_element(node1, "div");
    channel1.create_element(node2, "span");
    channel1.append_child(node1, node2);
    channel1.set_text(node2, "Hello World!");
    channel1.append_child(main, node1);
    channel1.flush();

    let typed: &NodeInterpreter = channel1.js_channel().as_ref();
    console::log_1(&typed.get_node(0).into());
}
