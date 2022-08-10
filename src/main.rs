
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

    fn is_bomb(& self)->bool {
        self.number == 10
    }

    fn is_empty(& self)->bool 
    {
        self.number == 0
    }

    fn is_hidden(& self)->bool {
        self.hidden
    }

    fn set_bomb(& mut self) {
        self.number = 10;
    }

    fn uncover(& mut self) {
        self.hidden = false;
    }
    fn draw(& self, draw: & Draw, x: usize, y: usize) {
        let (x_temp ,y_temp) = field_to_cord(x, y);
        let size = SCALE * 0.92;
        if self.is_hidden() {
            draw.rect().x_y(x_temp, y_temp).w_h(size, size).color(DARKGREY);
        } else {
            if self.is_bomb() {
                draw.rect().x_y(x_temp, y_temp).w_h(size, size).color(RED);
            } else if self.is_empty() {
                draw.rect().x_y(x_temp, y_temp).w_h(size, size).color(LIGHTGRAY);                
            } else {
                draw.text(&self.number.to_string()).x_y(x_temp, y_temp + 1.0).w_h(SCALE, SCALE).color(BLACK).font_size((SCALE as u32) - 2);
            }
        }
    }
}

fn cord_2_field(point: Point2) -> (isize, isize) {
    let x_temp = map_range(point.x, -(N as f32) * 0.5 * SCALE, (N as f32) * 0.5 * SCALE, 0.0, N as f32);
    let y_temp = map_range(point.y, -(N as f32) * 0.5 * SCALE, (N as f32) * 0.5 * SCALE, 0.0, N as f32);
    (x_temp as isize, y_temp as isize)
}

fn field_to_cord(x: usize, y: usize) -> (f32, f32){
    let x_temp = map_range(x as f32, 0.0, N as f32, -(N as f32 / 2.0) * SCALE, (N as f32 / 2.0) * SCALE) + SCALE / 2.0;
    let y_temp = map_range(y as f32, 0.0, N as f32, -(N as f32 / 2.0) * SCALE, (N as f32 / 2.0) * SCALE) + SCALE / 2.0;
    (x_temp, y_temp)
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model 
{
    field: [[Place; N]; N],
    success: bool,
    bomb: bool
}


impl Model {
    fn new() -> Self { 
        let place = Place::new();
        let mut model = Self { field: [[place; N]; N], success : false, bomb : false};
        model.place_bombs();
        for x in 0..N {
            for y in 0..N {
                if !model.field[x][y].is_bomb() {
                    model.surrounding(x, y);
                }
            }
        }
        model
    }

    fn draw(&self, draw : &Draw) {
        for x in 0..N {
            for y in 0..N {
                self.field[x][y].draw(draw, x, y);
            }
        }
    }

    fn uncover(& mut self, x: i16, y: i16) {
        if self.is_in_range(x, y) {
            let place = & mut self.field[x as usize][y as usize];
            if place.is_hidden() {
                place.uncover();
                if place.is_empty() {
                    self.uncover(x + 1, y);
                    self.uncover(x - 1, y);
                    self.uncover(x, y + 1);
                    self.uncover(x, y - 1);
                } else if place.is_bomb() {
                    self.bomb = true;
                }
            }
        }
    }

    fn place_bombs(& mut self) {
        for _i in 0..BOMBS_NUMBER {
            let x:usize = rand::thread_rng().gen_range(0..N);
            let y:usize = rand::thread_rng().gen_range(0..N);
            self.field[x][y].set_bomb();
        }
    }

    fn is_in_range(&self, x: i16, y : i16) -> bool{
        x >= 0 && x < (N as i16) && y >= 0 && y < (N as i16)
    }

    fn is_bomb(& self, x: usize, y: usize, dx: i16, dy: i16) -> u8 {
        let x_temp: i16 = x as i16 + dx;
        let y_temp: i16 = y as i16 + dy;
        
        if self.is_in_range(x_temp, y_temp){
            if self.field[x_temp as usize][y_temp as usize].is_bomb() { 
                return 1 }
        } 
        return 0;
    }

    fn surrounding(& mut self, x: usize, y: usize) {
        let mut count: u8 = 0;
        count += self.is_bomb(x, y, -1, -1);
        count += self.is_bomb(x, y,  0, -1);
        count += self.is_bomb(x, y,  1, -1);
        count += self.is_bomb(x, y, -1,  0);
        count += self.is_bomb(x, y,  1,  0);
        count += self.is_bomb(x, y, -1,  1);
        count += self.is_bomb(x, y,  0,  1);
        count += self.is_bomb(x, y,  1,  1);
        self.field[x][y].number = count;
    }
}


fn update(_app: &App, _m: &mut Model, _update: Update) {
}


fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(_) => {
            let (x_temp, y_temp) = cord_2_field(app.mouse.position());
            m.uncover(x_temp as i16, y_temp as i16);
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