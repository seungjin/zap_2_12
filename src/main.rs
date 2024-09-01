use anyhow::{Error, Result};
use async_recursion::async_recursion;
use openssh::Session;
use std::sync::Arc;

use zap_2_12::command::apt::Apt;
use zap_2_12::debian::Debian;
use zap_2_12::server::Server;

#[tokio::main]
async fn main() {
    let server = "ssh://root@2.zap.seungjin.net:22";
    let s = Server::check(0, server).await;
    let a = Apt::update(s.clone()).await;
    println!("{:?}", a);
    if a.is_err() {
        panic!("STOP! HERE");
    }
    //let b = Apt::upgrade(s.clone()).await;
    //println!("{:?}", b);
    //let c = Server::reboot(s.clone(), server).await;
    //println!("{:?}", c);
}

#[async_recursion]
async fn upgrade(s: Arc<Session>) -> Result<()> {
    let debian_ver = Server::check_debian_ver(s.clone()).await;
    println!("{}", debian_ver);

    match debian_ver {
        10u8 => {
            upgrade_to(s.clone(), 11).await;
            return upgrade(s).await;
        }
        11u8 => {
            upgrade_to(s.clone(), 12).await;
            return upgrade(s).await;
        }
        12u8 => {
            println!("Debian is 12. I am Happy");
            return Ok(());
        }
        v => {
            println!("Version not considered: {}", v);
            return Err(Error::msg("rasarsasr"));
        }
    }
}

async fn upgrade_to(s: Arc<Session>, v: u8) {
    println!("Upgrading to version {}", v);
    // sudo apt update
    // sudo apt upgrade
    // sudo apt full-upgrade
    // sudo apt autoremove
    // sudo sed -i 's/buster/bullseye/g' /etc/apt/sources.list
    // sudo sed -i 's/buster/bullseye/g' /etc/apt/sources.list.d/*.list
    // sudo sed -i 's#/debian-security bullseye/updates# bullseye-security#g' /etc/apt/sources.list
    // sudo apt update
    // sudo apt upgrade
    // sudo apt full-upgrade
    // sudo apt autoremove
    //reboot();
    Apt::update(s.clone()).await;
    Apt::upgrade(s.clone()).await;
}
