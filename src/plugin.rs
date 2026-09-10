use std::fmt::Display;
use std::str::FromStr;
use crate::config_parser::ConfigParser;
use crate::PluginResult;

pub trait RbrPlugin {
    const NAME: &'static str;

    fn new() -> Self;

    fn get_field<T>(&self, field_name: &str) -> PluginResult<T>
    where T: FromStr, T::Err: Display {
        ConfigParser::new(Self::NAME)?.get_field(field_name)
    }

    fn set_field<T>(&self, field_name: &str, value: T) -> PluginResult<()>
    where T: Display {
        ConfigParser::new(Self::NAME)?.set_field(field_name, value)
    }

}