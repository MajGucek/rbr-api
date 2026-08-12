use crate::{hook, EventRegistry, PluginContext, RbrPlugin, StartEvent, StopEvent, UpdateEvent};
use crate::rbr::Rbr;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::{ffi::{c_char, c_void, CString}, fs, fs::OpenOptions, path::PathBuf, ptr};
use crate::event_controller::RbrEventController;

#[derive(Debug)]
pub enum PluginError {
    Initialization(String),
    Hook(String)
}

pub type PluginResult<T> = Result<T, PluginError>;

struct PluginRuntime<P: RbrPlugin> {
    plugin: P,
    name: CString,
    events: EventRegistry<P>,
    event_controller: RbrEventController,
    frame: u64,
    started: bool,
}

pub fn create<P: RbrPlugin>(events: EventRegistry<P>) -> *mut c_void {
    let name = CString::new(P::NAME).expect("plugin name contains a null byte");

    let runtime = PluginRuntime {
        plugin: P::new(),
        name,
        events,
        event_controller: RbrEventController::new(),
        frame: 0,
        started: false,
    };

    Box::into_raw(Box::new(runtime)).cast()
}

unsafe fn update<P: RbrPlugin>(state: *mut c_void, rbr: &Rbr) {
    if state.is_null() {
        return;
    }

    unsafe {
        let runtime = &mut *state.cast::<PluginRuntime<P>>();

        let mut context = PluginContext::new(rbr);

        let event = UpdateEvent {
            frame: runtime.frame,
        };

        if let Err(error) = runtime.event_controller.update(&mut runtime.plugin, &runtime.events, &mut context) {
            log::error!("RBR event detection failed: {:?}", error);
        }
        
        if let Err(error) = runtime.events.dispatch(&mut runtime.plugin, &event, &mut context) {
            log::error!("UpdateEvent failed: {error:?}");
        }

        runtime.frame += 1;
    }
}

pub unsafe fn get_name<P: RbrPlugin>(state: *mut c_void) -> *const c_char {
    if state.is_null() {
        return ptr::null();
    }

    unsafe {
        let runtime = &mut *state.cast::<PluginRuntime<P>>();

        if !runtime.started {
            if let Err(error) = start::<P>(state, runtime) {
                log::error!("{error:?}");
            }
        }

        runtime.name.as_ptr()
    }
}

unsafe fn start<P: RbrPlugin>(state: *mut c_void, runtime: &mut PluginRuntime<P>) -> PluginResult<()> {
    initialize_logger(P::ID).map_err(PluginError::Initialization)?;

    log::info!("Initializing plugin {}", P::NAME);

    unsafe {
        hook::install(state, update::<P>).map_err(PluginError::Hook)?;

        let Some(rbr) = hook::rbr() else {
            hook::uninstall();

            return Err(
                PluginError::Initialization("RBR was not initialized".to_owned())
            );
        };

        let mut context = PluginContext::new(rbr);

        if let Err(error) = runtime.events.dispatch(&mut runtime.plugin, &StartEvent, &mut context) {
            hook::uninstall();
            return Err(error);
        }

        runtime.started = true;

        Ok(())
    }
}

pub unsafe fn destroy<P: RbrPlugin>(state: *mut c_void) {
    if state.is_null() {
        return;
    }

    unsafe {
        let mut runtime = Box::from_raw(
            state.cast::<PluginRuntime<P>>(),
        );

        if runtime.started {
            if let Some(rbr) = hook::rbr() {
                let mut context = PluginContext::new(rbr);

                if let Err(error) = runtime.events.dispatch(&mut runtime.plugin, &StopEvent, &mut context) {
                    log::error!("StopEvent failed: {error:?}");
                }
            }

            hook::uninstall();
            runtime.started = false;
        }
    }
}

fn initialize_logger(plugin_id: &str) -> Result<PathBuf, String> {
    let executable = std::env::current_exe().map_err(|error| error.to_string())?;

    let rbr_directory = executable.parent().ok_or("RBR executable has no parent directory")?;

    let log_directory = rbr_directory
        .join("Plugins")
        .join(plugin_id);

    fs::create_dir_all(&log_directory).map_err(|error| error.to_string())?;

    let log_path = log_directory.join("plugin.log");

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_path)
        .map_err(|error| error.to_string())?;

    WriteLogger::init(LevelFilter::Debug, Config::default(), file).map_err(|error| error.to_string())?;

    Ok(log_path)
}

impl From<windows::core::Error> for PluginError {
    fn from(error: windows::core::Error) -> Self {
        Self::Initialization(error.to_string())
    }
}