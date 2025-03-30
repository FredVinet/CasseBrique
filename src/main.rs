use macroquad::prelude::*;
use std::collections::HashSet;

const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 10.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPEED: f32 = 5.0;
const NB_LIGNES:i32 = 5;
const NB_COLONNES:i32 = 10;
const BRICK_WIDTH:f32 = 50.0;
const BRICK_HEIGHT:f32 = 20.0;
const BRICK_SPACING_X:f32 = 5.0;
const BRICK_SPACING_Y:f32 = 5.0;

struct Paddle {
    x: f32,
    y: f32,
}

impl Paddle {
    fn new() -> Self {
        Self {
            x: screen_width() / 2.0 - PADDLE_WIDTH / 2.0,
            y: screen_height() - 15.0,
        }
    }

    fn move_paddle(&mut self, keys: &HashSet<KeyCode>) {
        if keys.contains(&KeyCode::Q) && self.x > 0.0 {
            self.x -= PADDLE_SPEED;
        }
        if keys.contains(&KeyCode::D) && self.x + PADDLE_WIDTH < screen_width() {
            self.x += PADDLE_SPEED;
        }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
    }
}

struct Ball {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
}

impl Ball {
    fn new() -> Self {
        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            speed_x: BALL_SPEED,
            speed_y: BALL_SPEED,
        }
    }

    fn update(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;

        if self.x - BALL_RADIUS < 0.0 || self.x + BALL_RADIUS >= screen_width() {
            self.speed_x = -self.speed_x;
        }
        if self.y - BALL_RADIUS < 0.0 || self.y + BALL_RADIUS >= screen_height() {
            self.speed_y = -self.speed_y;
        }
    }

    fn check_paddle_collision(&mut self, paddle: &Paddle) {
        if self.y + BALL_RADIUS >= paddle.y
            && self.x >= paddle.x
            && self.x <= paddle.x + PADDLE_WIDTH
        {
            self.speed_y = -self.speed_y;
            self.y = paddle.y - BALL_RADIUS;
        }
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, BALL_RADIUS, RED);
    }
}

struct Brick {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Brick {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            width: 50.0,
            height: 20.0,
        }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, BLUE);
    }
    fn check_collision(&self, ball: &Ball) -> bool {
            ball.x + BALL_RADIUS >= self.x && 
            ball.x - BALL_RADIUS <= self.x + self.width && 
            ball.y + BALL_RADIUS >= self.y && 
            ball.y - BALL_RADIUS <= self.y + self.height
    }
    fn handle_collision(&self, ball: &mut Ball) {
        let ball_next_x = ball.x + ball.speed_x;
        let ball_next_y = ball.y + ball.speed_y;

        let hit_top = ball_next_y + BALL_RADIUS >= self.y && ball.y < self.y;
        let hit_bottom = ball_next_y - BALL_RADIUS <= self.y + self.height && ball.y > self.y + self.height;
        let hit_left = ball_next_x + BALL_RADIUS >= self.x && ball.x < self.x;
        let hit_right = ball_next_x - BALL_RADIUS <= self.x + self.width && ball.x > self.x + self.width;

        if hit_top || hit_bottom {
            ball.speed_y = -ball.speed_y;
        }
        if hit_left || hit_right {
            ball.speed_x = -ball.speed_x;
        }

        if hit_top {
            ball.y = self.y - BALL_RADIUS;
        } else if hit_bottom {
            ball.y = self.y + self.height + BALL_RADIUS;
        }
        
        if hit_left {
            ball.x = self.x - BALL_RADIUS;
        } else if hit_right {
            ball.x = self.x + self.width + BALL_RADIUS;
        }
    }
}

struct Game {
    paddle: Paddle,
    ball: Ball,
    bricks: Vec<Brick>,
    score: u32,
    is_running: bool,
    is_game_over: bool,
    lives: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            paddle: Paddle::new(),
            ball: Ball::new(),
            bricks: vec![],
            score: 0,
            is_running: false,
            is_game_over: false,
            lives: 3,
        }
    }

    async fn run(&mut self) {
        loop {
            clear_background(BLACK);
    
            if !self.is_running && !self.is_game_over {
                draw_text("Casse-Briques", screen_width() / 3.0 + 25.0, screen_height() / 3.0, 40.0, WHITE);
                draw_text("Appuyez sur ESPACE pour jouer", screen_width() / 3.0 - 50.0, screen_height() / 2.0, 30.0, GRAY);
                
                if is_key_pressed(KeyCode::Space) {
                    self.is_running = true;
                    self.is_game_over = false;
                    self.score = 0;
                    self.ball = Ball::new();
                    self.paddle = Paddle::new();
                    self.spawn_bricks();
                    
                }
            } else if self.is_game_over && self.lives == 0 {
                draw_text("GAME OVER", screen_width() / 3.0 + 40.0 , screen_height() / 3.0, 40.0, RED);
                draw_text("Appuyez sur ESPACE pour rejouer", screen_width() / 3.0 - 70.0, screen_height() / 2.0, 30.0, GRAY);
                
                if is_key_pressed(KeyCode::Space) {
                    self.is_running = true;
                    self.is_game_over = false;
                    self.score = 0;
                    self.ball = Ball::new();
                    self.paddle = Paddle::new();
                    self.spawn_bricks();
                }
            } else {
                let key_down = get_keys_down();
                self.paddle.move_paddle(&key_down);
    
                self.ball.update();
                self.ball.check_paddle_collision(&self.paddle);
    
                if self.ball.y + BALL_RADIUS >= screen_height() {
                    self.lives -= 1;
                    if self.lives == 0 {
                        self.is_game_over = true;
                        self.is_running = false;
                    }
                }
    
                let mut bricks_to_remove = vec![];
                for (i, brick) in self.bricks.iter_mut().enumerate() {
                    if brick.check_collision(&self.ball) {
                        brick.handle_collision(&mut self.ball);
                        bricks_to_remove.push(i);
                        self.score += 1;
                    }
                }
    
                for &index in bricks_to_remove.iter().rev() {
                    self.bricks.remove(index);
                }
    
                draw_text(&format!("Score: {}", self.score), 20.0, 30.0, 30.0, YELLOW);
                draw_text(&format!("Vie: {}", self.lives), 200.0, 30.0, 30.0, YELLOW);
    
                self.paddle.draw();
                self.ball.draw();
                for brick in &self.bricks {
                    brick.draw();
                }
            }
    
            next_frame().await;
        }
    }
    fn spawn_bricks(&mut self) {
        self.bricks.clear();

        let total_width = (NB_COLONNES as f32 * (BRICK_WIDTH + BRICK_SPACING_X)) - BRICK_SPACING_X;
    
        let start_x = (screen_width() - total_width) / 2.0;
        
        let start_y = screen_height() / 4.0;
    
        for i in 0..NB_LIGNES {
            for j in 0..NB_COLONNES {
                let x = start_x + j as f32 * (BRICK_WIDTH + BRICK_SPACING_X);
                let y = start_y + i as f32 * (BRICK_HEIGHT + BRICK_SPACING_Y);
                self.bricks.push(Brick::new(x, y));
            }
        }
    }
}

#[macroquad::main("CasseBrique")]
async fn main() {
    let mut game = Game::new();

    game.run().await;
}
