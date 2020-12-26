use imgui::{im_str, ClipboardBackend, CollapsingHeader, Condition, ImString, Key, Window};
use raylib::ffi::TextureFilterMode::FILTER_BILINEAR;
use raylib::prelude::*;
use std::{
    convert::TryInto,
    ffi::{CStr, CString},
};
mod imgui_raylib;
use imgui_raylib::ImguiRl;

const GAME_WIDTH: u32 = 640;
const GAME_HEIGHT: u32 = 480;
const GAME_HALF_WIDTH: u32 = GAME_WIDTH / 2;
const GAME_HALF_HEIGHT: u32 = GAME_HEIGHT / 2;

extern "C" {
    fn glfwGetProcAddress(s: *const u8) -> extern "C" fn();
    fn rlGetVersion() -> i32;
    fn rlglClose();
    fn rlglDraw();
    fn gl(x: i32) -> bool;
}
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(GAME_WIDTH as i32, GAME_HEIGHT as i32)
        .title("example")
        .resizable()
        .vsync()
        .build();

    let mut imgui = imgui::Context::create();

    let renderer = unsafe {
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
            let s = CString::new(s).unwrap();
            //  dbg!(&s);
            glfwGetProcAddress(s.as_ptr() as *const u8) as _
        })
    };
    let mut imgui_rl = ImguiRl::new(&mut imgui);

    while !rl.window_should_close() {
        let scale = (rl.get_screen_width() as f32 / GAME_WIDTH as f32)
            .min(rl.get_screen_height() as f32 / GAME_HEIGHT as f32);
        let right = rl.is_key_down(KeyboardKey::KEY_D);
        let down = rl.is_key_down(KeyboardKey::KEY_S);
        let left = rl.is_key_down(KeyboardKey::KEY_A);
        let up = rl.is_key_down(KeyboardKey::KEY_W);
        let is_backspace_down = rl.is_key_down(KeyboardKey::KEY_BACKSPACE);
        let is_tab_down = rl.is_key_down(KeyboardKey::KEY_TAB);
        let debug = rl.is_key_down(KeyboardKey::KEY_F3);
        let window_width = rl.get_screen_width();
        let window_height = rl.get_screen_height();
        let left_click = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);
        let right_click = rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON);
        let middle_click = rl.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON);
        let screen_mouse_position = rl.get_mouse_position();
        let mouse_scroll = rl.get_mouse_wheel_move(); // -1 for down, 0 for none, 1 for up
        imgui_rl.update_input(&mut imgui, &rl);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GREEN);
        if debug {
            println!(
                "mouse_x: {} mousey: {}",
                screen_mouse_position.x, screen_mouse_position.y
            );
            println!("width: {} height: {}", window_width, window_height);
            dbg!(mouse_scroll);
        }
        d.draw_rectangle(0, 0, 640, 480, Color::PINK);
        unsafe {
            rlglDraw(); // must be called to fix z issue of the imgui rendering behind the game
        }
        imgui_rl.prepare_frame(imgui.io_mut(), &d, 1.0);
        let ui = imgui.frame();
        ui.show_demo_window(&mut true);

        renderer.render(ui);
    }
}
