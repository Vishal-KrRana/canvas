use canvas::{Canvas, Shape};

fn main() {
    let (width, height) = (800, 600);
    let mut canvas = Canvas::new(width, height);
    canvas.fill(0xFF112233);
    let (rw, rh) = (300, 400);
    canvas.draw(Shape::Rectangle {
        x: (width / 4 - rw / 2) as isize,
        y: (height / 2 - rh / 2) as isize,
        width: rw,
        height: rh,
        color: 0xFFAABBCC,
    });
    canvas.draw(Shape::Line {
        xi: (width / 2) as isize - 2,
        yi: 0,
        xf: (width / 2) as isize - 2,
        yf: height as isize,
        color: 0xFFBBCCDD,
    });
    canvas.draw(Shape::Line {
        xi: (width / 2) as isize + 2,
        yi: 0,
        xf: (width / 2) as isize + 2,
        yf: height as isize,
        color: 0xFFBBCCDD,
    });
    canvas.draw(Shape::Circle {
        cx: (width - width / 4) as isize,
        cy: (height / 2) as isize,
        r: 150,
        color: 0xFFFF7766,
    });
    canvas::render_ppm(&canvas, "gen_img/shapes.ppm");
}
