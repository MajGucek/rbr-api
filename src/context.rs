use crate::rbr::{Rbr};

// This struct is provided to an event to pass to the plugin implementor, lifetime so its bounded
pub struct PluginContext<'a> {
    rbr: &'a Rbr
}

impl<'a> PluginContext<'a> {
    pub (crate) fn new(rbr: &'a Rbr) -> Self {
        Self { rbr }
    }
    pub fn rbr(&self) -> &'a Rbr {
        self.rbr
    }
}