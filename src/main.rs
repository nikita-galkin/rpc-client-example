extern crate jsonrpc;
extern crate serde;
use serde_derive::{ Deserialize };

#[derive(Deserialize, Debug, Clone)]
struct Key {
    index: u32,
    key_from_node: [u8; 32],
}
fn main() {
    let client = jsonrpc::client::Client::simple_http("127.0.0.1:9933", None, None).unwrap();
    match client.send_request(client.build_request("getActualKey", &vec![])).and_then(|res| jsonrpc::Response::result::<Key>(&res)) {
        Ok(key_from_node) => {
            println!("Get key {:?} index {:?}", key_from_node.key_from_node, key_from_node.index);
        },
        Err(err) => {
            println!("Can't get key! Error {}", err);
        },
    }
}
