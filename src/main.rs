extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use rand::random;

use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

#[derive(Debug)]
enum Steer {
    U,
    D,
    L,
    R,
    P,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn steer(steer: Steer, keycode: Keycode) -> Steer {
    let newsteer = match keycode {
        Keycode::P => { Steer::P },
        Keycode::W => {
            if let Steer::D = steer {
                Steer::D
            } else {
                Steer::U
            }
        },
        Keycode::A => {
            if let Steer::R = steer {
                Steer::R
            } else {
                Steer::L
            }
        },
        Keycode::S => {
            if let Steer::U = steer {
                Steer::U
            } else {
                Steer::D
            }
        },
        Keycode::D => {
            if let Steer::L = steer {
                Steer::L
            } else {
                Steer::R
            }
        },
        _ => { steer }
    };
    newsteer
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();

    let sdl_video = sdl_context.video().unwrap();
    let win = sdl_video.window("rsnake", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = win.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(20, 20, 20));
    canvas.clear();
    canvas.present();

    let mut len = 4;
    let mut segments: Vec<Position> = vec![Position { x: 9, y: 9 }];
    let mut apple = Position { x: 15, y: 15 };
    let mut direction = Steer::P;
    let mut frame = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        frame += 1;
        let now = std::time::SystemTime::now();
        
        // inputs
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running; },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => { direction = Steer::P; },
                Event::KeyDown { keycode: Some(keycode), .. } => { direction = steer(direction, keycode) },
                _ => {}
            }
        }

        // move
        if (frame % 15) == 0 {
            let last_index = segments.len() - 1;

            let mut x = segments[last_index].x;
            x = match direction {
                Steer::L => x - 1,
                Steer::R => x + 1,
                _ => x
            };
            x %= 40;
            if x < 0 {
                x += 40;
            }

            let mut y = segments[last_index].y;
            y = match direction {
                Steer::U => y - 1,
                Steer::D => y + 1,
                _ => y
            };
            y %= 30;
            if y < 0 {
                y += 30;
            }

            let new_segment = match direction {
                Steer::U => Some(Position{ x: x, y: y }),
                Steer::L => Some(Position{ x: x, y: y }),
                Steer::D => Some(Position{ x: x, y: y }),
                Steer::R => Some(Position{ x: x, y: y }),
                Steer::P => None,
            };
            if let Some(_position) = new_segment {
                if segments.contains(&new_segment.unwrap()) {
                    break 'running
                }
                segments.push(new_segment.unwrap());
                if new_segment.unwrap() == apple {
                    len += 1;
                    let mut x = random::<i32>() % 40;
                    if x < 0 {
                        x += 40;
                    }
                    let mut y = random::<i32>() % 30;
                    if y < 0 {
                        y += 30;
                    }
                    loop {
                        let newpos = Position { x: x, y: y };
                        if !segments.contains(&newpos) {
                            apple = newpos;
                            break
                        }
                    }
                }
            }
            if segments.len() > len {
                segments.remove(0);
            }
        }

        // drawing
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        let rect = Rect::new(0, 0, 800, 600);
        canvas.fill_rect(rect)?;

        for segment in &segments {
            canvas.set_draw_color(Color::RGB(200, 200, 200));
            canvas.fill_rect(Rect::new(segment.x * 20, segment.y * 20, 20, 20))?;
        }

        canvas.set_draw_color(Color::RGB(200, 100, 100));
        canvas.fill_rect(Rect::new(apple.x * 20, apple.y * 20, 20, 20))?;
        canvas.present();

        sleep(Duration::from_millis(20) - now.elapsed().unwrap());
    }

    println!("Game Over");
    println!("Score: {}", len);

    Ok(())
}
