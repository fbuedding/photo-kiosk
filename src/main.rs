use core::panic;
use std::error::Error;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::{self};
use std::time::Instant;

use color::{RED, WHITE};
use config_flags::FLAG_WINDOW_RESIZABLE;
use display_options::{FILL, SHOW_DEBUG_IMAGE, SHOW_DEBUG_INFO};
use opencv::core::{flip, Vector};
use opencv::imgcodecs::imwrite;
use opencv::imgproc::{cvt_color, COLOR_BGR2RGB};
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};
use peak_alloc::PeakAlloc;

mod raylib;
use crate::raylib::*;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

enum State {
    Startup,
    Idle,
    Countdown(Instant),
    Capturing,
    Waiting,
    Presenting { texture: Texture, start: Instant },
}

struct KioskState {
    state: State,
}

impl KioskState {
    fn poll(&mut self) {
        match &self.state {
            State::Startup => self.state = State::Idle,
            State::Idle => {}
            State::Countdown(i) => {
                let duration = i.elapsed().as_secs_f32();
                if duration > 5. {
                    self.state = State::Capturing
                }
            }
            State::Capturing => self.state = State::Waiting,
            State::Waiting => {}
            State::Presenting { start, .. } => {
                let duration = start.elapsed().as_secs_f32();
                if duration > 5. {
                    self.state = State::Idle;
                }
            }
        };
    }
    fn new() -> Self {
        Self {
            state: State::new(),
        }
    }
}

impl State {
    fn new() -> Self {
        Self::Startup
    }
}

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

fn get_new_display_frame(cap: &mut VideoCapture) -> Result<Option<Mat>, Box<dyn Error>> {
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
    pub const SHOW_DEBUG_INFO: u32 = 1;
    pub const FILL: u32 = 2;
    pub const SHOW_DEBUG_IMAGE: u32 = 4;
}

enum Commands {
    Stop,
    Capture,
}

fn main() {
    let (capture_command_tx, capture_command_rx) = channel::<Commands>();
    let (captured_img_tx, captured_img_rx) = channel::<Image>();

    let mut state = KioskState::new();
    let debug_img = include_bytes!("img/test.png");

    let debug_img = Image::new_from_memory(".png", debug_img);

    let Ok(mut display_cap) = VideoCapture::new(0, CAP_ANY) else {
        panic!("Could not open VideoCapture");
    };
    if !display_cap.is_opened().unwrap() {
        panic!("Could not open camera")
    }

    let _ = display_cap.set(CAP_PROP_FRAME_WIDTH, 1024.);
    let _ = display_cap.set(CAP_PROP_FRAME_HEIGHT, 576.);

    let Ok(mut record_cap) = VideoCapture::new(0, CAP_ANY) else {
        panic!("Could not open VideoCapture");
    };
    if !record_cap.is_opened().unwrap() {
        panic!("Could not open camera")
    }

    let _ = record_cap.set(CAP_PROP_FRAME_WIDTH, 1920.);
    let _ = record_cap.set(CAP_PROP_FRAME_HEIGHT, 1080.);

    let frame = WebcamFrame::new(Mat::default());

    let frame = Arc::new(Mutex::new(frame));

    let capture_frame = Arc::clone(&frame);
    let mut start = Instant::now();
    let handle = thread::spawn(move || loop {
        if let Ok(command) = capture_command_rx.try_recv() {
            match command {
                Commands::Stop => break,
                Commands::Capture => {
                    if let Ok(Some(picture)) = get_new_display_frame(&mut record_cap) {
                        let param = Vector::new();
                        let _ = imwrite("test.png", &picture, &param);
                        let image: Image = picture.into();
                        captured_img_tx.send(image).expect("Could not send");
                    }
                }
            }
        }
        if let Ok(Some(new_frame)) = get_new_display_frame(&mut display_cap) {
            if new_frame.empty() {
                panic!("Webcam frame should not be empty");
            }
            if let Ok(mut frame) = capture_frame.lock() {
                frame.frame = new_frame;

                frame.delta_times.copy_within(1..10, 0);
                frame.delta_times[9] = start.elapsed().as_secs_f32();
                start = Instant::now();
            };
        };
    });
    let mut display_options_state = FILL;

    // For development in drm it dowsn't even work
    set_config_flags(FLAG_WINDOW_RESIZABLE);

    init_window(1024, 600, "Susi");

    while frame.lock().unwrap().frame.typ() != 16 {}
    {}

    let debug_texture = Texture::from(&debug_img);
    let texture: Texture;
    if let Ok(frame) = frame.lock() {
        let img = Image::from(frame.frame.clone());
        texture = Texture::from(&img);
    } else {
        panic!("Different thread panicked");
    }
    while !window_should_close() {
        state.poll();
        if is_key_pressed(KeyboardKeys::KEY_F) {
            display_options_state ^= FILL;
        }
        if is_key_pressed(KeyboardKeys::KEY_D) {
            display_options_state ^= SHOW_DEBUG_INFO;
        }
        if is_key_pressed(KeyboardKeys::KEY_C) {
            state.state = State::Countdown(Instant::now());
        }
        if is_key_pressed(KeyboardKeys::KEY_I) {
            display_options_state ^= SHOW_DEBUG_IMAGE;
        }

        let mut webcam_fps: f32 = 0.;
        let screen_size = Vector2(get_screen_width() as f32, get_screen_height() as f32);

        if let Ok(frame) = frame.lock() {
            if display_options_state & SHOW_DEBUG_IMAGE == 0 {
                texture.update(&frame.frame);
            }
            webcam_fps = frame.avg_fps();
        }

        let texture = if display_options_state & SHOW_DEBUG_IMAGE != 0 {
            &debug_texture
        } else {
            &texture
        };

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
        begin_drawing();
        clear_background(WHITE);
        match &state.state {
            State::Startup => todo!(),
            State::Idle => {
                texture.draw_ex(pos, 0., scale, WHITE);

                draw_ring(ring_position, 30., ring_outer_radius, 0., 360., 360, WHITE);
            }
            State::Countdown(instant) => {
                texture.draw_ex(pos, 0., scale, WHITE);
                draw_text(
                    &format!("{}", instant.elapsed().as_secs()),
                    (screen_size.0 / 2. - 50.).round() as i32,
                    (screen_size.1 / 2. - 50.).round() as i32,
                    100,
                    RED,
                );
            }
            State::Capturing => {
                let _ = capture_command_tx.send(Commands::Capture);
            }
            State::Waiting => {
                if let Ok(image) = captured_img_rx.try_recv() {
                    state.state = State::Presenting {
                        texture: (&image).into(),
                        start: Instant::now(),
                    }
                }
            }

            State::Presenting { texture, .. } => {
                texture.draw(0, 0, WHITE);
            }
        }
        if display_options_state & SHOW_DEBUG_IMAGE != 0 {
            texture.draw(0, 0, WHITE);
        }

        if (display_options_state & SHOW_DEBUG_INFO) != 0 {
            draw_debug_info(texture, webcam_fps);
        }
        end_drawing();
    }
    let _ = capture_command_tx.send(Commands::Stop);
    handle.join().unwrap();
    close_window();
}

fn get_new_record_frame(record_cap: &mut VideoCapture) -> Result<Option<Mat>, Box<dyn Error>> {
    let mut frame = Mat::default();
    let mut frame_1 = Mat::default();

    let res = match record_cap.read(&mut frame) {
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

    Ok(Some(frame_1))
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
