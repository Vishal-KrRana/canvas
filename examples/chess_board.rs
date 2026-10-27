use canvas::{Canvas, Shape};

fn main() {
    let mut canvas = Canvas::new(800, 800);
    let (rows, cols) = (8, 8);
    let (cell_width, cell_height) = (800 / cols, 800 / rows);
    canvas.fill(0xFF112233);
    for y in 0..rows {
        for x in 0..cols {
            if (x + y) % 2 == 0 {
                canvas.draw(Shape::Rectangle {
                    x: x * cell_width,
                    y: y * cell_height,
                    width: cell_width as usize,
                    height: cell_height as usize,
                    color: 0xFFAABBCC,
                });
            }
        }
    }

    canvas::render_ppm(&canvas, "gen_img/chess_board.ppm");
}
