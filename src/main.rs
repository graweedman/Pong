
use core::f32;

use macroquad::prelude::*;

struct Position{
    x:f32, y:f32
}
struct Size {
    width: f32,
    height: f32
}

enum Move {
    Up,
    Down,
    None,
}

enum Board {
    Top,
    Bottom,
    Left,
    Right,
}
enum Side {
    Enemy,
    Player
}


struct Pad {
    position: Position,
    color: Color,
    side: Side,
    size: Size,
}

impl Default for Pad {
    fn default() -> Pad {
        Pad {
            color: Color::new(255.0, 255.0, 255.0, 100.0),
            position: Position {x: 0.0, y: 0.0},
            side: Side::Player,
            size: Size {width: 20.0, height: 200.0}
        }
    }
}

impl Default for Pong {
    fn default() -> Pong {
        Pong {
            position: Position { x: screen_width()/2.0, y: screen_height()/2.0 },
            color: Color::new(0.0, 0.0, 255.0, 100.0),
            velocity: (5.0, 5.0),
            radius: 20.0,
        }
    }
}

impl Pad {
    pub fn create(side: Side) -> Pad {
        match side {
            Side::Enemy =>
            Pad {
                    color: Color::new(255.0, 0.0, 0.0, 100.0),
                    position:Position{ x: screen_width() - 40.0, y: screen_height()/2.0},
                    side: side,
                    ..Default::default()
            },
            Side::Player =>
            Pad {
                    color: Color::new(0.0, 255.0, 0.0, 100.0),
                    position:Position{ x: 20.0, y: screen_height()/2.0},
                    side: side,
                    ..Default::default()
            }
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.width, self.size.height, self.color)
    }
    pub fn update(&mut self, speed:f32, direction:f32)
    {
            
        if self.check_wall() {
            self.position.y = self.position.y;
            
        }
        else {
            self.position.y += direction * speed;
        }
        
    }
}

impl Collision for Pad {
    fn check_wall(&self) -> Board {
        if self.position.y <= 5.0 {
            Board::Top
        }
        if self.position.y >= screen_height() - (self.size.height + 5.0)
        {
            Board::Bottom
        }
    }
}

trait Collision {
    fn check_wall(&self) -> Board;
}


struct Pong {
    position: Position,
    velocity: (f32, f32),
    color: Color,
    radius: f32,
}

impl Pong {
    pub fn launch(vx:f32, vy:f32) -> Pong {
        Pong {
            velocity: (vx, vy),
            ..Default::default()
        }
    }
    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color)
    }
    pub fn update(&self){

    }
}

impl Collision for Pong {
    fn check_wall(&self) -> Board {
        if self.position.y < 5.0
        {
            Board::Top
        }
        if self.position.y >= screen_height() - (self.radius + 5.0) {
            Board::Bottom
        }
        if self.position.x < 5.0 {
            Board::Left
        }
        if self.position.x > screen_width() - (self.radius + 5.0)
        {
            Board::Right
        }
    }

}

#[macroquad::main("Pong")]
async fn main() {
    let mut pad = Pad::create(Side::Player);
    let mut enemy = Pad::create(Side::Enemy);
    let mut pong = Pong::launch(2.0, 2.0);
    let mut direction:f32 = 0.0;
    const speed:f32 = 5.0;
    loop {
        if is_key_down(KeyCode::Up) {
            direction = -1.0
        }
        else if is_key_down(KeyCode::Down) {
            direction = 1.0
        }
        else {
            direction = 0.0
        }
        
        clear_background(BLACK);
        pad.update(speed, direction);
        enemy.update(speed, 1.0);
        
        //draw_grid(20, 20.0, GREEN, YELLOW);
        // draw_grid_2d();
        pong.draw();
        pad.draw();
        enemy.draw();
        next_frame().await;
    }
}

