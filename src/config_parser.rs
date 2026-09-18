use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use ini::Ini;
use crate::{PluginError, PluginResult};
use crate::runtime::get_plugin_folder;

const CONFIG_FILE_NAME: &str = "config.ini";

pub struct ConfigParser {
    path: PathBuf,
}


impl ConfigParser {
    pub fn new(plugin_name: &str) -> PluginResult<Self> {
        let path = get_plugin_folder(plugin_name)
            .map_err(PluginError::Initialization)?
            .join(CONFIG_FILE_NAME);

        if !path.exists() {
            Ini::new()
                .write_to_file(&path)
                .map_err(|error| PluginError::WriteError(error.to_string()))?;
        }

        Ok(Self { path })
    }

    fn load(&self) -> PluginResult<Ini> {
        Ini::load_from_file(&self.path)
            .map_err(|error| PluginError::ReadError(error.to_string()))
    }


    pub fn get_field<T>(&self, field_name: &str) -> PluginResult<T>
    where
        T: FromStr,
        T::Err: Display,
    {
        let ini = self.load()?;

        let raw = ini.general_section().get(field_name).ok_or_else(|| {
            PluginError::ReadError(format!(
                "field '{field_name}' was not found in {}",
                self.path.display()
            ))
        })?;

        raw.parse::<T>().map_err(|error| {
            PluginError::ReadError(format!(
                "failed to parse field '{field_name}' in {}: {error}",
                self.path.display()
            ))
        })
    }

    pub fn set_field<T>(&self, field_name: &str, value: T) -> PluginResult<()>
    where
        T: Display,
    {
        let mut ini = self.load().unwrap_or_else(|_| Ini::new());

        ini.with_section(None::<String>)
            .set(field_name, value.to_string());

        ini.write_to_file(&self.path)
            .map_err(|error| PluginError::WriteError(error.to_string()))
    }
}
