use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};
use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.00];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.00];
const GAMEOVER_COLOR: Color = [0.80, 0.00, 0.00, 5.00];

const MOVING_PERIOD: f64 = 0.3;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exist: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exist: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        // if next direction is opposite to the original direction
        // return
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.snake.draw(ctx, g);

        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, ctx, g);
        }

        // draw the upper border
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, ctx, g);
        // draw the bottom border
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, ctx, g);
        // draw the left border
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, ctx, g);
        // draw the right border
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, ctx, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, ctx, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exist {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    /// restart the game
    /// reset properties
    pub fn restart(&mut self) {
        self.waiting_time = 0.0;
        self.game_over = false;
        self.snake = Snake::new(2, 2);
        self.food_exist = true;
        self.food_x = 6;
        self.food_y = 4;
    }

    /// randomly generate a food
    pub fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exist = true;
    }

    /// update snake
    /// if snake alive then move forward and check eat food
    /// else game over
    pub fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_snake_ailve(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    /// check if the snake reach out to a food
    /// if true, eat the food.
    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exist && self.food_x == head_x && self.food_y == head_y {
            self.food_exist = false;
            self.snake.restore_tail();
        }
    }

    fn check_snake_ailve(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        return next_x > 0 && next_x < self.width - 1 && next_y > 0 && next_y < self.height - 1;
    }
}
