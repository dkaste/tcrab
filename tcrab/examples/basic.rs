use tcrab_console::{Console, Color, Canvas, Event, ControlFlow};
use tcrab_console::event::{KeyCode, ButtonState};
use tcrab_console::canvas::{Cell, CellBuffer};
use tcrab_console_gl::TextureRegion;

const TILESET_IMAGE_DATA: &[u8] = include_bytes!("./terminal.png");
const TILESET_CELL_WIDTH: u32 = 8;
const TILESET_CELL_HEIGHT: u32 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CustomGlyph {
    HappyFace,
}

impl tcrab_console::canvas::CustomGlyph for CustomGlyph {}

fn draw<C: Canvas<CustomGlyph>>(canvas: &mut C, player_pos: (i32, i32)) {
    canvas.fill(Cell {
        glyph: '.'.into(),
        foreground_color: Color::from_rgba_u8([25, 25, 25, 255]),
        background_color: Color::from_rgba_u8([15, 15, 15, 255]),
    });
    canvas.set_cell(player_pos.0 as usize, player_pos.1 as usize, Cell {
        glyph: '@'.into(),
        foreground_color: Color::from_rgba_u8([255, 0, 0, 255]),
        background_color: Color::BLACK,
    });
    canvas.set_cell(15, 10, Cell {
        glyph: CustomGlyph::HappyFace.into(),
        foreground_color: Color::from_rgba_u8([0, 255, 0, 255]),
        background_color: Color::from_rgba_u8([53, 35, 156, 255]),
    });
}

fn main() {
    pretty_env_logger::init();

    let tileset_image = image::load_from_memory(TILESET_IMAGE_DATA)
        .unwrap()
        .to_rgba();
    
    let glyph_lib = create_glyph_lib();

    let mut canvas = CellBuffer::new(80, 50, Cell::default());
    let mut console = tcrab_console_gl::Console::new(tcrab_console_gl::Settings {
        title: "tcrab example".into(),
        texture_width: tileset_image.width(),
        texture_height: tileset_image.height(),
        texture_data: tileset_image.into_vec(),
        cell_width: TILESET_CELL_WIDTH,
        cell_height: TILESET_CELL_HEIGHT,
    }).unwrap();

    let mut player_pos = (5, 10);
    let mut is_running = true;
    while is_running {
        let (canvas_width, canvas_height) = canvas.size();
        draw(&mut canvas, player_pos);
        console.present(&canvas, &glyph_lib);
        console.wait_for_events_forever(|event| match event {
            Event::Quit |
            Event::KeyboardInput { key_code: Some(KeyCode::Escape), .. } => {
                is_running = false;
                ControlFlow::Break
            }
            Event::KeyboardInput { key_code: Some(key_code), key_state: ButtonState::Pressed } => {
                let mut moved = false;
                let mut move_player = |x, y| {
                    let mut new_pos = player_pos;
                    new_pos.0 += x;
                    new_pos.1 += y;
                    if new_pos.0 < 0 {
                        new_pos.0 = 0;
                    }
                    if new_pos.0 > canvas_width as i32 - 1 {
                        new_pos.0 = canvas_width as i32 - 1;
                    }
                    if new_pos.1 < 0 {
                        new_pos.1 = 0;
                    }
                    if new_pos.1 > canvas_height as i32 - 1 {
                        new_pos.1 = canvas_height as i32 - 1;
                    }
                    if new_pos != player_pos {
                        moved = true;
                        player_pos = new_pos;
                        ControlFlow::Break
                    } else { ControlFlow::Continue }
                };
                match key_code {
                    KeyCode::Up => move_player(0, -1),
                    KeyCode::Down => move_player(0, 1),
                    KeyCode::Left => move_player(-1, 0),
                    KeyCode::Right => move_player(1, 0),
                    _ => ControlFlow::Continue,
                }
            }
            _ => ControlFlow::Continue,
        });
    }
}

fn create_glyph_lib() -> tcrab_console::canvas::GlyphLibrary<CustomGlyph, TextureRegion> {
    let mut glyph_lib = tcrab_console::canvas::GlyphLibrary::<CustomGlyph, _>::new();
    glyph_lib.define_glyph('.', tcrab_console_gl::TextureRegion {
        x: 0,
        y: 7 * TILESET_CELL_HEIGHT,
        width: TILESET_CELL_WIDTH,
        height: TILESET_CELL_HEIGHT,
    });
    glyph_lib.define_glyph('@', tcrab_console_gl::TextureRegion {
        x: 4 * TILESET_CELL_WIDTH,
        y: 0,
        width: TILESET_CELL_WIDTH,
        height: TILESET_CELL_HEIGHT,
    });
    glyph_lib.define_glyph(CustomGlyph::HappyFace, tcrab_console_gl::TextureRegion {
        x: 0,
        y: TILESET_CELL_HEIGHT,
        width: TILESET_CELL_WIDTH,
        height: TILESET_CELL_HEIGHT,
    });
    glyph_lib
}