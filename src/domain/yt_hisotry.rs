use anyhow::Result;
use sqlx::prelude::FromRow;

#[derive(Default, FromRow)]
pub struct YtHistory {
    pub id: i32,
    pub title: String,
    pub app_external_id: String,
}

pub trait YtHisotryMapper {
    async fn is_exist_record(&self, id: i64) -> Result<bool>;
    async fn select_one_with_id(&self, id: i64) -> Result<Option<YtHistory>>;
}
