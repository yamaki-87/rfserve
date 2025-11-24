use std::sync::Arc;

use crate::{
    domain::{download_links::DownloadLinksMapper, yt_hisotry::YtHisotryMapper},
    infra::db::{DB, DownloadLinksMapperImpl, YtHistoryMapperImpl},
    service::{download_service::DownloadService, register_service::RegisterService},
};

#[derive(Clone)]
pub struct AppState<T: YtHisotryMapper, S: DownloadLinksMapper> {
    pub download_service: Arc<DownloadService<S>>,
    pub register_service: Arc<RegisterService<T, S>>,
}

impl AppState<YtHistoryMapperImpl, DownloadLinksMapperImpl> {
    pub fn new(db: DB) -> Self {
        let yt_history_mapper = YtHistoryMapperImpl::new(db.clone());
        let download_links_mapper = DownloadLinksMapperImpl::new(db);

        let download_service = DownloadService::new(download_links_mapper.clone());
        let register_service = RegisterService::new(yt_history_mapper, download_links_mapper);

        Self {
            download_service: Arc::new(download_service),
            register_service: Arc::new(register_service),
        }
    }
}
