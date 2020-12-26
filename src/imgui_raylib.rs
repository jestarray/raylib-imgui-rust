use imgui::Context;
use imgui::{ClipboardBackend, FontConfig, FontGlyphRanges, FontSource, ImString, Key};
use prelude::RaylibDrawHandle;
use raylib::consts::KeyboardKey;
use raylib::consts::MouseButton;
use raylib::ffi::GetClipboardText;
use raylib::ffi::IsKeyDown;
use raylib::prelude;
use raylib::{ffi::SetClipboardText, RaylibHandle};
use std::ffi::{CStr, CString};
/*
    todo:
        maybe use rlgl to get the raw glfw window itself
*/
static KEYS: [u32; 105] = [
    39, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 59, 61, 65, 66, 67, 68, 69, 70, 71,
    72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 32, 256, 257, 258,
    259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 280, 281, 282, 283, 284, 290, 291, 292,
    293, 294, 295, 296, 297, 298, 299, 300, 301, 340, 341, 342, 343, 344, 345, 346, 347, 348, 91,
    92, 93, 96, 320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 330, 331, 332, 333, 334, 335,
    336,
];

#[derive(Debug, Copy, Clone)]
pub struct ImguiRl {
    mouse_press: [bool; 5],
    ignore_mouse: bool,
    ignore_keyboard: bool,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct InputState {
    is_backspace_down: bool,
    is_tab_down: bool,
}

#[inline]
fn is_key_down_number(key: u32) -> bool {
    unsafe { IsKeyDown((key as u32) as i32) }
}

struct Clipboard;

impl ClipboardBackend for Clipboard {
    fn get(&mut self) -> Option<ImString> {
        unsafe {
            let c = GetClipboardText();
            let c = CStr::from_ptr(c as *mut i8);
            match c.to_str().map(|s| s.to_owned()) {
                Ok(item) => Some(ImString::from(item)),
                Err(_) => None,
            }
        }
    }
    fn set(&mut self, value: &imgui::ImStr) {
        let s = CString::new(value.to_str());
        unsafe {
            match s {
                Ok(item) => {
                    SetClipboardText(item.as_ptr());
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
    }
}

impl ImguiRl {
    pub fn new(imgui: &mut Context) -> Self {
        imgui.set_ini_filename(None);
        imgui.set_clipboard_backend(Box::new(Clipboard));

        imgui.io_mut().key_map[Key::Tab as usize] = KeyboardKey::KEY_TAB as u32;
        imgui.io_mut().key_map[Key::LeftArrow as usize] = KeyboardKey::KEY_LEFT as u32;
        imgui.io_mut().key_map[Key::RightArrow as usize] = KeyboardKey::KEY_RIGHT as u32;
        imgui.io_mut().key_map[Key::UpArrow as usize] = KeyboardKey::KEY_UP as u32;
        imgui.io_mut().key_map[Key::DownArrow as usize] = KeyboardKey::KEY_DOWN as u32;
        imgui.io_mut().key_map[Key::PageUp as usize] = KeyboardKey::KEY_PAGE_UP as u32;
        imgui.io_mut().key_map[Key::PageDown as usize] = KeyboardKey::KEY_PAGE_DOWN as u32;
        imgui.io_mut().key_map[Key::Home as usize] = KeyboardKey::KEY_HOME as u32;
        imgui.io_mut().key_map[Key::End as usize] = KeyboardKey::KEY_END as u32;
        imgui.io_mut().key_map[Key::Delete as usize] = KeyboardKey::KEY_DELETE as u32;
        imgui.io_mut().key_map[Key::Backspace as usize] = KeyboardKey::KEY_BACKSPACE as u32;
        imgui.io_mut().key_map[Key::Enter as usize] = KeyboardKey::KEY_ENTER as u32;
        imgui.io_mut().key_map[Key::Escape as usize] = KeyboardKey::KEY_ESCAPE as u32;
        imgui.io_mut().key_map[Key::Space as usize] = KeyboardKey::KEY_SPACE as u32;
        imgui.io_mut().key_map[Key::C as usize] = KeyboardKey::KEY_C as u32;
        imgui.io_mut().key_map[Key::A as usize] = KeyboardKey::KEY_A as u32;
        imgui.io_mut().key_map[Key::V as usize] = KeyboardKey::KEY_V as u32;
        imgui.io_mut().key_map[Key::X as usize] = KeyboardKey::KEY_X as u32;
        imgui.io_mut().key_map[Key::Y as usize] = KeyboardKey::KEY_Y as u32;
        imgui.io_mut().key_map[Key::Z as usize] = KeyboardKey::KEY_Z as u32;

        /*       let font_size = 16.0;
               imgui.fonts().add_font(&[
                   FontSource::TtfData {
                       data: include_bytes!("../assets/SourceCodePro-Regular.ttf"),
                       size_pixels: font_size,
                       config: Some(FontConfig {
                           rasterizer_multiply: 1.75,
                           glyph_ranges: FontGlyphRanges::japanese(),
                           ..FontConfig::default()
                       }),
                   },
                   FontSource::DefaultFontData {
                       config: Some(FontConfig {
                           size_pixels: font_size,
                           ..FontConfig::default()
                       }),
                   },
               ]);
        */
        Self {
            mouse_press: [false; 5],
            ignore_keyboard: false,
            ignore_mouse: false,
        }
    }
    pub fn update_input(&self, imgui: &mut Context, rl: &RaylibHandle) {
        imgui.io_mut().keys_down[KeyboardKey::KEY_BACKSPACE as usize] =
            rl.is_key_down(KeyboardKey::KEY_BACKSPACE);

        imgui.io_mut().keys_down[KeyboardKey::KEY_TAB as usize] =
            rl.is_key_down(KeyboardKey::KEY_TAB);

        for key in KEYS.iter() {
            let key = *key;
            imgui.io_mut().keys_down[key as usize] = is_key_down_number(key);
        }

        match rl.get_key_pressed_number() {
            Some(pressed) => {
                imgui.io_mut().add_input_character(pressed as u8 as char);
            }
            None => {}
        };
    }

    pub fn prepare_frame(&mut self, io: &mut imgui::Io, rl: &RaylibDrawHandle, scale: f32) {
        // take in all of these individually and not the handle
        let window_width = rl.get_screen_width();
        let window_height = rl.get_screen_height();
        let left_click = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);
        let right_click = rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON);
        let middle_click = rl.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON);
        let screen_mouse_position = rl.get_mouse_position();
        let mouse_scroll = rl.get_mouse_wheel_move(); // -1 for down, 0 for none, 1 for up

        io.delta_time = rl.get_frame_time();

        io.display_size = [window_width as f32, window_height as f32];

        //io.font_global_scale = scale;

        io.mouse_down = [left_click, right_click, middle_click, false, false];
        io.mouse_pos = [screen_mouse_position.x, screen_mouse_position.y];
        io.mouse_wheel = mouse_scroll as f32;

        //self.ignore_keyboard = io.want_capture_keyboard;
        //self.ignore_mouse = io.want_capture_mouse;
        io.want_capture_keyboard = false;
        io.want_capture_mouse = false;
    }
}
