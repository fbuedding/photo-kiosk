use std::error::Error;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use display_options::{FILL, SHOW_DEBUG_INFO};
use opencv::core::flip;
use opencv::imgproc::{cvt_color, COLOR_BGR2RGB};
use opencv::prelude::*;
mod raylib;
use opencv::videoio::{VideoCapture, CAP_ANY, CAP_PROP_BUFFERSIZE, CAP_PROP_FPS};
use peak_alloc::PeakAlloc;
use raylib::config_flags::*;

use colors::{RED, WHITE};
use raylib::*;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

struct WebcamFrame {
    frame: Mat,
    delta_times: [f32; 10],
}

impl WebcamFrame {
    fn new(frame: Mat) -> Self {
        Self {
            frame,
            delta_times: [0.; 10],
        }
    }
    fn avg_fps(&self) -> f32 {
        let avg_delta_time: f32 =
            self.delta_times.iter().sum::<f32>() / self.delta_times.len() as f32;
        f32::round(1. / avg_delta_time)
    }
}

fn get_new_frame(cap: &mut VideoCapture) -> Result<Option<Mat>, Box<dyn Error>> {
    let mut frame = Mat::default();
    let mut frame_1 = Mat::default();

    let res = match cap.read(&mut frame) {
        Err(e) => return Err(Box::new(e)),
        Ok(b) => b,
    };

    if !res {
        return Ok(None);
    }

    match flip(&frame, &mut frame_1, 1) {
        Ok(_) => (),
        Err(e) => return Err(Box::new(e)),
    };

    match cvt_color(&frame_1, &mut frame, COLOR_BGR2RGB, 0) {
        Ok(_) => (),
        Err(e) => return Err(Box::new(e)),
    };

    Ok(Some(frame))
}

mod display_options {
    pub const SHOW_DEBUG_INFO: u32 = 0b0000_0001;
    pub const FILL: u32 = 0b0000_0010;
}
fn main() {
    let thread_should_stop = Arc::new(AtomicBool::new(false));
    let mut cap = VideoCapture::new(0, CAP_ANY).unwrap();
    if !cap.is_opened().unwrap() {
        panic!("Could not open camera")
    }
    let frame = WebcamFrame::new(get_new_frame(&mut cap).unwrap().unwrap());
    let frame = Arc::new(Mutex::new(frame));

    let capture_frame = Arc::clone(&frame);
    let capture_thread_should_stop = thread_should_stop.clone();
    let mut start = Instant::now();
    let handle = thread::spawn(move || loop {
        if capture_thread_should_stop.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
        if let Ok(new_frame) = get_new_frame(&mut cap) {
            match new_frame {
                Some(new_frame) => {
                    if let Ok(mut frame) = capture_frame.lock() {
                        (*frame).frame = new_frame;

                        (*frame).delta_times.copy_within(1..10, 0);
                        (*frame).delta_times[9] = start.elapsed().as_secs_f32();
                        start = Instant::now();
                    };
                }

                None => {}
            }
        };
    });

    let mut display_options_state = FILL;

    set_config_flags(FLAG_WINDOW_RESIZABLE | FLAG_WINDOW_ALWAYS_RUN);
    init_window(1080, 720, "Susi");

    let mut texture = load_texture_mat(&frame.lock().unwrap().frame);
    while !window_should_close() {
        let mut webcam_fps: f32 = 0.;
        let screen_size = Vector2(get_screen_width() as f32, get_screen_height() as f32);

        if let Ok(frame) = frame.lock() {
            update_texture(&mut texture, &frame.frame);
            webcam_fps = frame.avg_fps();
        }

        if is_key_pressed(KeyboardKeys::KEY_F) {
            display_options_state ^= FILL;
        }
        if is_key_pressed(KeyboardKeys::KEY_D) {
            display_options_state ^= SHOW_DEBUG_INFO;
        }

        let scale = if (display_options_state & FILL) != 0 {
            f32::max(
                screen_size.0 / texture.width as f32,
                screen_size.1 / texture.height as f32,
            )
        } else {
            f32::min(
                screen_size.0 / texture.width as f32,
                screen_size.1 / texture.height as f32,
            )
        };
        let pos = Vector2(
            (screen_size.0 - texture.width as f32 * scale) / 2.,
            (screen_size.1 - texture.height as f32 * scale) / 2.,
        );

        let mut ring_position = screen_size / 2.0;
        let ring_outer_radius = 40.;
        let ring_bottum_padding = 40.;
        ring_position.1 = screen_size.1 - ring_outer_radius - ring_bottum_padding;

        //------------------------drawing----------------------------
        begin_drawing();
        clear_background(0xa9_a9_a9_ff.into());
        draw_texture_ex(&texture, pos, 0., scale, WHITE);

        draw_ring(ring_position, 30., ring_outer_radius, 0., 360., 360, WHITE);

        if (display_options_state & SHOW_DEBUG_INFO) != 0 {
            draw_debug_info(&texture, webcam_fps);
        }
        //------------------------end drawing----------------------------
        end_drawing();
    }
    thread_should_stop.swap(true, std::sync::atomic::Ordering::Relaxed);
    handle.join().unwrap();
    close_window();
}

fn draw_debug_info(texture: &Texture, webcam_fps: f32) {
    draw_fps(5, 5);
    draw_text(
        &format!(
            "Currently uses {} KB of RAM.",
            PEAK_ALLOC.current_usage_as_kb()
        ),
        5,
        25,
        20,
        RED,
    );
    draw_text(
        &format!(
            "The max amount that was used {} KB",
            PEAK_ALLOC.peak_usage_as_kb()
        ),
        5,
        45,
        20,
        RED,
    );
    draw_text(
        &format!(
            "Screen size: {}, {}",
            get_screen_width(),
            get_screen_height()
        ),
        5,
        65,
        20,
        RED,
    );
    draw_text(
        &format!("Webcam capture size: {}, {}", texture.width, texture.height),
        5,
        85,
        20,
        RED,
    );
    draw_text(&format!("Webcam FPS:  {}", webcam_fps), 5, 105, 20, RED);
}
