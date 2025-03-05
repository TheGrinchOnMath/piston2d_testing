use piston_window::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::types::Color;

pub struct App<'a> {
    
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {}
    }
    
    
    // TODO add code, edit function to properly render the rays
    pub fn render_rays(&self, c: &Context, gl: &mut GlGraphics) {
        Line::new([1.0, 1.0, 0.6, 0.05], 1.0)
            .shape()
            .draw(
                &DrawState::default(),
                c.trans(),
                gl
            );
    }
    
    pub fn render_mirrors() {
        
    }
    
    pub fn render_debug() {
        
    }

    // FIXME finish writing app.rs, add missing parts in Main.rs, test everything.
    pub fn render_text(
        c: &Context,
        g: &mut G2d,
        glyphs: &mut Glyphs,
        color: Color,
        pos: Position,
        text: &str,
    ) {
        Text::new_color(color, 20)
            .draw(
                text,
                glyphs,
                &c.draw_state,
                c.transform.trans(pos.x as f64, pos.y as f64),
                g,
            )
            .unwrap();
    }
}