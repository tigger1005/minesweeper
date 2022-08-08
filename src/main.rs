
use nannou::prelude::*;

const N: usize = 20;
const SCALE : f32 = 30.0;

// #[derive(Copy, Clone)]
// enum Stone {
//     Free,
//     Normal,
//     Bomb,
// }


#[derive(Copy, Clone)]
struct Place {
//    stone: Stone,
    hidden: bool,
}


impl Place {
    fn new() -> Self {
        Self {
//            stone : Stone::Free, 
            hidden : true}
    }

    fn draw(& self, draw: & Draw, x: f32, y: f32) {
        let x_temp = map_range(x, 0.0, N as f32, -(N as f32 / 2.0) * SCALE, (N as f32 / 2.0) * SCALE) + SCALE / 2.0;
        let y_temp = map_range(y, 0.0, N as f32, -(N as f32 / 2.0) * SCALE, (N as f32 / 2.0) * SCALE) + SCALE / 2.0;
        let size = SCALE - 1.0;
        match self.hidden {
            true => draw.rect().x_y(x_temp, y_temp).w_h(size, size).color(GREY),
            false => draw.rect().x_y(x_temp, y_temp).w_h(size, size).color(BLUE),
        };
    }
}


// impl Field {
//     fn new()->Self {
//         let place = Place::new();
//         Self { elements: [[place; N]; N]}
//     }

//     fn surrounding(&self, x: u16, y: u16)->u16  {
//         return 0;
//     }
// }


fn main() {
    nannou::app(model).update(update).run();
}

struct Model 
{
    field: [[Place; N]; N]
}

impl Model {
    fn new() -> Self { 
        let place = Place::new();
        Self { field: [[place; N]; N]}
    }

    fn draw(&self, draw : &Draw) {
        for x in 0..N {
            for y in 0..N {
                self.field[x][y].draw(draw, x as f32, y as f32);
            }
        }
    }
}



fn update(_app: &App, _m: &mut Model, _update: Update) {
}

fn event(_app: &App, _m: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(_) => (),
        _other => (),
    }
}


fn model(app: &App) -> Model {
    let _window = app.new_window().size(N as u32 * SCALE as u32, N as u32 * SCALE as u32)
                                  .view(view)
                                  .event(event)
                                  .build()
                                  .unwrap();
    Model::new()
}


fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);
    // Draw field
    m.draw(&draw);
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}