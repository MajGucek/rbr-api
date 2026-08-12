#![allow(dead_code, unused_imports)]

mod context;
mod event;
mod hook;
mod plugin;
mod raw;
mod runtime;

pub mod rbr;
mod event_controller;

pub use context::PluginContext;
pub use event::{
    Event,
    EventListener,
    EventRegistry,
    StartEvent,
    StopEvent,
    UpdateEvent,
    GameModeChangedEvent,
};
pub use plugin::RbrPlugin;
pub use runtime::{
    PluginError,
    PluginResult,
};

pub mod prelude {
    pub use crate::{
        Event,
        EventListener,
        PluginContext,
        PluginResult,
        RbrPlugin,
        StartEvent,
        UpdateEvent,
        StopEvent,
        GameModeChangedEvent,
    };
    pub use crate::rbr::{
        GameMode,
        RbrReader
    };
}


pub use log::{
    debug,
    error,
    info,
    trace,
    warn,
};

unsafe extern "C" {
    fn cpp_create_plugin(
        game: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void;
}

#[doc(hidden)]
pub unsafe fn create_cpp_plugin(
    game: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    unsafe {
        cpp_create_plugin(game)
    }
}


#[doc(hidden)]
pub mod __private {
    pub use crate::{
        create_cpp_plugin,
        event::EventRegistry,
        runtime::{
            create,
            destroy,
            get_name,
        },
    };
}

// Register a Rust type as an RBR plugin.
#[macro_export]
macro_rules! export_plugin {
    (
        $plugin:ty,
        events = [$($event:ty),* $(,)?]
    ) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn RBR_CreatePlugin(
            game: *mut std::ffi::c_void,
        ) -> *mut std::ffi::c_void {
            unsafe {
                $crate::__private::create_cpp_plugin(game)
            }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn rust_plugin_create(
            _game: *mut std::ffi::c_void,
        ) -> *mut std::ffi::c_void {
            let mut events =
                $crate::__private::EventRegistry::<
                    $plugin
                >::new();

            $(
                events.register::<$event>();
            )*

            $crate::__private::create::<$plugin>(
                events,
            )
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn rust_plugin_destroy(
            state: *mut std::ffi::c_void,
        ) {
            unsafe {
                $crate::__private::destroy::<$plugin>(
                    state,
                );
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn rust_plugin_get_name(
            state: *mut std::ffi::c_void,
        ) -> *const std::ffi::c_char {
            unsafe {
                $crate::__private::get_name::<$plugin>(
                    state,
                )
            }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn rust_plugin_draw_frontend_page(
            _state: *mut std::ffi::c_void,
        ) {
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn rust_plugin_draw_results_ui(
            _state: *mut std::ffi::c_void,
        ) {
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn rust_plugin_handle_frontend_events(
            _state: *mut std::ffi::c_void,
            _keyboard: std::ffi::c_char,
            _up: u8,
            _down: u8,
            _left: u8,
            _right: u8,
            _select: u8,
        ) {
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn rust_plugin_tick_frontend_page(
            _state: *mut std::ffi::c_void,
            _delta: f32,
        ) {
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn rust_plugin_stage_started(
            _state: *mut std::ffi::c_void,
            _map: i32,
            _player_name: *const std::ffi::c_char,
            _false_start: u8,
        ) {
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn rust_plugin_handle_results(
            _state: *mut std::ffi::c_void,
            _checkpoint1: f32,
            _checkpoint2: f32,
            _finish_time: f32,
            _player_name: *const std::ffi::c_char,
        ) {
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn rust_plugin_checkpoint(
            _state: *mut std::ffi::c_void,
            _checkpoint_time: f32,
            _checkpoint_id: i32,
            _player_name: *const std::ffi::c_char,
        ) {
        }
    };
}