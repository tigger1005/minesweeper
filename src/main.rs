
use nannou::prelude::*;
use rand::Rng;

const N: usize = 20;
const SCALE : f32 = 30.0;
const BOMBS_NUMBER: usize = 50;

#[derive(Copy, Clone)]
struct Place {
    number: u8,
    hidden: bool,
}

impl Place {
    fn new() -> Self {
        Self {
            number : 0, 
            hidden : true}
    }

    fn draw(& self, draw: & Draw, x: f32, y: f32) {
        let x_temp = map_range(x, 0.0, N as f32, -(N as f32 / 2.0) * SCALE, (N as f32 / 2.0) * SCALE) + SCALE / 2.0;
        let y_temp = map_range(y, 0.0, N as f32, -(N as f32 / 2.0) * SCALE, (N as f32 / 2.0) * SCALE) + SCALE / 2.0;
        let size = SCALE * 0.92;
        if self.hidden {
            draw.rect().x_y(x_temp, y_temp).w_h(size, size).color(DARKGREY);
        } else {
            if self.number == 10 {
                draw.rect().x_y(x_temp, y_temp).w_h(size, size).color(RED);
            } else if self.number > 0 {
                draw.text(&self.number.to_string()).x_y(x_temp, y_temp).w_h(SCALE, SCALE).color(BLACK).font_size((SCALE as u32) - 2);
            } else {
                draw.rect().x_y(x_temp, y_temp).w_h(size, size).color(LIGHTGRAY);
            }
        }
    }
}


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
        let mut model = Self { field: [[place; N]; N]};
        model.place_bombs();
        for x in 0..N {
            for y in 0..N {
                if model.field[x][y].number != 10 {
                    model.surrounding(x, y);
                }
            }
        }
        model
    }

    fn draw(&self, draw : &Draw) {
        for x in 0..N {
            for y in 0..N {
                self.field[x][y].draw(draw, x as f32, y as f32);
            }
        }
    }

    fn place_bombs(& mut self) {
        for _i in 0..BOMBS_NUMBER {
            let x:usize = rand::thread_rng().gen_range(0..N);
            let y:usize = rand::thread_rng().gen_range(0..N);
            self.field[x][y].number = 10;
        }
        
    }

    fn surrounding(& mut self, x: usize, y: usize) {
        let mut count: u8 = 0;
        if x > 0 {
            if y > 0 {
                count += if self.field[x - 1][y - 1].number == 10 {1} else {0}; }
            count += if self.field[x - 1][y    ].number == 10 {1} else {0};
            if y < (N-1) {
                count += if self.field[x - 1][y + 1].number == 10 {1} else {0}; }
        }

        if y > 0 {
            count += if self.field[x    ][y - 1].number == 10 {1} else {0}; }
        if y < (N-1) {
            count += if self.field[x    ][y + 1].number == 10 {1} else {0}; }

        if x < (N-1) {
            if y > 0 {
                count += if self.field[x + 1][y - 1].number == 10 {1} else {0}; }
            count += if self.field[x + 1][y    ].number == 10 {1} else {0};
            if y < (N-1) {
                count += if self.field[x + 1][y + 1].number == 10 {1} else {0}; }
        }
       
        self.field[x][y].number = count;
    }
}


fn update(_app: &App, _m: &mut Model, _update: Update) {
    // let x:usize = rand::thread_rng().gen_range(0..N);
    // let y:usize = rand::thread_rng().gen_range(0..N);
    // m.field[x][y].hidden = !m.field[x][y].hidden;
}


fn event(_app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(point) => {
            let x_temp = map_range(point.x, -(N as f32) * 0.5 * SCALE, (N as f32) * 0.5 * SCALE, 0.0, N  as f32);
            let y_temp = map_range(point.y, -(N as f32) * 0.5 * SCALE, (N as f32) * 0.5 * SCALE, 0.0, N as f32);
            m.field[x_temp as usize][y_temp as usize].hidden = false;
        },
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
    draw.background().color(LIGHTGRAY);
    // Draw field
    m.draw(&draw);
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}