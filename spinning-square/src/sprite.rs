use std::collections::LinkedList;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use rand::thread_rng;

trait Repr<R> {
    fn repr(&self) -> R;
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}


pub struct Snake {
    pub body: LinkedList<(i32, i32)>,
    pub direction: Direction,
}

impl Snake {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        gl.draw(args.viewport(), |context, gl| {
            let transform: [[f64; 3]; 2] = context.transform;
            self.repr().into_iter().for_each(|rect| {
                graphics::rectangle(RED, rect, transform, gl)
            })
        })
    }

    pub fn update(&mut self, inc: bool) {
        let mut head = self.body.front().expect("no snake!!!").clone();

        match self.direction {
            Direction::Right => head.0 += 1,
            Direction::Left => head.0 -= 1,
            Direction::Up => head.1 -= 1,
            Direction::Down => head.1 += 1,
        }

        self.body.push_front(head);
        if !inc {
            self.body.pop_back().unwrap();
        }
    }
}

impl Repr<Vec<graphics::types::Rectangle>> for Snake {

    fn repr(&self) -> Vec<[f64; 4]> {
        self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64)
            }).collect()
    }
}

pub struct Food {
    pub x: i32,
    pub y: i32,
}

impl Food {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let color = [0.0, 1.0, 1.0, 1.0];

        let food: [f64; 4] = graphics::rectangle::square((self.x * 20) as f64, (self.y * 20) as f64, 20_f64);

        gl.draw(args.viewport(), |ctx, gl| {
            let transform = ctx.transform;
            graphics::rectangle(color, food, transform, gl)
        })
    }

    pub fn update(&mut self) {
        let mut rng = thread_rng();
        self.x = rng.gen_range(0, 30);
        self.y = rng.gen_range(0, 30);
    }
}

