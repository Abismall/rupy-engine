use bitflags::bitflags;

bitflags! {
    pub struct ApplicationStateFlags: u32 {
        const RUNNING               = 0b00000001; // 1
        const PAUSED                = 0b00000010; // 2
        const LOADING               = 0b00000100; // 4
        const SHUTTING_DOWN         = 0b00001000; // 8
    }
}

impl ApplicationStateFlags {
    fn set_single_flag(&mut self, flag: ApplicationStateFlags) {
        self.iter_names().for_each(|f| self.remove(f.1));
        self.insert(flag);
    }

    pub fn set_loading(&mut self) {
        self.set_single_flag(ApplicationStateFlags::LOADING);
    }

    pub fn set_running(&mut self) {
        self.set_single_flag(ApplicationStateFlags::RUNNING);
    }

    pub fn set_paused(&mut self) {
        self.set_single_flag(ApplicationStateFlags::PAUSED);
    }

    pub fn set_shutting_down(&mut self) {
        self.set_single_flag(ApplicationStateFlags::SHUTTING_DOWN);
    }

    pub fn is_running(&self) -> bool {
        self.contains(ApplicationStateFlags::RUNNING)
    }

    pub fn is_paused(&self) -> bool {
        self.contains(ApplicationStateFlags::PAUSED)
    }

    pub fn is_loading(&self) -> bool {
        self.contains(ApplicationStateFlags::LOADING)
    }

    pub fn is_shutting_down(&self) -> bool {
        self.contains(ApplicationStateFlags::SHUTTING_DOWN)
    }

    pub fn set_flag(&mut self, flag: ApplicationStateFlags) {
        self.set_single_flag(flag);
    }

    pub fn remove_flag(&mut self, flag: ApplicationStateFlags) {
        self.remove(flag);
    }

    pub fn contains_flag(&self, flag: ApplicationStateFlags) -> bool {
        self.contains(flag)
    }
}
