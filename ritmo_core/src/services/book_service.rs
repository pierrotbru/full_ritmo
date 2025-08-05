use ritmo_core::dto::BookDto;
use ritmo_db::repository::FormatRepository;
use ritmo_ml::utils::MLStringUtils;

pub struct BookService<'a> {
    ml_utils: &'a MLStringUtils,
    format_repo: &'a FormatRepository,
}

impl<'a> BookService<'a> {
    pub fn normalize_and_save_book(&self, dto: BookDto) -> Result<(), String> {
        // in questo punto assumo che i dati siano già stati elaborati, e quindi qui non devo più fare nessun controllo: i dati sono validi.

        Ok(())
    }
}
