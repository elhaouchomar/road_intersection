use macroquad::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LightState {
    Red,
    Green,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

pub struct TrafficLight {
    pub pos: Vec2,
    pub state: LightState,
    pub direction: Direction,
}

impl TrafficLight {
    pub fn new(pos: Vec2, direction: Direction) -> Self {
        Self {
            pos,
            state: LightState::Red,
            direction,
        }
    }

    pub fn set_state(&mut self, state: LightState) {
        self.state = state;
    }

    pub fn draw(&self, size: Vec2) {
        let color = match self.state {
            LightState::Green => GREEN,
            LightState::Red => RED,
        };
        draw_rectangle(self.pos.x, self.pos.y, size.x, size.y, color);
    }
}
