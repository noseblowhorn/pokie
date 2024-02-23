use beryllium::*;
use std::{thread, time};

pub mod config;

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    #[cfg(target_os = "macos")] {
        sdl
            .set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
            .unwrap();
    }

    let win_args = video::CreateWinArgs {
        title: "POKIE",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: true,
    };

    let _win = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");

    let _config = config::init_pokie_config();

    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }

        let sleep_time = time::Duration::from_millis(_config.sleep_duration);
        thread::sleep(sleep_time);

        // now the events are clear

        // here's where we could change the world state and draw.
    }
}
