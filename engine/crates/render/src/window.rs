use glfw::{fail_on_errors, Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};


pub struct Window {
    glfw: Glfw,
    handle: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut handle, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        handle.make_current();
        handle.set_framebuffer_size_polling(true);
        handle.set_key_polling(true);

        let mut window = Self {
            glfw,
            handle,
            events
        };

        gl::load_with(|s| window.handle.get_proc_address(s) as *const _);

        window
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                _ => {}
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self.handle.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.handle.swap_buffers();
    }
}
