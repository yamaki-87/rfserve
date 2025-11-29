use std::path::PathBuf;

use crate::{
    config::envs,
    domain::{
        download_links::{DownloadLinks, DownloadLinksBuilder, DownloadLinksMapper},
        yt_hisotry::{YtHisotryMapper, YtHistory},
    },
};
use anyhow::{Result, anyhow};
use chrono::{Days, Local};
use uuid::Uuid;
pub struct RegisterService<T: YtHisotryMapper, S: DownloadLinksMapper> {
    yt_mapper: T,
    download_links_mapper: S,
}
pub struct RegisterServiceOutput {
    pub url: String,
}

impl<T: YtHisotryMapper, S: DownloadLinksMapper> RegisterService<T, S> {
    pub fn new(yt_mapper: T, download_links_mapper: S) -> Self {
        Self {
            yt_mapper: yt_mapper,
            download_links_mapper: download_links_mapper,
        }
    }

    fn data_check(&self, record: &Option<YtHistory>, yt_history_id: i64) -> Result<()> {
        if record.is_none() {
            return Err(anyhow!(
                "指定されたIDが存在しません。 id = {}",
                yt_history_id
            ));
        }
        let is_external_id_emp = record
            .as_ref()
            .map(|e| e.app_external_id.trim().is_empty())
            .unwrap_or(true);
        if is_external_id_emp {
            return Err(anyhow!("外部キーが空文字です。 id = {}", yt_history_id));
        }

        Ok(())
    }

    pub async fn execute(&self, yt_history_id: i64) -> Result<RegisterServiceOutput> {
        let record = self.yt_mapper.select_one_with_id(yt_history_id).await?;
        self.data_check(&record, yt_history_id)?;

        let insert = self.create_download_links_entity(record)?;
        let output = RegisterServiceOutput {
            url: insert.url.clone(),
        };
        self.download_links_mapper.insert(insert).await?;
        Ok(output)
    }

    fn create_download_links_entity(&self, record: Option<YtHistory>) -> Result<DownloadLinks> {
        let record = record.unwrap_or_default();
        let env = envs::get_instance();

        let now = Local::now().fixed_offset();
        let expired = now
            .checked_add_days(Days::new(env.get_expred_store_days()))
            .ok_or(anyhow!(format!("expired処理にて加算処理失敗")))?;

        let uuid = Uuid::new_v4();
        let url = self.create_url(&uuid);

        let result = DownloadLinksBuilder::default()
            .id(uuid)
            .yt_history_id(record.id)
            .url(url)
            .created_at(now)
            .expires_at(expired)
            .object_path(record.app_external_id)
            .build()?;

        Ok(result)
    }

    fn create_url(&self, uuid: &Uuid) -> String {
        let env = envs::get_instance();
        format!(
            "http://{}:{}/rfserve/download/{}",
            env.get_hostname(),
            env.get_addr().port(),
            uuid
        )
    }
}
