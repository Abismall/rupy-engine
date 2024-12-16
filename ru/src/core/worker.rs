use super::events::RupyAppEvent;
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub enum WorkerTask {
    Load,
}

pub struct RupyWorker {
    pub task_sender: Sender<WorkerTask>,
}

impl RupyWorker {
    pub fn spawn(task_receiver: Receiver<WorkerTask>, result_sender: Sender<RupyAppEvent>) {
        let result_tx = result_sender.clone();
        tokio::spawn(async move { while let Ok(task) = task_receiver.recv() {} });
    }
}
