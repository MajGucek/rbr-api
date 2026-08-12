use crate::rbr::{Rbr, RbrReader};

// This struct is provided to an event to pass to the RbrPlugin implementor, lifetime so its bounded
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
    fn reader(&self) -> RbrReader<'_> {
        self.rbr.reader()
    }
}