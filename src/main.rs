mod state;

use comfy::*;
use comfy::egui::emath::Numeric;
use state::*;
use std::process::exit;

const SPRITE_W: f32 = 16.0;
const DROP_SHADOW_OFFSET: f32 = 1.0;

const UI_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.33);

impl State {
    pub fn new(_c: &EngineState) -> Self {
        Self::new_()
    }
}

simple_game!("merge or die", State, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(1000, 1000),
        min_resolution: ResolutionConfig::Physical(128, 128),
        bloom_enabled: false,
        lighting_enabled: false,
        dev: DevConfig {
            ..Default::default()
        },
        ..config
    }
}

fn setup(_state: &mut State, c: &mut EngineContext) {
    // fonts
    // c.load_fonts_from_bytes(&[(
    //     "uni",
    //     include_bytes!("../assets/fonts/univers-light-normal.ttf"),
    // )]);

    // sprites
    c.load_texture_from_bytes("highlight", include_bytes!("../assets/sprites/highlight.png"));
    c.load_texture_from_bytes("1", include_bytes!("../assets/sprites/1.png"));
    c.load_texture_from_bytes("2", include_bytes!("../assets/sprites/2.png"));
    c.load_texture_from_bytes("3", include_bytes!("../assets/sprites/3.png"));
    c.load_texture_from_bytes("4", include_bytes!("../assets/sprites/4.png"));
    c.load_texture_from_bytes("5", include_bytes!("../assets/sprites/5.png"));
    c.load_texture_from_bytes("6", include_bytes!("../assets/sprites/6.png"));
    c.load_texture_from_bytes("7", include_bytes!("../assets/sprites/7.png"));
    c.load_texture_from_bytes("8", include_bytes!("../assets/sprites/8.png"));

    // sfx
    load_sound_from_bytes(
        "fill",
        include_bytes!("../assets/sfx/fill.wav"),
        StaticSoundSettings::default(),
    );
    load_sound_from_bytes(
        "merge",
        include_bytes!("../assets/sfx/merge.wav"),
        StaticSoundSettings::default(),
    );
    load_sound_from_bytes(
        "mistake",
        include_bytes!("../assets/sfx/mistake.wav"),
        StaticSoundSettings::default(),
    );
    load_sound_from_bytes(
        "penalty",
        include_bytes!("../assets/sfx/penalty.wav"),
        StaticSoundSettings::default(),
    );
    load_sound_from_bytes(
        "incoming_tick",
        include_bytes!("../assets/sfx/incoming_tick.wav"),
        StaticSoundSettings::default(),
    );
    
    let mut cam = main_camera_mut();
    cam.zoom = 280.0 * 0.5;
}

fn get_cell_canvas_vector(pos: &Coords, size: &Coords) -> Vec2 {
    let x = pos.0 as f32;
    let y = pos.1 as f32;
    Vec2::new(
        (x - size.0 as f32 * 0.5 + 0.5) * SPRITE_W,
        (y - size.1 as f32 * 0.5 + 0.5) * SPRITE_W,
    )
}

fn draw_cell(cell: &Cell, pos: &Coords, size: &Coords) {
    let vec = get_cell_canvas_vector(pos, size);
    
    if cell.number == 0 {
        return;
    }
    
    let t = num_to_char(cell.number);
    let t = t.to_string();
    let t = t.as_str();
    
    let vec2 = Vec2::new(vec.x + DROP_SHADOW_OFFSET, vec.y - DROP_SHADOW_OFFSET);
    draw_sprite(texture_id(t), vec2, BLACK, 0, splat(SPRITE_W));
    draw_sprite(texture_id(t), vec, WHITE, 0, splat(SPRITE_W));
}

fn update(state: &mut State, c: &mut EngineContext) {
    clear_background(Color::new(0.25, 0.25, 0.25, 1.0));
    
    if is_key_down(KeyCode::Escape) {
        exit(0); // TODO
    }
    
    if !state.game_ended {
        state.handle_countdowns(c.delta.to_f64());
        
        if state.has_won() {
            state.game_ended = true;
            // TODO: elapsed time to be captured in board
        }
    }
    
    clear_background(Color::new(0.25, 0.25, 0.25, 1.0));
    
    let size = state.level_params.size.clone();
    
    if !state.game_ended && is_mouse_button_pressed(MouseButton::Left) {
        if !state.game_started {
            state.game_started = true;
        }
        
        let world_pos = mouse_world();
        let x: i32 = (world_pos.x / SPRITE_W).floor() as i32 + size.0 as i32 / 2;
        let y: i32 = (world_pos.y / SPRITE_W).floor() as i32 + size.1 as i32 / 2;
        if x < 0 || x >= size.0 as i32 || y < 0 || y >= size.1 as i32 {
            return;
        }

        let pos = (x as u8, y as u8);
        state.add_to_selection(&pos);
    }

    for y in 0..size.1 {
        for x in 0..size.0 {
            let pos = (x, y);
            let cell = state.get_cell(&pos).unwrap();
            draw_cell(cell, &pos, &size);
        }
    }
    
    if let Some(pos) = state.selection {
        let vec = get_cell_canvas_vector(&pos, &size);
        draw_sprite(texture_id("highlight"), vec, WHITE, 0, splat(SPRITE_W));
    }
    
    // UI overlay
    
    if !state.game_started {
        let label = format!("merge until you get a {}
be careful with The Bar
click to start!", state.level_params.goal_number);
        let label = label.as_str();
        draw_text(
            label,
            Vec2::new(0.0, 0.0),
            UI_COLOR,
            TextAlign::Center,
        );
    } else if state.game_ended {
        let label = format!("Finished in {:.2} secs!", state.elapsed_time);
        let label = label.as_str();
        draw_text(
            label,
            Vec2::new(0.0, 0.0),
            UI_COLOR,
            TextAlign::Center,
        );
    } else {
        let label = format!("elapsed: {:.0}", state.elapsed_time);
        let label = label.as_str();
        draw_text(
            label,
            Vec2::new(0.0, 62.0),
            UI_COLOR,
            TextAlign::Center,
        );
    }
    
    // draw penalty border
    let p = state.penalty_countdown / state.level_params.penalty_countdown;
    
    draw_rect(
        Vec2::new(0.0, -62.0),
        Vec2::new(120.0 * p as f32, 6.0),
        Color::new(1.0, p as f32, 0.0, 1.0),
        0
    );
    
    
}
