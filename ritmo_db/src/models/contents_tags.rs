use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct ContentTag {
    pub content_id: i64,
    pub tag_id: i64,
}

//#[derive(Debug)]
//pub struct NewContentTag {
//    pub content_id: i64,
//    pub tag_id: i64,
//}

impl ContentTag {
    pub async fn create(pool: &sqlx::SqlitePool, new_link: &ContentTag) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO contents_tags (content_id, tag_id) VALUES (?, ?)"
        )
        .bind(new_link.content_id)
        .bind(new_link.tag_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get(pool: &sqlx::SqlitePool, content_id: i64, tag_id: i64) -> Result<Option<ContentTag>, sqlx::Error> {
        let link = sqlx::query_as::<_, ContentTag>(
            "SELECT * FROM contents_tags WHERE content_id = ? AND tag_id = ?"
        )
        .bind(content_id)
        .bind(tag_id)
        .fetch_optional(pool)
        .await?;
        Ok(link)
    }

    pub async fn delete(pool: &sqlx::SqlitePool, content_id: i64, tag_id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM contents_tags WHERE content_id = ? AND tag_id = ?"
        )
        .bind(content_id)
        .bind(tag_id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_by_content(pool: &sqlx::SqlitePool, content_id: i64) -> Result<Vec<ContentTag>, sqlx::Error> {
        let links = sqlx::query_as::<_, ContentTag>(
            "SELECT * FROM contents_tags WHERE content_id = ?"
        )
        .bind(content_id)
        .fetch_all(pool)
        .await?;
        Ok(links)
    }

    pub async fn list_by_tag(pool: &sqlx::SqlitePool, tag_id: i64) -> Result<Vec<ContentTag>, sqlx::Error> {
        let links = sqlx::query_as::<_, ContentTag>(
            "SELECT * FROM contents_tags WHERE tag_id = ?"
        )
        .bind(tag_id)
        .fetch_all(pool)
        .await?;
        Ok(links)
    }
}