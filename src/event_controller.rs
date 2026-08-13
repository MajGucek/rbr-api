use crate::rbr::GameMode;
use crate::{EventRegistry, PluginContext, PluginResult, RbrPlugin};

pub (crate) struct RbrEventController {
    previous_game_mode: Option<GameMode>
}

impl RbrEventController {
    pub (crate) fn new() -> Self {
        Self {
            previous_game_mode: None,
        }
    }

    pub (crate) fn update<P: RbrPlugin>(&mut self, plugin: &mut P, events: &EventRegistry<P>, context: &mut PluginContext<'_>) -> PluginResult<()> {
        // TODO call update fields

        Ok(())
    }

}


impl Default for RbrEventController {
    fn default() -> Self {
        Self::new()
    }
}