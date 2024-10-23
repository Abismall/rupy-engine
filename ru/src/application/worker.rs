use crate::events::RupyAppEvent;
use crate::graphics::texture::loader::texture_file_cache_setup_task;
use crate::log_info;
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub enum RupyWorkerTask {
    TextureFileCacheSetup(String, String),
}

pub struct RupyTaskWorker {
    pub task_sender: Sender<RupyWorkerTask>,
}

impl RupyTaskWorker {
    pub fn spawn(task_receiver: Receiver<RupyWorkerTask>, result_sender: Sender<RupyAppEvent>) {
        tokio::spawn(async move {
            while let Ok(task) = task_receiver.recv() {
                log_info!("{:?}", task);
                match task {
                    RupyWorkerTask::TextureFileCacheSetup(path, extension) => {
                        let result_tx = result_sender.clone();
                        match texture_file_cache_setup_task(path, extension).await {
                            Ok(textures) => {
                                result_tx
                                    .send(RupyAppEvent::TextureCacheSetupCompleted(textures))
                                    .unwrap();
                            }
                            Err(e) => {
                                result_tx
                                    .send(RupyAppEvent::TextureCacheSetupFailed(format!(
                                        "Failed to set up texture cache: {}",
                                        e
                                    )))
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        });
    }
}
