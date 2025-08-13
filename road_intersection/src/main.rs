use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

#[derive(PartialEq, Eq, Clone, Copy)]
enum LightState {
    Red,
    Green,
}
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn str_drection(&self)-> &'static str{
        match self {
            Direction::East => "East",
            Direction::North=>"North",
            Direction::South=>"South",
            _=>"West"
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
        Car {
            start_direction,
            end_direction :Direction::East,
            color: CarColor::random(),
            position: start_pos,
            speed: 2.0,
        }
    }

    fn update(&mut self) {
        match self.start_direction {
            Direction::North => self.position.y += self.speed,
            Direction::South => self.position.y -= self.speed,
            Direction::East => self.position.x += self.speed,
            Direction::West => self.position.x -= self.speed,
        }
    }

    fn draw(&self, size: Vec2) {
        draw_rectangle(self.position.x, self.position.y, size.x, size.y, self.color.color());
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

    let mut lights = vec![
        TrafficLight::new(vec2(center_x - lane_space - 50., center_y - 50. - 60.0), Direction::North),
        TrafficLight::new(vec2(center_x + lane_space, center_y - lane_space - 50.0), Direction::West),
        TrafficLight::new(vec2(center_x + lane_space, center_y + lane_space), Direction::South),
        TrafficLight::new(vec2(center_x - lane_space - 50., center_y + lane_space), Direction::East),
    ];

    let mut cars: Vec<Car> = Vec::new();
    let mut current_green = 0;
    let green_duration = 3.0;
    let mut timer = 0.0;
    let mut click = 0.0 ;

    loop {
        clear_background(BLACK);

        let mut can_xhenge_light = false ;
        println!("{}",can_xhenge_light);

        draw_line(center_x, 0.0, center_x, height_screen, 2.0, WHITE);
        draw_line(center_x - lane_space, 0.0, center_x - lane_space, height_screen, 1.0, GRAY);
        draw_line(center_x + lane_space, 0.0, center_x + lane_space, height_screen, 1.0, GRAY);

        draw_line(0.0, center_y, width_screen, center_y, 2.0, WHITE);
        draw_line(0.0, center_y - lane_space, width_screen, center_y - lane_space, 1.0, GRAY);
        draw_line(0.0, center_y + lane_space, width_screen, center_y + lane_space, 1.0, GRAY);

       if click >= 0.40 {
        if is_key_pressed(KeyCode::Up) {
            click = 0.0 ;
            cars.push(Car::new(Direction::South, vec2(center_x , height_screen)));
        }
        if is_key_pressed(KeyCode::Down) {
            click = 0.0 ;
              cars.push(Car::new(Direction::North, vec2(center_x - lane_space, 0.0)));
        }
        if is_key_pressed(KeyCode::Left) {
            click = 0.0 ;
            cars.push(Car::new(Direction::East, vec2(0.0, center_y )));
        }
        if is_key_pressed(KeyCode::Right) {
            click = 0.0 ;
            cars.push(Car::new(Direction::West, vec2(width_screen, center_y - lane_space)));
        }
        
    }

        let dt = 0.02;
        click += dt;
        

        for (i, light) in lights.iter_mut().enumerate() {
            light.draw(light_size);
        }
        let mut canmove = lights[0].direction.str_drection();
        for i in &lights {
            if i.state == LightState::Green {
                canmove = i.direction.str_drection();
            }
        }
        for car in cars.iter_mut() {
            if car.start_direction.str_drection() == canmove  {
                car.update();
                car.draw(car_size);
                 if car.start_direction.str_drection() == "North" || car.start_direction.str_drection() == "South"{
                    if car.position.y > center_y- 60. -30. || car.position.y < center_y+60.0 {
                        can_xhenge_light = true;
                    }
                }else {
                    if car.position.x > center_x- 60. - 30. || car.position.x < center_x+60.0 {
                        can_xhenge_light = true;
                    }
                }

                continue;
            }else {
                if car.start_direction.str_drection() == "North" || car.start_direction.str_drection() == "South" {
                if car.position.y < center_y- 60. -30. || car.position.y > center_y+60.0{
                    car.update();
                    car.draw(car_size);
                    continue;
                    //  if car.position.y > center_y- 60. || car.position.y < center_y+60.0
                }
            } else {
                if car.position.x < center_x- 60. -30. || car.position.x > center_x+60.0{
                    car.update();
                    car.draw(car_size);
                    continue;
                }
            }
            }
            car.draw(car_size);

        }
        println!("{}",can_xhenge_light);
        timer += dt;
        if timer > green_duration  {
            lights[current_green].set_state(LightState::Red);
            current_green = (current_green + 1) % lights.len();
            lights[current_green].set_state(LightState::Green);
            timer = 0.0;
        }

        next_frame().await;
    }
}
