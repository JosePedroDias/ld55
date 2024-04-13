mod state;

use comfy::*;
use state::*;
use std::process::exit;

const W: u8 = 16;
const H: u8 = 16;
const NUM_MINES: u16 = 32;
const SPRITE_W: f32 = 16.0;

struct State {
    pub board: Board,
    //pub music_playing: bool,
}

impl State {
    pub fn new(_c: &EngineState) -> Self {
        Self {
            board: Board::new((W, H)),
            //music_playing: false,
        }
    }
}

simple_game!("merge or die", State, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(700, 700),
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
    c.load_texture_from_bytes("1", include_bytes!("../assets/sprites/1.png"));
    c.load_texture_from_bytes("2", include_bytes!("../assets/sprites/2.png"));
    c.load_texture_from_bytes("3", include_bytes!("../assets/sprites/3.png"));
    c.load_texture_from_bytes("4", include_bytes!("../assets/sprites/4.png"));
    c.load_texture_from_bytes("5", include_bytes!("../assets/sprites/5.png"));
    c.load_texture_from_bytes("6", include_bytes!("../assets/sprites/6.png"));
    c.load_texture_from_bytes("7", include_bytes!("../assets/sprites/7.png"));
    c.load_texture_from_bytes("8", include_bytes!("../assets/sprites/8.png"));

    c.load_texture_from_bytes("empty", include_bytes!("../assets/sprites/empty.png"));
    c.load_texture_from_bytes("exploded", include_bytes!("../assets/sprites/exploded.png"));
    c.load_texture_from_bytes("flag", include_bytes!("../assets/sprites/flag.png"));
    c.load_texture_from_bytes("mine", include_bytes!("../assets/sprites/mine.png"));
    c.load_texture_from_bytes("unknown", include_bytes!("../assets/sprites/unknown.png"));

    // sfx
    // load_sound_from_bytes(
    //     "music",
    //     include_bytes!("../assets/sfx/comfy-music.ogg"),
    //     StaticSoundSettings::new().loop_region(..),
    // );
    // load_sound_from_bytes(
    //     "sound",
    //     include_bytes!("../assets/sfx/bell-sfx.ogg"),
    //     StaticSoundSettings::default(),
    // );

    let mut cam = main_camera_mut();
    cam.zoom = 280.0;
}

fn draw_cell(cell: &Cell, pos: &(u8, u8)) {
    let x = pos.0 as f32;
    let y = pos.1 as f32;
    let vec = Vec2::new(
        (x - W as f32 * 0.5 + 0.5) * SPRITE_W,
        (y - H as f32 * 0.5 + 0.5) * SPRITE_W,
    );
    let t = "1";
    draw_sprite(texture_id(t), vec, WHITE, 0, splat(SPRITE_W));
}

fn update(state: &mut State, _c: &mut EngineContext) {
    if is_key_down(KeyCode::Escape) {
        //c.quit_flag = true;
        exit(0); // TODO
    }

    let is_left_down: bool = is_mouse_button_pressed(MouseButton::Left);
    let is_right_down: bool = is_mouse_button_pressed(MouseButton::Right);
    let is_down = is_left_down || is_right_down;
    if is_down {
        let world_pos = mouse_world();
        let x: i32 = (world_pos.x / SPRITE_W).floor() as i32 + W as i32 / 2;
        let y: i32 = (world_pos.y / SPRITE_W).floor() as i32 + H as i32 / 2;
        if x < 0 || x >= state.board.size.0 as i32 || y < 0 || y >= state.board.size.1 as i32 {
            return;
        }

        let pos = (x as u8, y as u8);

        if is_left_down {
            println!("{:?}", pos);
        }
    }

    for y in 0..state.board.size.1 {
        for x in 0..state.board.size.0 {
            let pos = (x, y);
            let cell = state.board.get_cell(&pos).unwrap();
            draw_cell(cell, &pos);
        }
    }
}
