use macroquad::prelude::*;
use crate::car::Car;
use crate::traffic_light::Direction;

pub fn count_cars_by_direction(cars: &Vec<Car>) -> [usize; 4] {
    let mut counts = [0; 4];
    for car in cars {
        match car.start_direction {
            Direction::North => counts[0] += 1,
            Direction::East => counts[1] += 1,
            Direction::South => counts[2] += 1,
            Direction::West => counts[3] += 1,
        }
    }
    counts
}

pub fn find_busiest_direction(cars: &Vec<Car>) -> usize {
    let counts = count_cars_by_direction(cars);
    let mut max_index = 0;
    let mut max_count = counts[0];
    for i in 1..4 {
        if counts[i] > max_count {
            max_count = counts[i];
            max_index = i;
        } else if max_count  == counts[i]{
            max_index = i;
        }
    }
    max_index
}

pub fn is_lane_full(cars: &Vec<Car>, direction: Direction, _start_pos: Vec2) -> bool {
    
     let width_screen = screen_width();
    let height_screen = screen_height();
    for car in cars {
        if car.start_direction == direction {
            let is_can_create = match direction {
                Direction::North => car.position.y - 50.0>= 0.0,
                Direction::South => car.position.y <= height_screen-50.0,
                Direction::East => car.position.x - 50.0>= 0.0,
                Direction::West => car.position.x<= width_screen-50.0,
            };
            if !is_can_create {
                return  true;
            }
        }
    }
    return false;
}
