use crate::config::UserConfig;
use anyhow::Context;
use openssh::Session;

pub trait OperatingSystemCommands {
    fn add_user(&self, user: &UserConfig) -> String;
}

pub struct Ubuntu;

impl OperatingSystemCommands for Ubuntu {
    fn add_user(&self, user: &UserConfig) -> String {
        return format!("adduser {}", user.username);
    }
}

pub struct Debian;

impl OperatingSystemCommands for Debian {
    fn add_user(&self, user: &UserConfig) -> String {
        return format!("adduser {}", user.username);
    }
}

pub struct Fedora;

impl OperatingSystemCommands for Fedora {
    fn add_user(&self, user: &UserConfig) -> String {
        return format!("useradd {}", user.username);
    }
}

pub struct RedHat;

impl OperatingSystemCommands for RedHat {
    fn add_user(&self, user: &UserConfig) -> String {
        return format!("useradd {}", user.username);
    }
}

pub enum OsType {
    Debian(Debian),
    Fedora(Fedora),
    RedHat(RedHat),
    Ubuntu(Ubuntu),
    Unknown,
}

pub async fn detect_os(session: &Session) -> anyhow::Result<OsType> {
    let os_release_output = session
        .command("cat")
        .arg("/etc/os-release")
        .output()
        .await
        .context("cat /etc/os-release")?;

    let os_release_str = String::from_utf8_lossy(&os_release_output.stdout);

    if os_release_str.contains("Ubuntu") {
        Ok(OsType::Ubuntu(Ubuntu {}))
    } else if os_release_str.contains("Red Hat") {
        Ok(OsType::RedHat(RedHat {}))
    } else if os_release_str.contains("Debian") {
        Ok(OsType::Debian(Debian {}))
    } else if os_release_str.contains("Fedora") {
        Ok(OsType::Fedora(Fedora {}))
    } else {
        Ok(OsType::Unknown)
    }
}
