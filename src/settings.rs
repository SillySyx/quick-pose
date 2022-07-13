use std::time::Duration;
use std::env::var_os;

#[derive(Clone, Debug)]
pub struct Settings {
    pub folder: String,
    pub images_number: usize,
    pub duration: Option<Duration>,
    pub pause: Option<Duration>,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            folder: resolve_home_folder("~/Pictures"),
            images_number: 10,
            duration: None,
            pause: None,
        }
    }

    pub fn resolve_folder(&self) -> String {
        resolve_home_folder(&self.folder)
    }
}

fn resolve_home_folder<T: Into<String>>(path: T) -> String {
    let path = path.into();
    
    if !path.starts_with("~/") {
        return path;
    }

    let home = match read_home_from_environment_variables() {
        Some(value) => value,
        None => return path,
    };

    format!("{}/{}", home, &path[2..])
}

fn read_home_from_environment_variables() -> Option<String> {
    let value = var_os("HOME")?;
    let value = value.to_str()?;

    Some(value.to_owned())
}