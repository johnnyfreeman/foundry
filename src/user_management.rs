// user_management.rs

use anyhow::Context;
use openssh::Session;

use crate::{config::UserConfig, os::OperatingSystemCommands, ui::UI};

pub struct UserManager;

impl UserManager {
    pub async fn ensure_user_present(
        session: &Session,
        user: &UserConfig,
        os: Box<&dyn OperatingSystemCommands>,
        ui: &UI,
    ) -> anyhow::Result<()> {
        let check_user_cmd = format!("id -u {}", user.username);

        // ui.progress_update(
        //     host.name.clone(),
        //     format!("Checking if user exists: {}", check_user_cmd),
        // )
        // .await?;

        if let Ok(_) = session.command(&check_user_cmd).output().await {
            println!("User {} already exists", user.username);
        } else {
            let create_user_cmd = os.add_user(user);

            session
                .command(&create_user_cmd)
                .output()
                .await
                .context(create_user_cmd)?;

            if !user.groups.is_empty() {
                let group_str = user.groups.join(",");
                let add_groups_cmd = format!("usermod -aG {} {}", group_str, user.username);
                session
                    .command(&add_groups_cmd)
                    .output()
                    .await
                    .context(add_groups_cmd)?;
            }

            println!(
                "User {} created and added to groups {:?}",
                user.username, user.groups
            );
        }
        Ok(())
    }
}
