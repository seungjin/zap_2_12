use anyhow::{Error, Result};
use openssh::Session;
use std::str;
use std::string::String;
use std::sync::Arc;

pub struct Apt;

type Connection = Arc<Session>;

impl Apt {
    pub async fn update(c: Connection) -> Result<String> {
        println!("APT UPDATE");
        let cmd = c
            .arc_command("apt")
            .arg("--yes")
            .arg("update")
            .output()
            .await;

        match cmd {
            Err(e) => {
                return Err(anyhow::Error::from(e));
            }
            Ok(c) => Ok(String::from_utf8(c.stdout).unwrap()),
        }
    }

    pub async fn upgrade(c: Connection) -> Result<String> {
        println!("APT UPGRADE");
        let cmd = c.arc_raw_command("apt").arg("upgrade").output().await;

        match cmd {
            Err(e) => {
                return Err(anyhow::Error::from(e));
            }
            Ok(c) => Ok(String::from_utf8(c.stdout).unwrap()),
        }
    }

    pub async fn autoremove(c: Connection) -> Result<String> {
        let cmd = c
            .arc_raw_command("sudo")
            .arg("apt")
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
        let cmd = c
            .arc_raw_command("sudo")
            .arg("apt")
            .arg("full-upgrade")
            .output()
            .await;

        match cmd {
            Err(e) => {
                return Err(anyhow::Error::from(e));
            }
            Ok(c) => Ok(String::from_utf8(c.stdout).unwrap()),
        }
    }

    pub async fn source_list(c: Connection) -> Result<String> {
        // cat /etc/apt/sources.list | grep ^deb | awk '{print $3}'
        let cmd = c
            .arc_raw_command("cat")
            .arg("/etc/apt/sources.list")
            .arg("|")
            .arg("grep")
            .arg("^deb")
            .arg("|")
            .arg("awk")
            .arg("'{print $3}'")
            .output()
            .await;

        match cmd {
            Err(e) => {
                return Err(anyhow::Error::from(e));
            }
            Ok(c) => Ok(String::from_utf8(c.stdout).unwrap()),
        }
    }

    pub async fn install(
        c: Connection,
        packages: Vec<&str>,
    ) -> Result<String> {
        println!("APT INSTALL");
        let mut stdout = String::new();
        for package in packages.into_iter() {
            let cmd = c
                .clone()
                .arc_command("apt")
                .arg("install")
                .arg("--yes")
                .arg(package)
                .output()
                .await;

            match cmd {
                Err(e) => {
                    return Err(anyhow::Error::from(e));
                }
                Ok(c) => stdout
                    .push_str(str::from_utf8(c.stdout.as_slice()).unwrap()),
            }
        }

        Ok(stdout)
    }
}
