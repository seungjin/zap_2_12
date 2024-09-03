use anyhow::Result;
use core::time::Duration;
use futures::future::{BoxFuture, FutureExt};
use openssh::KnownHosts;
use openssh::Session;
use openssh::SessionBuilder;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;

use crate::command::uptime::Uptime;

pub struct Server;

pub enum DistroVer {
    Fedora(u8),
    Debian(u8),
}

type Connection = Arc<Session>;

impl Server {
    pub fn check(try_count: u64, server: &'static str) -> BoxFuture<'static, Arc<Session>> {
        if try_count > 8 {
            panic!("Connection is not avaiable");
        }
        async move {
            let mut sess = SessionBuilder::default();
            sess.connect_timeout(Duration::from_secs(8));
            sess.known_hosts_check(KnownHosts::Accept);

            //match sess.connect_mux(server).await {
            match sess.connect_mux(server).await {
                Ok(s) => Arc::new(s),
                Err(_) => {
                    println!("Waiting for server is ready ({})", try_count);
                    thread::sleep(Duration::from_secs(4));
                    Self::check(try_count + 1, server).await
                }
            }
        }
        .boxed()
    }

    pub async fn reboot(s: Arc<Session>, server: &'static str) -> Result<Arc<Session>> {
        println!("REBOOTING");
        let cmd = s
            .clone()
            .arc_raw_command("systemctl")
            .arg("reboot")
            .output()
            .await;

        match cmd {
            Err(e) => {
                eprintln!("{:?}", e);
                return Err(anyhow::Error::from(e));
            }
            Ok(_c) => (),
        }

        Server::wait_until_down(s).await;
        println!("Server is rebooted.");
        let new_session = Server::check(0, server).await;
        let uptime = Uptime::exec(new_session.clone()).await;
        println!("Server is back.\nUptime: {}", uptime.unwrap());
        Ok(new_session)
    }

    pub async fn alive(s: Arc<Session>) -> bool {
        let a = s.arc_raw_command("#").status().await;
        if a.is_err() {
            return false;
        };
        return true;
    }

    pub async fn wait_until_down(s: Arc<Session>) -> Result<()> {
        while Self::alive(s.clone()).await == true {
            thread::sleep(Duration::from_secs(8));
            println!("Shutting down..");
        }
        Ok(())
    }

    pub async fn run_cmd(c: Connection, cmd_str: &str) -> Result<String> {
        let cmd = c.raw_command(cmd_str).output().await;
        let a = match cmd {
            Err(e) => {
                return Err(anyhow::Error::from(e));
            }
            Ok(c) => String::from_utf8(c.stdout).unwrap(),
        };
        Ok(a)
    }

    pub async fn check_debian_ver(s: Arc<Session>) -> u8 {
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
                let mut version_str = line.split("=").collect::<Vec<&str>>()[1].chars();
                version_str.next();
                version_str.next_back();
                ver = version_str.as_str().parse::<u8>().unwrap();
            }
        }
        ver
    }

    pub async fn check_fedora_ver(s: Arc<Session>) -> u8 {
        let os_release = s
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
                let version_str = line.split("=").collect::<Vec<&str>>()[1];
                ver = version_str.parse::<u8>().unwrap();
            }
        }
        ver
    }

    pub async fn read_file(s: Arc<Session>, path: &str) -> Result<String> {
        let content = s.arc_raw_command("cat").arg(path).output().await;
        match content {
            Err(e) => {
                return Err(anyhow::Error::from(e));
            }
            Ok(c) => Ok(String::from_utf8(c.stdout).unwrap()),
        }
    }

    /// todo:
    pub async fn write_file(s: Arc<Session>, path: &str, content: &str) {

        //cat > readme.txt << EOF
        //This is an input stream literal
        //EOF
    }

    pub async fn read_directory() {}

    pub async fn whoami() {}
}
