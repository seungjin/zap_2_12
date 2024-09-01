use anyhow::{Error, Result};
use openssh::Session;
use std::sync::Arc;

use crate::server::Server;

pub struct Base;

impl Base {
    pub async fn reboot(s: Arc<Session>, server: &'static str) -> Result<Arc<Session>> {
        println!("REBOOTING");
        s.clone()
            .arc_raw_command("sudo")
            .arg("systemctl")
            .arg("reboot")
            .output()
            .await
            .unwrap();
        Server::wait_until_down(s).await;
        println!("Server is rebooted.");
        let s1 = Server::check(0, server).await;
        let uptime = Base::uptime(s1.clone()).await;
        println!("Server is back.\nUptime: {}", uptime.unwrap());
        Ok(s1)
    }

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
