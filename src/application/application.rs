use glfw::Context;

use super::window::Window;

pub trait Application: Sized {
    fn new(width: u32, height: u32, title: &str) -> Self;
    fn window(&self) -> &Window;
    fn window_mut(&mut self) -> &mut Window;
    fn delta_time(&self) -> f32;
    fn delta_time_mut(&mut self) -> &mut f32;
    fn current_time(&self) -> f32;
    fn current_time_mut(&mut self) -> &mut f32;

    fn run(mut self) {
        // If the application is not in error state, run

        self.initialize();

        // current time when the application started
        let start_time = std::time::Instant::now();

        // Main loop
        while self.is_running() {
            // set current time relative to start time
            let duration = std::time::Instant::now() - start_time;
            self.update_time(duration.as_secs_f32());

            self.update();

            self.render();

            // Swap buffers and poll events at the end of the frame
            self.window_mut().inner_window.swap_buffers();
            self.window_mut().glfw_mut().poll_events();
        }
    }
    fn initialize(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn is_running(&self) -> bool {
        !self.window().inner_window.should_close()
    }
    fn update_time(&mut self, new_current_time: f32) {
        *self.delta_time_mut() = new_current_time - self.current_time();
        *self.current_time_mut() = new_current_time;
    }
}
