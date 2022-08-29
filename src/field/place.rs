use super::{SCALE};
use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Place {
    pub number: u8,
    hidden: bool,
    marker: bool,
}

impl Place {
    pub fn new() -> Self {
        Self {
            number: 0,
            hidden: true,
            marker: false,
        }
    }

    pub fn is_bomb(&self) -> bool {
        self.number == 10
    }

    pub fn is_empty(&self) -> bool {
        self.number == 0
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn set_bomb(&mut self) {
        self.number = 10;
    }

    pub fn uncover(&mut self) {
        self.hidden = false;
    }

    pub fn toggle_marker(&mut self) {
        self.marker = !self.marker;
    }

    pub fn draw(&self, draw: &Draw, x: usize, y: usize) {
        let (x_temp, y_temp) = field_to_cord(x, y);
        let size = SCALE * 0.92;
        if self.is_hidden() {
            draw.rect()
                .x_y(x_temp, y_temp)
                .w_h(size, size)
                .color(DARKGREY);
            if self.marker {
                draw.ellipse()
                    .x_y(x_temp, y_temp)
                    .w_h(size / 2.0, size / 2.0)
                    .color(BLUE);
            }
        } else {
            if self.is_bomb() {
                draw.rect().x_y(x_temp, y_temp).w_h(size, size).color(RED);
            } else if self.is_empty() {
                draw.rect()
                    .x_y(x_temp, y_temp)
                    .w_h(size, size)
                    .color(LIGHTGRAY);
            } else {
                draw.text(&self.number.to_string())
                    .x_y(x_temp, y_temp + 1.0)
                    .w_h(SCALE, SCALE)
                    .color(BLACK)
                    .font_size((SCALE as u32) - 2);
            }
        }
    }
}

fn field_to_cord(x: usize, y: usize) -> (f32, f32) {
    (x as f32 * SCALE, y as f32 * SCALE)
}
