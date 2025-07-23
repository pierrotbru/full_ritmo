-- PRAGMA journal_mode = WAL;
-- PRAGMA synchronous = NORMAL;
-- PRAGMA cache_size = 10000;
-- PRAGMA foreign_keys = ON;
-- PRAGMA temp_store = MEMORY;
-- PRAGMA auto_vacuum = INCREMENTAL;

-- BEGIN TRANSACTION;

CREATE TABLE system_config (
    key TEXT PRIMARY KEY,
    value TEXT,
    description TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_name TEXT NOT NULL,
    record_id INTEGER NOT NULL,
    operation TEXT NOT NULL CHECK (operation IN ('INSERT', 'UPDATE', 'DELETE')),
    old_values TEXT,
    new_values TEXT,
    timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    user_id TEXT
);

CREATE TABLE stats_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cache_key TEXT UNIQUE NOT NULL,
    cache_value TEXT NOT NULL,
    expires_at INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE formats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE publishers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    country TEXT,
    website TEXT,
    notes TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE series (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    total_books INTEGER,
    completed INTEGER NOT NULL DEFAULT 0 CHECK (completed IN (0, 1)),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    color TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE languages_names (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    iso_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    iso_code_2char TEXT,
    native_name TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE languages_roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE running_languages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    iso_code TEXT,
    role INTEGER,
    FOREIGN KEY (iso_code) REFERENCES languages_names(iso_code) ON DELETE SET NULL,
    FOREIGN KEY (role) REFERENCES languages_roles(id) ON DELETE SET NULL
);

CREATE TABLE people (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    display_name TEXT,
    given_name TEXT,
    surname TEXT,
    middle_names TEXT,
    title TEXT,
    suffix TEXT,
    nationality TEXT,
    birth_date INTEGER,
    death_date INTEGER,
    biography TEXT,
    normalized_key TEXT,
    confidence REAL NOT NULL DEFAULT 1.0 CHECK (confidence >= 0.0 AND confidence <= 1.0),
    source TEXT NOT NULL DEFAULT 'biblioteca',
    verified INTEGER NOT NULL DEFAULT 0 CHECK (verified IN (0, 1)),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE aliases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    person_id INTEGER NOT NULL,
    alias_normalized TEXT,
    confidence REAL NOT NULL DEFAULT 0.9 CHECK (confidence >= 0.0 AND confidence <= 1.0),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE,
    UNIQUE(person_id, name)
);

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
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    read_status TEXT CHECK (read_status IN ('unread', 'reading', 'read', 'abandoned')),
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
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (type_id) REFERENCES types(id) ON DELETE SET NULL
);

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
    contents_id INTEGER NOT NULL,
    languages_id INTEGER NOT NULL,
    PRIMARY KEY (contents_id, languages_id),
    FOREIGN KEY (contents_id) REFERENCES contents(id) ON DELETE CASCADE,
    FOREIGN KEY (languages_id) REFERENCES running_languages(id) ON DELETE CASCADE
);

CREATE TABLE ml_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    data_type TEXT NOT NULL UNIQUE,
    data_json TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX idx_books_name_search ON books (name COLLATE NOCASE);
CREATE INDEX idx_contents_name_search ON contents (name COLLATE NOCASE);
CREATE INDEX idx_people_name_search ON people (name COLLATE NOCASE);
CREATE INDEX idx_series_name_search ON series (name COLLATE NOCASE);
CREATE INDEX idx_publishers_name_search ON publishers (name COLLATE NOCASE);
CREATE INDEX idx_tags_name_search ON tags (name COLLATE NOCASE);

CREATE INDEX idx_books_search_optimized ON books (name, publication_date, series_id);
CREATE INDEX idx_contents_search_optimized ON contents (name, type_id, publication_date);
CREATE INDEX idx_books_series_lookup ON books (series_id, series_index);
CREATE INDEX idx_books_metadata ON books (publisher_id, format_id, series_id);

CREATE INDEX idx_books_people_roles_person_role ON books_people_roles (person_id, role_id);
CREATE INDEX idx_books_people_roles_book_lookup ON books_people_roles (book_id, person_id);
CREATE INDEX idx_contents_people_roles_person_role ON contents_people_roles (person_id, role_id);
CREATE INDEX idx_contents_people_roles_content_lookup ON contents_people_roles (content_id, person_id);
CREATE INDEX idx_books_contents_junction ON books_contents (book_id, content_id);
CREATE INDEX idx_books_tags_lookup ON books_tags (book_id, tag_id);
CREATE INDEX idx_contents_tags_lookup ON contents_tags (content_id, tag_id);

CREATE INDEX idx_books_dates_combined ON books (publication_date, acquisition_date, last_modified_date);
CREATE INDEX idx_contents_dates ON contents (publication_date, created_at);
CREATE INDEX idx_people_dates ON people (birth_date, death_date);

CREATE INDEX idx_people_normalized_search ON people (normalized_key COLLATE NOCASE) WHERE normalized_key IS NOT NULL;
CREATE INDEX idx_aliases_normalized_search ON aliases (alias_normalized COLLATE NOCASE) WHERE alias_normalized IS NOT NULL;
CREATE INDEX idx_aliases_person_lookup ON aliases (person_id, name);

CREATE INDEX idx_audit_log_lookup ON audit_log (table_name, record_id, timestamp);
CREATE INDEX idx_audit_log_timestamp ON audit_log (timestamp);
CREATE INDEX idx_stats_cache_key ON stats_cache (cache_key);
CREATE INDEX idx_stats_cache_expires ON stats_cache (expires_at);

CREATE INDEX idx_books_file_info ON books (file_link, file_size, file_hash) WHERE file_link IS NOT NULL;
CREATE INDEX idx_books_rating ON books (rating) WHERE rating IS NOT NULL;
CREATE INDEX idx_books_read_status ON books (read_status) WHERE read_status IS NOT NULL;

CREATE TRIGGER update_books_modified_date
    AFTER UPDATE ON books
    FOR EACH ROW
    WHEN NEW.last_modified_date = OLD.last_modified_date
BEGIN
    UPDATE books SET last_modified_date = strftime('%s', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER normalize_person_name
    BEFORE INSERT ON people
    FOR EACH ROW
    WHEN NEW.normalized_key IS NULL
BEGIN
    UPDATE people SET normalized_key = LOWER(TRIM(NEW.name)) WHERE id = NEW.id;
END;

CREATE TRIGGER update_people_timestamp
    AFTER UPDATE ON people
    FOR EACH ROW
BEGIN
    UPDATE people SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER update_contents_timestamp
    AFTER UPDATE ON contents
    FOR EACH ROW
BEGIN
    UPDATE contents SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER update_series_timestamp
    AFTER UPDATE ON series
    FOR EACH ROW
BEGIN
    UPDATE series SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER update_publishers_timestamp
    AFTER UPDATE ON publishers
    FOR EACH ROW
BEGIN
    UPDATE publishers SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER normalize_alias_name
    BEFORE INSERT ON aliases
    FOR EACH ROW
    WHEN NEW.alias_normalized IS NULL
BEGIN
    UPDATE aliases SET alias_normalized = LOWER(TRIM(NEW.name)) WHERE id = NEW.id;
END;

CREATE TRIGGER update_config_timestamp
    AFTER UPDATE ON system_config
    FOR EACH ROW
BEGIN
    UPDATE system_config SET updated_at = strftime('%s', 'now') WHERE key = NEW.key;
END;

CREATE TRIGGER audit_books_insert
    AFTER INSERT ON books
    FOR EACH ROW
BEGIN
    INSERT INTO audit_log (table_name, record_id, operation, new_values)
    VALUES ('books', NEW.id, 'INSERT', 
            json_object('name', NEW.name, 'publication_date', NEW.publication_date, 'series_id', NEW.series_id));
END;

CREATE TRIGGER audit_books_update
    AFTER UPDATE ON books
    FOR EACH ROW
BEGIN
    INSERT INTO audit_log (table_name, record_id, operation, old_values, new_values)
    VALUES ('books', NEW.id, 'UPDATE',
            json_object('name', OLD.name, 'publication_date', OLD.publication_date, 'series_id', OLD.series_id),
            json_object('name', NEW.name, 'publication_date', NEW.publication_date, 'series_id', NEW.series_id));
END;

CREATE TRIGGER audit_books_delete
    AFTER DELETE ON books
    FOR EACH ROW
BEGIN
    INSERT INTO audit_log (table_name, record_id, operation, old_values)
    VALUES ('books', OLD.id, 'DELETE',
            json_object('name', OLD.name, 'publication_date', OLD.publication_date, 'series_id', OLD.series_id));
END;

CREATE TRIGGER audit_people_insert
    AFTER INSERT ON people
    FOR EACH ROW
BEGIN
    INSERT INTO audit_log (table_name, record_id, operation, new_values)
    VALUES ('people', NEW.id, 'INSERT', 
            json_object('name', NEW.name, 'nationality', NEW.nationality, 'verified', NEW.verified));
END;

CREATE TRIGGER audit_people_update
    AFTER UPDATE ON people
    FOR EACH ROW
BEGIN
    INSERT INTO audit_log (table_name, record_id, operation, old_values, new_values)
    VALUES ('people', NEW.id, 'UPDATE',
            json_object('name', OLD.name, 'nationality', OLD.nationality, 'verified', OLD.verified),
            json_object('name', NEW.name, 'nationality', NEW.nationality, 'verified', NEW.verified));
END;

CREATE TRIGGER audit_people_delete
    AFTER DELETE ON people
    FOR EACH ROW
BEGIN
    INSERT INTO audit_log (table_name, record_id, operation, old_values)
    VALUES ('people', OLD.id, 'DELETE',
            json_object('name', OLD.name, 'nationality', OLD.nationality, 'verified', OLD.verified));
END;

CREATE VIEW BooksSearchOptimized AS
SELECT 
    b.id,
    b.name,
    b.original_title,
    b.publication_date,
    b.series_index,
    b.isbn,
    b.pages,
    b.rating,
    b.read_status,
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

CREATE VIEW ContentsSearchOptimized AS
SELECT 
    c.id,
    c.name,
    c.original_title,
    c.publication_date,
    c.pages,
    c.rating,
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

CREATE VIEW BooksFullDetails AS
SELECT
    b.id AS book_id,
    b.name AS book_name,
    b.original_title,
    b.publication_date,
    b.acquisition_date,
    b.last_modified_date,
    b.isbn,
    b.pages,
    b.rating,
    b.read_status,
    b.has_cover,
    b.has_paper,
    b.file_link,
    pub.name AS publisher_name,
    f.name AS format_name,
    s.name AS series_name,
    b.series_index,
    b.notes AS book_notes,
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

CREATE VIEW ContentsFullDetails AS
SELECT
    c.id AS content_id,
    c.name AS content_name,
    c.original_title,
    c.publication_date,
    c.pages,
    c.rating,
    c.notes AS content_notes,
    t.name AS type_name,
    GROUP_CONCAT(DISTINCT p.id) AS person_ids,
    GROUP_CONCAT(DISTINCT p.name) AS person_names,
    GROUP_CONCAT(DISTINCT r.name) AS role_names,
    GROUP_CONCAT(DISTINCT tag.name) AS tag_names,
    GROUP_CONCAT(DISTINCT ln.name) AS language_names,
    GROUP_CONCAT(DISTINCT lr.name) AS language_roles
FROM contents c
LEFT JOIN types t ON c.type_id = t.id
LEFT JOIN contents_people_roles cpr ON c.id = cpr.content_id
LEFT JOIN people p ON cpr.person_id = p.id
LEFT JOIN roles r ON cpr.role_id = r.id
LEFT JOIN contents_tags ct ON c.id = ct.content_id
LEFT JOIN tags tag ON ct.tag_id = tag.id
LEFT JOIN contents_languages cl ON c.id = cl.contents_id
LEFT JOIN running_languages rl ON cl.languages_id = rl.id
LEFT JOIN languages_names ln ON rl.iso_code = ln.iso_code
LEFT JOIN languages_roles lr ON rl.role = lr.id
GROUP BY c.id;

CREATE VIEW PeopleMatchingOptimized AS
SELECT 
    p.id,
    p.name,
    p.display_name,
    p.given_name,
    p.surname,
    p.middle_names,
    p.normalized_key,
    p.confidence,
    p.nationality,
    p.birth_date,
    p.death_date,
    p.source,
    p.verified,
    p.created_at,
    p.updated_at,
    GROUP_CONCAT(a.name, '; ') as aliases,
    COUNT(a.id) as alias_count
FROM people p
LEFT JOIN aliases a ON p.id = a.person_id
WHERE p.normalized_key IS NOT NULL
GROUP BY p.id;

CREATE VIEW LibraryStats AS
SELECT 
    'books' as entity_type,
    COUNT(*) as total_count,
    COUNT(CASE WHEN has_cover = 1 THEN 1 END) as with_cover,
    COUNT(CASE WHEN has_paper = 1 THEN 1 END) as with_paper,
    COUNT(CASE WHEN rating IS NOT NULL THEN 1 END) as rated,
    ROUND(AVG(rating), 2) as avg_rating,
    COUNT(CASE WHEN read_status = 'read' THEN 1 END) as read_count,
    COUNT(CASE WHEN read_status = 'reading' THEN 1 END) as reading_count,
    COUNT(CASE WHEN read_status = 'unread' THEN 1 END) as unread_count
FROM books
UNION ALL
SELECT 
    'contents' as entity_type,
    COUNT(*) as total_count,
    0 as with_cover,
    0 as with_paper,
    COUNT(CASE WHEN rating IS NOT NULL THEN 1 END) as rated,
    ROUND(AVG(rating), 2) as avg_rating,
    0 as read_count,
    0 as reading_count,
    0 as unread_count
FROM contents
UNION ALL
SELECT 
    'people' as entity_type,
    COUNT(*) as total_count,
    COUNT(CASE WHEN verified = 1 THEN 1 END) as verified,
    0 as with_paper,
    0 as rated,
    0 as avg_rating,
    0 as read_count,
    0 as reading_count,
    0 as unread_count
FROM people
UNION ALL
SELECT 
    'series' as entity_type,
    COUNT(*) as total_count,
    COUNT(CASE WHEN completed = 1 THEN 1 END) as completed,
    0 as with_paper,
    0 as rated,
    0 as avg_rating,
    0 as read_count,
    0 as reading_count,
    0 as unread_count
FROM series;

CREATE VIEW PossibleDuplicates AS
SELECT 
    p1.id as person1_id,
    p1.name as person1_name,
    p1.confidence as confidence1,
    p2.id as person2_id,
    p2.name as person2_name,
    p2.confidence as confidence2,
    p1.created_at as created1,
    p2.created_at as created2
FROM people p1
JOIN people p2 ON p1.normalized_key = p2.normalized_key
WHERE p1.id < p2.id
  AND p1.normalized_key IS NOT NULL
  AND p2.normalized_key IS NOT NULL
  AND p1.normalized_key != '';

CREATE VIEW BooksWithoutAuthor AS
SELECT 
    b.id, 
    b.name, 
    b.publication_date,
    s.name as series_name,
    b.created_at
FROM books b
LEFT JOIN series s ON b.series_id = s.id
WHERE b.id NOT IN (
    SELECT DISTINCT book_id 
    FROM books_people_roles bpr
    JOIN roles r ON bpr.role_id = r.id
    WHERE r.name IN ('Autore', 'Author', 'Scrittore')
);

CREATE VIEW ContentsWithoutAuthor AS
SELECT 
    c.id, 
    c.name, 
    c.publication_date,
    t.name as type_name,
    c.created_at
FROM contents c
LEFT JOIN types t ON c.type_id = t.id
WHERE c.id NOT IN (
    SELECT DISTINCT content_id 
    FROM contents_people_roles cpr
    JOIN roles r ON cpr.role_id = r.id
    WHERE r.name IN ('Autore', 'Author', 'Scrittore')
);

