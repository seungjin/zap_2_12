use anyhow::Result;
use core::time::Duration;
use futures::future::{BoxFuture, FutureExt};
use openssh::KnownHosts;
use openssh::Session;
use openssh::SessionBuilder;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let server = "ssh://seungjin@free.gcp.seungjin.net:432";
    let s = Arc::new(server_check(0, server).await);
    //let debian_ver = check_debian_ver(s).await;
    //println!("{}", debian_ver);
    // match debian_ver {
    //     10u8 => upgrade_to(11).await,
    //     11u8 => upgrade_to(12).await,
    //     12u8 => println!("Debian is 12. I am Happy"),
    //     v => println!("Version not considered: {}", v),
    // }
    reboot(s, server).await;
}

fn server_check(
    try_count: u64,
    server: &'static str,
) -> BoxFuture<'static, Session> {
    if try_count > 74 {
        panic!("Connection is not avaiable");
    }
    async move {
        let ten_seconds = Duration::new(8, 0);

        let mut sess = SessionBuilder::default();
        sess.connect_timeout(ten_seconds);

        match sess.connect_mux(server).await {
            Ok(s) => s,
            Err(_) => {
                println!("Waiting for server ready ({})", try_count);
                server_check(try_count + 1, server).await
            }
        }
    }
    .boxed()
}

async fn check_debian_ver(s: Arc<Session>) -> u8 {
    let mut os_release = s
        .arc_raw_command("cat")
        .arg("/etc/os-release")
        .output()
        .await
        .unwrap();
    let mut ver = 0u8;
    for line in String::from_utf8(os_release.stdout)
        .unwrap()
        .split("\n")
        .into_iter()
    {
        if line.starts_with("VERSION_ID=") {
            let mut version_str =
                line.split("=").collect::<Vec<&str>>()[1].chars();
            version_str.next();
            version_str.next_back();
            ver = version_str.as_str().parse::<u8>().unwrap();
        }
    }

    ver
}

async fn upgrade_to(v: u8) {
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
}

async fn apt_update(s: Arc<Session>) {
    s.arc_raw_command("sudo")
        .arg("apt")
        .arg("update")
        .output()
        .await
        .unwrap();
}

async fn reboot(s: Arc<Session>, server: &'static str) -> Result<Session> {
    println!("REBOOTING");
    s.arc_raw_command("sudo")
        .arg("systemctl")
        .arg("reboot")
        .output()
        .await
        .unwrap();
    wait_until_server_down();
    println!("Server is rebooted.");
    let s = server_check(0, server).await;
    println!("Server is back.");
    Ok(s)
}

fn wait_until_server_down() {
    thread::sleep(Duration::from_secs(4))
}

fn wait_until_server_up() {
    thread::sleep(Duration::from_secs(4))
}
