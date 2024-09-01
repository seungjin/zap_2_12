use anyhow::Result;
use core::time::Duration;
use futures::future::{BoxFuture, FutureExt};
use openssh::Session;
use openssh::SessionBuilder;
use std::sync::Arc;
use std::thread;

pub struct Debian;

impl Debian {
    // cat /etc/apt/sources.list | grep ^deb | awk '{print $3}'

    pub async fn upgrade_2_11(s: Arc<Session>) -> Result<()> {
        let a = s
            .clone()
            .arc_raw_command("sudo")
            .arg("sed")
            .arg("-i")
            .arg("'s/buster/bullseye/g'")
            .arg("/etc/apt/sources.list")
            .output()
            .await;

        let b = s
            .clone()
            .arc_raw_command("sudo")
            .arg("sed")
            .arg("-i")
            .arg("'s/buster/bullseye/g'")
            .arg("/etc/apt/sources.list.d/*.list")
            .output()
            .await;

        let c = s
            .clone()
            .arc_raw_command("sudo")
            .arg("sed")
            .arg("-i")
            .arg("'s#/debian-security bullseye/updates# bullseye-security#g'")
            .arg("/etc/apt/sources.list.list")
            .output()
            .await;

        // sudo sed -i 's/buster/bullseye/g' /etc/apt/sources.list
        // sudo sed -i 's/buster/bullseye/g' /etc/apt/sources.list.d/*.list
        // sudo sed -i 's#/debian-security bullseye/updates# bullseye-security#g' /etc/apt/sources.list

        Ok(())
    }

    pub async fn upgrade_2_12(s: Arc<Session>) -> Result<()> {
        let a = s
            .clone()
            .arc_raw_command("sudo")
            .arg("sed")
            .arg("-i")
            .arg("'s/buster/bullseye/g'")
            .arg("/etc/apt/sources.list")
            .output()
            .await;

        let b = s
            .clone()
            .arc_raw_command("sudo")
            .arg("sed")
            .arg("-i")
            .arg("'s/buster/bullseye/g'")
            .arg("/etc/apt/sources.list.d/*.list")
            .output()
            .await;

        let c = s
            .clone()
            .arc_raw_command("sudo")
            .arg("sed")
            .arg("-i")
            .arg("'s#/debian-security bullseye/updates# bullseye-security#g'")
            .arg("/etc/apt/sources.list.list")
            .output()
            .await;

        // sudo sed -i 's/buster/bullseye/g' /etc/apt/sources.list
        // sudo sed -i 's/buster/bullseye/g' /etc/apt/sources.list.d/*.list
        // sudo sed -i 's#/debian-security bullseye/updates# bullseye-security#g' /etc/apt/sources.list

        Ok(())
    }
}
