use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ArtOutput {
    pub satz_id: String,
    pub score: i64,
}

impl ArtOutput {
    fn new(satz_id: String, score: i64) -> ArtOutput {
        ArtOutput { satz_id, score }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ArtikelListe {
    pub artikel_liste: Vec<Artikel>,
}

impl ArtikelListe {
    pub fn from_json(json_string: &str) -> ArtikelListe {
        serde_json::from_str(json_string).unwrap()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Artikel {
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

/// Input Format
/// ```
/// let json_string = r#"
/// {
///    "artikel_liste": [
///        {
///            "satz_id": "SATZ91239812",
///            "art_nr01": "e123",
///            "art_nr02": "e969696",
///            "bezeich": "Zange",
///            "merkmale": [
///                {
///                    "merkmal": "Größe",
///                    "wert": "15",
///                    "einheit": "mm"
///                },
///                {
///                    "merkmal": "Gewicht",
///                    "wert": "0.1",
///                    "einheit": "KG"
///                }
///            ]
///        },
///        {
///            "satz_id": "SATZ91239812",
///            "art_nr01": "e123",
///            "art_nr02": "e969696",
///            "bezeich": "Zange",
///            "merkmale": [
///                {
///                    "merkmal": "Größe",
///                    "wert": "15",
///                    "einheit": "mm"
///                },
///                {
///                    "merkmal": "Gewicht",
///                    "wert": "0.1",
///                    "einheit": "KG"
///                }
///            ]
///        }
///    ]
/// }
/// "#;
/// ```
pub fn article_search(art_list: &ArtikelListe, search: &str) -> Vec<ArtOutput> {
    let mut art_out: Vec<ArtOutput> = Vec::new();

    let matcher = SkimMatcherV2::default();
    for a in &art_list.artikel_liste {
        let art_str = a.get_string();
        match matcher.fuzzy_match(&art_str, search) {
            Some(score) => {
                art_out.push(ArtOutput::new(a.satz_id.clone(), score));
            }
            None => {}
        }
    }

    art_out.sort_by(|a, b| b.score.cmp(&a.score));
    art_out
}
