use macroquad::prelude::*;



mod pad {
    pub struct Pad {
        position: Position,
        color: Color,
        side: Side,
        size: Size,
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
            if !self.check_wall() {
                self.position.y += direction * speed;
            }
            
        }
    }
}