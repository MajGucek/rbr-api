use std::any::{Any, TypeId};
use std::collections::HashMap;
use egui::Context;
use crate::context::PluginContext;
use crate::PluginResult;

// Base trait for registering an event
pub trait Event {}

pub trait EventListener<E: Event> {
    fn on_event(&mut self, event: &E, context: &mut PluginContext<'_>) -> PluginResult<()>;
}

#[derive(Debug, Clone, Copy)]
pub struct StartEvent;
impl Event for StartEvent {}

#[derive(Debug, Clone, Copy)]
pub struct UpdateEvent {
    pub frame: u64,
}
impl Event for UpdateEvent {}

#[derive(Debug, Clone, Copy)]
pub struct StopEvent;
impl Event for StopEvent {}

pub struct DrawEvent {
    context: egui::Context,
}
impl DrawEvent {
    pub(crate) fn new(context: egui::Context) -> Self {
        Self { context }
    }

    pub fn egui(&self) -> &egui::Context {
        &self.context
    }
}
impl Event for DrawEvent {}


pub struct EguiSetupEvent {
    context: egui::Context,
}
impl EguiSetupEvent {
    pub (crate) fn new(context: Context) -> Self {
        Self { context }
    }
    pub fn egui(&self) -> &Context {
        &self.context
    }
    pub fn configure_style(&self, configure: impl FnOnce(&mut egui::Style)) {
        let mut style = (*self.context.style()).clone();

        configure(&mut style);

        self.context.set_style(style);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RaceReplayStartEvent;
impl Event for RaceReplayStartEvent {}

impl Event for EguiSetupEvent {}

type EventHandler<P> = for<'a> fn(plugin: &mut P, event: &dyn Any, context: &mut PluginContext<'a>) -> PluginResult<()>;

pub struct EventRegistry<P> {
    handlers: HashMap<TypeId, EventHandler<P>>
}

impl<P> EventRegistry<P> {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<E: Event + 'static>(&mut self) where P: EventListener<E> {
        self.handlers.insert(
            TypeId::of::<E>(),
            invoke_listener::<E, P>,
        );
    }

    pub (crate) fn dispatch<E: Event + 'static>(&self, plugin: &mut P, event: &E, context: &mut PluginContext<'_>) -> PluginResult<()> {
        let Some(handler) =
            self.handlers.get(&TypeId::of::<E>())
        else {
            return Ok(());
        };
        handler(plugin, event, context)
    }
}

impl<P> Default for EventRegistry<P> {
    fn default() -> Self {
        Self::new()
    }
}

fn invoke_listener<'a, E: Event + 'static, P>(plugin: &mut P, event: &dyn Any, context: &mut PluginContext<'_>) -> PluginResult<()>
    where
    P: EventListener<E> {
    let event = event
        .downcast_ref::<E>()
        .expect("event registry contained the wrong handler");

    plugin.on_event(event, context)
}





