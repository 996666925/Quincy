pub struct ProjectConfig {
    pub name: String,
    pub path: String,
}

impl ProjectConfig {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_owned(),
            path: path.to_owned(),
        }
    }
}
