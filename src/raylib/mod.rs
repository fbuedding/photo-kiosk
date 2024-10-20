#![allow(dead_code)]
pub mod color;
pub mod config_flags;
pub mod sys;
use std::{
    ffi::{c_void, CString},
    mem::{self, forget, ManuallyDrop},
    ops,
    os::raw::c_uint,
};

use color::Color;
use opencv::core::{Mat, MatTraitConst};
use sys::*;

#[allow(non_camel_case_types)]
pub type int = i32;
#[allow(non_camel_case_types)]
type uint = u32;

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
        SetConfigFlags(config_flags as c_uint);
    };
}

pub fn set_target_fps(fps: int) {
    unsafe {
        SetTargetFPS(fps);
    }
}

pub fn load_texture_mat(m: Mat) -> RTexture {
    unsafe {
        let img: RImage = (&m).into();

        LoadTextureFromImage(img)
    }
}

pub fn draw_texture(texture: &RTexture, pos_x: int, pos_y: int, tint: Color) {
    unsafe {
        DrawTexture(*texture, pos_x, pos_y, tint.into());
    }
}

pub fn draw_texture_ex(
    texture: &RTexture,
    position: Vector2,
    rotation: f32,
    scale: f32,
    tint: Color,
) {
    unsafe {
        DrawTextureEx(
            *texture,
            RVector2(position.0, position.1),
            rotation,
            scale,
            tint.into(),
        )
    };
}

pub fn update_texture(texture: &RTexture, m: &Mat) {
    unsafe {
        UpdateTexture(*texture, m.data() as *mut c_void);
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
pub fn load_image(file: &str) -> RImage {
    let file_type = CString::new(file).unwrap();
    unsafe { LoadImage(file_type.as_ptr()) }
}
pub fn load_image_from_memory(file_type: &str, file_data: &[u8], data_size: usize) -> RImage {
    let file_type = CString::new(file_type).unwrap();
    unsafe { LoadImageFromMemory(file_type.as_ptr(), file_data.as_ptr(), data_size as i32) }
}

pub fn load_texture_from_image(image: RImage) -> RTexture {
    unsafe { LoadTextureFromImage(image) }
}

/// Convenience function, calls begin_drawing() before and end_drawing() after the closure
pub fn draw<F: FnMut()>(mut f: F) {
    begin_drawing();
    f();
    end_drawing();
}

/// Keyboard keys (US keyboard layout)
// NOTE: Use GetKeyPressed() to allow redefining
// required keys for alternative layouts
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum KeyboardKeys {
    /// Key: NULL, used for no key pressed
    KEY_NULL = 0,
    // Alphanumeric keys
    /// Key: '
    KEY_APOSTROPHE = 39,
    /// Key: ,
    KEY_COMMA = 44,
    /// Key: -
    KEY_MINUS = 45,
    /// Key: .
    KEY_PERIOD = 46,
    /// Key: /
    KEY_SLASH = 47,
    /// Key: 0
    KEY_ZERO = 48,
    /// Key: 1
    KEY_ONE = 49,
    /// Key: 2
    KEY_TWO = 50,
    /// Key: 3
    KEY_THREE = 51,
    /// Key: 4
    KEY_FOUR = 52,
    /// Key: 5
    KEY_FIVE = 53,
    /// Key: 6
    KEY_SIX = 54,
    /// Key: 7
    KEY_SEVEN = 55,
    /// Key: 8
    KEY_EIGHT = 56,
    /// Key: 9
    KEY_NINE = 57,
    /// Key: ;
    KEY_SEMICOLON = 59,
    /// Key: =
    KEY_EQUAL = 61,
    /// Key: A | a
    KEY_A = 65,
    /// Key: B | b
    KEY_B = 66,
    /// Key: C | c
    KEY_C = 67,
    /// Key: D | d
    KEY_D = 68,
    /// Key: E | e
    KEY_E = 69,
    /// Key: F | f
    KEY_F = 70,
    /// Key: G | g
    KEY_G = 71,
    /// Key: H | h
    KEY_H = 72,
    /// Key: I | i
    KEY_I = 73,
    /// Key: J | j
    KEY_J = 74,
    /// Key: K | k
    KEY_K = 75,
    /// Key: L | l
    KEY_L = 76,
    /// Key: M | m
    KEY_M = 77,
    /// Key: N | n
    KEY_N = 78,
    /// Key: O | o
    KEY_O = 79,
    /// Key: P | p
    KEY_P = 80,
    /// Key: Q | q
    KEY_Q = 81,
    /// Key: R | r
    KEY_R = 82,
    /// Key: S | s
    KEY_S = 83,
    /// Key: T | t
    KEY_T = 84,
    /// Key: U | u
    KEY_U = 85,
    /// Key: V | v
    KEY_V = 86,
    /// Key: W | w
    KEY_W = 87,
    /// Key: X | x
    KEY_X = 88,
    /// Key: Y | y
    KEY_Y = 89,
    /// Key: Z | z
    KEY_Z = 90,
    /// Key: [
    KEY_LEFT_BRACKET = 91,
    /// Key: '\'
    KEY_BACKSLASH = 92,
    /// Key: ]
    KEY_RIGHT_BRACKET = 93,
    /// Key: `
    KEY_GRAVE = 96,
    // Function keys
    /// Key: Space
    KEY_SPACE = 32,
    /// Key: Esc
    KEY_ESCAPE = 256,
    /// Key: Enter
    KEY_ENTER = 257,
    /// Key: Tab
    KEY_TAB = 258,
    /// Key: Backspace
    KEY_BACKSPACE = 259,
    /// Key: Ins
    KEY_INSERT = 260,
    /// Key: Del
    KEY_DELETE = 261,
    /// Key: Cursor right
    KEY_RIGHT = 262,
    /// Key: Cursor left
    KEY_LEFT = 263,
    /// Key: Cursor down
    KEY_DOWN = 264,
    /// Key: Cursor up
    KEY_UP = 265,
    /// Key: Page up
    KEY_PAGE_UP = 266,
    /// Key: Page down
    KEY_PAGE_DOWN = 267,
    /// Key: Home
    KEY_HOME = 268,
    /// Key: End
    KEY_END = 269,
    /// Key: Caps lock
    KEY_CAPS_LOCK = 280,
    /// Key: Scroll down
    KEY_SCROLL_LOCK = 281,
    /// Key: Num lock
    KEY_NUM_LOCK = 282,
    /// Key: Print screen
    KEY_PRINT_SCREEN = 283,
    /// Key: Pause
    KEY_PAUSE = 284,
    /// Key: F1
    KEY_F1 = 290,
    /// Key: F2
    KEY_F2 = 291,
    /// Key: F3
    KEY_F3 = 292,
    /// Key: F4
    KEY_F4 = 293,
    /// Key: F5
    KEY_F5 = 294,
    /// Key: F6
    KEY_F6 = 295,
    /// Key: F7
    KEY_F7 = 296,
    /// Key: F8
    KEY_F8 = 297,
    /// Key: F9
    KEY_F9 = 298,
    /// Key: F10
    KEY_F10 = 299,
    /// Key: F11
    KEY_F11 = 300,
    /// Key: F12
    KEY_F12 = 301,
    /// Key: Shift left
    KEY_LEFT_SHIFT = 340,
    /// Key: Control left
    KEY_LEFT_CONTROL = 341,
    /// Key: Alt left
    KEY_LEFT_ALT = 342,
    /// Key: Super left
    KEY_LEFT_SUPER = 343,
    /// Key: Shift right
    KEY_RIGHT_SHIFT = 344,
    /// Key: Control right
    KEY_RIGHT_CONTROL = 345,
    /// Key: Alt right
    KEY_RIGHT_ALT = 346,
    /// Key: Super right
    KEY_RIGHT_SUPER = 347,
    /// Key: KB menu
    KEY_KB_MENU = 348,
    // Keypad keys
    /// Key: Keypad 0
    KEY_KP_0 = 320,
    /// Key: Keypad 1
    KEY_KP_1 = 321,
    /// Key: Keypad 2
    KEY_KP_2 = 322,
    /// Key: Keypad 3
    KEY_KP_3 = 323,
    /// Key: Keypad 4
    KEY_KP_4 = 324,
    /// Key: Keypad 5
    KEY_KP_5 = 325,
    /// Key: Keypad 6
    KEY_KP_6 = 326,
    /// Key: Keypad 7
    KEY_KP_7 = 327,
    /// Key: Keypad 8
    KEY_KP_8 = 328,
    /// Key: Keypad 9
    KEY_KP_9 = 329,
    /// Key: Keypad .
    KEY_KP_DECIMAL = 330,
    /// Key: Keypad /
    KEY_KP_DIVIDE = 331,
    /// Key: Keypad *
    KEY_KP_MULTIPLY = 332,
    /// Key: Keypad -
    KEY_KP_SUBTRACT = 333,
    /// Key: Keypad +
    KEY_KP_ADD = 334,
    /// Key: Keypad Enter
    KEY_KP_ENTER = 335,
    /// Key: Keypad =
    KEY_KP_EQUAL = 336,
    // Android key buttons
    /// Key: Android back button
    KEY_BACK = 4,
    /// Key: Android menu button
    KEY_MENU = 5,
    /// Key: Android volume up button
    KEY_VOLUME_UP = 24,
    /// Key: Android volume down button
    KEY_VOLUME_DOWN = 25,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2(pub f32, pub f32);

impl ops::Div<f32> for Vector2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Vector2(self.0 / rhs, self.1 / rhs)
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum PixelFormat {
    /// 8 bit per pixel (no alpha)
    PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    /// 8*2 bpp (2 channels)
    PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA,
    /// 16 bpp
    PIXELFORMAT_UNCOMPRESSED_R5G6B5,
    /// 24 bpp
    PIXELFORMAT_UNCOMPRESSED_R8G8B8,
    /// 16 bpp (1 bit alpha)
    PIXELFORMAT_UNCOMPRESSED_R5G5B5A1,
    /// 16 bpp (4 bit alpha)
    PIXELFORMAT_UNCOMPRESSED_R4G4B4A4,
    /// 32 bpp
    PIXELFORMAT_UNCOMPRESSED_R8G8B8A8,
    /// 32 bpp (1 channel - float)
    PIXELFORMAT_UNCOMPRESSED_R32,
    /// 32*3 bpp (3 channels - float)
    PIXELFORMAT_UNCOMPRESSED_R32G32B32,
    /// 32*4 bpp (4 channels - float)
    PIXELFORMAT_UNCOMPRESSED_R32G32B32A32,
    /// 16 bpp (1 channel - half float)
    PIXELFORMAT_UNCOMPRESSED_R16,
    /// 16*3 bpp (3 channels - half float)
    PIXELFORMAT_UNCOMPRESSED_R16G16B16,
    /// 16*4 bpp (4 channels - half float)
    PIXELFORMAT_UNCOMPRESSED_R16G16B16A16,
    /// 4 bpp (no alpha)
    PIXELFORMAT_COMPRESSED_DXT1_RGB,
    /// 4 bpp (1 bit alpha)
    PIXELFORMAT_COMPRESSED_DXT1_RGBA,
    /// 8 bpp
    PIXELFORMAT_COMPRESSED_DXT3_RGBA,
    /// 8 bpp
    PIXELFORMAT_COMPRESSED_DXT5_RGBA,
    /// 4 bpp
    PIXELFORMAT_COMPRESSED_ETC1_RGB,
    /// 4 bpp
    PIXELFORMAT_COMPRESSED_ETC2_RGB,
    /// 8 bpp
    PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA,
    /// 4 bpp
    PIXELFORMAT_COMPRESSED_PVRT_RGB,
    /// 4 bpp
    PIXELFORMAT_COMPRESSED_PVRT_RGBA,
    /// 8 bpp
    PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA,
    /// 2 bpp
    PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA,
}

#[derive(Debug)]
pub struct Texture {
    pub id: u32,             // OpenGL texture idb
    pub width: int,          // Texture base width
    pub height: int,         // Texture base height
    pub mipmaps: int,        // Mipmap levels, 1 by default
    pub format: PixelFormat, // Data format (PixelFormat type)
}

impl From<&Texture> for RTexture {
    fn from(texture: &Texture) -> Self {
        Self {
            id: texture.id,
            width: texture.width,
            height: texture.height,
            mipmaps: texture.mipmaps,
            format: texture.format,
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            UnloadTexture(RTexture {
                id: self.id,
                width: self.width,
                height: self.height,
                mipmaps: self.mipmaps,
                format: self.format,
            })
        };
    }
}

impl From<RTexture> for Texture {
    fn from(value: RTexture) -> Self {
        Self {
            id: value.id,
            width: value.width,
            height: value.height,
            mipmaps: value.mipmaps,
            format: value.format,
        }
    }
}

impl From<&Image> for Texture {
    fn from(image: &Image) -> Self {
        load_texture_from_image(RImage {
            data: image.image.data,
            width: image.image.width,
            height: image.image.height,
            mipmaps: image.image.mipmaps,
            format: image.image.format,
        })
        .into()
    }
}

impl From<Mat> for Texture {
    fn from(mat: Mat) -> Self {
        let image: Image = mat.into();
        (&image).into()
    }
}

impl Texture {
    pub fn update(&self, mat: &Mat) {
        update_texture(&self.into(), mat);
    }

    pub fn draw_ex(&self, position: Vector2, rotation: f32, scale: f32, tint: Color) {
        draw_texture_ex(&self.into(), position, rotation, scale, tint);
    }
    pub fn draw(&self, pos_x: int, pos_y: int, tint: Color) {
        draw_texture(&self.into(), pos_x, pos_y, tint);
    }
}

#[derive(Debug)]
pub struct Image {
    image: RImage,
    mat: Option<ManuallyDrop<Mat>>,
}

unsafe impl Send for Image {}

impl From<Mat> for Image {
    fn from(mat: Mat) -> Self {
        Self {
            image: (&mat).into(),
            mat: Some(ManuallyDrop::new(mat)),
        }
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Self {
            image: self.image.clone(),
            mat: None,
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        if let Some(mat) = &mut self.mat {
            // following values won't be leaked
            let _ = self.image.width;
            let _ = self.image.height;
            let _ = self.image.mipmaps;
            let _ = self.image.format;
            unsafe { ManuallyDrop::drop(mat) };
            println!("OpenCV mat unloaded")
        } else {
            unsafe {
                UnloadImage(RImage {
                    data: self.image.data,
                    width: self.image.width,
                    height: self.image.height,
                    mipmaps: self.image.mipmaps,
                    format: self.image.format,
                })
            };
            println!("Raylib image unloaded")
        }
    }
}

impl Image {
    pub fn new(file: &str) -> Self {
        Self {
            image: load_image(file),
            mat: None,
        }
    }

    pub fn new_from_memory(file_type: &str, file_data: &[u8]) -> Self {
        Image {
            image: load_image_from_memory(file_type, file_data, file_data.len()),
            mat: None,
        }
    }
}
