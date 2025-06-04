use macroquad::prelude::*;
use crate::{player::Player, utils};

pub async fn island_scene() -> super::super::GameScene {
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);

    loop {
        clear_background(DARKGREEN);

        let delta = get_frame_time();
        player.update(delta);
        player.draw();

        draw_text("Island Scene", 20.0, 20.0, 30.0, WHITE);
        draw_text("Approach buildings to trigger links (placeholder)", 20.0, 60.0, 20.0, LIGHTGRAY);

        if is_key_pressed(KeyCode::Key1) {
            utils::open_link("https://github.com/yourusername");
        }
        if is_key_pressed(KeyCode::Key2) {
            utils::open_link("https://linkedin.com/in/yourusername");
        }
        if is_key_pressed(KeyCode::Key3) {
            let playlists = [
                "https://youtube.com/playlist?list=PLx...",
                "https://youtube.com/playlist?list=PLy...",
            ];
            utils::open_link(utils::choose_random_link(&playlists));
        }
        if is_key_pressed(KeyCode::Key4) {
            return super::super::GameScene::Glitch;
        }
        if is_key_pressed(KeyCode::F) {
            return super::super::GameScene::Fog;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }

    super::super::GameScene::Island
}

