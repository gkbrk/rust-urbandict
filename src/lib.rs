#[macro_use] extern crate serde_derive;
extern crate reqwest;

#[derive(Deserialize, Debug)]
pub struct Definition {
    pub author: String,
    pub definition: String
}

#[derive(Deserialize)]
struct ApiWord {
    list: Vec<Definition>
}

pub fn get_definitions(word: &str) -> Result<Vec<Definition>, reqwest::Error> {
    let mut res = try!(reqwest::get(&format!("https://api.urbandictionary.com/v0/define?term={}", word)));
    let decoded: ApiWord = try!(res.json());

    Ok(decoded.list)
}
