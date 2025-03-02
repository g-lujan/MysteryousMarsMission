use glam::vec3;
use macroquad::prelude::*;
mod camera;
pub mod static_actor;
use camera::{Camera, Direction};
use static_actor::load_static_actors;
mod mesh_converter;
mod materials;
mod aabb;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

// New helper function to handle camera movement
fn handle_camera_movement(camera: &mut Camera, delta: f32) {
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        camera.move_camera(delta, Direction::Forward);
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        camera.move_camera(delta, Direction::Backward);
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        camera.move_camera(delta, Direction::Left);
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        camera.move_camera(delta, Direction::Right);
    }
}

#[macroquad::main(conf)]
async fn main() {
    let bounds = 8.0;
    let world_up = vec3(0.0, 1.0, 0.0);

    let floor_texture = load_texture("textures/mars_floor.png").await.unwrap();

    let mut camera = Camera::new(vec3(0.0, 1.0, 0.0), world_up);
    let mut x = 0.0;
    let mut switch = false;
    let mut grabbed = true;
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let screen_texture = render_target(1280, 720); // Set to your window size
    let noise_material = materials::get_noise();

    let mut rocks = Vec::new();
    match load_static_actors("map_data/props_positions.json").await {
        Ok(actors) => {
            rocks = actors;
        }
        Err(e) => eprintln!("Error loading actors: {}", e),
    }

    set_cursor_grab(grabbed);
    show_mouse(false);

    let mut time = 0.0;

    loop {
        let delta = get_frame_time();
        time += delta;

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        // Handle camera movement using the new function
        handle_camera_movement(&mut camera, delta);

        // Handle mouse movement for rotation
        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        if grabbed {
            camera.update_rotation(mouse_delta, delta);

            x += if switch { 0.04 } else { -0.04 };
            if x >= bounds || x <= -bounds {
                switch = !switch;
            }
        }

        // Set the camera for 3D scene
        set_camera(&Camera3D {
            position: camera.position,
            up: camera.up,
            target: camera.position + camera.front,
            ..Default::default()
        });

        clear_background(LIGHTGRAY);

        // Draw 3D objects as usual
        draw_plane(
            vec3(0.0, 0.0, 0.0),
            vec2(2000.0, 2000.0),
            Some(&floor_texture),
            WHITE,
        );

        for rock in &rocks {
            draw_mesh(&rock.mesh);
        }
        
        // Set the "time" uniform for animation
        noise_material.set_uniform("time", time);

        // Reset to the default render target (the screen)
        set_default_camera();

        // Apply the shader and draw the texture as a full-screen quad
        gl_use_material(&noise_material);
        draw_texture_ex(
            &screen_texture.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        // Render text
        set_default_camera();
        draw_hud();
        next_frame().await;
    }
}

fn draw_hud(){
    draw_text("FORWARD: ", 10.0, 20.0, 20.0, LIGHTGRAY);
    draw_text("ENABLED", 130.0, 20.0, 20.0, GREEN);
    draw_text("BACKWARDS: ", 10.0, 40.0, 20.0, LIGHTGRAY);
    if (get_time() as u64) % 2 == 0 {
        draw_text("DISABLED", 130.0, 40.0, 20.0, RED);
    }
}
