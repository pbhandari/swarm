extern crate libc;
extern crate xlib;
extern crate xinerama;

use window_system::WindowSystem;
mod window_system;

fn main() {
    let window_system = WindowSystem::new();
     loop { /*NO OP */ }
}
