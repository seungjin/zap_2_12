use anyhow::{Error, Result};
use openssh::Session;
use std::sync::Arc;

pub struct Uptime;

impl Uptime {
    pub async fn exec(s: Arc<Session>) -> Result<String> {
        let uptime = s.arc_raw_command("uptime").output().await.unwrap();

        if !uptime.status.success() {
            let stderr = String::from_utf8(uptime.stderr).unwrap();
            return Err(Error::msg(stderr));
        };

        let stdout = match String::from_utf8(uptime.stdout) {
            Ok(a) => a,
            Err(e) => return Err(Error::msg(format!("{:?}", e))),
        };
        Ok(stdout)
    }
}
