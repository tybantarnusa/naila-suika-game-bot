use core::time;
use std::fs;
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

    let x_origin = 745;
    let y_origin = 270;

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

        if is_lose || is_confused {
            if let Ok(state) = fs::read_to_string(state_file) {
                if state.is_empty() {
                    if is_lose {
                        is_lose = false;
                        enigo.mouse_move_to(1920+x_origin + found_pos.0, y_origin + found_pos.1-45);
                        enigo.mouse_click(MouseButton::Left);
                        thread::sleep(time::Duration::from_secs(5));
                    }

                    if is_confused {
                        found_pos = (rng.gen_range(0..414), rng.gen_range(0..630));
                        enigo.mouse_move_to(1920+x_origin + found_pos.0, y_origin + found_pos.1-45);
                        enigo.mouse_down(MouseButton::Left);
                        thread::sleep(time::Duration::from_millis(rng.gen_range(500..700)));
                        enigo.mouse_up(MouseButton::Left);
                        thread::sleep(time::Duration::from_millis(rng.gen_range(1000..1500)));
                        enigo.mouse_up(MouseButton::Left);
                    }

                    println!("Continue running...");
                } else {
                    continue;
                }
            }
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
                    found_pos = ((x as i32)+15, y as i32);
                    break 'scan;
                }
            }

            if y == 629 && is_confused {
                let talk_chance = rng.gen_range(0..10);
                if talk_chance < 2 {
                    // fs::write(state_file, "####CONFUSED####").unwrap();
                    println!("* confused state!");
                }
                continue 'main_loop;
            }
        }

        if found_pos.0 > 429 {
            found_pos.0 = 425
        }

        enigo.mouse_move_to(1920+x_origin + found_pos.0, y_origin + found_pos.1-45);
        enigo.mouse_down(MouseButton::Left);
        thread::sleep(time::Duration::from_millis(rng.gen_range(500..700)));
        enigo.mouse_up(MouseButton::Left);
        thread::sleep(time::Duration::from_millis(rng.gen_range(1000..1500)));
        enigo.mouse_up(MouseButton::Left);
        
        let game_over_color = [249, 159, 10, 255];
        'all_loop: for y in 0..630 {
            for x in 0..430 {
                let pixel_color = game_area.get_pixel(x, y).0;
                if is_same_color(&pixel_color, &game_over_color) {
                    is_lose = true;
                    found_pos = ((x as i32)+15, (y as i32)+15);
                    break 'all_loop;
                }
            }
        }

        if is_lose && fs::write(state_file, "####GAMEOVER####").is_ok() {
            println!("* lose state!");
        }
    }
}

fn is_same_color(color1: &[u8], color2: &[u8]) -> bool {
    color1[0] == color2[0] && color1[1] == color2[1] && color1[2] == color2[2]
}