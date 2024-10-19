#![allow(dead_code)]
use std::ffi::{c_char, c_float, c_int, c_uchar, c_uint, c_void};

#[link(name = "raylib")]
extern "C" {
    pub(super) fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    pub(super) fn SetTargetFPS(fps: c_int); // Set target FPS (maximum)
    pub(super) fn CloseWindow();
    pub(super) fn WindowShouldClose() -> bool;
    pub(super) fn BeginDrawing();
    pub(super) fn EndDrawing();
    pub(super) fn ClearBackground(color: RColor);
    pub(super) fn SetConfigFlags(config_flags: u32);
    pub(super) fn LoadShader(vsFileName: *const c_char, fsFileName: *const c_char) -> RShader;
    pub(super) fn LoadTextureFromImage(image: RImage) -> RTexture;
    pub(super) fn DrawTexture(texture: RTexture, posX: c_int, posY: c_int, tint: RColor);
    pub(super) fn UpdateTexture(texture: RTexture, pixels: *mut c_void);
    pub(super) fn DrawFPS(posX: c_int, posY: c_int); // Draw current FPS
    pub(super) fn DrawText(
        text: *const c_char,
        posX: c_int,
        posY: c_int,
        fontSize: c_int,
        color: RColor,
    ); // Draw text (using default font)
    pub(super) fn GetScreenWidth() -> c_int;
    pub(super) fn GetScreenHeight() -> c_int;
    pub(super) fn ToggleFullscreen();
    pub(super) fn SetWindowSize(width: c_int, height: c_int);
    pub(super) fn DrawTextureEx(
        texture: RTexture,
        position: RVector2,
        rotation: c_float,
        scale: c_float,
        tint: RColor,
    );
    // Check if a key has been pressed once
    pub(super) fn IsKeyPressed(key: KeyboardKeys) -> bool;
    // Draw ring
    pub(super) fn DrawRing(
        center: RVector2,
        innerRadius: c_float,
        outerRadius: c_float,
        startAngle: c_float,
        endAngle: c_float,
        segments: c_int,
        color: RColor,
    );
    pub(super) fn LoadImageFromMemory(
        fileType: *const c_char,
        fileData: *const c_uchar,
        dataSize: c_int,
    ) -> RImage;
    pub(super) fn LoadImage(fileName: *const c_char) -> RImage;

    /// Unload texture from GPU memory (VRAM)
    pub fn UnloadTexture(texture: RTexture);
    /// Create an image duplicate (useful for transformations)
    fn ImageCopy(image: RImage) -> RImage;
    /// Unload image from CPU memory (RAM)
    pub fn UnloadImage(image: RImage);

}

use opencv::core::{Mat, MatTraitConst, CV_8UC3};

use super::{KeyboardKeys, PixelFormat, Vector2};
/// Gesture
/// NOTE: Provided as bit-wise flags to enable only desired gestures2
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

#[repr(C)]
pub struct RVector2(pub c_float, pub c_float);

impl From<Vector2> for RVector2 {
    fn from(value: Vector2) -> Self {
        Self(value.0, value.1)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RImage {
    pub data: *mut c_void,
    pub width: c_int,        // Image base width
    pub height: c_int,       // Image base height
    pub mipmaps: c_int,      // Mipmap levels, 1 by default
    pub format: PixelFormat, // Data format (PixelFormat type)
}

unsafe impl Send for RImage {}

impl Clone for RImage {
    fn clone(&self) -> Self {
        unsafe {
            ImageCopy(RImage {
                data: self.data,
                width: self.width,
                height: self.height,
                mipmaps: self.mipmaps,
                format: self.format,
            })
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RTexture {
    pub id: c_uint,          // OpenGL texture idb
    pub width: c_int,        // Texture base width
    pub height: c_int,       // Texture base height
    pub mipmaps: c_int,      // Mipmap levels, 1 by default
    pub format: PixelFormat, // Data format (PixelFormat type)
}

impl From<&Mat> for RImage {
    fn from(m: &Mat) -> Self {
        let f = match m.typ() {
            CV_8UC3 => PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8,
            _ => panic!("Matrix Type not supported\n {:#?}", m),
        };

        let size = m.size().unwrap();
        let data = m.data() as *mut c_void;

        Self {
            data,
            width: size.width,
            height: size.height,
            mipmaps: 1,
            format: f,
        }
    }
}

#[repr(C)]
pub struct RShader {
    id: c_uint,       // Shader program id
    locs: *mut c_int, // Shader locations array (RL_MAX_SHADER_LOCATIONS)
}

#[repr(C)]
pub struct RColor {
    pub r: c_uchar,
    pub g: c_uchar,
    pub b: c_uchar,
    pub a: c_uchar,
}
