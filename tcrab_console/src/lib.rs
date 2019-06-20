mod color;
pub mod canvas;
pub mod event;

pub use self::color::Color;
pub use self::canvas::Canvas;
pub use self::event::Event;

use self::canvas::GlyphLibrary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlow {
    Continue,
    Break,
}

pub trait Console {
    type GlyphDef;
    
    fn wait_for_events_forever<F>(&mut self, event_handler: F)
    where
        F: FnMut(Event) -> ControlFlow;

    fn present<G, C>(
        &mut self,
        canvas: &C,
        glyph_lib: &GlyphLibrary<G, Self::GlyphDef>,
    ) where G: canvas::CustomGlyph, C: Canvas<G>;
}