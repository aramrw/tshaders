pub mod shaders;

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
    },
};
use std::io::{self, Write};
use std::time::{Duration, Instant};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::shaders::wave;

const RAMP: &[u8] = b" .:-=+*#%@";
const RAMP_LEN: f64 = RAMP.len() as f64;

pub struct Pixel {
    x: f64,
    y: f64,
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let start_time = Instant::now();
    let mut last_frame = Instant::now();
    let target_fps = 60;
    let frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);

    let shader_names = vec!["wave", "tan_wave", "bumps", "plasma", "spiral", "ripples", "interference"];
    let mut shader_i = 0;

    // Main event loop
    loop {
        // 2. Non-blocking input handling: exit immediately on 'q'
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('l') => {
                        shader_i = (shader_i + 1) % shader_names.len();
                    }
                    _ => {}
                }
            }
        }

        // Frame timing constraint
        if last_frame.elapsed() < frame_duration {
            std::thread::sleep(Duration::from_millis(1));
            continue;
        }
        last_frame = Instant::now();

        // Get live terminal boundaries
        let (width, height) = size()?;
        let width = width as usize;
        let height = height as usize;
        let i_time = start_time.elapsed().as_secs_f64();

        // 3. Pre-allocate buffer to hold the screen frames
        // Allocating width * height characters + vertical newlines (\r\n)
        let mut frame_buffer = String::with_capacity((width + 2) * height);

        // 4. Fragment shader loops over every pixel cell
        for y in 0..height {
            for x in 0..width {
                // Normalize coordinates (compensate for ~2.0 vertical text aspect ratio)
                let uv_x = x as f64 / width as f64;
                let uv_y = (y as f64 / height as f64) * 0.5;

                let p = Pixel { x: uv_x, y: uv_y };
                match shader_names[shader_i] {
                    "tan_wave" => crate::shaders::tan_wave(p, i_time, &mut frame_buffer),
                    "bumps" => crate::shaders::bumps(p, i_time, &mut frame_buffer),
                    "plasma" => crate::shaders::plasma(p, i_time, &mut frame_buffer),
                    "spiral" => crate::shaders::spiral(p, i_time, &mut frame_buffer),
                    "ripples" => crate::shaders::ripples(p, i_time, &mut frame_buffer),
                    "interference" => crate::shaders::interference(p, i_time, &mut frame_buffer),
                    _ => wave(p, i_time, &mut frame_buffer),
                }
            }
            // Move down to the next row natively in terminal raw mode
            frame_buffer.push_str("\r\n");
        }

        // 5. Draw entire screen string at once, forcing cursor back to origins (0,0)
        execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
        stdout.write_all(frame_buffer.as_bytes())?;
        stdout.flush()?;
    }

    // 6. Graceful cleanup back to normal user shell settings
    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}
