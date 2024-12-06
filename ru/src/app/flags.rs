use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct BitFlags: u64 {
        const RUNNING         = 0b00000001; // 1
        const LOADING         = 0b00000100; // 4
        const SHUTTING_DOWN   = 0b00001000; // 8

    }
}

impl BitFlags {
    fn set_single_flag(&mut self, flag: BitFlags) {
        self.iter_names().for_each(|f| self.remove(f.1));
        self.insert(flag);
    }

    pub fn set_loading(&mut self) {
        if self.is_running() {
            self.remove_flag(BitFlags::LOADING);
        };
        self.insert(BitFlags::LOADING);
    }

    pub fn set_running(&mut self) {
        if self.is_loading() {
            self.remove_flag(BitFlags::LOADING);
        };
        self.insert(BitFlags::RUNNING);
    }

    pub fn set_paused(&mut self) {
        if self.is_running() {
            self.remove_flag(BitFlags::RUNNING);
        };
        if self.is_loading() {
            self.remove_flag(BitFlags::LOADING);
        };
    }

    pub fn set_shutting_down(&mut self) {
        self.set_single_flag(BitFlags::SHUTTING_DOWN);
    }

    pub fn is_running(&self) -> bool {
        self.contains(BitFlags::RUNNING)
    }

    pub fn is_loading(&self) -> bool {
        self.contains(BitFlags::LOADING)
    }

    pub fn is_shutting_down(&self) -> bool {
        self.contains(BitFlags::SHUTTING_DOWN)
    }

    pub fn set_flag(&mut self, flag: BitFlags) {
        self.set_single_flag(flag);
    }

    pub fn remove_flag(&mut self, flag: BitFlags) {
        self.remove(flag);
    }
}
