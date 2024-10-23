use bitflags::bitflags;

bitflags! {
    pub struct AppState: u32 {
        const INITIALIZED             = 0b00000001;
        const SHUTDOWN                = 0b00000010;
        const SURFACE                = 0b000000100;
    }
}

impl AppState {
    pub fn contains_flag(&self, flag: AppState) -> bool {
        self.contains(flag)
    }
    pub fn set_flag(&mut self, flag: AppState) {
        self.insert(flag);
    }
    pub fn remove_flag(&mut self, flag: AppState) {
        self.remove(flag);
    }
}
