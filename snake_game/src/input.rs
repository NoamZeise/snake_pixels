use crate::shapes;

pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            up: false,
            down: false,
            left: false,
            right: false
        }
    }

    pub fn get_dir_vec(&self) -> shapes::Vector2 {
        let mut dir = shapes::Vector2 { x: 0.0, y: 0.0 };
        if self.up {
            dir.y -= 1.0;
        }
        if self.down {
            dir.y += 1.0;
        }
        if self.left {
            dir.x -= 1.0;
        }
        if self.right {
            dir.x += 1.0;
        }
        dir
    }
}
