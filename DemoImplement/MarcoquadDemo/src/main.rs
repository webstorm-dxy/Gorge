pub mod adaptor;
pub mod loader;

use macroquad::prelude::*;

use adaptor::{install_macroquad_platform, render_all, render_resource_counts};
use loader::GameLoader;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gorge Macroquad Demo - Dremu".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    install_macroquad_platform();

    let mut loader = GameLoader::new();
    match loader.load_all() {
        Ok(()) => {
            let (textures, sprites, nine_slices, curves) = render_resource_counts();
            println!(
                "[Gorge] 加载成功: textures={} sprites={} nine_slices={} curves={}",
                textures, sprites, nine_slices, curves,
            );
        }
        Err(e) => {
            eprintln!("[Gorge] 加载失败: {}", e);
            return;
        }
    }

    let mut last_time = get_time() as f32;
    let mut screenshot_taken = false;

    loop {
        let current_time = get_time() as f32;
        let delta = (current_time - last_time).min(0.1);
        last_time = current_time;

        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        loader.drive(delta);
        render_all();

        // 验收辅助：仿真约 5 秒时保存一帧画面，供人工确认判定线可见
        if !screenshot_taken && loader.simulation_time() >= 5.0 {
            screenshot_taken = true;
            let image = get_screen_data();
            image.export_png("test_output/screenshot.png");
            eprintln!("[Gorge] 已保存验收截图 test_output/screenshot.png");
        }

        draw_text(
            &format!("FPS: {}", get_fps()),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Time: {:.2}", loader.simulation_time()),
            10.0,
            45.0,
            20.0,
            GRAY,
        );

        next_frame().await;
    }
}

