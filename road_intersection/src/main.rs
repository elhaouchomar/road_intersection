use ::rand::{Rng, thread_rng};
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

impl Direction {
    fn str_drection(&self) -> &str {
        match self {
            Direction::East => "East",
            Direction::North => "North",
            Direction::South => "South",
            _ => "West",
        }
    }
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

struct Car {
    start_direction: Direction,
    end_direction: Direction,
    color: CarColor,
    position: Vec2,
    speed: f32,
}

#[derive(PartialEq, Eq, Clone, Copy)]
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
        Car {
            start_direction,
            end_direction,
            color,
            position: start_pos,
            speed: 2.0,
        }
    }

    fn update(&mut self, center_x: f32, center_y: f32, lights: &Vec<TrafficLight>) {
        // east 
        if self.position.x >= center_x
            && self.position.y == center_y
            && Direction::East == self.start_direction
            && self.color == CarColor::Yellow
        {
            self.start_direction = self.end_direction;
        }

        if self.position.x + 30.0 >= center_x
            && self.position.y >= center_y
            && Direction::East == self.start_direction
            && self.color == CarColor::Purple
        {
            self.start_direction = self.end_direction;
        }

        // west
        if self.position.x <= center_x
            && self.position.y == center_y
            && Direction::West == self.start_direction
            && self.color == CarColor::Yellow
        {
            self.start_direction = self.end_direction;
        }

        if self.position.x - 30.0 <= center_x
            && self.position.y >= center_y
            && Direction::West == self.start_direction
            && self.color == CarColor::Purple
        {
            self.start_direction = self.end_direction;
        }

        // 
        // if self.position.x >= center_x
        //     && self.position.y == center_y
        //     && Direction::East == self.start_direction
        //     && self.color == CarColor::Yellow
        // {
        //     self.start_direction = self.end_direction;
        // }

        // if self.position.x + 30.0 >= center_x
        //     && self.position.y >= center_y
        //     && Direction::East == self.start_direction
        //     && self.color == CarColor::Purple
        // {
        //     self.start_direction = self.end_direction;
        // }

        // if self.position.x >= center_x
        //     && self.position.y == center_y
        //     && Direction::East == self.start_direction
        //     && self.color == CarColor::Yellow
        // {
        //     self.start_direction = self.end_direction;
        // }

        // if self.position.x + 30.0 >= center_x
        //     && self.position.y >= center_y
        //     && Direction::East == self.start_direction
        //     && self.color == CarColor::Purple
        // {
        //     self.start_direction = self.end_direction;
        // }

        match self.start_direction {
            Direction::North => {
                let next_step = self.position.y + self.speed + 88.0;
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
                let next_step = self.position.y - self.speed - 60.0;
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
                let next_step = self.position.x + self.speed + 88.0;
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
                let next_step = self.position.x - self.speed - 60.0;
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
    // fn CanMove(&self , dr : &Direction )-> bool{
    //     match self.start_direction =>
    // }

    fn draw(&self, size: Vec2) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            size.x,
            size.y,
            self.color.color(),
        );
    }
}

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

    loop {
        // clear_background(BLACK);

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
            if is_key_pressed(KeyCode::Up) {
                click = 0.0;
                // println!("up");
                cars.push(Car::new(Direction::South, vec2(center_x, height_screen)));
            }
            if is_key_pressed(KeyCode::Down) {
                click = 0.0;
                //  println!("Down");
                cars.push(Car::new(Direction::North, vec2(center_x - 30.0, 0.0)));
            }
            if is_key_pressed(KeyCode::Left) {
                click = 0.0;
                //  println!("Left");
                cars.push(Car::new(Direction::East, vec2(0.0, center_y)));
            }
            if is_key_pressed(KeyCode::Right) {
                click = 0.0;
                //  println!("Right");
                cars.push(Car::new(
                    Direction::West,
                    vec2(width_screen, center_y - 30.0),
                ));
            }
        }

        let dt = 0.02;
        click += dt;
        timer += dt;
        if timer > green_duration {
            lights[current_green].set_state(LightState::Red);
            current_green = (current_green + 1) % lights.len();
            lights[current_green].set_state(LightState::Green);
            // lights[1].set_state(LightState::Green);
            timer = 0.0;
        }

        for (i, light) in lights.iter_mut().enumerate() {
            // if i == current_green {
            //     light.set_state(LightState::Green);
            // } else {
            //     light.set_state(LightState::Red);
            // }
            light.draw(light_size);
        }
        // let mut canmove = lights[0].direction.str_drection();
        // for i in lights {
        //     if i.state == LightState::Green {
        //         canmove = i.direction.str_drection();
        //     }
        // }
        for car in cars.iter_mut() {
            // if car.start_direction.str_drection() == canmove {
            car.update(center_x, center_y, &lights);
            car.draw(car_size);
            // }
        }

        next_frame().await;
    }
}
