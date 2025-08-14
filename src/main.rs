mod car;
mod traffic_light;
mod utils;

use car::Car;
use traffic_light::{Direction, LightState, TrafficLight};
use utils::{count_cars_by_direction, find_busiest_direction, is_lane_full};

use macroquad::prelude::*;

#[macroquad::main("Traffic Lights Example")]
async fn main() {
    let width_screen = screen_width();
    let height_screen = screen_height();
    let center_x = width_screen / 2.0;
    let center_y = height_screen / 2.0;

    let lane_space = 60.0;
    let light_size = vec2(50.0, 50.0);
    let car_size = vec2(30.0, 30.0);


    let mut lights: Vec<TrafficLight> = vec![
        TrafficLight::new(
            vec2(center_x - lane_space - 50.0, center_y - 50.0 - 60.0),
            Direction::North,
        ),
        TrafficLight::new(
            vec2(center_x + lane_space, center_y - lane_space - 50.0),
            Direction::West,
        ),
        TrafficLight::new(
            vec2(center_x + lane_space, center_y + lane_space),
            Direction::South,
        ),
        TrafficLight::new(
            vec2(center_x - lane_space - 50.0, center_y + lane_space),
            Direction::East,
        ),
    ];

    let mut cars: Vec<Car> = Vec::new();
    let mut current_green: usize = 0;
    let green_duration: f64 = 3.0;
    let mut timer: f64 = 0.0;
    let mut click: f64 = 0.0;
    let mut priority_check_timer: f64 = 0.0;

    loop {
        draw_line(center_x, 0.0, center_x, height_screen, 2.0, WHITE);
        draw_line(
            center_x - lane_space,
            0.0,
            center_x - lane_space,
            height_screen,
            1.0,
            GRAY,
        );
        draw_line(
            center_x + lane_space,
            0.0,
            center_x + lane_space,
            height_screen,
            1.0,
            GRAY,
        );

        draw_line(0.0, center_y, width_screen, center_y, 2.0, WHITE);
        draw_line(
            0.0,
            center_y - lane_space,
            width_screen,
            center_y - lane_space,
            1.0,
            GRAY,
        );
        draw_line(
            0.0,
            center_y + lane_space,
            width_screen,
            center_y + lane_space,
            1.0,
            GRAY,
        );

        if click >= 0.4 {
            if is_key_pressed(KeyCode::Up)
                && !is_lane_full(&cars, Direction::South, vec2(center_x, height_screen))
            {
                click = 0.0;
                cars.push(Car::new(Direction::South, vec2(center_x, height_screen)));
            }
            if is_key_pressed(KeyCode::Down)
                && !is_lane_full(&cars, Direction::North, vec2(center_x - 30.0, 0.0))
            {
                click = 0.0;
                cars.push(Car::new(Direction::North, vec2(center_x - 30.0, 0.0)));
            }
            if is_key_pressed(KeyCode::Left)
                && !is_lane_full(&cars, Direction::East, vec2(0.0, center_y))
            {
                click = 0.0;
                cars.push(Car::new(Direction::East, vec2(0.0, center_y)));
            }
            if is_key_pressed(KeyCode::Right)
                && !is_lane_full(&cars, Direction::West, vec2(width_screen, center_y - 30.0))
            {
                click = 0.0;
                cars.push(Car::new(
                    Direction::West,
                    vec2(width_screen, center_y - 30.0),
                ));
            }
        }

        let dt = 0.02;
        click += dt;
        timer += dt;
        priority_check_timer += dt;
 
        if priority_check_timer > 1.0 {
            let busiest_direction = find_busiest_direction(&cars);
            let counts = count_cars_by_direction(&cars);
            
                let current_count = match current_green {
                    0 => counts[0],
                    1 => counts[3],
                    2 => counts[2],
                    3 => counts[1],
                    _ => 0,
                };
            
            
                let busiest_count = counts[busiest_direction];
                if busiest_count > current_count + 1 && timer > 1.0 {
            
                    lights[current_green].set_state(LightState::Red);
                    current_green = match busiest_direction {
                        0 => 0,
                        1 => 3,
                        2 => 2,
                        3 => 1,
                        _ => current_green,
                    };
                }

                lights[current_green].set_state(LightState::Green);
                timer = 0.0;

                priority_check_timer = 0.0;
    
            
        }

        if timer > green_duration {
            lights[current_green].set_state(LightState::Red);
            current_green = (current_green + 1) % lights.len();
            lights[current_green].set_state(LightState::Green);
            timer = 0.0;
        }

        cars.retain(|car| {
            car.position.x > -50.0
                && car.position.x < width_screen + 50.0
                && car.position.y > -50.0
                && car.position.y < height_screen + 50.0
        });

        for light in &lights {
            light.draw(light_size);
        }

        let other_cars_data: Vec<(Vec2, Direction)> = cars
            .iter()
            .map(|car| (car.position, car.start_direction))
            .collect();

        for (idx, car) in cars.iter_mut().enumerate() {
            let other_cars: Vec<(Vec2, Direction)> = other_cars_data
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, &data)| data)
                .collect();

            car.update(center_x, center_y, &lights, &other_cars);
            car.draw(car_size);
        }

        next_frame().await;
    }
}
