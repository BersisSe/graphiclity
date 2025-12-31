use graphiclity::{Color, Config, run_with};

fn main() {
    let c = Config::builder()
        .set_logical_size((640,320))
        .set_window_size((1280,640))
        .build();

    run_with(c, |g| {
        g.clear(Color::WHITE);

        // Triangle vertices
        let (x1, y1) = (100, 50);
        let (x2, y2) = (500, 50);
        let (x3, y3) = (300, 250);

        // Draw triangle
        g.triangle(x1, y1, x2, y2, x3, y3, Color::RED);
    
        // Draw rectangle at the centroid of the triangle
        g.rect(300 - 50/2, 116 - 50/2, 50, 50, Color::RED);
    });
}