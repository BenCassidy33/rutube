use crate::db::{self};
use crate::models::HomeVideo;
use crate::{models, AppState};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};

use std::sync::Arc;

pub async fn get_home(State(app_state): State<Arc<AppState>>) -> Html<String> {
    let videos: Vec<models::Video> = db::fetch_random_videos(10, &app_state.pool).await.unwrap();
    let rendered: Vec<String> = videos
        .iter()
        .map(move |raw_video| {
            let home_video = HomeVideo {
                id: raw_video.id.clone(),
                title: raw_video.title.clone(),
                author_name: raw_video.author_name.clone(),
                thumbnail_url: raw_video.thumbnail_url.clone(),
                created_at: raw_video.created_at,
            };
            home_video.render().unwrap()
        })
        .collect();

    return Html(rendered.join("\n"));
}

pub async fn get_display_video(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Html<String> {
    println!("id: {}", id);
    let video: models::Video = db::fetch_video_by_id(id, &app_state.pool).await.unwrap();

    return Html(video.render().unwrap());
}
