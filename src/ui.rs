use anyhow::Result;
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::BTreeMap;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::time::Duration;

pub enum UIAction {
    ProgressStart(String, String),
    ProgressUpdate(String, String),
    ProgressSucceed(String, String),
    ProgressFail(String, String),
}

#[derive(Clone)]
pub struct UI {
    tx: Sender<UIAction>,
}

impl UI {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel(32);

        tokio::spawn(async move {
            let multiprogress = MultiProgress::new();

            let spinner_style =
                ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}").unwrap();

            let mut map: BTreeMap<String, ProgressBar> = BTreeMap::new();

            while let Some(action) = rx.recv().await {
                match action {
                    UIAction::ProgressStart(prefix, message) => {
                        let progress = multiprogress.add(ProgressBar::new_spinner());
                        progress.enable_steady_tick(Duration::from_millis(64));
                        progress.set_style(spinner_style.clone());
                        progress.set_prefix(prefix.clone());
                        progress.set_message(message.clone());
                        map.insert(prefix, progress.clone());
                    }
                    UIAction::ProgressUpdate(prefix, message) => {
                        let progress = map.get(&prefix).expect("Count not find ProgressBar");
                        progress.set_message(message.clone());
                    }
                    UIAction::ProgressSucceed(prefix, message) => {
                        multiprogress
                            .println(&format!(
                                "{} {} {}",
                                style(&prefix).bold().dim(),
                                style("✓").green(),
                                &message,
                            ))
                            .expect("Could not print line");
                        let progress = map.get(&prefix).expect("Count not find ProgressBar");
                        progress.finish_and_clear();
                        multiprogress.remove(progress);
                        map.remove(&prefix);
                    }
                    UIAction::ProgressFail(prefix, message) => {
                        multiprogress
                            .println(&format!(
                                "{} {} {}",
                                style(&prefix).bold().dim(),
                                style("✗").red(),
                                &message,
                            ))
                            .expect("Could not print line");
                        let progress = map.get(&prefix).expect("Count not find ProgressBar");
                        progress.finish_and_clear();
                        multiprogress.remove(progress);
                        map.remove(&prefix);
                    }
                }
            }
        });

        Self { tx }
    }

    pub async fn progress_start(&self, prefix: String, message: String) -> Result<()> {
        self.tx
            .send(UIAction::ProgressStart(prefix, message))
            .await?;
        Ok(())
    }

    pub async fn progress_update(&self, prefix: String, message: String) -> Result<()> {
        self.tx
            .send(UIAction::ProgressUpdate(prefix, message))
            .await?;
        Ok(())
    }

    pub async fn progress_success(&self, prefix: String, message: String) -> Result<()> {
        self.tx
            .send(UIAction::ProgressSucceed(prefix, message))
            .await?;
        Ok(())
    }

    pub async fn progress_fail(&self, prefix: String, message: String) -> Result<()> {
        self.tx
            .send(UIAction::ProgressFail(prefix, message))
            .await?;
        Ok(())
    }
}
