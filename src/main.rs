use macroquad::prelude::*;

#[macroquad::main("Casse-Briques")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_rectangle(screen_width() / 2.0 - 50.0, screen_height() - 30.0, 100.0, 10.0, WHITE);
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 10.0, RED);

        next_frame().await;
    }
}