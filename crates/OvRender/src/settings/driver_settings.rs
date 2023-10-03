pub struct DriverSettings {
    pub debugMode: bool,
}

impl Default for DriverSettings {
    fn default() -> Self {
        Self { debugMode: true }
    }
}
