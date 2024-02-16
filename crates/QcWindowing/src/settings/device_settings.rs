use glutin::context::Version;

pub struct DeviceSettings {
    pub debugProfile: bool,
    pub version: Version,
}

impl Default for DeviceSettings {
    fn default() -> Self {
        Self {
            debugProfile: true,
            version: Version::new(4, 6),
        }
    }
}

impl DeviceSettings {
    pub fn new(debugProfile: bool, version: Version) -> Self {
        Self {
            debugProfile,
            version,
        }
    }
}
