use ritmo_errors::RitmoResult;
use crate::dto::BookDto;

pub fn book_persistence (book: &mut BookDto) -> RitmoResult<()> {
    book.file_link = Some("ab/cb/abcdefghijklmnopqrstuvwxyz0123456789012345678901".to_string());
    book.file_size = Some(100000);
    book.file_hash = Some("abcdefghijklmnopqrstuvwxyz0123456789012345678901".to_string());
    Ok(())
}