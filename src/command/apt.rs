use anyhow::{Error, Result};
use openssh::Session;
use std::str;
use std::string::String;
use std::sync::Arc;

pub struct Apt;

type Connection = Arc<Session>;

impl Apt {
    pub async fn update(
        c: Connection,
        release_info_change: bool,
    ) -> Result<String> {
        println!("APT UPDATE");
        let mut cmd = c.arc_command("apt");

        cmd.arg("--yes");
        if release_info_change {
            cmd.arg("--allow-releaseinfo-change");
        }
        cmd.arg("update");
        let cmd_ran = cmd.output().await;

        match cmd_ran {
            Err(e) => {
                return Err(anyhow::Error::from(e));
            }
            Ok(c) => {
                let so = String::from_utf8(c.stdout).unwrap();
                println!("{}", so);
                Ok(so)
            }
        }
    }

    pub async fn upgrade(c: Connection, secure: bool) -> Result<String> {
        println!("APT UPGRADE");
        let mut cmd = c.arc_raw_command("DEBIAN_FRONTEND=noninteractive");
        cmd.arg("apt");
        cmd.arg("--yes");
        cmd.arg("upgrade");

        let cmd_run = cmd.output().await;
        match cmd_run {
            Err(e) => {
                eprintln!("apt --yes upgrade error");
                eprintln!("{:?}", e);
                return Err(anyhow::Error::from(e));
            }
            Ok(c) => {
                let out = String::from_utf8(c.stdout).unwrap();
                println!("{}", out);
                Ok(out)
            }
        }
    }

    pub async fn autoremove(c: Connection) -> Result<String> {
        let cmd = c
            .arc_raw_command("apt")
            .arg("--yes")
            .arg("--purge")
            .arg("autoremove")
            .output()
            .await;

        match cmd {
            Err(e) => {
                return Err(anyhow::Error::from(e));
            }
            Ok(c) => Ok(String::from_utf8(c.stdout).unwrap()),
        }
    }

    pub async fn full_upgrade(c: Connection) -> Result<String> {
        println!("full-upgrade");
        let cmd = c
            .arc_raw_command("DEBIAN_FRONTEND=noninteractive")
            .arg("apt")
            .arg("--yes")
            .arg("full-upgrade")
            .output()
            .await;

        match cmd {
            Err(e) => {
                eprintln!("apt pull-upgrade error");
                return Err(anyhow::Error::from(e));
            }
            Ok(c) => Ok(String::from_utf8(c.stdout).unwrap()),
        }
    }
}
