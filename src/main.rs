use chrono::prelude::*;
use std::fs;

mod article_search;

fn main() {
    let start_time = Local::now();

    let data = fs::read_to_string("./json/artikel.json").expect("Can't read file");

    let data: String = data
        .chars()
        .filter(|c| (c >= &'!' && c <= &'ÿ') || c == &' ')
        .collect();

    let art_all = article_search::ArtikelListe::from_json(&data);

    let search_time = Local::now();

    let mut art_list = article_search::article_search(&art_all, "test");

    art_list.truncate(50);

    for a in art_list {
        println!("{:?}", a);
    }

    let end_time = Local::now();
    let duration_all = end_time.signed_duration_since(start_time);
    let duration_search = end_time.signed_duration_since(search_time);

    println!("Search: {:?}", duration_search);
    println!("Complete: {:?}", duration_all);
}
