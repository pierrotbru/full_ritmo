#[derive(Debug, Clone, FromRow, Default)]
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
