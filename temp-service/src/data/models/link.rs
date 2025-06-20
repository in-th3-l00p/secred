use std::str::FromStr;

pub struct Link {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
}

impl Link {
    pub fn new(
        id: i32,
        name: String,
        description: String,
        content: String,
        created_at: String
    ) -> Self {
        Self {
            id, name, description, content,
            created_at: chrono::DateTime::from_str(created_at.as_str())
                .unwrap_or(Default::default())
        }
    }
}