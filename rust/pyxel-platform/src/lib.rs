#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_ptr_alignment,
    clippy::cast_sign_loss,
    clippy::must_use_candidate,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::wildcard_imports
)]

pub mod key;

mod event;
mod platform;
#[cfg(feature = "sdl2")]
mod sdl2;

pub use event::Event;
pub use platform::{
    display_size, export_browser_file, gl_context, gl_profile, init, init_event_filter,
    init_window, is_fullscreen, pause_audio, poll_events, quit, run_frame_loop, set_fullscreen,
    set_mouse_pos, set_mouse_visible, set_window_icon, set_window_pos, set_window_size,
    set_window_title, start_audio, step_frame, stop_text_input, ticks, window_pos, window_size,
    GLProfile,
};
