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
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        if fb.get_color(nx as usize, ny as usize) == alive_color {
                            live_neighbors += 1;
                        }
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

fn main() {
    let mut framebuffer = Framebuffer::new(100, 100);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);

    // Initial pattern (Glider)
    let cx = 400;
    let cy = 300;
    framebuffer.point(cx + 1, cy);
    framebuffer.point(cx + 2, cy + 1);
    framebuffer.point(cx, cy + 2);
    framebuffer.point(cx + 1, cy + 2);
    framebuffer.point(cx + 2, cy + 2);

    // You can uncomment the loop below to generate multiple steps,
    // or run a single step. We will run 10 steps as a demo.
    for _ in 0..10 {
        conway_step(&mut framebuffer);
    }

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
} 