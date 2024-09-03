use anyhow::{Error, Result};
use async_recursion::async_recursion;
use openssh::Session;
use std::sync::Arc;

use zap_2_12::command::apt::Apt;
use zap_2_12::server::Server;

#[tokio::main]
async fn main() {
    let server = "ssh://root@2.zap.seungjin.net:22";
    let s = Server::check(0, server).await;

    Apt::update(s.clone(), true)
        .await
        .map_err(anyhow::Error::from);
    Apt::upgrade(s.clone(), false)
        .await
        .map_err(anyhow::Error::from);
    Apt::full_upgrade(s.clone())
        .await
        .map_err(anyhow::Error::from);
    Apt::autoremove(s.clone())
        .await
        .map_err(anyhow::Error::from);
    let s = Server::reboot(s.clone(), server)
        .await
        .map_err(anyhow::Error::from)
        .unwrap();

    upgrade_2_11(s.clone()).await.map_err(anyhow::Error::from);
    Apt::update(s.clone(), false)
        .await
        .map_err(anyhow::Error::from);
    Apt::upgrade(s.clone(), false)
        .await
        .map_err(anyhow::Error::from);
    Apt::full_upgrade(s.clone())
        .await
        .map_err(anyhow::Error::from);
    Apt::autoremove(s.clone())
        .await
        .map_err(anyhow::Error::from);
    let s = Server::reboot(s.clone(), server)
        .await
        .map_err(anyhow::Error::from)
        .unwrap();

    // upgrade to 12
    upgrade_2_12(s.clone()).await.map_err(anyhow::Error::from);
    Apt::update(s.clone(), false)
        .await
        .map_err(anyhow::Error::from);
    Apt::upgrade(s.clone(), false)
        .await
        .map_err(anyhow::Error::from);
    Apt::full_upgrade(s.clone())
        .await
        .map_err(anyhow::Error::from);
    Apt::autoremove(s.clone())
        .await
        .map_err(anyhow::Error::from);
    let s = Server::reboot(s.clone(), server)
        .await
        .map_err(anyhow::Error::from)
        .unwrap();
}

async fn upgrade_2_11(s: Arc<Session>) -> Result<()> {
    println!("Upgrading to 11");
    Server::run_cmd(
        s.clone(),
        "sed -i \'s/buster/bullseye/g\' /etc/apt/sources.list",
    )
    .await?;
    Server::run_cmd(
        s.clone(),
        "sed -i \'s/buster/bullseye/g\' /etc/apt/sources.list.d/*.list",
    )
    .await?;
    Server::run_cmd(
        s.clone(),
        "sed -i \'s#/debian-security bullseye/updates# bullseye-security#g\' /etc/apt/sources.list",
    )
    .await?;
    Ok(())
}

async fn upgrade_2_12(s: Arc<Session>) -> Result<()> {
    println!("Upgrading to 12");
    Server::run_cmd(
        s.clone(),
        "sed -i \'s/bullseye/bookworm/g\' /etc/apt/sources.list",
    )
    .await?;
    Server::run_cmd(
        s.clone(),
        "sed -i \'s/bullseye/bookworm/g\' /etc/apt/sources.list.d/*.list",
    )
    .await?;
    Ok(())
}
