extern crate rustc_serialize;
extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::json;

#[derive(RustcDecodable)]
struct ApiDefinition {
    definition: String,
}

#[derive(RustcDecodable)]
struct ApiWord {
    list: Vec<ApiDefinition>
}

pub fn get_definitions(word: String) -> Vec<String> {
    let client = Client::new();
    let mut res = client.get(&format!("http://api.urbandictionary.com/v0/define?term={}", word))
        .header(Connection::close())
        .send().unwrap();

    let mut json_string = String::new();
    res.read_to_string(&mut json_string).unwrap();

    let decoded: ApiWord = json::decode(&json_string).unwrap();

    let mut results = Vec::new();
    for word in decoded.list {
        results.push(word.definition);
    }
    return results;
}
