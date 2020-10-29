extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::LinkedList;
use std::iter::FromIterator;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Button, ButtonEvent, ButtonState, EventLoop, Key};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use rand::{Rng, thread_rng};

use sprite::{Direction, Snake, Food};

mod sprite;

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        self.gl.draw(args.viewport(), |_, gl| {
            clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, args);
        self.food.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        if self.food.x == self.snake.body.front().expect("").0 &&
            self.food.y == self.snake.body.front().expect("").1 {
            self.snake.update(true);
            self.food.update();
        } else {
            self.snake.update(false)
        }
    }

    fn pressed(&mut self, button: &Button) {
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match button {
            Button::Keyboard(Key::Up) if last_direction != Direction::Up && last_direction != Direction::Down => {
                Direction::Up
            }
            Button::Keyboard(Key::Down) if last_direction != Direction::Down && last_direction != Direction::Up => {
                Direction::Down
            }
            Button::Keyboard(Key::Left) if last_direction != Direction::Left && last_direction != Direction::Right => {
                Direction::Left
            }
            Button::Keyboard(Key::Right) if last_direction != Direction::Right && last_direction != Direction::Left => {
                Direction::Right
            }
            _ => last_direction
        };
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter(vec![(0, 0)].into_iter()),
            direction: Direction::Right,
        },
        food: Food {
            x: 8,
            y: 8,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(5);

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            game.render(&args);
        }

        if let Some(_) = event.update_args() {
            game.update()
        }

        if let Some(k) = event.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button)
            }
        }
    }
}