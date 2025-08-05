use crate::dto::{BookDto, ContentDto};
use std::error::Error;

// Definizione dell'interfaccia per la risoluzione delle entità
pub trait EntityResolver {
    fn resolve_publisher(&self, name: &str) -> Result<Option<i64>, Box<dyn Error>>;
    fn resolve_format(&self, name: &str) -> Result<Option<i64>, Box<dyn Error>>;
    fn resolve_series(&self, name: &str) -> Result<Option<i64>, Box<dyn Error>>;
    fn resolve_person(&self, name: &str) -> Result<Option<i64>, Box<dyn Error>>;
    fn resolve_role(&self, name: &str) -> Result<Option<i64>, Box<dyn Error>>;
    fn resolve_tag(&self, name: &str) -> Result<Option<i64>, Box<dyn Error>>;
    fn resolve_content(&self, name: &str) -> Result<Option<i64>, Box<dyn Error>>;
    fn resolve_type(&self, name: &str) -> Result<Option<i64>, Box<dyn Error>>;
    fn resolve_language(&self, name: &str) -> Result<Option<i64>, Box<dyn Error>>;

    // Metodo per risolvere tutte le entità di un Book
    fn resolve_book_entities(&self, book: &mut BookDto) -> Result<(), Box<dyn Error>> {
        // Risolvi publisher
        if book.publisher_id.is_none()
            && !book.publisher_is_new
            && !book.publisher_name.trim().is_empty()
        {
            if let Some(id) = self.resolve_publisher(&book.publisher_name)? {
                book.publisher_id = Some(id);
            } else {
                book.publisher_is_new = true;
            }
        }

        // Risolvi format
        if book.format_id.is_none() && !book.format_is_new && !book.format_name.trim().is_empty() {
            if let Some(id) = self.resolve_format(&book.format_name)? {
                book.format_id = Some(id);
            } else {
                book.format_is_new = true;
            }
        }

        // Risolvi series
        if book.series_id.is_none() && !book.series_is_new && !book.series_name.trim().is_empty() {
            if let Some(id) = self.resolve_series(&book.series_name)? {
                book.series_id = Some(id);
            } else {
                book.series_is_new = true;
            }
        }

        // Risolvi authors
        for author in &mut book.authors {
            if author.person_id.is_none()
                && !author.person_is_new
                && !author.person_name.trim().is_empty()
            {
                if let Some(id) = self.resolve_person(&author.person_name)? {
                    author.person_id = Some(id);
                } else {
                    author.person_is_new = true;
                }
            }

            if author.role_id.is_none()
                && !author.role_is_new
                && !author.role_name.trim().is_empty()
            {
                if let Some(id) = self.resolve_role(&author.role_name)? {
                    author.role_id = Some(id);
                } else {
                    author.role_is_new = true;
                }
            }
        }

        // Risolvi tags
        for tag in &mut book.tags {
            if tag.id.is_none() && !tag.is_new && !tag.name.trim().is_empty() {
                if let Some(id) = self.resolve_tag(&tag.name)? {
                    tag.id = Some(id);
                } else {
                    tag.is_new = true;
                }
            }
        }

        // Risolvi contents
        for content in &mut book.contents {
            if content.content_id.is_none()
                && !content.content_is_new
                && !content.content_name.trim().is_empty()
            {
                if let Some(id) = self.resolve_content(&content.content_name)? {
                    content.content_id = Some(id);
                } else {
                    content.content_is_new = true;
                }
            }
        }

        Ok(())
    }

    // Metodo per risolvere tutte le entità di un Content
    fn resolve_content_entities(&self, content: &mut ContentDto) -> Result<(), Box<dyn Error>> {
        // Risolvi type
        if content.type_id.is_none() && !content.type_is_new && !content.type_name.trim().is_empty()
        {
            if let Some(id) = self.resolve_type(&content.type_name)? {
                content.type_id = Some(id);
            } else {
                content.type_is_new = true;
            }
        }

        // Risolvi authors
        for author in &mut content.authors {
            if author.person_id.is_none()
                && !author.person_is_new
                && !author.person_name.trim().is_empty()
            {
                if let Some(id) = self.resolve_person(&author.person_name)? {
                    author.person_id = Some(id);
                } else {
                    author.person_is_new = true;
                }
            }

            if author.role_id.is_none()
                && !author.role_is_new
                && !author.role_name.trim().is_empty()
            {
                if let Some(id) = self.resolve_role(&author.role_name)? {
                    author.role_id = Some(id);
                } else {
                    author.role_is_new = true;
                }
            }
        }

        // Risolvi tags
        for tag in &mut content.tags {
            if tag.id.is_none() && !tag.is_new && !tag.name.trim().is_empty() {
                if let Some(id) = self.resolve_tag(&tag.name)? {
                    tag.id = Some(id);
                } else {
                    tag.is_new = true;
                }
            }
        }

        // Risolvi languages
        for language in &mut content.languages {
            if language.id.is_none() && !language.is_new && !language.name.trim().is_empty() {
                if let Some(id) = self.resolve_language(&language.name)? {
                    language.id = Some(id);
                } else {
                    language.is_new = true;
                }
            }
        }

        Ok(())
    }
}
