use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BookDto {
    // Obbligatori
    pub title: String,
    pub format: String,                    // es: "epub", "pdf", ecc.
    pub user_content: Vec<UserContentDto>, // definiremo UserContentDto separatamente

    // Opzionali
    pub publisher: Option<String>,
    pub year: Option<u16>,
    pub series: Option<String>,
    pub series_index: Option<u32>,
    pub isbn: Option<String>,

    // Dati manuali immessi dall'utente
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub has_paper: Option<bool>,
    pub has_cover: Option<bool>,

    // Dati generati
    pub file_link: String,
    pub file_hash: String,
}
