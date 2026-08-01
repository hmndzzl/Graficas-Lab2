mod framebuffer;
mod line;
mod bmp;
mod scanlinefill;

use crate::framebuffer::Framebuffer;

fn conway_step(fb: &mut Framebuffer, frame: u32) {
    let width = fb.width;
    let height = fb.height;
    let mut next_state = vec![0; width * height];
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
                    
                    if fb.get_color(nx, ny) != dead_color {
                        live_neighbors += 1;
                    }
                }
            }

            let is_alive = fb.get_color(x, y) != dead_color;
            
            // Apply Conway rules
            if (is_alive && (live_neighbors == 2 || live_neighbors == 3)) || (!is_alive && live_neighbors == 3) {
                // Generate a psychedelic neon color using coordinates and frame counter
                let r = ((x as u32 * 123).wrapping_add(frame * 13) % 256) | 128; // Ensure brightness with | 128
                let g = ((y as u32 * 231).wrapping_add(frame * 17) % 256) | 128;
                let b = (((x + y) as u32 * 342).wrapping_add(frame * 23) % 256) | 128;
                next_state[y * width + x] = (r << 16) | (g << 8) | b;
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
    let framebuffer_width = 125;
    let framebuffer_height = 125;

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

    // --- Acorn (Methuselah: Explodes for 5000+ generations) ---
    let acorn = [
        " *     ",
        "   *   ",
        "**  ***"
    ];

    // --- Diehard (Methuselah: Vanishes after 130 generations) ---
    let diehard = [
        "       *",
        " **     ",
        "  *   ***"
    ];

    let w = framebuffer.width;
    let h = framebuffer.height;

    draw_pattern(&mut framebuffer, 50 * w / 100, 10 * h / 100, &loaf);
    draw_pattern(&mut framebuffer, 85 * w / 100, 80 * h / 100, &block);
    draw_pattern(&mut framebuffer, 70 * w / 100, 30 * h / 100, &beehive);
    draw_pattern(&mut framebuffer, 20 * w / 100, 45 * h / 100, &boat);

    draw_pattern(&mut framebuffer, 85 * w / 100, 45 * h / 100, &blinker);
    draw_pattern(&mut framebuffer, 65 * w / 100, 55 * h / 100, &toad);
    draw_pattern(&mut framebuffer, 10 * w / 100, 70 * h / 100, &beacon);
    draw_pattern(&mut framebuffer, 85 * w / 100, 5 * h / 100, &pulsar); 
    draw_pattern(&mut framebuffer, 15 * w / 100, 5 * h / 100, &pulsar); 

    draw_pattern(&mut framebuffer, 25 * w / 100, 80 * h / 100, &glider);
    draw_pattern(&mut framebuffer, 10 * w / 100, 85 * h / 100, &lwss);
    draw_pattern(&mut framebuffer, 50 * w / 100, 55 * h / 100, &lwss);
    draw_pattern(&mut framebuffer, 40 * w / 100, 85 * h / 100, &mwss);

    draw_pattern(&mut framebuffer, 55 * w / 100, 80 * h / 100, &acorn);
    draw_pattern(&mut framebuffer, 45 * w / 100, 35 * h / 100, &diehard);

    let frame_delay = Duration::from_millis(100);

    let mut frame = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        conway_step(&mut framebuffer, frame);
        frame = frame.wrapping_add(1);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
} 