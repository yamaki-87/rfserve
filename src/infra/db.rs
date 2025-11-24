use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{
    config::envs,
    domain::{
        download_links::{DownloadLinks, DownloadLinksMapper},
        yt_hisotry::{YtHisotryMapper, YtHistory},
    },
};

#[derive(Clone)]
pub struct DB(pub PgPool);

pub async fn db_init() -> Result<DB> {
    let url = envs::get_instance().get_db_url();
    let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

    Ok(DB(pool))
}

#[derive(Clone)]
pub struct YtHistoryMapperImpl {
    db: DB,
}

impl YtHistoryMapperImpl {
    pub fn new(db: DB) -> Self {
        Self { db: db }
    }
}

impl YtHisotryMapper for YtHistoryMapperImpl {
    async fn is_exist_record(&self, id: i64) -> anyhow::Result<bool> {
        let exist = sqlx::query_scalar::<_, bool>(
            r#"SELECT EXISTS (
                        SELECT 1 FROM yt_history WHERE id = $1
        )"#,
        )
        .bind(id)
        .fetch_one(&self.db.0)
        .await?;

        Ok(exist)
    }

    async fn select_one_with_id(
        &self,
        id: i64,
    ) -> anyhow::Result<Option<crate::domain::yt_hisotry::YtHistory>> {
        let record = sqlx::query_as::<_, YtHistory>(
            r#"
            SELECT id,title FROM yt_history WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db.0)
        .await?;

        Ok(record)
    }
}

#[derive(Clone)]
pub struct DownloadLinksMapperImpl {
    db: DB,
}

impl DownloadLinksMapperImpl {
    pub fn new(db: DB) -> Self {
        Self { db: db }
    }
}

impl DownloadLinksMapper for DownloadLinksMapperImpl {
    async fn insert(
        &self,
        entity: crate::domain::download_links::DownloadLinks,
    ) -> anyhow::Result<()> {
        let mut tx = self.db.0.begin().await?;
        sqlx::query(
            r#"
        INSERT INTO downloadlinks 
        (id,yt_history_id,url,object_path,expires_at,created_at) 
        VALUES ($1,$2,$3,$4,$5,$6)
        "#,
        )
        .bind(entity.id)
        .bind(entity.yt_history_id)
        .bind(entity.url)
        .bind(entity.object_path)
        .bind(entity.expires_at)
        .bind(entity.created_at)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn select_by_id(
        &self,
        uuid: uuid::Uuid,
    ) -> anyhow::Result<Option<crate::domain::download_links::DownloadLinks>> {
        let record = sqlx::query_as::<_, DownloadLinks>(
            r#"
            select * from downloadlinks where id = $1
            "#,
        )
        .bind(uuid)
        .fetch_optional(&self.db.0)
        .await?;

        Ok(record)
    }
}
