use std::{fs::File, io::Write, mem::swap};

#[derive(Default)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

pub type ARGB = u32;

pub enum Shape {
    Rectangle {
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        color: ARGB,
    },
    Circle {
        cx: isize,
        cy: isize,
        r: usize,
        color: ARGB,
    },
    Line {
        xi: isize,
        yi: isize,
        xf: isize,
        yf: isize,
        color: ARGB,
    },
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: Vec::with_capacity(width * height),
        }
    }
    pub fn with_bg(width: usize, height: usize, color: ARGB) -> Self {
        Self {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }
    pub fn fill(&mut self, color: ARGB) {
        let mut i = 0;
        while i < self.pixels.capacity() {
            self.pixels.push(color);
            i += 1;
        }
    }
    pub fn plot_point(&mut self, x: isize, y: isize, color: ARGB) {
        self.pixels[y as usize * self.width + x as usize] = color;
    }
    pub fn draw(&mut self, shape: Shape) {
        match shape {
            Shape::Rectangle {
                x,
                y,
                width,
                height,
                color,
            } => self.draw_rectangle(x, y, width, height, color),
            Shape::Circle { cx, cy, r, color } => self.draw_circle(cx, cy, r, color),
            Shape::Line {
                xi,
                yi,
                xf,
                yf,
                color,
            } => self.draw_line(xi, yi, xf, yf, color),
        }
    }

    fn draw_line(&mut self, xi: isize, yi: isize, xf: isize, yf: isize, color: ARGB) {
        let (mut xi, mut yi) = (xi, yi);
        let (mut xf, mut yf) = (xf, yf);
        let dx = xf - xi;
        let dy = yf - yi;
        if dx != 0 {
            let c = yf - dy * xf / dx;
            if xi > xf {
                swap(&mut xi, &mut xf);
            }
            for x in xi..xf {
                let mut syi = dy * x / dx + c;
                let mut syf = dy * (x + 1) / dx + c;
                if syi > syf {
                    swap(&mut syi, &mut syf);
                }
                for y in syi..syf {
                    if 0 <= y && y < self.height as isize {
                        self.pixels[y as usize * self.width + x as usize] = color;
                    }
                }
            }
        } else {
            let x = xi;
            if 0 <= x && x < self.width as isize {
                if yi > yf {
                    swap(&mut yi, &mut yf);
                }
                for y in yi..yf {
                    if 0 <= y && y < self.height as isize {
                        self.pixels[y as usize * self.width + x as usize] = color;
                    }
                }
            }
        }
    }

    fn draw_circle(&mut self, cx: isize, cy: isize, r: usize, color: ARGB) {
        let (x1, y1) = (cx - r as isize, cy - r as isize);
        let (x2, y2) = (cx + r as isize, cy + r as isize);
        for y in y1..(y2 + 1) {
            if 0 <= y && y < self.height as isize {
                for x in x1..(x2 + 1) {
                    if 0 <= x && x < self.width as isize {
                        let dx = x - cx;
                        let dy = y - cy;
                        if dx * dx + dy * dy <= (r * r) as isize {
                            self.pixels[y as usize * self.width + x as usize] = color;
                        }
                    }
                }
            }
        }
    }

    fn draw_rectangle(&mut self, x: isize, y: isize, width: usize, height: usize, color: ARGB) {
        for dy in 0..height {
            let y = y + dy as isize;
            if 0 <= y && y < self.height as isize {
                for dx in 0..width {
                    let x = x + dx as isize;
                    if 0 <= x && x < self.width as isize {
                        self.pixels[y as usize * self.width + x as usize] = color;
                    }
                }
            }
        }
    }
}

pub fn render_ppm(canvas: &Canvas, file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    let header = format!("P6\n{} {}\n255\n", canvas.width, canvas.height);
    let _ = file.write(&header.as_bytes());
    for pixel in &canvas.pixels {
        let bytes = [
            (pixel >> 2 * 8) as u8 & 0xFF,
            (pixel >> 1 * 8) as u8 & 0xFF,
            (pixel >> 0 * 8) as u8 & 0xFF,
        ];
        if let Err(e) = file.write(&bytes) {
            eprint!("Error: {e}");
        };
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
