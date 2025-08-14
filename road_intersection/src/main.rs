use ::rand::{ Rng, thread_rng };
use macroquad::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
enum LightState {
    Red,
    Green,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

struct TrafficLight {
    pos: Vec2,
    state: LightState,
    direction: Direction,
}

impl TrafficLight {
    fn new(pos: Vec2, direction: Direction) -> Self {
        TrafficLight {
            pos,
            state: LightState::Red,
            direction,
        }
    }

    fn set_state(&mut self, state: LightState) {
        self.state = state;
    }

    fn draw(&self, size: Vec2) {
        let color = match self.state {
            LightState::Green => GREEN,
            LightState::Red => RED,
        };
        draw_rectangle(self.pos.x, self.pos.y, size.x, size.y, color);
    }
}

#[derive(PartialEq)]
struct Car {
    start_direction: Direction,
    end_direction: Direction,
    color: CarColor,
    position: Vec2,
    speed: f32,
}

#[derive(PartialEq, Copy, Clone)]
enum CarColor {
    Yellow,
    Blue,
    Purple,
}

impl CarColor {
    fn random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..3) {
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

impl Car {
    fn new(start_direction: Direction, start_pos: Vec2) -> Self {
        let mut end_direction = start_direction;

        let color = CarColor::random();

        println!("++++++++++{:?}", end_direction);

        let end_direction = match (start_direction, color) {
            (Direction::South, CarColor::Purple) => Direction::East,
            (Direction::South, CarColor::Yellow) => Direction::West,
            (Direction::North, CarColor::Purple) => Direction::East,
            (Direction::East, CarColor::Yellow) => Direction::South,
            _ => start_direction,
        };

        Car {
            start_direction,
            end_direction,
            color: color,
            position: start_pos,
            speed: 2.0,
        }
    }

    fn can_move(
        &self,
        center_x: f32,
        center_y: f32,
        lights: &Vec<TrafficLight>,
        cars: &Vec<Car>,
        my_index: usize
    ) -> bool {
        let safety_distance = 35.0;
        let stop_line_distance = 90.0;
        let car_size = 59.0;

        match self.start_direction {
            Direction::North => {
                let next_y = self.position.y + self.speed;
                let car_front = self.position.y + car_size;
                for light in lights {
                    if light.direction == Direction::North && light.state == LightState::Red {
                        let stop_line = center_y - stop_line_distance;
                        if car_front < stop_line && next_y + car_size >= stop_line {
                            return false;
                        }
                    }
                }
                for (i, other_car) in cars.iter().enumerate() {
                    if i == my_index {
                        continue;
                    }
                    if other_car.start_direction == Direction::North {
                        let horizontal_overlap = !(
                            self.position.x + car_size <= other_car.position.x ||
                            self.position.x >= other_car.position.x + car_size
                        );
                        if horizontal_overlap && other_car.position.y > self.position.y {
                            let distance_between = other_car.position.y - car_front;
                            if distance_between < safety_distance {
                                return false;
                            }
                        }
                    }
                }
            }
            Direction::South => {
                let next_y = self.position.y - self.speed;
                for light in lights {
                    if light.direction == Direction::South && light.state == LightState::Red {
                        let stop_line = center_y + stop_line_distance;
                        if self.position.y > stop_line && next_y <= stop_line {
                            return false;
                        }
                    }
                }
                for (i, other_car) in cars.iter().enumerate() {
                    if i == my_index {
                        continue;
                    }
                    if other_car.start_direction == Direction::South {
                        let horizontal_overlap = !(
                            self.position.x + car_size <= other_car.position.x ||
                            self.position.x >= other_car.position.x + car_size
                        );
                        if horizontal_overlap && other_car.position.y + car_size < self.position.y {
                            let distance_between =
                                self.position.y - (other_car.position.y + car_size);
                            if distance_between < safety_distance {
                                return false;
                            }
                        }
                    }
                }
            }
            Direction::East => {
                let next_x = self.position.x + self.speed;
                let car_front = self.position.x + car_size;
                for light in lights {
                    if light.direction == Direction::East && light.state == LightState::Red {
                        let stop_line = center_x - stop_line_distance;
                        if car_front < stop_line && next_x + car_size >= stop_line {
                            return false;
                        }
                    }
                }
                for (i, other_car) in cars.iter().enumerate() {
                    if i == my_index {
                        continue;
                    }
                    if other_car.start_direction == Direction::East {
                        let vertical_overlap = !(
                            self.position.y + car_size <= other_car.position.y ||
                            self.position.y >= other_car.position.y + car_size
                        );
                        if vertical_overlap && other_car.position.x > self.position.x {
                            let distance_between = other_car.position.x - car_front;
                            if distance_between < safety_distance {
                                return false;
                            }
                        }
                    }
                }
            }
            Direction::West => {
                let next_x = self.position.x - self.speed;
                for light in lights {
                    if light.direction == Direction::West && light.state == LightState::Red {
                        let stop_line = center_x + stop_line_distance;
                        if self.position.x > stop_line && next_x <= stop_line {
                            return false;
                        }
                    }
                }
                for (i, other_car) in cars.iter().enumerate() {
                    if i == my_index {
                        continue;
                    }
                    if other_car.start_direction == Direction::West {
                        let vertical_overlap = !(
                            self.position.y + car_size <= other_car.position.y ||
                            self.position.y >= other_car.position.y + car_size
                        );
                        if vertical_overlap && other_car.position.x + car_size < self.position.x {
                            let distance_between =
                                self.position.x - (other_car.position.x + car_size);
                            if distance_between < safety_distance {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
    fn update(
        &mut self,
        can_move: bool,
        center_x: f32,
        center_y: f32,
        _lights: &Vec<TrafficLight>
    ) {
        // Check if car reached intersection based on color and direction
        match (self.start_direction, self.color) {
            // East
            (Direction::East, CarColor::Yellow) if
                self.position.x >= center_x &&
                self.position.y == center_y
            => {
                self.start_direction = self.end_direction;
            }
            (Direction::East, CarColor::Purple) if
                self.position.x + 30.0 >= center_x &&
                self.position.y >= center_y
            => {
                self.start_direction = self.end_direction;
            }
            // West
            (Direction::West, CarColor::Yellow) if
                self.position.x + 30.0 <= center_x &&
                self.position.y == center_y - 30.0
            => {
                self.start_direction = self.end_direction;
            }
            (Direction::West, CarColor::Purple) if
                self.position.x <= center_x &&
                self.position.y >= center_y - 30.0
            => {
                self.start_direction = self.end_direction;
            }
            // South
            (Direction::South, CarColor::Yellow) if
                self.position.y + 30.0 <= center_y &&
                self.position.x + 30.0 >= center_x
            => {
                self.start_direction = self.end_direction;
            }
            (Direction::South, CarColor::Purple) if
                self.position.y <= center_y &&
                self.position.x == center_x
            => {
                self.start_direction = self.end_direction;
            }
            // North
            (Direction::North, CarColor::Yellow) if
                self.position.y >= center_y &&
                self.position.x + 30.0 == center_x
            => {
                self.start_direction = self.end_direction;
            }
            (Direction::North, CarColor::Purple) if
                self.position.y + 30.0 >= center_y &&
                self.position.x + 30.0 >= center_x
            => {
                self.start_direction = self.end_direction;
            }
            _ => {}
        }

        // Move car if allowed
        if can_move {
            match self.start_direction {
                Direction::North => {
                    self.position.y += self.speed;
                }
                Direction::South => {
                    self.position.y -= self.speed;
                }
                Direction::East => {
                    self.position.x += self.speed;
                }
                Direction::West => {
                    self.position.x -= self.speed;
                }
            }
        }
    }

    fn draw(&self) {
        let car_size = 30.0;
        draw_rectangle(self.position.x, self.position.y, car_size, car_size, self.color.color());
    }
}

fn get_queue_counts(cars: &Vec<Car>, center_x: f32, center_y: f32) -> Vec<i32> {
    let mut queue_counts = vec![0; 4]; // North, West, South, East
    for car in cars {
        let distance_to_center = match car.start_direction {
            Direction::North => (center_y - car.position.y).abs(),
            Direction::South => (car.position.y - center_y).abs(),
            Direction::East => (center_x - car.position.x).abs(),
            Direction::West => (car.position.x - center_x).abs(),
        };
        if distance_to_center < 250.0 {
            match car.start_direction {
                Direction::North => {
                    queue_counts[0] += 1;
                }
                Direction::West => {
                    queue_counts[1] += 1;
                }
                Direction::South => {
                    queue_counts[2] += 1;
                }
                Direction::East => {
                    queue_counts[3] += 1;
                }
            }
        }
    }
    queue_counts
}

fn is_direction_crowded(
    cars: &Vec<Car>,
    direction: Direction,
    screen_width: f32,
    screen_height: f32
) -> bool {
    let max_cars_per_direction = 8;
    let spawn_area_length = 300.0;
    let mut car_count = 0;
    for car in cars {
        if car.start_direction == direction {
            let in_spawn_area = match direction {
                Direction::North => car.position.y <= spawn_area_length,
                Direction::South => car.position.y >= screen_height - spawn_area_length,
                Direction::East => car.position.x <= spawn_area_length,
                Direction::West => car.position.x >= screen_width - spawn_area_length,
            };
            if in_spawn_area {
                car_count += 1;
            }
        }
    }
    car_count >= max_cars_per_direction
}

fn is_spawn_free(cars: &Vec<Car>, spawn_pos: Vec2, car_size: f32) -> bool {
    for car in cars {
        let overlap_x = !(
            spawn_pos.x + car_size <= car.position.x || spawn_pos.x >= car.position.x + car_size
        );
        let overlap_y = !(
            spawn_pos.y + car_size <= car.position.y || spawn_pos.y >= car.position.y + car_size
        );
        if overlap_x && overlap_y {
            return false;
        }
    }
    true
}

#[macroquad::main("Traffic Lights Fullscreen")]
async fn main() {
    let lane_space = 60.0;
    let light_size = vec2(50.0, 50.0);

    let mut lights: Vec<TrafficLight> = Vec::new();
    let mut cars: Vec<Car> = Vec::new();
    let mut current_green: usize = 0;
    let green_duration: f64 = 5.0;
    let min_green_duration: f64 = 2.5;
    let mut timer: f64 = 0.0;
    let mut click: f64 = 0.0;

    loop {
        // Get current window size dynamically
        let width_screen = screen_width();
        let height_screen = screen_height();
        let center_x = width_screen / 2.0;
        let center_y = height_screen / 2.0;

        clear_background(BLACK);

        // Draw road lines dynamically
        draw_line(center_x, 0.0, center_x, height_screen, 2.0, WHITE);
        draw_line(center_x - lane_space, 0.0, center_x - lane_space, height_screen, 1.0, GRAY);
        draw_line(center_x + lane_space, 0.0, center_x + lane_space, height_screen, 1.0, GRAY);
        draw_line(0.0, center_y, width_screen, center_y, 2.0, WHITE);
        draw_line(0.0, center_y - lane_space, width_screen, center_y - lane_space, 1.0, GRAY);
        draw_line(0.0, center_y + lane_space, width_screen, center_y + lane_space, 1.0, GRAY);

        // Initialize lights positions dynamically
        if lights.is_empty() {
            lights = vec![
                TrafficLight::new(
                    vec2(center_x - lane_space - 50.0, center_y - 50.0 - 60.0),
                    Direction::North
                ),
                TrafficLight::new(
                    vec2(center_x + lane_space, center_y - lane_space - 50.0),
                    Direction::West
                ),
                TrafficLight::new(
                    vec2(center_x + lane_space, center_y + lane_space),
                    Direction::South
                ),
                TrafficLight::new(
                    vec2(center_x - lane_space - 50.0, center_y + lane_space),
                    Direction::East
                )
            ];
            lights[current_green].set_state(LightState::Green);
        }

        // Handle input for spawning cars
        let car_size = 59.0;
        let dt = 0.02;
        click += dt;
        timer += dt;

        if click >= 0.5 {
            let spawn_pos_south = vec2(
                center_x + lane_space / 2.0 - car_size / 2.0,
                height_screen + 60.0
            );
            if
                is_key_pressed(KeyCode::Up) &&
                !is_direction_crowded(&cars, Direction::South, width_screen, height_screen) &&
                is_spawn_free(&cars, spawn_pos_south, car_size)
            {
                click = 0.0;
                cars.push(Car::new(Direction::South, spawn_pos_south));
            }

            let spawn_pos_north = vec2(center_x - lane_space / 2.0 - car_size / 2.0, -60.0);
            if
                is_key_pressed(KeyCode::Down) &&
                !is_direction_crowded(&cars, Direction::North, width_screen, height_screen) &&
                is_spawn_free(&cars, spawn_pos_north, car_size)
            {
                click = 0.0;
                cars.push(Car::new(Direction::North, spawn_pos_north));
            }

            let spawn_pos_east = vec2(-60.0, center_y + lane_space / 2.0 - car_size / 2.0);
            if
                is_key_pressed(KeyCode::Left) &&
                !is_direction_crowded(&cars, Direction::East, width_screen, height_screen) &&
                is_spawn_free(&cars, spawn_pos_east, car_size)
            {
                click = 0.0;
                cars.push(Car::new(Direction::East, spawn_pos_east));
            }

            let spawn_pos_west = vec2(
                width_screen + 60.0,
                center_y - lane_space / 2.0 - car_size / 2.0
            );
            if
                is_key_pressed(KeyCode::Right) &&
                !is_direction_crowded(&cars, Direction::West, width_screen, height_screen) &&
                is_spawn_free(&cars, spawn_pos_west, car_size)
            {
                click = 0.0;
                cars.push(Car::new(Direction::West, spawn_pos_west));
            }
        }

        // Traffic light logic
        if timer > min_green_duration {
            let queue_counts = get_queue_counts(&cars, center_x, center_y);
            let current_queue = queue_counts[current_green];

            let mut max_queue = 0;
            let mut max_queue_direction = 0;
            for (i, &count) in queue_counts.iter().enumerate() {
                if count > max_queue {
                    max_queue = count;
                    max_queue_direction = i;
                }
            }

            let should_switch =
                timer > green_duration ||
                (max_queue > current_queue + 3 && max_queue > 2) ||
                (current_queue == 0 && max_queue > 1 && timer > min_green_duration);

            if should_switch {
                lights[current_green].set_state(LightState::Red);
                if max_queue > 1 {
                    current_green = max_queue_direction;
                } else {
                    current_green = (current_green + 1) % lights.len();
                }
                lights[current_green].set_state(LightState::Green);
                timer = 0.0;
            }
        }

        // Draw traffic lights
        for light in lights.iter() {
            light.draw(light_size);
        }

        // Update cars
        let mut can_move_flags: Vec<bool> = Vec::new();
        for i in 0..cars.len() {
            can_move_flags.push(cars[i].can_move(center_x, center_y, &lights, &cars, i));
        }
        for (i, car) in cars.iter_mut().enumerate() {
            car.update(can_move_flags[i], center_x, center_y, &lights);
        }

        // Draw all cars
        for car in cars.iter() {
            car.draw();
        }

        // Remove cars that left the screen dynamically
        cars.retain(|car| {
            car.position.x > -60.0 &&
                car.position.x < width_screen + 60.0 &&
                car.position.y > -60.0 &&
                car.position.y < height_screen + 60.0
        });

        next_frame().await;
    }
}
