use canvas::{ARGB, Canvas, Shape};

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn main() {
    let (width, height) = (1200, 800);
    let mut canvas = Canvas::new(width, height);
    let (cols, rows) = (15, 10);
    let (cell_width, cell_height) = (width / cols, height / rows);
    canvas.fill(0xFF0000FF);
    for y in 0..rows {
        for x in 0..cols {
            let u = x as f32 / cols as f32;
            let v = y as f32 / rows as f32;
            let t = (u + v) / 2.0;
            let mut r = cell_width as f32;
            if cell_height < cell_width {
                r = cell_height as f32;
            }
            let color: ARGB;
            if (x + y) % 2 == 0 {
                color = 0xFFAABBCC;
            } else {
                color = 0xFF112233;
            }
            canvas.draw(Shape::Circle {
                cx: (x * cell_width + cell_width / 2) as isize,
                cy: (y * cell_height + cell_height / 2) as isize,
                r: lerp(r / 6.0, r / 2.0, t) as usize,
                color,
            });
        }
    }

    canvas::render_ppm(&canvas, "gen_img/circle.ppm");
}
