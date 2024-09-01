use anyhow::{Error, Result};
use openssh::Session;
use std::sync::Arc;

use crate::server::Server;

pub struct Base;

impl Base {
    pub async fn uptime(s: Arc<Session>) -> Result<String> {
        let uptime = s.arc_raw_command("uptime").output().await.unwrap();

        if uptime.stderr.len() != 0 {
            return Err(Error::msg("Commend returns err!"));
        };

        let a = match String::from_utf8(uptime.stdout) {
            Ok(a) => a,
            Err(e) => return Err(Error::msg(format!("{:?}", e))),
        };
        Ok(a)
    }
}
