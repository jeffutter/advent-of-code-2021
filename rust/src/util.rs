use cookie_store::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use ureq::AgentBuilder;

pub fn read_input(day: usize) -> String {
    let filename = format!("../inputs/day{:0>2}", day);

    if !std::path::Path::new(&filename).exists() {
        let cookie_json = fs::read_to_string("../cookie.json").unwrap();
        let cookie_store = CookieStore::load_json(cookie_json.as_bytes()).unwrap();

        let url = format!("https://adventofcode.com/{:0>4}/day/{}/input", 2021, day);

        let body = AgentBuilder::new()
            .cookie_store(cookie_store)
            .build()
            .get(&url)
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        let mut writer = File::create(&filename).unwrap();
        write!(writer, "{}", body).unwrap();
    }

    std::fs::read_to_string(&filename).unwrap()
}
