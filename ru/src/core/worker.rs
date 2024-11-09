use crate::{
    events::{RupyAppEvent, WorkerTaskCompletion},
    log_warning,
    shader::library::try_list_shader_file_paths,
    texture::async_load_texture_config_files,
};
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub enum WorkerTask {
    LoadTextures(String, String),
    LoadShaderFiles,
}

pub struct RupyWorker {
    pub task_sender: Sender<WorkerTask>,
}

impl RupyWorker {
    pub fn spawn(task_receiver: Receiver<WorkerTask>, result_sender: Sender<RupyAppEvent>) {
        let result_tx = result_sender.clone();
        tokio::spawn(async move {
            while let Ok(task) = task_receiver.recv() {
                match task {
                    WorkerTask::LoadTextures(folder_path, extension) => {
                        match async_load_texture_config_files(folder_path, extension).await {
                            Ok(data) => {
                                if let Err(e) = result_tx.send(RupyAppEvent::TaskCompleted(
                                    WorkerTaskCompletion::LoadTextureFiles(data),
                                )) {
                                    log_warning!("{:?}", e);
                                }
                            }
                            Err(e) => {
                                log_warning!("{:?}", e);
                            }
                        }
                    }
                    WorkerTask::LoadShaderFiles => match try_list_shader_file_paths() {
                        Ok(data) => {
                            if let Err(e) = result_tx.send(RupyAppEvent::TaskCompleted(
                                WorkerTaskCompletion::LoadShaderFiles(data),
                            )) {
                                log_warning!("{:?}", e);
                            }
                        }
                        Err(e) => {
                            log_warning!("{:?}", e);
                        }
                    },
                }
            }
        });
    }
}
