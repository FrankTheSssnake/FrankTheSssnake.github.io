use macroquad::prelude::*;

pub async fn glitch_scene() -> super::super::GameScene {
    clear_background(RED);
    draw_text("Errno: unexpected NULL in dream zone", 100.0, 200.0, 30.0, WHITE);
    draw_text("Returning to island...", 100.0, 240.0, 20.0, LIGHTGRAY);

    for _ in 0..120 {
        next_frame().await;
    }

    super::super::GameScene::Island
}

