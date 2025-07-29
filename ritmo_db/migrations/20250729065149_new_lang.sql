-- Elimina le tabelle non più necessarie
DROP TABLE IF EXISTS contents_languages;
DROP TABLE IF EXISTS languages_names;
DROP TABLE IF EXISTS languages_roles;
DROP TABLE IF EXISTS running_languages;

-- Crea la nuova tabella running_languages con la struttura migliorata
CREATE TABLE running_languages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    iso_code_2char TEXT NOT NULL,
    iso_code_3char TEXT NOT NULL,
    official_name TEXT NOT NULL,
    language_role TEXT NOT NULL CHECK (language_role IN ('Original', 'Source', 'Actual')),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(iso_code_2char, iso_code_3char, language_role)
);

-- Crea la nuova tabella di relazione tra contents e languages
CREATE TABLE contents_languages (
    content_id INTEGER NOT NULL,
    language_id INTEGER NOT NULL,
    PRIMARY KEY (content_id, language_id),
    FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE,
    FOREIGN KEY (language_id) REFERENCES running_languages(id) ON DELETE CASCADE
);

-- Indici per ottimizzare le query
CREATE INDEX idx_running_languages_codes ON running_languages (iso_code_2char, iso_code_3char);
CREATE INDEX idx_running_languages_role ON running_languages (language_role);
CREATE INDEX idx_contents_languages_lookup ON contents_languages (content_id, language_id);
CREATE INDEX idx_contents_languages_by_language ON contents_languages (language_id, content_id);

-- Trigger per aggiornare il timestamp updated_at quando la lingua viene modificata
CREATE TRIGGER update_running_languages_timestamp
    AFTER UPDATE ON running_languages
    FOR EACH ROW
BEGIN
    UPDATE running_languages SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- Aggiorna le viste esistenti che potrebbero riferirsi alle vecchie tabelle delle lingue

-- Aggiorna la vista ContentsFullDetails per includere le nuove informazioni sulle lingue
DROP VIEW IF EXISTS ContentsFullDetails;
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

