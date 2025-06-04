use macroquad::prelude::*;

pub async fn fog_scene() -> super::super::GameScene {
    loop {
        clear_background(GRAY);
        draw_text("You are lost in the fog.", 100.0, 100.0, 30.0, WHITE);
        draw_text("There is no escape.", 100.0, 140.0, 20.0, DARKGRAY);

        if is_key_pressed(KeyCode::Escape) {
            return super::super::GameScene::Island;
        }

        next_frame().await;
    }
}

