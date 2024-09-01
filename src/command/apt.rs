use openssh::Session;
use std::sync::Arc;

pub struct Apt;

impl Apt {
    pub async fn update(s: Arc<Session>) {
        s.arc_raw_command("sudo")
            .arg("apt")
            .arg("update")
            .output()
            .await
            .unwrap();
    }

    pub async fn upgrade(s: Arc<Session>) {
        s.arc_raw_command("sudo")
            .arg("apt")
            .arg("upgrade")
            .output()
            .await
            .unwrap();
    }

    pub async fn autoremove(s: Arc<Session>) {
        s.arc_raw_command("sudo")
            .arg("apt")
            .arg("autoremove")
            .output()
            .await
            .unwrap();
    }

    pub async fn full_upgrade(s: Arc<Session>) {
        s.arc_raw_command("sudo")
            .arg("apt")
            .arg("full-upgrade")
            .output()
            .await
            .unwrap();
    }

    pub async fn source_list(s: Arc<Session>) {
        // cat /etc/apt/sources.list | grep ^deb | awk '{print $3}'
        let a = s
            .arc_raw_command("cat")
            .arg("/etc/apt/sources.list")
            .arg("|")
            .arg("grep")
            .arg("^deb")
            .arg("|")
            .arg("awk")
            .arg("'{print $3}'")
            .output()
            .await
            .unwrap();

        let b = String::from_utf8(a.stdout);
        println!("{}", b.unwrap());
    }

    pub async fn install(s: Arc<Session>, packages: Vec<&str>) {}
}
