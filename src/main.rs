use core::time;
use std::fs;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::thread;

use enigo::MouseButton;
use screenshots::Screen;
use enigo::Enigo;
use enigo::MouseControllable;
use winapi::um::winuser;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut enigo = Enigo::new();
    let state_file = "state.txt";

    let screen_offset_x = 1920;
    let screen_offset_y = -45;

    let x_origin = 745;
    let y_origin = 270;

    let y_anchor = 430;

    let screen = Screen::all().unwrap()[1];
    let mut is_lose = false;
    let mut is_confused = false;
    
    let mut found_pos = (0, 0);

    println!("Suika Game bot is running...");
    fs::write(state_file, "").unwrap();

    'main_loop: loop {
        if unsafe { winuser::GetAsyncKeyState(winuser::VK_ESCAPE) } != 0 {
            break;
        }

        if is_lose {
            thread::sleep(time::Duration::from_secs(1));
            let file = match fs::File::open(state_file) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    continue 'main_loop
                },
            };
            let mut reader = BufReader::new(file);
            let mut state = String::new();
            if reader.read_to_string(&mut state).is_ok() {
                if state.is_empty() {
                    is_lose = false;
                    enigo.mouse_move_to(screen_offset_x + x_origin + found_pos.0, screen_offset_y + y_origin + y_anchor);
                    enigo.mouse_click(MouseButton::Left);
                    thread::sleep(time::Duration::from_secs(5));

                    println!("Continue running...");
                } else {
                    continue;
                }
            }
        }

        if is_confused {
            found_pos = (rng.gen_range(0..414), rng.gen_range(0..630));
            enigo.mouse_move_to(screen_offset_x + x_origin + found_pos.0, screen_offset_y + y_origin + y_anchor);
            enigo.mouse_down(MouseButton::Left);
            thread::sleep(time::Duration::from_millis(rng.gen_range(500..700)));
            enigo.mouse_up(MouseButton::Left);
            thread::sleep(time::Duration::from_millis(rng.gen_range(1000..1500)));
            enigo.mouse_up(MouseButton::Left);
        }
        
        let current_fruit = screen.capture_area(945, 190, 30, 30).unwrap();
        let current_fruit_color = current_fruit.get_pixel(7, 7).0;

        let game_area = screen.capture_area(x_origin, y_origin, 430, 630).unwrap();
        is_confused = true;
        'scan: for y in 0..630 {
            for x in 0..430 {
                let pixel_color = game_area.get_pixel(x, y).0;
                if is_same_color(&current_fruit_color, &pixel_color) {
                    is_confused = false;
                    found_pos = ((x as i32) + 5, y_anchor);
                    break 'scan;
                }
            }

            if y == 629 && is_confused {
                continue 'main_loop;
            }
        }

        if found_pos.0 > 429 {
            found_pos.0 = 425
        }

        enigo.mouse_move_to(screen_offset_x + x_origin + found_pos.0, screen_offset_y + y_origin + y_anchor);
        enigo.mouse_down(MouseButton::Left);
        thread::sleep(time::Duration::from_millis(rng.gen_range(500..700)));
        enigo.mouse_up(MouseButton::Left);
        thread::sleep(time::Duration::from_millis(rng.gen_range(1000..1500)));
        enigo.mouse_up(MouseButton::Left);
        
        let game_over_color = [249, 159, 10, 255];
        let pixel_color = game_area.get_pixel(195, y_anchor as u32).0;
        if is_same_color(&pixel_color, &game_over_color) {
            is_lose = true;
            found_pos = (195, y_anchor);
        }

        if is_lose {
            let file = match fs::File::options().write(true).append(false).open(state_file) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    continue;
                }
            };
            let mut writer = BufWriter::new(file);
            if let Err(e) = writer.write_all("####GAMEOVER####".as_bytes()) {
                eprintln!("ERROR: {e}");
                continue;
            };
            println!("* lose state!");
        }
    }
}

fn is_same_color(color1: &[u8], color2: &[u8]) -> bool {
    color1[0] == color2[0] && color1[1] == color2[1] && color1[2] == color2[2]
}