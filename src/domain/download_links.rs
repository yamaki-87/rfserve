use chrono::FixedOffset;
use derive_builder::Builder;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Builder, FromRow, Debug)]
pub struct DownloadLinks {
    pub id: Uuid,
    pub yt_history_id: i32,
    pub url: String,
    pub object_path: String,
    pub expires_at: chrono::DateTime<FixedOffset>,
    pub created_at: chrono::DateTime<FixedOffset>,
}

pub trait DownloadLinksMapper {
    async fn insert(&self, entity: DownloadLinks) -> anyhow::Result<()>;
    async fn select_by_id(&self, uuid: Uuid) -> anyhow::Result<Option<DownloadLinks>>;
}
