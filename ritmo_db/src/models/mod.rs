pub mod aliases;
pub mod books;
pub mod books_contents;
pub mod books_people_roles;
pub mod books_tags;
pub mod contents;
pub mod contents_languages;
pub mod contents_people_roles;
pub mod contents_tags;
pub mod formats;
pub mod languages;
pub mod people;
pub mod publishers;
pub mod roles;
pub mod series;
pub mod tags;
pub mod types;

pub use self::aliases::*;
pub use self::books::*;
pub use self::books_contents::*;
pub use self::books_people_roles::*;
pub use self::books_tags::*;
pub use self::contents::*;
pub use self::contents_languages::*;
pub use self::contents_people_roles::*;
pub use self::contents_tags::*;
pub use self::formats::*;
pub use self::languages::*;
pub use self::people::*;
pub use self::publishers::*;
pub use self::roles::*;
pub use self::series::*;
pub use self::tags::*;
pub use self::types::*;

use ritmo_core::FullDto;

#[derive(Debug, Clone)]
pub struct FullBook {
    pub alias: Alias,
    pub book: Book,
    pub book_content: BookContent,
    pub book_people_roles: BookPersonRole,
    pub books_tags: BookTag,
    pub contents: Content,
    pub contents_languages: ContentLanguage,
    pub contents_people_roles: ContentPersonRole,
    pub contents_tags: ContentTag,
    pub formats: Format,
    pub languages: RunningLanguages,
    pub people: Person,
    pub publishers: Publisher,
    pub roles: Role,
    pub series: Serie,
    pub tags: Tag,
    pub types: Type,
}

impl FullBook {
    pub fn from_dto(dto: &FullDto) {}
}
