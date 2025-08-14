use crate::traffic_light::{Direction, LightState, TrafficLight};
use ::rand::Rng;
use ::rand::thread_rng;
use macroquad::prelude::*;

const CAR_SIZE: f32 = 30.0;
const MIN_DISTANCE: f32 = 100.0;
const NORTH_OFFSET: f32 = 88.0;
const EAST_OFFSET: f32 = 88.0;
const SOUTH_OFFSET: f32 = 60.0;
const WEST_OFFSET: f32 = 60.0;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CarColor {
    Yellow,
    Blue,
    Purple,
}

impl CarColor {
    fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => CarColor::Yellow,
            1 => CarColor::Blue,
            _ => CarColor::Purple,
        }
    }

    fn color(&self) -> Color {
        match self {
            CarColor::Yellow => YELLOW,
            CarColor::Blue => BLUE,
            CarColor::Purple => PURPLE,
        }
    }
}

pub struct Car {
    pub start_direction: Direction,
    pub end_direction: Direction,
    pub color: CarColor,
    pub position: Vec2,
    pub speed: f32,
}

impl Car {
    pub fn new(start_direction: Direction, start_pos: Vec2) -> Self {
        let color = CarColor::random();
        let end_direction = match color {
            CarColor::Blue => start_direction,
            CarColor::Purple => match start_direction {
                Direction::East => Direction::North,
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
            },
            CarColor::Yellow => match start_direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        };
        Self {
            start_direction,
            end_direction,
            color,
            position: start_pos,
            speed: 2.0,
        }
    }
    fn is_ahead(&self, other_pos: &Vec2) -> bool {
        match self.end_direction {
            Direction::North => {
                other_pos.y < self.position.y && other_pos.y > self.position.y - 30.0
            }
            Direction::South => {
                other_pos.y > self.position.y && other_pos.y < self.position.y + 30.0
            }
            Direction::East => {
                other_pos.x < self.position.x && other_pos.x > self.position.x - 30.0
            }
            Direction::West => {
                other_pos.x > self.position.x && other_pos.x < self.position.x + 30.0
            }
        }
    }
    fn is_in_same_lane(&self, other_pos: &Vec2) -> bool {
        match self.end_direction {
            Direction::North | Direction::South => (self.position.x - other_pos.x).abs() < 30.0,
            Direction::East | Direction::West => (self.position.y - other_pos.y).abs() < 30.0,
        }
    }
    fn can_move(&self, other_cars: &[(Vec2, Direction)]) -> bool {
        for &(other_pos, other_dir) in other_cars {
            let distance = match self.start_direction {
                Direction::North => {
                    if other_dir == Direction::North
                        && (other_pos.x - self.position.x).abs() < CAR_SIZE
                    {
                        other_pos.y - self.position.y
                    }else if self.is_in_same_lane(&other_pos) &&  self.is_ahead(&other_pos) {
                         other_pos.y - self.position.y
                    }else{
                        MIN_DISTANCE
                    }
                }
                Direction::South => {
                    if other_dir == Direction::South
                        && (other_pos.x - self.position.x).abs() < CAR_SIZE
                    {
                        self.position.y - other_pos.y
                    } else {
                        MIN_DISTANCE
                    }
                }
                Direction::East => {
                    if other_dir == Direction::East
                        && (other_pos.y - self.position.y).abs() < CAR_SIZE
                    {
                        other_pos.x - self.position.x
                    } else {
                        MIN_DISTANCE
                    }
                }
                Direction::West => {
                    if other_dir == Direction::West
                        && (other_pos.y - self.position.y).abs() < CAR_SIZE
                    {
                        self.position.x - other_pos.x
                    } else {
                        MIN_DISTANCE
                    }
                }
            };
            if distance < MIN_DISTANCE && distance > 0.0 {
                return false;
            }
        }
        true
    }

    pub fn update(
        &mut self,
        center_x: f32,
        center_y: f32,
        lights: &Vec<TrafficLight>,
        other_cars: &[(Vec2, Direction)],
    ) {
        if self.position.x >= center_x
            && self.position.y == center_y
            && Direction::East == self.start_direction
            && self.color == CarColor::Yellow
        {
            self.start_direction = self.end_direction;
        }
        if self.position.x + CAR_SIZE >= center_x
            && self.position.y >= center_y
            && Direction::East == self.start_direction
            && self.color == CarColor::Purple
        {
            self.start_direction = self.end_direction;
        }
        if self.position.x + CAR_SIZE <= center_x
            && self.position.y == center_y - CAR_SIZE
            && Direction::West == self.start_direction
            && self.color == CarColor::Yellow
        {
            self.start_direction = self.end_direction;
        }
        if self.position.x <= center_x
            && self.position.y >= center_y - CAR_SIZE
            && Direction::West == self.start_direction
            && self.color == CarColor::Purple
        {
            self.start_direction = self.end_direction;
        }
        if self.position.y + CAR_SIZE <= center_y
            && self.position.x + CAR_SIZE >= center_x
            && Direction::South == self.start_direction
            && self.color == CarColor::Yellow
        {
            self.start_direction = self.end_direction;
        }
        if self.position.y <= center_y
            && self.position.x == center_x
            && Direction::South == self.start_direction
            && self.color == CarColor::Purple
        {
            self.start_direction = self.end_direction;
        }
        if self.position.y >= center_y
            && self.position.x + CAR_SIZE == center_x
            && Direction::North == self.start_direction
            && self.color == CarColor::Yellow
        {
            self.start_direction = self.end_direction;
        }
        if self.position.y + CAR_SIZE >= center_y
            && self.position.x + CAR_SIZE >= center_x
            && Direction::North == self.start_direction
            && self.color == CarColor::Purple
        {
            self.start_direction = self.end_direction;
        }

        if !self.can_move(other_cars) {
            return;
        }

        match self.start_direction {
            Direction::North => {
                let next_step = self.position.y + self.speed + NORTH_OFFSET;
                for light in lights {
                    if next_step == center_y
                        && light.direction == Direction::North
                        && light.state == LightState::Red
                    {
                        return;
                    }
                }
                self.position.y += self.speed;
            }
            Direction::South => {
                let next_step = self.position.y - self.speed - SOUTH_OFFSET;
                for light in lights {
                    if next_step == center_y
                        && light.direction == Direction::South
                        && light.state == LightState::Red
                    {
                        return;
                    }
                }
                self.position.y -= self.speed;
            }
            Direction::East => {
                let next_step = self.position.x + self.speed + EAST_OFFSET;
                for light in lights {
                    if next_step == center_x
                        && light.direction == Direction::East
                        && light.state == LightState::Red
                    {
                        return;
                    }
                }
                self.position.x += self.speed;
            }
            Direction::West => {
                let next_step = self.position.x - self.speed - WEST_OFFSET;
                for light in lights {
                    if next_step == center_x
                        && light.direction == Direction::West
                        && light.state == LightState::Red
                    {
                        return;
                    }
                }
                self.position.x -= self.speed;
            }
        }
    }

    pub fn draw(&self, size: Vec2) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            size.x,
            size.y,
            self.color.color(),
        );
    }
}
