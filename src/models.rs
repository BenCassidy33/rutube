use askama::Template;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, FromRow};

pub enum FetchBy {
    Id(i32),
    Title(String),
    Description(String),
    Url(String),
    Thumbnail(String),
    Author(String),
    CreatedAt(chrono::DateTime<chrono::Utc>),
}

#[derive(FromRow, Serialize, Deserialize, Debug, Template)]
#[template(path = "viewer.html")]
pub struct Video {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub video_url: String,
    pub thumbnail_url: String,
    pub author_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub likes: i32,
    pub dislikes: i32,
}
#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeVideo {
    pub id: i32,
    pub title: String,
    pub author_name: String,
    pub thumbnail_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
