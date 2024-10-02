use chrono::prelude::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Debug, Default, Serialize, Deserialize)]
struct ArtOutput {
    satz_id: String,
    score: i64,
}

impl ArtOutput {
    fn new(satz_id: String, score: i64) -> ArtOutput {
        ArtOutput { satz_id, score }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ArtikelListe {
    artikel_liste: Vec<Artikel>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Artikel {
    satz_id: String,
    art_nr01: String,
    art_nr02: String,
    bezeich: String,
    merkmale: Vec<Merkmal>,
}

impl Artikel {
    fn get_string(&self) -> String {
        let mut s = format!(
            "{} {} {} {}",
            self.satz_id, self.art_nr01, self.art_nr02, self.bezeich
        );
        for m in &self.merkmale {
            s.push_str(&format!(" {} {}", m.wert, m.einheit));
        }
        s
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Merkmal {
    merkmal: String,
    wert: String,
    einheit: String,
}

fn main() {
    let start_time = Local::now();

    deserde_test();

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);

    println!("Complete: {:?}", duration);
}

fn deserde_test() {
    let _data = r#"
    {
        "artikel_liste": [
            {
                "satz_id": "SATZ91239812",
                "art_nr01": "e123",
                "art_nr02": "e969696",
                "bezeich": "Zange",
                "merkmale": [
                    {
                        "merkmal": "Größe",
                        "wert": "15",
                        "einheit": "mm"
                    },
                    {
                        "merkmal": "Gewicht",
                        "wert": "0.1",
                        "einheit": "KG"
                    }
                ]
            },
            {
                "satz_id": "SATZ91239812",
                "art_nr01": "e123",
                "art_nr02": "e969696",
                "bezeich": "Zange",
                "merkmale": [
                    {
                        "merkmal": "Größe",
                        "wert": "15",
                        "einheit": "mm"
                    },
                    {
                        "merkmal": "Gewicht",
                        "wert": "0.1",
                        "einheit": "KG"
                    }
                ]
            }
        ]
    }
    "#;

    let data = fs::read_to_string("./json/artikel.json").expect("Can't read file");

    let conv: String = data
        .chars()
        .filter(|c| (c >= &'!' && c <= &'ÿ') || c == &' ')
        .collect();

    let art_list: ArtikelListe = serde_json::from_str(&conv).unwrap();

    //println!("{:?}", art);

    let suche = "zange wasser pumpe";
    let mut anz = 0;
    let mut art_out: Vec<ArtOutput> = Vec::new();

    let start_time = Local::now();

    let matcher = SkimMatcherV2::default();
    for a in art_list.artikel_liste {
        let art_str = a.get_string();
        match matcher.fuzzy_match(&art_str, suche) {
            Some(score) => {
                anz += 1;
                art_out.push(ArtOutput::new(a.satz_id.clone(), score));
                println!("{}", art_str);
            }
            None => {}
        }
    }

    println!("Anzahl: {}", anz);

    art_out.sort_by(|a, b| b.score.cmp(&a.score));

    for (i, a) in art_out.iter().enumerate() {
        if i > 20 {
            break;
        }
        println!("{}: {} {}", i, a.score, a.satz_id);
    }

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    println!("Search: {:?}", duration);
}
