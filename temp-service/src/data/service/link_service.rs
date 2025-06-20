use std::error::Error;
use sqlx::{PgPool, Row};
use crate::data::models::link::Link;

#[derive(Clone)]
pub struct LinkService {
    pg_pool: PgPool,
}

impl LinkService {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }

    // returns id of the newly created link
    pub async fn create(self: &Self, mut link: Link) -> Result<Link, Box<dyn Error>> {
        let id: i32 = sqlx::query_scalar("INSERT INTO link (name, description, content) VALUES ($1, $2, $3) RETURNING id")
            .bind(&link.name)
            .bind(&link.description)
            .bind(&link.content)
            .fetch_optional(&self.pg_pool)
            .await?
            .ok_or("newly saved link did not returned an id")?;
        link.id = id;
        Ok(link)
    }

    // gets an encrypted link from its id
    pub async fn read(self: &Self, link_id: i32) -> Result<Link, Box<dyn Error>> {
        let row = sqlx::query("SELECT id, name, description, content, created_at FROM link WHERE id = $1")
            .bind(link_id)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(Link::new(
            row.get("id"),
            row.get("name"),
            row.get("description"),
            row.get("content"),
            row.get("created_at"),
        ))
    }
}