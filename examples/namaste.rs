use canvas::Canvas;

fn main() {
    // here a new canvas would be generated for you to use but the canvas needed to be filled with a background to be used this can be done by the fill method.
    let mut canvas = Canvas::new(800, 600);
    canvas.fill(0xFFAACCEE);
    for y in 50..100 {
        for x in 60..120 {
            canvas.plot_point(x, y, 0xFF223344);
        }
    }
    canvas::render_ppm(&canvas, "gen_img/blank.ppm");
}
