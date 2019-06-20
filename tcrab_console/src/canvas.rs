use std::hash::Hash;
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::Color;

pub trait CustomGlyph: Eq + Hash + Copy {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Glyph<C: CustomGlyph> {
    Char(char),
    Custom(C),
}

impl<C: CustomGlyph> From<char> for Glyph<C> {
    fn from(c: char) -> Glyph<C> {
        Glyph::Char(c)
    }
}

impl<C: CustomGlyph> From<C> for Glyph<C> {
    fn from(c: C) -> Glyph<C> {
        Glyph::Custom(c)
    }
}

pub struct GlyphLibrary<C: CustomGlyph, D> {
    glyphs: HashMap<Glyph<C>, D>,
}

impl<C: CustomGlyph, D> GlyphLibrary<C, D> {
    pub fn new() -> GlyphLibrary<C, D> {
        GlyphLibrary {
            glyphs: HashMap::new(),
        }
    }

    pub fn define_glyph<T: Into<Glyph<C>>>(&mut self, glyph: T, def: D) {
        self.glyphs.insert(glyph.into(), def);
    }

    pub fn get_glyph_def(&self, glyph: Glyph<C>) -> &D {
        self.glyphs.get(&glyph).unwrap()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell<C: CustomGlyph> {
    pub glyph: Glyph<C>,
    pub foreground_color: Color,
    pub background_color: Color,
}

impl<C: CustomGlyph> Default for Cell<C> {
    fn default() -> Cell<C> {
        Cell {
            glyph: ' '.into(),
            foreground_color: Color::WHITE,
            background_color: Color::BLACK,
        }
    }
}

pub trait Canvas<C: CustomGlyph> {
    fn size(&self) -> (usize, usize);
    
    fn get_cell(&self, x: usize, y: usize) -> Cell<C>;
    fn set_cell(&mut self, x: usize, y: usize, cell: Cell<C>);

    fn fill(&mut self, cell: Cell<C>) {
        let (width, height) = self.size();
        for y in 0..height {
            for x in 0..width {
                self.set_cell(x, y, cell);
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CellBuffer<C: CustomGlyph> {
    width: usize,
    height: usize,
    cells: Box<[Cell<C>]>,
}

impl<C: CustomGlyph> CellBuffer<C> {
    pub fn new(width: usize, height: usize, fill_cell: Cell<C>) -> CellBuffer<C> {
        CellBuffer {
            width,
            height,
            cells: vec![fill_cell; width * height].into_boxed_slice(),
        }
    }
}

impl<C: CustomGlyph> Canvas<C> for CellBuffer<C> {
    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    
    fn get_cell(&self, x: usize, y: usize) -> Cell<C> {
        self.cells[y * self.width + x]
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell<C>) {
        self.cells[y * self.width + x] = cell;
    }
}