mod input;
mod render;
mod shapes;
mod player;

pub mod snake
{
    pub use crate::input::Input;
    use crate::render;
    use crate::shapes;
    use crate::player;
    use rand::Rng;

    const ADD_FOOD_DELAY : f64 = 3.0;
    const INITIAL_TIMESTEP: f64 = 0.1;

    pub struct Game {
        frame_info: render::FrameInfo,
        player: player::Player,
        food: Vec<shapes::Rect>,
        timestep: f64,
        update_timer: f64,
        food_timer: f64,
    }

    impl Game {
        pub fn new(width: u32, height: u32) -> Game {
            Game {
                frame_info: render::FrameInfo{width, height},
                player: player::Player::new(),
                food: vec![],
                timestep: INITIAL_TIMESTEP,
                update_timer: 0.0,
                food_timer: 0.0,
            }
        }

        fn add_food(&mut self) {
            let mut rng = rand::thread_rng();
            self.food.push(
            shapes::Rect {
                pos: shapes::Vector2 { x: rng.gen_range(0, self.frame_info.width) as f64, y: rng.gen_range(0, self.frame_info.height) as f64},
                dim: shapes::Vector2 { x: 1.0, y: 1.0 }
            }
            );
        }

        fn food_update(&mut self) {
            if self.food_timer > ADD_FOOD_DELAY {
                self.food_timer = 0.0;
                self.add_food();
                self.timestep *= 0.995
            }
            let mut i = 0;
            while i < self.food.len() {
                if self.player.check_collision(self.food[i]) {
                    self.player.add_body();
                    self.food.remove(i);
                    break;
                }
                i += 1;
            }
        }

        pub fn update(&mut self, dt: f64, input: Input) {
            self.update_timer += dt;
            self.food_timer += dt;

            if self.update_timer > self.timestep {
                self.update_timer = 0.0;
                self.player.update(self.frame_info, input.get_dir_vec());
                self.food_update();
            }

            if !self.player.alive() {
                self.player = player::Player::new();
                self.food = vec![];
                self.timestep = INITIAL_TIMESTEP;
            }
        }

        pub fn draw(&self, frame: &mut [u8]) {
            render::background(frame, &[0x64,0x54,0x91,0xff]);
            self.food.iter().for_each( |r| render::rect(self.frame_info, frame, *r, &[0x09,0xa5,0xfe,0xff]) );
            self.player.draw(self.frame_info, frame);
        }
    }

}
