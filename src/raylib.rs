use core::panic;
use std::{
    ffi::{c_char, c_float, c_int, c_uchar, c_uint, c_void, CString},
    ops, u32, u8,
};

use opencv::core::{Mat, MatTraitConst, CV_8UC3};
#[allow(dead_code)]
pub mod config_flags {
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
}

/// Gesture
/// NOTE: Provided as bit-wise flags to enable only desired gestures
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub mod gestures {
    /// No gesture
    pub const GESTURE_NONE: u32 = 0;
    /// Tap gesture
    pub const GESTURE_TAP: u32 = 1;
    /// Double tap gesture
    pub const GESTURE_DOUBLETAP: u32 = 2;
    /// Hold gesture
    pub const GESTURE_HOLD: u32 = 4;
    /// Drag gesture
    pub const GESTURE_DRAG: u32 = 8;
    /// Swipe right gesture
    pub const GESTURE_SWIPE_RIGHT: u32 = 16;
    /// Swipe left gesture
    pub const GESTURE_SWIPE_LEFT: u32 = 32;
    /// Swipe up gesture
    pub const GESTURE_SWIPE_UP: u32 = 64;
    /// Swipe down gesture
    pub const GESTURE_SWIPE_DOWN: u32 = 128;
    /// Pinch in gesture
    pub const GESTURE_PINCH_IN: u32 = 256;
    /// Pinch out gesture
    pub const GESTURE_PINCH_OUT: u32 = 512;
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub mod colors {
    use super::Color;

    pub const LIGHTGRAY: Color = Color {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
    }; // Light Gray
    pub const GRAY: Color = Color {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
    }; // Gray
    pub const DARKGRAY: Color = Color {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
    }; // Dark Gray
    pub const YELLOW: Color = Color {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
    }; // Yellow
    pub const GOLD: Color = Color {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
    }; // Gold
    pub const ORANGE: Color = Color {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
    }; // Orange
    pub const PINK: Color = Color {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
    }; // Pink
    pub const RED: Color = Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
    }; // Red
    pub const MAROON: Color = Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
    }; // Maroon
    pub const GREEN: Color = Color {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
    }; // Green
    pub const LIME: Color = Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
    }; // Lime
    pub const DARKGREEN: Color = Color {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
    }; // Dark Green
    pub const SKYBLUE: Color = Color {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
    }; // Sky Blue
    pub const BLUE: Color = Color {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
    }; // Blue
    pub const DARKBLUE: Color = Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
    }; // Dark Blue
    pub const PURPLE: Color = Color {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
    }; // Purple
    pub const VIOLET: Color = Color {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
    }; // Violet
    pub const DARKPURPLE: Color = Color {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
    }; // Dark Purple
    pub const BEIGE: Color = Color {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
    }; // Beige
    pub const BROWN: Color = Color {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
    }; // Brown
    pub const DARKBROWN: Color = Color {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
    }; // Dark Brown
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    }; // White
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    }; // Black
    pub const BLANK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }; // Blank (Transparent)
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    }; // Magenta
    pub const RAYWHITE: Color = Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    }; // My own White (raylib logo)
}
/// Keyboard keys (US keyboard layout)
// NOTE: Use GetKeyPressed() to allow redefining
// required keys for alternative layouts
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum KeyboardKeys {
    KEY_NULL = 0, // Key: NULL, used for no key pressed
    // Alphanumeric keys
    KEY_APOSTROPHE = 39,    // Key: '
    KEY_COMMA = 44,         // Key: ,
    KEY_MINUS = 45,         // Key: -
    KEY_PERIOD = 46,        // Key: .
    KEY_SLASH = 47,         // Key: /
    KEY_ZERO = 48,          // Key: 0
    KEY_ONE = 49,           // Key: 1
    KEY_TWO = 50,           // Key: 2
    KEY_THREE = 51,         // Key: 3
    KEY_FOUR = 52,          // Key: 4
    KEY_FIVE = 53,          // Key: 5
    KEY_SIX = 54,           // Key: 6
    KEY_SEVEN = 55,         // Key: 7
    KEY_EIGHT = 56,         // Key: 8
    KEY_NINE = 57,          // Key: 9
    KEY_SEMICOLON = 59,     // Key: ;
    KEY_EQUAL = 61,         // Key: =
    KEY_A = 65,             // Key: A | a
    KEY_B = 66,             // Key: B | b
    KEY_C = 67,             // Key: C | c
    KEY_D = 68,             // Key: D | d
    KEY_E = 69,             // Key: E | e
    KEY_F = 70,             // Key: F | f
    KEY_G = 71,             // Key: G | g
    KEY_H = 72,             // Key: H | h
    KEY_I = 73,             // Key: I | i
    KEY_J = 74,             // Key: J | j
    KEY_K = 75,             // Key: K | k
    KEY_L = 76,             // Key: L | l
    KEY_M = 77,             // Key: M | m
    KEY_N = 78,             // Key: N | n
    KEY_O = 79,             // Key: O | o
    KEY_P = 80,             // Key: P | p
    KEY_Q = 81,             // Key: Q | q
    KEY_R = 82,             // Key: R | r
    KEY_S = 83,             // Key: S | s
    KEY_T = 84,             // Key: T | t
    KEY_U = 85,             // Key: U | u
    KEY_V = 86,             // Key: V | v
    KEY_W = 87,             // Key: W | w
    KEY_X = 88,             // Key: X | x
    KEY_Y = 89,             // Key: Y | y
    KEY_Z = 90,             // Key: Z | z
    KEY_LEFT_BRACKET = 91,  // Key: [
    KEY_BACKSLASH = 92,     // Key: '\'
    KEY_RIGHT_BRACKET = 93, // Key: ]
    KEY_GRAVE = 96,         // Key: `
    // Function keys
    KEY_SPACE = 32,          // Key: Space
    KEY_ESCAPE = 256,        // Key: Esc
    KEY_ENTER = 257,         // Key: Enter
    KEY_TAB = 258,           // Key: Tab
    KEY_BACKSPACE = 259,     // Key: Backspace
    KEY_INSERT = 260,        // Key: Ins
    KEY_DELETE = 261,        // Key: Del
    KEY_RIGHT = 262,         // Key: Cursor right
    KEY_LEFT = 263,          // Key: Cursor left
    KEY_DOWN = 264,          // Key: Cursor down
    KEY_UP = 265,            // Key: Cursor up
    KEY_PAGE_UP = 266,       // Key: Page up
    KEY_PAGE_DOWN = 267,     // Key: Page down
    KEY_HOME = 268,          // Key: Home
    KEY_END = 269,           // Key: End
    KEY_CAPS_LOCK = 280,     // Key: Caps lock
    KEY_SCROLL_LOCK = 281,   // Key: Scroll down
    KEY_NUM_LOCK = 282,      // Key: Num lock
    KEY_PRINT_SCREEN = 283,  // Key: Print screen
    KEY_PAUSE = 284,         // Key: Pause
    KEY_F1 = 290,            // Key: F1
    KEY_F2 = 291,            // Key: F2
    KEY_F3 = 292,            // Key: F3
    KEY_F4 = 293,            // Key: F4
    KEY_F5 = 294,            // Key: F5
    KEY_F6 = 295,            // Key: F6
    KEY_F7 = 296,            // Key: F7
    KEY_F8 = 297,            // Key: F8
    KEY_F9 = 298,            // Key: F9
    KEY_F10 = 299,           // Key: F10
    KEY_F11 = 300,           // Key: F11
    KEY_F12 = 301,           // Key: F12
    KEY_LEFT_SHIFT = 340,    // Key: Shift left
    KEY_LEFT_CONTROL = 341,  // Key: Control left
    KEY_LEFT_ALT = 342,      // Key: Alt left
    KEY_LEFT_SUPER = 343,    // Key: Super left
    KEY_RIGHT_SHIFT = 344,   // Key: Shift right
    KEY_RIGHT_CONTROL = 345, // Key: Control right
    KEY_RIGHT_ALT = 346,     // Key: Alt right
    KEY_RIGHT_SUPER = 347,   // Key: Super right
    KEY_KB_MENU = 348,       // Key: KB menu
    // Keypad keys
    KEY_KP_0 = 320,        // Key: Keypad 0
    KEY_KP_1 = 321,        // Key: Keypad 1
    KEY_KP_2 = 322,        // Key: Keypad 2
    KEY_KP_3 = 323,        // Key: Keypad 3
    KEY_KP_4 = 324,        // Key: Keypad 4
    KEY_KP_5 = 325,        // Key: Keypad 5
    KEY_KP_6 = 326,        // Key: Keypad 6
    KEY_KP_7 = 327,        // Key: Keypad 7
    KEY_KP_8 = 328,        // Key: Keypad 8
    KEY_KP_9 = 329,        // Key: Keypad 9
    KEY_KP_DECIMAL = 330,  // Key: Keypad .
    KEY_KP_DIVIDE = 331,   // Key: Keypad /
    KEY_KP_MULTIPLY = 332, // Key: Keypad *
    KEY_KP_SUBTRACT = 333, // Key: Keypad -
    KEY_KP_ADD = 334,      // Key: Keypad +
    KEY_KP_ENTER = 335,    // Key: Keypad Enter
    KEY_KP_EQUAL = 336,    // Key: Keypad =
    // Android key buttons
    KEY_BACK = 4,         // Key: Android back button
    KEY_MENU = 5,         // Key: Android menu button
    KEY_VOLUME_UP = 24,   // Key: Android volume up button
    KEY_VOLUME_DOWN = 25, // Key: Android volume down button
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
enum PixelFormat {
    PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1, // 8 bit per pixel (no alpha)
    PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA,    // 8*2 bpp (2 channels)
    PIXELFORMAT_UNCOMPRESSED_R5G6B5,        // 16 bpp
    PIXELFORMAT_UNCOMPRESSED_R8G8B8,        // 24 bpp
    PIXELFORMAT_UNCOMPRESSED_R5G5B5A1,      // 16 bpp (1 bit alpha)
    PIXELFORMAT_UNCOMPRESSED_R4G4B4A4,      // 16 bpp (4 bit alpha)
    PIXELFORMAT_UNCOMPRESSED_R8G8B8A8,      // 32 bpp
    PIXELFORMAT_UNCOMPRESSED_R32,           // 32 bpp (1 channel - float)
    PIXELFORMAT_UNCOMPRESSED_R32G32B32,     // 32*3 bpp (3 channels - float)
    PIXELFORMAT_UNCOMPRESSED_R32G32B32A32,  // 32*4 bpp (4 channels - float)
    PIXELFORMAT_UNCOMPRESSED_R16,           // 16 bpp (1 channel - half float)
    PIXELFORMAT_UNCOMPRESSED_R16G16B16,     // 16*3 bpp (3 channels - half float)
    PIXELFORMAT_UNCOMPRESSED_R16G16B16A16,  // 16*4 bpp (4 channels - half float)
    PIXELFORMAT_COMPRESSED_DXT1_RGB,        // 4 bpp (no alpha)
    PIXELFORMAT_COMPRESSED_DXT1_RGBA,       // 4 bpp (1 bit alpha)
    PIXELFORMAT_COMPRESSED_DXT3_RGBA,       // 8 bpp
    PIXELFORMAT_COMPRESSED_DXT5_RGBA,       // 8 bpp
    PIXELFORMAT_COMPRESSED_ETC1_RGB,        // 4 bpp
    PIXELFORMAT_COMPRESSED_ETC2_RGB,        // 4 bpp
    PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA,   // 8 bpp
    PIXELFORMAT_COMPRESSED_PVRT_RGB,        // 4 bpp
    PIXELFORMAT_COMPRESSED_PVRT_RGBA,       // 4 bpp
    PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA,   // 8 bpp
    PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA,   // 2 bpp
}

#[repr(C)]
struct RVector2(c_float, c_float);

impl From<Vector2> for RVector2 {
    fn from(value: Vector2) -> Self {
        Self(value.0, value.1)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Vector2(pub f32, pub f32);

impl ops::Div<f32> for Vector2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Vector2(self.0.clone() / rhs, self.1.clone() / rhs)
    }
}

#[repr(C)]
struct Image {
    data: *mut c_void,
    width: c_int,        // Image base width
    height: c_int,       // Image base height
    mipmaps: c_int,      // Mipmap levels, 1 by default
    format: PixelFormat, // Data format (PixelFormat type)
}

#[repr(C)]
#[derive(Debug)]
pub struct Texture {
    id: c_uint,          // OpenGL texture idb
    pub width: c_int,    // Texture base width
    pub height: c_int,   // Texture base height
    mipmaps: c_int,      // Mipmap levels, 1 by default
    format: PixelFormat, // Data format (PixelFormat type)
}

impl From<&Mat> for Image {
    fn from(m: &Mat) -> Self {
        let f = match m.typ() {
            CV_8UC3 => PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8,
            _ => panic!("Type not supported {:#?}", m),
        };

        let size = m.size().unwrap();

        Self {
            data: m.ptr(0).unwrap() as *mut c_void,
            width: size.width,
            height: size.height,
            mipmaps: 1,
            format: f,
        }
    }
}

#[repr(C)]
struct RShader {
    id: c_uint,       // Shader program id
    locs: *mut c_int, // Shader locations array (RL_MAX_SHADER_LOCATIONS)
}

pub struct Shader<'a> {
    id: u32,
    locs: Option<&'a mut i32>,
}

#[repr(C)]
struct RColor {
    r: c_uchar,
    g: c_uchar,
    b: c_uchar,
    a: c_uchar,
}
#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let c = Self {
            r: (value >> 24) as u8,
            g: ((value >> 16) & 0xff) as u8,
            b: ((value >> 8) & 0xff) as u8,
            a: (value & 0xff) as u8,
        };
        c
    }
}

impl From<Color> for RColor {
    fn from(value: Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type uint = u32;

#[link(name = "raylib")]
extern "C" {
    fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    fn SetTargetFPS(fps: c_int); // Set target FPS (maximum)
    fn CloseWindow();
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: RColor);
    fn SetConfigFlags(config_flags: u32);
    fn LoadShader(vsFileName: *const c_char, fsFileName: *const c_char) -> RShader;
    fn LoadTextureFromImage(image: Image) -> Texture;
    fn DrawTexture(texture: &Texture, posX: c_int, posY: c_int, tint: RColor);
    fn UpdateTexture(texture: &mut Texture, pixels: *mut c_void);
    fn DrawFPS(posX: c_int, posY: c_int); // Draw current FPS
    fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: RColor); // Draw text (using default font)
    fn GetScreenWidth() -> c_int;
    fn GetScreenHeight() -> c_int;
    fn ToggleFullscreen();
    fn SetWindowSize(width: int, height: int);
    fn DrawTextureEx(
        texture: &Texture,
        position: RVector2,
        rotation: c_float,
        scale: c_float,
        tint: RColor,
    );
    // Check if a key has been pressed once
    fn IsKeyPressed(key: KeyboardKeys) -> bool;
    // Draw ring
    fn DrawRing(
        center: RVector2,
        innerRadius: c_float,
        outerRadius: c_float,
        startAngle: c_float,
        endAngle: c_float,
        segments: c_int,
        color: RColor,
    );

}

pub fn init_window(width: i32, height: i32, title: &str) {
    let c_title = CString::new(title).unwrap();
    unsafe {
        InitWindow(width, height, c_title.as_ptr());
    }
}

pub fn window_should_close() -> bool {
    unsafe { WindowShouldClose() }
}

pub fn begin_drawing() {
    unsafe {
        BeginDrawing();
    }
}
pub fn end_drawing() {
    unsafe {
        EndDrawing();
    }
}

pub fn close_window() {
    unsafe { CloseWindow() };
}

pub fn clear_background(color: Color) {
    unsafe { ClearBackground(color.into()) };
}

pub fn set_config_flags(config_flags: u32) {
    unsafe {
        SetConfigFlags(config_flags);
    };
}

pub fn load_shader(vertex_shader: Option<&String>, fragment_shader: Option<&String>) {
    unimplemented!("implement loading from disk")
}

pub fn set_target_fps(fps: int) {
    unsafe {
        SetTargetFPS(fps);
    }
}

pub fn load_texture_mat(m: &Mat) -> Texture {
    unsafe {
        let img: Image = m.into();
        let text = LoadTextureFromImage(img);
        return text;
    }
}

pub fn draw_texture(texture: &Texture, pos_x: int, pos_y: int, tint: Color) {
    unsafe {
        DrawTexture(texture, pos_x, pos_y, tint.into());
    }
}

pub fn draw_texture_ex(
    texture: &Texture,
    position: Vector2,
    rotation: f32,
    scale: f32,
    tint: Color,
) {
    unsafe {
        DrawTextureEx(
            texture,
            RVector2(position.0, position.1),
            rotation,
            scale,
            tint.into(),
        )
    };
}

pub fn update_texture(texture: &mut Texture, m: &Mat) {
    unsafe {
        UpdateTexture(texture, m.ptr(0).unwrap() as *mut c_void);
    }
}
pub fn draw_text(text: &str, pos_x: int, pos_y: int, font_size: int, color: Color) {
    let text = CString::new(text).unwrap();
    unsafe { DrawText(text.as_ptr(), pos_x, pos_y, font_size, color.into()) };
}

pub fn draw_fps(pos_x: int, pos_y: int) {
    unsafe { DrawFPS(pos_x, pos_y) };
}

pub fn get_screen_width() -> int {
    unsafe { GetScreenWidth() }
}
pub fn get_screen_height() -> int {
    unsafe { GetScreenHeight() }
}
pub fn toggle_fullscreen() {
    unsafe { ToggleFullscreen() }
}
pub fn set_window_size(width: int, height: int) {
    unsafe { SetWindowSize(width, height) }
}

pub fn is_key_pressed(key: KeyboardKeys) -> bool {
    unsafe { IsKeyPressed(key) }
}

pub fn draw_ring(
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: int,
    color: Color,
) {
    unsafe {
        DrawRing(
            center.into(),
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color.into(),
        )
    };
}
