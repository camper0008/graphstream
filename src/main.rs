mod position;
mod source;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn input_thread(
    mut source: impl source::Source + Send + 'static,
    values: Arc<Mutex<Vec<Vec<f64>>>>,
) {
    std::thread::spawn(move || loop {
        let Some(value) = source.next() else {
            continue;
        };
        values.lock().unwrap().push(value);
    });
}

fn color_from_string(text: &str) -> Color {
    if text.len() == 0 {
        return Color::RGB(255,0,0);
    }
    let [r, g, b]: [u8; 3] = text
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    Color::RGB(r, g, b)
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

    let args = std::env::args().nth(1).unwrap_or_else(|| String::new());
    let colors: Vec<_> = args.split(';').map(color_from_string).collect();

    let mut canvas = window.into_canvas().build().unwrap();

    let values = Arc::new(Mutex::new(Vec::new()));
    input_thread(source::Stdin, values.clone());

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::RGB(25, 25, 25));
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
            let mut last: Option<(f64, Vec<f64>)> = None;
            let Some(values) = position::values_to_fractions(&values.lock().unwrap()) else {
                break 'draw_values;
            };

            fn point(x: f64, y: f64) -> Point {
                Point::new(x as i32, y as i32)
            }

            for (x, y_points) in values {
                for (ydx, y) in y_points.iter().enumerate() {
                    let color = colors
                        .get(ydx)
                        .map(|x| x.to_owned())
                        .unwrap_or_else(|| Color::RGB(255, 0, 0));
                    canvas.set_draw_color(color);
                    let offset = 50.0;
                    let size = canvas.output_size()?;
                    let width = size.0 as f64 - offset * 2.0;
                    let height = size.1 as f64 - offset * 2.0;

                    let radius = 4.0;
                    let xp1 = offset + width * x - radius / 2.0;
                    let yp1 = offset + height * y - radius / 2.0;
                    if let Some(last) = last.as_ref() {
                        canvas.draw_line(
                            point(offset + width * x, offset + height * y),
                            point(offset + width * last.0, offset + height * last.1[ydx]),
                        )?;
                    }
                    canvas.fill_rect(Rect::new(
                        xp1 as i32,
                        yp1 as i32,
                        radius as u32,
                        radius as u32,
                    ))?;
                }
                last = Some((x, y_points))
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
