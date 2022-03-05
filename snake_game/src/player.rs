use crate::shapes;
use crate::render;

pub struct Player {
    body: Vec<shapes::Rect>,
    dir: shapes::Vector2,
    last_rect: shapes::Rect,
    alive: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            body: vec![
                shapes::Rect {
                    pos: shapes::Vector2 { x: 0.0, y: 0.0 },
                    dim: shapes::Vector2 { x: 1.0, y: 1.0 }
                }],
            dir: shapes::Vector2 { x: 0.0, y: 1.0 },
            last_rect:
                shapes::Rect {
                    pos: shapes::Vector2 { x: 0.0, y: 0.0 },
                    dim: shapes::Vector2 { x: 0.0, y: 0.0 }
            },
            alive: true
        }
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    fn change_dir(&mut self, dir: shapes::Vector2) {
        if self.dir.x == -dir.x ||
           self.dir.y == -dir.y ||
           (dir.x == 0.0 && dir.y == 0.0){
               return;
        }
        if dir.x.abs() > 0.0 && dir.y.abs() > 0.0 {
            self.dir.x = dir.x;
            self.dir.y = 0.0;
            return;
        }
        self.dir = dir;
    }

    fn move_snake(&mut self, frame_info: render::FrameInfo) {
        let mut i = self.body.len() - 1;
        while i > 0 {
            self.body[i] = self.body[i-1];
            i -= 1;
        }
        self.body[0].pos += self.dir;
        self.body[0].pos.x = (self.body[0].pos.x + frame_info.width as f64) % frame_info.width as f64;
        self.body[0].pos.y = (self.body[0].pos.y + frame_info.height as f64) % frame_info.height as f64;
    }

    fn check_head_collision(&self) -> bool {
        let mut i = 4;
        while i < self.body.len() {
            if self.body[i].intersects(self.body[0]) {
                return true;
            }
            i+=1;
        }
        false
    }

    pub fn update(&mut self, frame_info: render::FrameInfo, dir: shapes::Vector2) {
        self.change_dir(dir);
        self.last_rect = self.body[self.body.len() - 1].clone();
        self.move_snake(frame_info);
        self.alive = !self.check_head_collision();
    }

    pub fn add_body(&mut self) {
        self.body.push(self.last_rect);
    }

    pub fn check_collision(&self, rect: shapes::Rect) -> bool {
        self.body[0].intersects(rect)
    }

    pub fn draw(&self, frame_info: render::FrameInfo, frame: &mut [u8]) {
        let mut i = 0;
        while i < self.body.len() {
            render::rect(frame_info, frame, self.body[i], &[0xff, 0x99, 0x00, 0xff]);
            i += 1;
        }
    }
}
