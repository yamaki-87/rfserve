use std::{net::SocketAddr, sync::OnceLock};

pub struct Envs {
    addr: SocketAddr,
    db_url: String,
    video_root: String,
    expired_store_days: u64,
    host_name: String,
}

impl Envs {
    fn new() -> Self {
        Self {
            addr: env_unwarp("ADDR").parse().expect("failed parse addr"),
            db_url: env_unwarp("DATABASE_URL"),
            video_root: env_unwarp("VIDEO_ROOT"),
            expired_store_days: env_unwarp("STORE_DAYS")
                .parse()
                .expect("failed to parse days"),
            host_name: env_unwarp("HOST_NAME"),
        }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }
    pub fn get_db_url(&self) -> &str {
        &self.db_url
    }

    pub fn get_expred_store_days(&self) -> u64 {
        self.expired_store_days
    }

    pub fn get_video_root(&self) -> &str {
        &self.video_root
    }

    pub fn get_hostname(&self) -> &str {
        &self.host_name
    }
}

static INSTNACE: OnceLock<Envs> = OnceLock::new();

pub fn get_instance() -> &'static Envs {
    INSTNACE.get_or_init(|| Envs::new())
}

fn env_unwarp(key: &str) -> String {
    std::env::var(key).unwrap()
}
