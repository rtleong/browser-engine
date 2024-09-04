use crate::layout::{AnonymousBlock, BlockNode, InlineNode, LayoutBox, Rect};
use crate::css::{Value, Color};

pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

/// Paint a tree of LayoutBoxes to an array of pixels.
pub fn paint(layout_root: &LayoutBox, bounds: Rect) -> Canvas {
    let display_list = build_display_list(layout_root);
    let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);
    for item in display_list {
        canvas.paint_item(&item);
    }
    canvas
}

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
}

pub type DisplayList = Vec<DisplayCommand>;