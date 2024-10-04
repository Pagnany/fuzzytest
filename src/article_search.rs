use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ArtOutput {
    pub satz_id: String,
    pub bezeich: String,
    pub score: i64,
}

impl ArtOutput {
    pub fn new(satz_id: String, bezeich: String, score: i64) -> ArtOutput {
        ArtOutput {
            satz_id,
            bezeich,
            score,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ArtInput {
    pub search: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ArtikelListe {
    pub artikel_liste: Vec<Artikel>,
}

impl ArtikelListe {
    pub fn from_json(json_string: &str) -> Result<ArtikelListe, serde_json::Error> {
        serde_json::from_str(json_string)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Artikel {
    pub satz_id: String,
    pub art_nr01: String,
    pub art_nr02: String,
    pub bezeich01: String,
    pub bezeich02: String,
    pub bezeich03: String,
    pub bezeich04: String,
    pub match01: String,
    pub match02: String,
    pub farbcode: String,
    pub abmessung: String,
    pub gewicht: String,
    pub sn: String,
    pub diniso: String,
    pub gtyp: String,
    pub katbest01: String,
    pub ean_code: String,
    pub merkmale: Vec<Merkmal>,
}

impl Artikel {
    pub fn get_string(&self) -> String {
        let mut s = format!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            self.bezeich01,
            self.bezeich02,
            self.bezeich03,
            self.bezeich04,
            self.match01,
            self.match02,
            self.farbcode,
            self.abmessung,
            self.gewicht,
            self.sn,
            self.diniso,
            self.gtyp,
            self.art_nr01,
            self.art_nr02,
            self.katbest01,
            self.ean_code,
            self.satz_id,
        );
        for m in &self.merkmale {
            s.push_str(&format!(" {} {}", m.wert, m.einheit));
        }
        s
    }

    pub fn get_string_merkmale(&self) -> String {
        let mut s = String::new();
        for m in &self.merkmale {
            s.push_str(&format!("{} {} ", m.wert, m.einheit));
        }
        s
    }

    pub fn get_string_matchcode(&self) -> String {
        format!("{} {}", self.match01, self.match02)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Merkmal {
    pub merkmal: String,
    pub wert: String,
    pub einheit: String,
}
