mod framebuffer;
mod line;
mod bmp;
mod scanlinefill;

use crate::framebuffer::Framebuffer;
use crate::line::Line;
use crate::bmp::WriteBmp;
use crate::scanlinefill::ScanlineFill;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);

    // Polígono 1
    framebuffer.line(165, 380, 185, 360);
    framebuffer.line(185, 360, 180, 330);
    framebuffer.line(180, 330, 207, 345);
    framebuffer.line(207, 345, 233, 330);
    framebuffer.line(233, 330, 230, 360);
    framebuffer.line(230, 360, 250, 380);
    framebuffer.line(250, 380, 220, 385);
    framebuffer.line(220, 385, 205, 410);
    framebuffer.line(205, 410, 193, 383);
    framebuffer.line(193, 383, 165, 380);

    // Polígono 2
    framebuffer.line(321, 335, 288, 286);
    framebuffer.line(288, 286, 339, 251);
    framebuffer.line(339, 251, 374, 302);
    framebuffer.line(374, 302, 321, 335);
    
    // Polígono 3
    framebuffer.line(377, 249, 411, 197);
    framebuffer.line(411, 197, 436, 249);
    framebuffer.line(436, 249, 377, 249);

    // Polígono 4
    framebuffer.line(413, 177, 448, 159);
    framebuffer.line(448, 159, 502, 88);
    framebuffer.line(502, 88, 553, 53);
    framebuffer.line(553, 53, 535, 36);
    framebuffer.line(535, 36, 676, 37);
    framebuffer.line(676, 37, 660, 52);
    framebuffer.line(660, 52, 750, 145);
    framebuffer.line(750, 145, 761, 179);
    framebuffer.line(761, 179, 672, 192);
    framebuffer.line(672, 192, 659, 214);
    framebuffer.line(659, 214, 615, 214);
    framebuffer.line(615, 214, 632, 230);
    framebuffer.line(632, 230, 580, 230);
    framebuffer.line(580, 230, 597, 215);
    framebuffer.line(597, 215, 552, 214);
    framebuffer.line(552, 214, 517, 144);
    framebuffer.line(517, 144, 466, 180);
    framebuffer.line(466, 180, 413, 177);

    let polygon1 = vec![(165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)];
    let polygon2 = vec![(321, 335), (288, 286), (339, 251), (374, 302)];
    let polygon3 = vec![(377, 249), (411, 197), (436, 249)];
    let polygon4 = vec![(413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52), (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230), (597, 215), (552, 214), (517, 144), (466, 180)];
    let polygon5 = vec![(682, 175), (708, 120), (735, 148), (739, 170)];

    framebuffer.scanline_fill(&polygon1);
    framebuffer.scanline_fill(&polygon2);
    framebuffer.scanline_fill(&polygon3);
    framebuffer.scanline_fill(&polygon4);

    framebuffer.set_current_color(0x000000);
    framebuffer.scanline_fill(&polygon5);

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.line(682, 175, 708, 120);
    framebuffer.line(708, 120, 735, 148);
    framebuffer.line(735, 148, 739, 170);
    framebuffer.line(739, 170, 682, 175);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
} 