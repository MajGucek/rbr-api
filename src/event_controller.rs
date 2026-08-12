use std::cmp::PartialEq;
use crate::rbr::GameMode;
use crate::{EventRegistry, GameModeChangedEvent, PluginContext, PluginResult, RbrPlugin};

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
        self.update_game_mode(
            plugin,
            events,
            context
        )?;

        Ok(())
    }

    fn update_game_mode<P: RbrPlugin>(&mut self, plugin: &mut P, events: &EventRegistry<P>, context: &mut PluginContext<'_>) -> PluginResult<()> {
        let current = context.rbr().reader().game_mode();

        match (self.previous_game_mode, current) {
            (Some(previous), Some(current))
            if previous != current => {
                    events.dispatch(
                        plugin,
                        &GameModeChangedEvent {
                            previous,
                            current,
                        },
                        context,
                    )?;
                }
            _ => {}
        }

        self.previous_game_mode = current;

        Ok(())
    }
}


impl Default for RbrEventController {
    fn default() -> Self {
        Self::new()
    }
}