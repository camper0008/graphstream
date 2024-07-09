mod position;
mod source;
mod value;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use value::Value;

fn input_thread(mut source: impl source::Source + Send + 'static, values: Arc<Mutex<Vec<Value>>>) {
    std::thread::spawn(move || loop {
        let Some(value) = source.next() else {
            continue;
        };
        values.lock().unwrap().push(value);
    });
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("graphstream", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let values = Arc::new(Mutex::new(Vec::new()));
    input_thread(source::Stdin, values.clone());

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running Ok(()),
                _ => {}
            }
        }

        'draw_values: {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            let mut last: Option<(f64, f64)> = None;
            let Some(values) = position::values_to_fractions(&values.lock().unwrap()) else {
                break 'draw_values;
            };

            fn point(x: Value, y: Value) -> Point {
                Point::new(x as i32, y as i32)
            }

            for value in values.positions {
                let offset = 50.0;
                let size = canvas.output_size()?;
                let width = size.0 as Value - offset * 2.0;
                let height = size.1 as Value - offset * 2.0;

                let radius = 4.0;
                let x = offset + width * value.0 - radius / 2.0;
                let y = offset + height * value.1 - radius / 2.0;
                if let Some(last) = last {
                    canvas.draw_line(
                        point(offset + width * value.0, offset + height * value.1),
                        point(offset + width * last.0, offset + height * last.1),
                    )?;
                }
                canvas.fill_rect(Rect::new(x as i32, y as i32, radius as u32, radius as u32))?;

                last = Some(value);
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
