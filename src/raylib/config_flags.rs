#![allow(dead_code)]
/// Set to try enabling V-Sync on GPU
pub const FLAG_VSYNC_HINT: u32 = 0x00000040;
/// Set to run program in fullscreen
pub const FLAG_FULLSCREEN_MODE: u32 = 0x00000002;
/// Set to allow resizable window
pub const FLAG_WINDOW_RESIZABLE: u32 = 0x00000004;
/// Set to disable window decoration (frame and buttons)
pub const FLAG_WINDOW_UNDECORATED: u32 = 0x00000008;
/// Set to hide window
pub const FLAG_WINDOW_HIDDEN: u32 = 0x00000080;
/// Set to minimize window (iconify)
pub const FLAG_WINDOW_MINIMIZED: u32 = 0x00000200;
/// Set to maximize window (expanded to monitor)
pub const FLAG_WINDOW_MAXIMIZED: u32 = 0x00000400;
/// Set to window non focused
pub const FLAG_WINDOW_UNFOCUSED: u32 = 0x00000800;
/// Set to window always on top
pub const FLAG_WINDOW_TOPMOST: u32 = 0x00001000;
/// Set to allow windows running while minimized
pub const FLAG_WINDOW_ALWAYS_RUN: u32 = 0x00000100;
/// Set to allow transparent framebuffer
pub const FLAG_WINDOW_TRANSPARENT: u32 = 0x00000010;
/// Set to support HighDPI
pub const FLAG_WINDOW_HIGHDPI: u32 = 0x00002000;
/// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
pub const FLAG_WINDOW_MOUSE_PASSTHROUGH: u32 = 0x00004000;
/// Set to run program in borderless windowed mode
pub const FLAG_BORDERLESS_WINDOWED_MODE: u32 = 0x00008000;
/// Set to try enabling MSAA 4X
pub const FLAG_MSAA_4X_HINT: u32 = 0x00000020;
/// Set to try enabling interlaced video format (for V3D)
pub const FLAG_INTERLACED_HINT: u32 = 0x00010000;
