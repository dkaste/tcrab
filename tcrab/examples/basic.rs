use tcrab_console::{Console, Color, Canvas};

const TILESET_IMAGE_DATA: &[u8] = include_bytes!("./terminal.png");
const TILESET_CELL_WIDTH: u32 = 8;
const TILESET_CELL_HEIGHT: u32 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CustomGlyph {
    HappyFace,
}

impl tcrab_console::canvas::CustomGlyph for CustomGlyph {}

fn main() {
    pretty_env_logger::init();

    let tileset_image = image::load_from_memory(TILESET_IMAGE_DATA)
        .unwrap()
        .to_rgba();
    
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
    let mut canvas = tcrab_console::canvas::CellBuffer::new(80, 50, tcrab_console::canvas::Cell {
        glyph: '.'.into(),
        foreground_color: Color::from_rgba_u8([25, 25, 25, 255]),
        background_color: Color::from_rgba_u8([15, 15, 15, 255]),
    });
    canvas.set_cell(5, 10, tcrab_console::canvas::Cell {
        glyph: '@'.into(),
        foreground_color: Color::from_rgba_u8([255, 0, 0, 255]),
        background_color: Color::BLACK,
    });
    canvas.set_cell(15, 10, tcrab_console::canvas::Cell {
        glyph: CustomGlyph::HappyFace.into(),
        foreground_color: Color::from_rgba_u8([0, 255, 0, 255]),
        background_color: Color::from_rgba_u8([53, 35, 156, 255]),
    });
    let mut console = tcrab_console_gl::Console::new(tcrab_console_gl::Settings {
        title: "tcrab example".into(),
        texture_width: tileset_image.width(),
        texture_height: tileset_image.height(),
        texture_data: tileset_image.into_vec(),
        cell_width: TILESET_CELL_WIDTH,
        cell_height: TILESET_CELL_HEIGHT,
    }).unwrap();
    console.present(&canvas, &glyph_lib);
    console.wait_for_events_forever(|event| match event {
        tcrab_console::Event::Quit => tcrab_console::ControlFlow::Break,
    });
}