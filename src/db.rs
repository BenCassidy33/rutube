use crate::models::Video;

use sqlx::postgres::PgPool;

pub async fn fetch_video_by_id(id: i32, pool: &PgPool) -> Result<Video, sqlx::Error> {
    Ok(
        sqlx::query_as::<_, Video>("SELECT * FROM videos WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap(),
    )
}

pub async fn fetch_all_videos(pool: &PgPool) -> Result<Vec<Video>, sqlx::Error> {
    Ok(sqlx::query_as::<_, Video>("SELECT * FROM videos")
        .fetch_all(pool)
        .await
        .unwrap())
}

pub async fn fetch_random_videos(amount: i32, pool: &PgPool) -> Result<Vec<Video>, sqlx::Error> {
    Ok(
        sqlx::query_as::<_, Video>("SELECT * FROM videos ORDER BY RANDOM() LIMIT $1")
            .bind(amount)
            .fetch_all(pool)
            .await
            .unwrap(),
    )
}

pub async fn insert_video(video: Video, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO videos (id, title, description, video_url, thumbnail_url, author_name, created_at, likes, dislikes) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
        .bind(video.id)
        .bind(video.title)
        .bind(video.description)
        .bind(video.video_url)
        .bind(video.thumbnail_url)
        .bind(video.author_name)
        .bind(video.created_at)
        .bind(video.likes)
        .bind(video.dislikes)
        .execute(pool)
        .await?
    ;

    Ok(())
}
