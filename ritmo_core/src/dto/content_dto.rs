use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentDto {
    // Obbligatori
    pub title: String,
    pub author: String,

    // Opzionali
    pub people: Vec<(String, String)>,
    pub pub_date: Option<u64>,
    pub orig_lang: Option<string>,
    pub curr_lang: Option<string>,
    pub src_lang: Option<string>,

    // Dati manuali immessi dall'utente
    pub notes: Option<String>,
    pub tags: Vec<String>,
}
