use crate::rbr::GameMode;
use crate::{EventRegistry, PluginContext, PluginResult, RbrPlugin};
use crate::event::{GameModeChangedEvent, RaceReplayStartEvent};

pub (crate) struct EventController {
    previous_game_mode: Option<GameMode>,
    race_replay_initialized: bool,
}

impl EventController {
    pub (crate) fn new() -> Self {
        Self {
            previous_game_mode: None,
            race_replay_initialized: false,
        }
    }

    pub(crate) fn update<P: RbrPlugin>(&mut self, plugin: &mut P, events: &EventRegistry<P>, context: &mut PluginContext<'_>
    ) -> PluginResult<()> {
        let current_mode = context.rbr().reader().get_game_mode();

        self.update_race_replay_start(
            current_mode,
            plugin,
            events,
            context,
        )?;

        self.update_game_mode(
            current_mode,
            plugin,
            events,
            context,
        )
    }

    fn update_game_mode<P: RbrPlugin>(&mut self, current: Option<GameMode>, plugin: &mut P, events: &EventRegistry<P>, context: &mut PluginContext<'_>
    ) -> PluginResult<()> {
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

    fn update_race_replay_start<P: RbrPlugin>(&mut self, current: Option<GameMode>, plugin: &mut P, events: &EventRegistry<P>, context: &mut PluginContext<'_>
    ) -> PluginResult<()> {
        let session_inactive = matches!(
            current,
            Some(
                    GameMode::Menu |
                    GameMode::LoadingTrack |
                    GameMode::ExitingToMenu |
                    GameMode::MenuTransition |
                    GameMode::LoadingCompleteTransition
            )
        );

        if session_inactive {
            self.race_replay_initialized = false;
            return Ok(());
        }

        let session_available = matches!(
            current,
            Some(
                    GameMode::PreStart |
                    GameMode::Driving |
                    GameMode::Pause |
                    GameMode::Replay
            )
        );

        if !session_available || self.race_replay_initialized {
            return Ok(());
        }

        if let Err(error) = context.rbr().initialize_race_time_object_references() {
            log::debug!("Race-time references are not ready: {error:?}"
        );

            return Ok(());
        }

        self.race_replay_initialized = true;

        events.dispatch(
            plugin,
            &RaceReplayStartEvent,
            context,
        )
    }
}


impl Default for EventController {
    fn default() -> Self {
        Self::new()
    }
}