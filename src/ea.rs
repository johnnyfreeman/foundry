use crate::config::Config;
use crate::os::{detect_os, OsType};
use crate::ui::UI;
use crate::user_management::UserManager;
use anyhow::Context;
use openssh::{KnownHosts, Session};

pub struct EaEngine {
    config: Config,
    ui: UI,
}

impl EaEngine {
    pub fn new(config: Config) -> Self {
        EaEngine {
            config,
            ui: UI::new(),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        for host in self.config.get_hosts() {
            // TODO: Run this inside tokio task
            self.ui
                .progress_start(host.name.clone(), "Connecting".to_string())
                .await?;

            let session = Session::connect(&host.ip, KnownHosts::Add)
                .await
                .context(format!("Failed to connect: {:?}", host))?;

            match Session::connect(&host.ip, KnownHosts::Add).await {
                Ok(session) => {
                    self.ui
                        .progress_update(host.name.clone(), "Determining OS...".to_string())
                        .await?;

                    let os_type = detect_os(&session).await?;

                    self.ui
                        .progress_update(
                            host.name.clone(),
                            match os_type {
                                OsType::Debian(_) => "Detected Debian".to_string(),
                                OsType::Fedora(_) => "Detected Fedora".to_string(),
                                OsType::RedHat(_) => "Detected Red Hat".to_string(),
                                OsType::Ubuntu(_) => "Detected Ubuntu".to_string(),
                                OsType::Unknown => "Could not detect OS".to_string(),
                            },
                        )
                        .await?;

                    for user in &self.config.users {
                        match os_type {
                            OsType::Debian(ref os) => {
                                UserManager::ensure_user_present(
                                    &session,
                                    &user,
                                    Box::new(os),
                                    &self.ui,
                                )
                                .await?;
                            }
                            OsType::Fedora(ref os) => {
                                UserManager::ensure_user_present(
                                    &session,
                                    &user,
                                    Box::new(os),
                                    &self.ui,
                                )
                                .await?;
                            }
                            OsType::RedHat(ref os) => {
                                UserManager::ensure_user_present(
                                    &session,
                                    &user,
                                    Box::new(os),
                                    &self.ui,
                                )
                                .await?;
                            }
                            OsType::Ubuntu(ref os) => {
                                UserManager::ensure_user_present(
                                    &session,
                                    &user,
                                    Box::new(os),
                                    &self.ui,
                                )
                                .await?;
                            }
                            OsType::Unknown => {}
                        }
                    }

                    self.ui
                        .progress_success(host.name.clone(), "Done".to_string())
                        .await?;
                }
                Err(error) => {
                    self.ui
                        .progress_fail(host.name.clone(), error.to_string())
                        .await?;
                }
            }

            session.close().await?;
        }

        Ok(())
    }
}
