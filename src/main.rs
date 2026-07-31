mod framebuffer;
mod line;
mod bmp;
mod scanlinefill;

use crate::framebuffer::Framebuffer;
use crate::bmp::WriteBmp;

fn conway_step(fb: &mut Framebuffer) {
    let width = fb.width;
    let height = fb.height;
    let mut next_state = vec![0; width * height];
    let alive_color = 0xFFFFFF;
    let dead_color = 0x000000;

    for y in 0..height {
        for x in 0..width {
            let mut live_neighbors = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    // Toroidal loop (wrap around edges)
                    let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
                    let ny = (y as isize + dy).rem_euclid(height as isize) as usize;
                    
                    if fb.get_color(nx, ny) == alive_color {
                        live_neighbors += 1;
                    }
                }
            }

            let is_alive = fb.get_color(x, y) == alive_color;
            if is_alive && (live_neighbors == 2 || live_neighbors == 3) {
                next_state[y * width + x] = alive_color;
            } else if !is_alive && live_neighbors == 3 {
                next_state[y * width + x] = alive_color;
            } else {
                next_state[y * width + x] = dead_color;
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            fb.set_current_color(next_state[y * width + x]);
            fb.point(x, y);
        }
    }
}

// Helper function to easily draw organisms using strings
fn draw_pattern(fb: &mut Framebuffer, x_offset: usize, y_offset: usize, pattern: &[&str]) {
    for (y, row) in pattern.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '*' {
                fb.point(x_offset + x, y_offset + y);
            }
        }
    }
}

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);

    // --- Still Lifes ---
    let block = [
        "**",
        "**"
    ];
    let beehive = [
        " ** ",
        "*  *",
        " ** "
    ];
    let loaf = [
        " ** ",
        "*  *",
        " * *",
        "  * "
    ];
    let boat = [
        "** ",
        "* *",
        " **"
    ];

    // --- Oscillators ---
    let blinker = [
        "***",
    ];
    let toad = [
        " ***",
        "*** "
    ];
    let beacon = [
        "**  ",
        "**  ",
        "  **",
        "  **"
    ];
    let pulsar = [
        "  ***   ***  ",
        "             ",
        "*    * *    *",
        "*    * *    *",
        "*    * *    *",
        "  ***   ***  ",
        "             ",
        "  ***   ***  ",
        "*    * *    *",
        "*    * *    *",
        "*    * *    *",
        "             ",
        "  ***   ***  "
    ];

    // --- Spaceships ---
    let glider = [
        " * ",
        "  *",
        "***"
    ];
    let lwss = [
        " *  *",
        "    *",
        "*   *",
        " ****"
    ];
    let mwss = [
        "   *  ",
        " *   *",
        "     *",
        " *   *",
        "  ****"
    ];

    // --- Gosper Glider Gun ---
    let gosper_glider_gun = [
        "                                    ",
        "                        *           ",
        "                      * *           ",
        "            **      **            **",
        "           *   *    **            **",
        "**        *     *   **              ",
        "**        *   * **    * *           ",
        "          *     *       *           ",
        "           *   *                    ",
        "            **                      "
    ];

    // Draw all patterns on the framebuffer
    draw_pattern(&mut framebuffer, 5, 5, &block);
    draw_pattern(&mut framebuffer, 15, 5, &beehive);
    draw_pattern(&mut framebuffer, 25, 5, &loaf);
    draw_pattern(&mut framebuffer, 35, 5, &boat);

    draw_pattern(&mut framebuffer, 5, 15, &blinker);
    draw_pattern(&mut framebuffer, 15, 15, &toad);
    draw_pattern(&mut framebuffer, 25, 15, &beacon);
    draw_pattern(&mut framebuffer, 40, 15, &pulsar);

    draw_pattern(&mut framebuffer, 10, 35, &glider);
    draw_pattern(&mut framebuffer, 10, 50, &lwss);
    draw_pattern(&mut framebuffer, 10, 65, &mwss);

    draw_pattern(&mut framebuffer, 50, 40, &gosper_glider_gun);

    let frame_delay = Duration::from_millis(100);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        conway_step(&mut framebuffer);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
} 