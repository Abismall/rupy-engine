use crate::{
    events::RupyAppEvent,
    graphics::{
        shader::library::try_list_shader_file_paths, texture::loader::async_load_textures_from_dir,
    },
    log_warning,
};
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub enum RupyWorkerTask {
    LoadTextures(String, String, wgpu::TextureDimension, u32, u32),
    ListShaderFiles,
}

pub struct RupyTaskWorker {
    pub task_sender: Sender<RupyWorkerTask>,
}

impl RupyTaskWorker {
    pub fn spawn(task_receiver: Receiver<RupyWorkerTask>, result_sender: Sender<RupyAppEvent>) {
        let result_tx = result_sender.clone();
        tokio::spawn(async move {
            while let Ok(task) = task_receiver.recv() {
                match task {
                    RupyWorkerTask::LoadTextures(
                        path,
                        extension,
                        dimension,
                        mip_level_count,
                        sample_count,
                    ) => match async_load_textures_from_dir(
                        path,
                        extension,
                        dimension,
                        mip_level_count,
                        sample_count,
                    )
                    .await
                    {
                        Ok(data) => {
                            if let Err(e) =
                                result_tx.send(RupyAppEvent::LoadTextureTaskCompleted(data))
                            {
                                log_warning!(
                                    "Failed to send LoadTextureFilesComplete event: {:?}",
                                    e
                                );
                            }
                        }
                        Err(e) => {
                            log_warning!("Warning: async_load_textures_from_dir failed: {:?}", e);
                        }
                    },
                    RupyWorkerTask::ListShaderFiles => match try_list_shader_file_paths() {
                        Ok(data) => {
                            if let Err(e) =
                                result_tx.send(RupyAppEvent::ListShaderFilesTaskCompleted(data))
                            {
                                log_warning!("Warning: Failed to send shader files event: {:?}", e);
                            }
                        }
                        Err(e) => {
                            log_warning!("Warning: try_list_shader_file_paths failed: {:?}", e);
                        }
                    },
                }
            }
        });
    }
}
