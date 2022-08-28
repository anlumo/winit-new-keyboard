#![allow(clippy::single_match)]

use simple_logger::SimpleLogger;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::Key;
use winit::monitor::{MonitorHandle, VideoMode};
use winit::window::{Fullscreen, WindowBuilder};

fn main() {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    let mut decorations = true;
    let mut minimized = false;

    let window = WindowBuilder::new()
        .with_title("Hello world!")
        .build(&event_loop)
        .unwrap();

    let mut monitor_index = 0;
    let mut monitor = event_loop
        .available_monitors()
        .next()
        .expect("no monitor found!");
    println!("Monitor: {:?}", monitor.name());

    let mut mode_index = 0;
    let mut mode = monitor.video_modes().next().expect("no mode found");
    println!("Mode: {}", mode);

    println!("Keys:");
    println!("- Esc\tExit");
    println!("- F\tToggle exclusive fullscreen mode");
    println!("- B\tToggle borderless mode");
    println!("- S\tNext screen");
    println!("- M\tNext mode for this screen");
    println!("- D\tToggle window decorations");
    println!("- X\tMaximize window");
    println!("- Z\tMinimize window");

    event_loop.run(move |event, elwt, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: key,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match key {
                    Key::Escape => control_flow.set_exit(),

                    // WARNING: Consider using `key_without_modifers()` if available on your platform.
                    // See the `key_binding` example
                    Key::Character("f") => {
                        if window.fullscreen().is_some() {
                            window.set_fullscreen(None);
                        } else {
                            monitor_index = 0;
                            monitor = elwt.available_monitors().next().expect("no monitor found!");
                        }
                        println!("Monitor: {:?}", monitor.name());

                        mode_index = 0;
                        mode = monitor.video_modes().next().expect("no mode found");
                        println!("Mode: {}", mode);
                    }
                    Key::Character("s") => {
                        println!("window.fullscreen {:?}", window.fullscreen());
                    }
                    Key::Character("m") => {
                        let is_maximized = window.is_maximized();
                        window.set_maximized(!is_maximized);
                    }
                    Key::Character("d") => {
                        decorations = !decorations;
                        window.set_decorations(decorations);
                    }
                    VirtualKeyCode::X => {
                        let is_maximized = window.is_maximized();
                        window.set_maximized(!is_maximized);
                    }
                    VirtualKeyCode::Z => {
                        minimized = !minimized;
                        window.set_minimized(minimized);
>>>>>>> 2e4338bb8dddf820c9bcda23d6b7a0d8a6208831
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => {}
        }
    });
}
