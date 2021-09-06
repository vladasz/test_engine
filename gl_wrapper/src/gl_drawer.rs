#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Context, Window, WindowEvent};
use gm::{Point, Size};
use tools::new;

use crate::{GLLoader, Screen};

pub struct GLDrawer<ScreenT: Screen> {
    window: Window,
    events: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
    size:   Size,
    screen: ScreenT,
}

impl<ScreenT: Screen> GLDrawer<ScreenT> {
    pub fn with_size(size: Size) -> GLDrawer<ScreenT> {
        let loader = GLLoader::with_size(size);
        GLDrawer {
            window: loader.window,
            events: loader.events,
            size,
            screen: new(),
        }
    }

    pub fn update(&mut self) { self.screen.update(); }

    pub fn start_main_loop(&mut self) {
        self.screen.init();

        self.window.set_key_polling(true);
        self.window.set_size_polling(true);
        self.window.set_cursor_pos_polling(true);
        self.window.set_mouse_button_polling(true);

        self.screen.set_size(self.size);

        while !self.window.should_close() {
            self.window.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(key, _, action, _) => {
                        if key == glfw::Key::Escape {
                            self.window.set_should_close(true)
                        }
                        self.screen.on_key_pressed(key, action);
                    }
                    glfw::WindowEvent::CursorPos(xpos, ypos) => {
                        self.screen.on_cursor_moved(Point {
                            x: xpos as f32,
                            y: ypos as f32,
                        });
                    }
                    glfw::WindowEvent::Size(width, height) => {
                        self.screen.set_size(Size {
                            width:  width as f32,
                            height: height as f32,
                        });
                    }
                    glfw::WindowEvent::MouseButton(btn, action, _) => {
                        self.screen.on_mouse_key_pressed(btn, action)
                    }
                    _ => {}
                }
            }

            self.update();
            self.window.swap_buffers();
        }
    }
}
