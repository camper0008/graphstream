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

fn color_from_string<T: AsRef<str>>(text: T) -> Color {
    let text = text.as_ref();
    if text.len() == 0 {
        unreachable!("filtered empty strings");
    }
    if let Some(text) = text.strip_prefix("#") {
        if text.len() != 3 && text.len() != 6 {
            panic!("invalid hex '#{text}'");
        }
        let text = if text.len() == 3 {
            text.chars().map(|x| format!("{x}{x}")).collect::<String>()
        } else {
            text.to_string()
        };
        let r = u8::from_str_radix(&text[0..2], 16).unwrap();
        let g = u8::from_str_radix(&text[2..4], 16).unwrap();
        let b = u8::from_str_radix(&text[4..6], 16).unwrap();
        return Color::RGB(r, g, b);
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

    let default_colors = [
        "#ff0000", "#ff6000", "#ffbf00", "#27a246", "#00ffff", "#2279ae",
    ]
    .map(color_from_string);

    let args = std::env::args().nth(1).unwrap_or_else(|| String::new());
    let colors: Vec<_> = args
        .split(';')
        .filter(|x| x.trim() != "")
        .map(color_from_string)
        .collect();

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

            let draw_points = values.len() < 25;
            for (x, y_points) in values {
                for (y_idx, y) in y_points.iter().enumerate() {
                    let color = colors
                        .get(y_idx)
                        .map(|x| x.to_owned())
                        .unwrap_or_else(|| default_colors[y_idx % default_colors.len()]);
                    canvas.set_draw_color(color);
                    let offset = 50.0;
                    let size = canvas.output_size()?;
                    let width = size.0 as f64 - offset * 2.0;
                    let height = size.1 as f64 - offset * 2.0;

                    let radius = 30.0;
                    if let Some(last) = last.as_ref() {
                        canvas.draw_line(
                            point(offset + width * x, offset + height * y),
                            point(offset + width * last.0, offset + height * last.1[y_idx]),
                        )?;
                    }
                    if draw_points {
                        let x = offset + width * x - radius / 2.0;
                        let y = offset + height * y - radius / 2.0;
                        canvas.fill_rect(Rect::new(
                            x as i32,
                            y as i32,
                            radius as u32,
                            radius as u32,
                        ))?;
                    }
                }
                last = Some((x, y_points))
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
