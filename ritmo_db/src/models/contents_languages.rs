use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct ContentLanguage {
    pub content_id: i64,
    pub language_id: i64,
}

#[derive(Debug)]
pub struct NewContentLanguage {
    pub content_id: i64,
    pub language_id: i64,
}

impl ContentLanguage {
    pub async fn create(pool: &sqlx::SqlitePool, new_link: &NewContentLanguage) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO contents_languages (content_id, language_id) VALUES (?, ?)"
        )
        .bind(new_link.content_id)
        .bind(new_link.language_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get(pool: &sqlx::SqlitePool, content_id: i64, language_id: i64) -> Result<Option<ContentLanguage>, sqlx::Error> {
        let link = sqlx::query_as::<_, ContentLanguage>(
            "SELECT * FROM contents_languages WHERE content_id = ? AND language_id = ?"
        )
        .bind(content_id)
        .bind(language_id)
        .fetch_optional(pool)
        .await?;
        Ok(link)
    }

    pub async fn delete(pool: &sqlx::SqlitePool, content_id: i64, language_id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM contents_languages WHERE content_id = ? AND language_id = ?"
        )
        .bind(content_id)
        .bind(language_id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_by_content(pool: &sqlx::SqlitePool, content_id: i64) -> Result<Vec<ContentLanguage>, sqlx::Error> {
        let links = sqlx::query_as::<_, ContentLanguage>(
            "SELECT * FROM contents_languages WHERE content_id = ?"
        )
        .bind(content_id)
        .fetch_all(pool)
        .await?;
        Ok(links)
    }

    pub async fn list_by_language(pool: &sqlx::SqlitePool, language_id: i64) -> Result<Vec<ContentLanguage>, sqlx::Error> {
        let links = sqlx::query_as::<_, ContentLanguage>(
            "SELECT * FROM contents_languages WHERE language_id = ?"
        )
        .bind(language_id)
        .fetch_all(pool)
        .await?;
        Ok(links)
    }
}