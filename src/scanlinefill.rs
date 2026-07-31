use crate::framebuffer::Framebuffer;

pub trait ScanlineFill {
    fn scanline_fill(&mut self, polygon: &[(i32, i32)]);
}

impl ScanlineFill for Framebuffer {
    fn scanline_fill(&mut self, polygon: &[(i32, i32)]) {
        if polygon.len() < 3 {
            return;
        }

        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;

        for &(_, y) in polygon {
            if y < y_min {
                y_min = y;
            }
            if y > y_max {
                y_max = y;
            }
        }

        for y in y_min..=y_max {
            let mut intersections = Vec::new();

            for i in 0..polygon.len() {
                let (x1, y1) = polygon[i];
                let (x2, y2) = polygon[(i + 1) % polygon.len()];

                if let Some(x_intersection) = edge_intersection(x1, y1, x2, y2, y) {
                    intersections.push(x_intersection);
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 >= intersections.len() {
                    break;
                }

                let x_start = intersections[i].round() as i32;
                let x_end = intersections[i + 1].round() as i32;
                let start = x_start.clamp(0, self.width as i32 - 1);
                let end = x_end.clamp(0, self.width as i32 - 1);

                for x in start.min(end)..=start.max(end) {
                    self.point(x as usize, y as usize);
                }
            }
        }
    }
}

fn edge_intersection(x1: i32, y1: i32, x2: i32, y2: i32, scan_y: i32) -> Option<f64> {
    let y1_f = y1 as f64;
    let y2_f = y2 as f64;
    let scan_y_f = scan_y as f64;

    let crosses = (y1_f > scan_y_f && y2_f <= scan_y_f) || (y2_f > scan_y_f && y1_f <= scan_y_f);
    if !crosses {
        return None;
    }

    if (y2_f - y1_f).abs() < f64::EPSILON {
        return None;
    }

    let t = (scan_y_f - y1_f) / (y2_f - y1_f);
    let x_intersection = x1 as f64 + t * (x2 as f64 - x1 as f64);
    Some(x_intersection)
}
