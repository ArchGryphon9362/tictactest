#![allow(dead_code)]

use std::ffi::CString;
use fermium::prelude::*;
pub struct Window {
    window: *mut SDL_Window
}

impl Window {
    /// Initializes the Window object
    ///
    /// # Arguments
    /// - name: String
    /// - width: i32
    /// - height: i32
    /// - x_pos: i32 - Must be one of the window positions from the SDL library
    /// - y_pos: i32 - Must be one of the window positions from the SDL library
    ///
    /// # Returns
    /// Returns a new Window object
    pub fn new(name: String, width: i32, height: i32, x_pos: i32, y_pos: i32) -> Self {
        let name_cstr = CString::new(name).unwrap();
        let window;

        unsafe {
            if SDL_Init(SDL_INIT_VIDEO) < 0 {
                panic!("SDL_VideoInit Error");
            }
            window = SDL_CreateWindow(name_cstr.as_ptr(), x_pos, y_pos, width, height, 0);
            let window_surface = SDL_GetWindowSurface(window);

            SDL_UpdateWindowSurface(window);
            SDL_Delay(5000);
        }
        Window {
            window
        }
    }
}
