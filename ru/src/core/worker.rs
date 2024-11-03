use crate::{
    events::{RupyAppEvent, WorkerTaskCompletion},
    log_warning,
    shader::library::try_list_shader_file_paths,
    texture::loader::texture_load_files_async,
};
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub enum WorkerTask {
    LoadTextures(
        String,
        String,
        wgpu::TextureFormat,
        wgpu::TextureDimension,
        u32,
        u32,
    ),
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
                    WorkerTask::LoadTextures(
                        folder_path,
                        extension,
                        format,
                        dimension,
                        mip_level_count,
                        sample_count,
                    ) => match texture_load_files_async(
                        folder_path,
                        extension,
                        format,
                        dimension,
                        mip_level_count,
                        sample_count,
                    )
                    .await
                    {
                        Ok(data) => {
                            if let Err(e) = result_tx.send(RupyAppEvent::TaskCompleted(
                                WorkerTaskCompletion::LoadTextureFiles(data),
                            )) {
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
                    WorkerTask::LoadShaderFiles => match try_list_shader_file_paths() {
                        Ok(data) => {
                            if let Err(e) = result_tx.send(RupyAppEvent::TaskCompleted(
                                WorkerTaskCompletion::LoadShaderFiles(data),
                            )) {
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
