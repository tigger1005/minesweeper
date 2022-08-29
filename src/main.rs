mod field;

use field::*;
use nannou::prelude::*;

const N: usize = 20;
const SCALE: f32 = 30.0;
const BOMBS_NUMBER: usize = 50;

fn cord_2_field(point: Point2) -> (isize, isize) {
    let x_temp = map_range(
        point.x,
        -(N as f32) * 0.5 * SCALE,
        (N as f32) * 0.5 * SCALE,
        0.0,
        N as f32,
    );
    let y_temp = map_range(
        point.y,
        -(N as f32) * 0.5 * SCALE,
        (N as f32) * 0.5 * SCALE,
        0.0,
        N as f32,
    );
    (x_temp as isize, y_temp as isize)
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    playground: Field,
    success: bool,
    bomb: bool,
}

impl Model {
    fn new() -> Self {
        Self {
            playground: Field::new(),
            success: false,
            bomb: false,
        }
    }

    fn draw(&self, draw: &Draw) {
        for x in 0..N {
            for y in 0..N {
                self.playground.field[x][y].draw(draw, x, y);
            }
        }
        if self.success {
            draw.text("Solved")
                .x_y((N / 2) as f32 * SCALE, (N / 2) as f32 * SCALE)
                .color(GREEN)
                .font_size((SCALE as u32) * 2);
        }
        if self.bomb {
            draw.text("BAAAM")
                .x_y((N / 2) as f32 * SCALE, (N / 2) as f32 * SCALE)
                .color(RED)
                .font_size((SCALE as u32) * 2);
        }
    }
}

fn update(_app: &App, _m: &mut Model, _update: Update) {}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    let (x_temp, y_temp) = cord_2_field(app.mouse.position());
    match event {
        MousePressed(button) => {
            if button == nannou::state::mouse::Button::Left {
                m.playground.uncover(x_temp as i16, y_temp as i16);
                m.bomb |= m.playground.is_bomb(x_temp as usize, y_temp as usize);
                m.success = m.playground.is_solved();
            }
            if button == nannou::state::mouse::Button::Right {
                m.playground.toggle_marker(x_temp as usize, y_temp as usize);
            }
        }
        _other => (),
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(N as u32 * SCALE as u32, N as u32 * SCALE as u32)
        .view(view)
        .event(event)
        .build()
        .unwrap();
    Model::new()
}

fn view(app: &App, m: &Model, frame: Frame) {
    let translate = (-(N as f32) * 0.5 * SCALE) + SCALE / 2.0;
    // Begin drawing
    let draw = app.draw().x_y(translate, translate);
    draw.background().color(LIGHTGRAY);
    // Draw field
    m.draw(&draw);
    // Write the result of our drawing to the window's frame.
    draw.x_y(-100.0, -100.0).to_frame(app, &frame).unwrap();
}
