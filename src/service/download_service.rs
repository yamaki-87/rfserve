use anyhow::{Result, anyhow};
use chrono::Local;
use std::{ffi::OsStr, path::PathBuf};
use uuid::Uuid;

use crate::{
    config::envs,
    domain::download_links::{DownloadLinks, DownloadLinksMapper},
};

pub struct DownloadService<T: DownloadLinksMapper> {
    download_links_mapper: T,
}
pub struct DownloadServiceOutput {
    pub file: tokio::fs::File,
    pub fname: String,
}

impl<T: DownloadLinksMapper> DownloadService<T> {
    pub fn new(download_links_mapper: T) -> Self {
        Self {
            download_links_mapper: download_links_mapper,
        }
    }
    pub async fn execute(&self, fileid: String) -> Result<DownloadServiceOutput> {
        let uuid = match Uuid::parse_str(&fileid) {
            Ok(u) => u,
            Err(e) => {
                return Err(anyhow!(format!(
                    "指定された idは不正です。 id:{} {}",
                    &fileid, e
                )));
            }
        };

        let record = match self.download_links_mapper.select_by_id(uuid).await? {
            Some(r) => r,
            None => {
                return Err(anyhow!(format!("recordが見つかりません。 id:{}", &fileid)));
            }
        };
        let is_download_ok = self.check_record_expired(&record);
        if !is_download_ok {
            return Err(anyhow!("対象期間が過ぎています。 {:?}", &record));
        }

        let path = self.file_search(&record).await?;

        let file = tokio::fs::File::open(&path).await?;
        Ok(DownloadServiceOutput {
            file: file,
            fname: path
                .file_name()
                .unwrap_or(OsStr::new("unknow"))
                .to_str()
                .unwrap_or_default()
                .to_string(),
        })
    }

    fn check_record_expired(&self, record: &DownloadLinks) -> bool {
        let now = Local::now().fixed_offset();
        record.created_at <= now && now <= record.expires_at
    }

    async fn file_search(&self, record: &DownloadLinks) -> Result<PathBuf> {
        let envs = envs::get_instance();
        let root = PathBuf::from(envs.get_video_root());
        let mut dir = tokio::fs::read_dir(&root).await?;

        let mut result_path = None;
        while let Some(entry) = dir.next_entry().await? {
            let fname = entry
                .file_name()
                .into_string()
                .map_err(|e| anyhow!("String 変換失敗 変換元文字列：{:?}", e))?;

            if fname.contains(&record.object_path) {
                result_path = Some(entry.path());
                break;
            }
        }

        match result_path {
            Some(v) => Ok(v),
            None => Err(anyhow!(
                "対象のファイルが見つかりませんでした。 対象レコード:{:?}",
                record
            )),
        }
    }
}
