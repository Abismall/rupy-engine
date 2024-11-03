use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct AppState: u64 {
        const RUNNING         = 0b00000001; // 1
        const LOADING         = 0b00000100; // 4
        const SHUTTING_DOWN   = 0b00001000; // 8

    }
}

impl AppState {
    fn set_single_flag(&mut self, flag: AppState) {
        self.iter_names().for_each(|f| self.remove(f.1));
        self.insert(flag);
    }

    pub fn set_loading(&mut self) {
        if self.is_running() {
            self.remove_flag(AppState::LOADING);
        };
        self.insert(AppState::LOADING);
    }

    pub fn set_running(&mut self) {
        if self.is_loading() {
            self.remove_flag(AppState::LOADING);
        };
        self.insert(AppState::RUNNING);
    }

    pub fn set_paused(&mut self) {
        if self.is_running() {
            self.remove_flag(AppState::RUNNING);
        };
        if self.is_loading() {
            self.remove_flag(AppState::LOADING);
        };
    }

    pub fn set_shutting_down(&mut self) {
        self.set_single_flag(AppState::SHUTTING_DOWN);
    }

    pub fn is_running(&self) -> bool {
        self.contains(AppState::RUNNING)
    }

    pub fn is_loading(&self) -> bool {
        self.contains(AppState::LOADING)
    }

    pub fn is_shutting_down(&self) -> bool {
        self.contains(AppState::SHUTTING_DOWN)
    }

    pub fn set_flag(&mut self, flag: AppState) {
        self.set_single_flag(flag);
    }

    pub fn remove_flag(&mut self, flag: AppState) {
        self.remove(flag);
    }
}
