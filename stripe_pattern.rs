use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(720, 260).run();
}

fn view(app: &App, frame: Frame) {
    let mut draw = app.draw();
    let win = app.window_rect();
    draw.background().rgb(1., 1., 1.);
    draw = draw.translate(vec3(win.left(), 0., 0.));

    let length = 140.;
    let max = win.w() as usize / 10;
    let angle = 2. * PI / max as f32;
    for i in 0..=max {
        let i = i as f32;
        let x = 10f32;
        let y = 0.;
        draw = draw.translate(vec3(x, y, 0f32));
        let length = length + length * (2. * PI / max as f32 * i as f32).sin().abs();
        let draw = draw.rotate(angle * i as f32);
        let c = hsl(1. / max as f32 * i, 1., 0.5);
        let triangle = vec![
            ((0., 0.), c),
            ((length * 0.5, length * 1.7/2.), c),
            ((-length * 0.5, length * 1.7/2.), c),
            ((0., 0.), c),
        ];
        draw.polyline().weight(3.).points_colored(triangle);
    }

    draw.to_frame(app, &frame).unwrap();
}
