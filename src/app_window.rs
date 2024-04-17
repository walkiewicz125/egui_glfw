#![warn(clippy::all)]
#![allow(clippy::single_match)]

use std::{sync::mpsc::Receiver, time::Instant};

use egui::{vec2, Pos2, Rect};
use glfw::{Context, Glfw, WindowEvent, WindowHint};

use crate::painter;
use painter::Painter;

use crate::back_end;
use back_end::{copy_to_clipboard, handle_event, EguiInputState};

pub struct AppWindow {
    pub glfw: Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, WindowEvent)>,
    pub painter: Painter,
    pub egui_context: egui::Context,
    pub egui_input_state: EguiInputState,
    pub start_time: Instant,
}

impl AppWindow {
    const DEFAULT_WINDOW_HINTS: [WindowHint; 5] = [
        glfw::WindowHint::ContextVersion(4, 3),
        glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core),
        glfw::WindowHint::DoubleBuffer(true),
        glfw::WindowHint::Resizable(true),
        glfw::WindowHint::Samples(Some(8)),
    ];

    pub fn new_default(screen_width: u32, scree_height: u32) -> AppWindow {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        Self::DEFAULT_WINDOW_HINTS.iter().for_each(|hint| {
            glfw.window_hint(hint.clone());
        });

        let (mut window, events) = glfw
            .create_window(
                screen_width,
                scree_height,
                "Egui in GLFW!",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.set_char_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_size_polling(true);
        window.make_current();

        gl::load_with(|s| window.get_proc_address(s));
        unsafe { gl::Enable(gl::MULTISAMPLE) };

        let painter = Painter::new(&mut window);
        let egui_context = egui::Context::default();

        let (width, height) = window.get_framebuffer_size();
        let native_pixels_per_point = window.get_content_scale().0;

        let egui_input_state = EguiInputState::new(egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(width as f32, height as f32) / native_pixels_per_point,
            )),
            ..Default::default() // todo: add pixels_per_point
        });

        AppWindow {
            glfw,
            window,
            events,
            painter,
            egui_context,
            egui_input_state,
            start_time: Instant::now(),
        }
    }

    pub fn get_window_mut(&mut self) -> &mut glfw::Window {
        &mut self.window
    }

    pub fn get_window(&self) -> &glfw::Window {
        &self.window
    }

    pub fn begin_frame(&mut self) {
        self.egui_input_state.input.time = Some(self.start_time.elapsed().as_secs_f64());
        self.egui_context
            .begin_frame(self.egui_input_state.input.take());
    }

    pub fn end_frame(&mut self) {
        let native_pixels_per_point = self.window.get_content_scale().0;
        self.egui_context
            .set_pixels_per_point(native_pixels_per_point);
        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point: native_pixels_per_point,
            viewport_output: _,
        } = self.egui_context.end_frame();

        //Handle cut, copy text from egui
        if !platform_output.copied_text.is_empty() {
            copy_to_clipboard(&mut self.egui_input_state, platform_output.copied_text);
        }

        //Note: passing a bg_color to paint_jobs will clear any previously drawn stuff.
        //Use this only if egui is being used for all drawing and you aren't mixing your own Open GL
        //drawing calls with it.
        let clipped_shapes: Vec<egui::ClippedPrimitive> = self
            .egui_context
            .tessellate(shapes, native_pixels_per_point);

        self.painter
            .paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Close => self.window.set_should_close(true),
                glfw::WindowEvent::Size(width, height) => {
                    self.painter.set_size(width as _, height as _);
                    handle_event(event, &mut self.egui_input_state);
                }
                glfw::WindowEvent::ContentScale(x, y) => {
                    print!("{} {}", x, y)
                }
                glfw::WindowEvent::FramebufferSize(x, y) => {
                    print!("{} {}", x, y)
                }
                _ => {
                    // println!("{:?}", event);
                    handle_event(event, &mut self.egui_input_state);
                }
            }
        }

        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn get_egui_context(&self) -> &egui::Context {
        &self.egui_context
    }
}
