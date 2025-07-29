-- Migrazione per rimuovere i campi 'rating' e 'read_status' dalle tabelle 'books' e 'contents'
-- Elimina completamente le tabelle e le ricrea senza usare ALTER TABLE

-- Fase 1: Elimina gli indici che potrebbero dipendere dalle colonne da rimuovere
DROP INDEX IF EXISTS idx_books_rating;
DROP INDEX IF EXISTS idx_books_read_status;
DROP INDEX IF EXISTS idx_books_name_search;
DROP INDEX IF EXISTS idx_contents_name_search;
DROP INDEX IF EXISTS idx_books_search_optimized;
DROP INDEX IF EXISTS idx_contents_search_optimized;
DROP INDEX IF EXISTS idx_books_series_lookup;
DROP INDEX IF EXISTS idx_books_metadata;
DROP INDEX IF EXISTS idx_books_file_info;
DROP INDEX IF EXISTS idx_books_dates_combined;
DROP INDEX IF EXISTS idx_contents_dates;

-- Fase 2: Aggiorna le viste che potrebbero dipendere dalle colonne da rimuovere

-- Aggiorna la vista BooksSearchOptimized
DROP VIEW IF EXISTS BooksSearchOptimized;
CREATE VIEW BooksSearchOptimized AS
SELECT
    b.id,
    b.name,
    b.original_title,
    b.publication_date,
    b.series_index,
    b.isbn,
    b.pages,
    -- rimosso rating
    -- rimosso read_status
    b.has_cover,
    b.has_paper,
    s.name as series_name,
    p.name as main_author,
    p.id as author_id,
    f.name as format_name,
    pub.name as publisher_name,
    b.created_at,
    b.last_modified_date
FROM books b
LEFT JOIN series s ON b.series_id = s.id
LEFT JOIN formats f ON b.format_id = f.id
LEFT JOIN publishers pub ON b.publisher_id = pub.id
LEFT JOIN books_people_roles bpr ON b.id = bpr.book_id
LEFT JOIN people p ON bpr.person_id = p.id
LEFT JOIN roles r ON bpr.role_id = r.id
WHERE r.name IN ('Autore', 'Author', 'Scrittore')
   OR bpr.role_id = (
       SELECT MIN(role_id)
       FROM books_people_roles
       WHERE book_id = b.id
   );

-- Aggiorna la vista ContentsSearchOptimized
DROP VIEW IF EXISTS ContentsSearchOptimized;
CREATE VIEW ContentsSearchOptimized AS
SELECT
    c.id,
    c.name,
    c.original_title,
    c.publication_date,
    c.pages,
    -- rimosso rating
    t.name as type_name,
    p.name as main_author,
    p.id as author_id,
    c.created_at,
    c.updated_at
FROM contents c
LEFT JOIN types t ON c.type_id = t.id
LEFT JOIN contents_people_roles cpr ON c.id = cpr.content_id
LEFT JOIN people p ON cpr.person_id = p.id
LEFT JOIN roles r ON cpr.role_id = r.id
WHERE r.name IN ('Autore', 'Author', 'Scrittore')
   OR cpr.role_id = (
       SELECT MIN(role_id)
       FROM contents_people_roles
       WHERE content_id = c.id
   );

-- Aggiorna la vista ContentsFullDetails
DROP VIEW IF EXISTS ContentsFullDetails;
CREATE VIEW ContentsFullDetails AS
SELECT
    c.id AS content_id,
    c.name AS content_name,
    c.original_title,
    c.publication_date,
    c.pages,
    -- rimosso rating
    c.notes AS content_notes,
    t.name AS type_name,
    GROUP_CONCAT(DISTINCT p.id) AS person_ids,
    GROUP_CONCAT(DISTINCT p.name) AS person_names,
    GROUP_CONCAT(DISTINCT r.name) AS role_names,
    GROUP_CONCAT(DISTINCT tag.name) AS tag_names,
    GROUP_CONCAT(DISTINCT rl.official_name || ' (' || rl.language_role || ')') AS language_info
FROM contents c
LEFT JOIN types t ON c.type_id = t.id
LEFT JOIN contents_people_roles cpr ON c.id = cpr.content_id
LEFT JOIN people p ON cpr.person_id = p.id
LEFT JOIN roles r ON cpr.role_id = r.id
LEFT JOIN contents_tags ct ON c.id = ct.content_id
LEFT JOIN tags tag ON ct.tag_id = tag.id
LEFT JOIN contents_languages cl ON c.id = cl.content_id
LEFT JOIN running_languages rl ON cl.language_id = rl.id
GROUP BY c.id;

-- Aggiorna la vista BooksFullDetails (se esiste)
DROP VIEW IF EXISTS BooksFullDetails;
CREATE VIEW BooksFullDetails AS
SELECT
    b.id AS book_id,
    b.name AS book_name,
    b.original_title,
    b.publication_date,
    b.acquisition_date,
    b.isbn,
    b.pages,
    b.notes AS book_notes,
    b.has_cover,
    b.has_paper,
    b.file_link,
    b.file_size,
    b.file_hash,
    -- rimosso rating
    -- rimosso read_status
    s.name AS series_name,
    b.series_index,
    pub.name AS publisher_name,
    f.name AS format_name,
    GROUP_CONCAT(DISTINCT p.id) AS person_ids,
    GROUP_CONCAT(DISTINCT p.name) AS person_names,
    GROUP_CONCAT(DISTINCT r.name) AS role_names,
    GROUP_CONCAT(DISTINCT tag.name) AS tag_names,
    GROUP_CONCAT(DISTINCT c.id) AS content_ids,
    GROUP_CONCAT(DISTINCT c.name) AS content_names
FROM books b
LEFT JOIN publishers pub ON b.publisher_id = pub.id
LEFT JOIN formats f ON b.format_id = f.id
LEFT JOIN series s ON b.series_id = s.id
LEFT JOIN books_people_roles bpr ON b.id = bpr.book_id
LEFT JOIN people p ON bpr.person_id = p.id
LEFT JOIN roles r ON bpr.role_id = r.id
LEFT JOIN books_tags bt ON b.id = bt.book_id
LEFT JOIN tags tag ON bt.tag_id = tag.id
LEFT JOIN books_contents bc ON b.id = bc.book_id
LEFT JOIN contents c ON bc.content_id = c.id
GROUP BY b.id;

-- Aggiorna la vista StatsOverview
DROP VIEW IF EXISTS StatsOverview;
CREATE VIEW StatsOverview AS
SELECT
    'books' as entity_type,
    COUNT(*) as total_count,
    COUNT(CASE WHEN has_cover = 1 THEN 1 END) as with_cover,
    COUNT(CASE WHEN has_paper = 1 THEN 1 END) as with_paper,
    -- rimosso rated
    -- rimosso avg_rating
    -- rimosso read_count
    -- rimosso reading_count
    -- rimosso unread_count
    0 as dummy_field
FROM books
UNION ALL
SELECT
    'contents' as entity_type,
    COUNT(*) as total_count,
    0 as with_cover,
    0 as with_paper,
    -- rimosso rated
    -- rimosso avg_rating
    0 as dummy_field
FROM contents
UNION ALL
SELECT
    'people' as entity_type,
    COUNT(*) as total_count,
    COUNT(CASE WHEN verified = 1 THEN 1 END) as verified,
    0 as with_paper,
    0 as dummy_field
FROM people
UNION ALL
SELECT
    'series' as entity_type,
    COUNT(*) as total_count,
    COUNT(CASE WHEN completed = 1 THEN 1 END) as completed,
    0 as with_paper,
    0 as dummy_field
FROM series;

-- Fase 3: Elimina i trigger esistenti
DROP TRIGGER IF EXISTS update_books_modified_date;

-- Fase 4: Backup dati esistenti in tabelle temporanee
CREATE TABLE books_backup AS
SELECT 
    id, name, original_title, publisher_id, format_id, series_id, series_index,
    publication_date, acquisition_date, last_modified_date, isbn, pages, notes,
    has_cover, has_paper, file_link, file_size, file_hash, created_at
FROM books;

CREATE TABLE contents_backup AS
SELECT 
    id, name, original_title, type_id, publication_date, pages, notes, created_at, updated_at
FROM contents;

-- Fase 5: Elimina le tabelle esistenti con le relative foreign keys
DROP TABLE IF EXISTS books_contents;
DROP TABLE IF EXISTS books_people_roles;
DROP TABLE IF EXISTS books_tags;
DROP TABLE IF EXISTS contents_people_roles;
DROP TABLE IF EXISTS contents_tags;
DROP TABLE IF EXISTS contents_languages;
DROP TABLE IF EXISTS books;
DROP TABLE IF EXISTS contents;

-- Fase 6: Ricrea le tabelle senza i campi rating e read_status
CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    original_title TEXT,
    publisher_id INTEGER,
    format_id INTEGER,
    series_id INTEGER,
    series_index INTEGER CHECK (series_index > 0),
    publication_date INTEGER,
    acquisition_date INTEGER,
    last_modified_date INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    isbn TEXT,
    pages INTEGER CHECK (pages > 0),
    notes TEXT,
    has_cover INTEGER NOT NULL DEFAULT 0 CHECK (has_cover IN (0, 1)),
    has_paper INTEGER NOT NULL DEFAULT 0 CHECK (has_paper IN (0, 1)),
    file_link TEXT UNIQUE,
    file_size INTEGER,
    file_hash TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (format_id) REFERENCES formats(id) ON DELETE SET NULL,
    FOREIGN KEY (series_id) REFERENCES series(id) ON DELETE SET NULL,
    FOREIGN KEY (publisher_id) REFERENCES publishers(id) ON DELETE SET NULL
);

CREATE TABLE contents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    original_title TEXT,
    type_id INTEGER,
    publication_date INTEGER,
    pages INTEGER CHECK (pages > 0),
    notes TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (type_id) REFERENCES types(id) ON DELETE SET NULL
);

-- Fase 7: Ripristina i dati dalle tabelle di backup
INSERT INTO books (
    id, name, original_title, publisher_id, format_id, series_id, series_index,
    publication_date, acquisition_date, last_modified_date, isbn, pages, notes,
    has_cover, has_paper, file_link, file_size, file_hash, created_at
)
SELECT 
    id, name, original_title, publisher_id, format_id, series_id, series_index,
    publication_date, acquisition_date, last_modified_date, isbn, pages, notes,
    has_cover, has_paper, file_link, file_size, file_hash, created_at
FROM books_backup;

INSERT INTO contents (
    id, name, original_title, type_id, publication_date, pages, notes, created_at, updated_at
)
SELECT 
    id, name, original_title, type_id, publication_date, pages, notes, created_at, updated_at
FROM contents_backup;

-- Fase 8: Ricrea le tabelle di relazione
CREATE TABLE books_contents (
    book_id INTEGER NOT NULL,
    content_id INTEGER NOT NULL,
    page_start INTEGER,
    page_end INTEGER,
    PRIMARY KEY (book_id, content_id),
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE
);

CREATE TABLE books_people_roles (
    book_id INTEGER NOT NULL,
    person_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    PRIMARY KEY (book_id, person_id, role_id),
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

CREATE TABLE books_tags (
    book_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (book_id, tag_id),
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE contents_people_roles (
    content_id INTEGER NOT NULL,
    person_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    PRIMARY KEY (content_id, person_id, role_id),
    FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE,
    FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

CREATE TABLE contents_tags (
    content_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (content_id, tag_id),
    FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE contents_languages (
    content_id INTEGER NOT NULL,
    language_id INTEGER NOT NULL,
    PRIMARY KEY (content_id, language_id),
    FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE,
    FOREIGN KEY (language_id) REFERENCES running_languages(id) ON DELETE CASCADE
);

-- Fase 9: Ricrea gli indici
CREATE INDEX idx_books_name_search ON books (name COLLATE NOCASE);
CREATE INDEX idx_contents_name_search ON contents (name COLLATE NOCASE);
CREATE INDEX idx_books_search_optimized ON books (name, publication_date, series_id);
CREATE INDEX idx_contents_search_optimized ON contents (name, type_id, publication_date);
CREATE INDEX idx_books_series_lookup ON books (series_id, series_index);
CREATE INDEX idx_books_metadata ON books (publisher_id, format_id, series_id);
CREATE INDEX idx_books_file_info ON books (file_link, file_size, file_hash) WHERE file_link IS NOT NULL;
CREATE INDEX idx_books_dates_combined ON books (publication_date, acquisition_date, last_modified_date);
CREATE INDEX idx_contents_dates ON contents (publication_date, created_at);
CREATE INDEX idx_books_people_roles_person_role ON books_people_roles (person_id, role_id);
CREATE INDEX idx_books_people_roles_book_lookup ON books_people_roles (book_id, person_id);
CREATE INDEX idx_contents_people_roles_person_role ON contents_people_roles (person_id, role_id);
CREATE INDEX idx_contents_people_roles_content_lookup ON contents_people_roles (content_id, person_id);
CREATE INDEX idx_books_contents_junction ON books_contents (book_id, content_id);
CREATE INDEX idx_books_tags_lookup ON books_tags (book_id, tag_id);
CREATE INDEX idx_contents_tags_lookup ON contents_tags (content_id, tag_id);
CREATE INDEX idx_contents_languages_lookup ON contents_languages (content_id, language_id);
CREATE INDEX idx_contents_languages_by_language ON contents_languages (language_id, content_id);

-- Fase 10: Ricrea i trigger
CREATE TRIGGER update_books_modified_date
    AFTER UPDATE ON books
    FOR EACH ROW
    WHEN NEW.last_modified_date = OLD.last_modified_date
BEGIN
    UPDATE books SET last_modified_date = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- Fase 11: Elimina le tabelle di backup temporanee
DROP TABLE books_backup;
DROP TABLE contents_backup;