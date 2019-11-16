use std::io::{self, Seek, Write};

const PT_PER_INCH: f32 = 72.0;

mod canvas;
mod pdf;
#[cfg(target_arch = "wasm32")]
mod wasm;

use pdf::Pdf;

pub struct Config {
    pub width: f32,
    pub height: f32,
    pub margin: f32,
    pub color: (u8, u8, u8),
    pub num_x: u32,
    pub num_y: u32,
    pub d: f32,
    pub num_pages: usize,
}

pub fn dot<T: Write + Seek>(output: T, config: Config) -> io::Result<()> {
    let width = config.width * PT_PER_INCH;
    let height = config.height * PT_PER_INCH;
    let margin = config.margin * PT_PER_INCH;
    let d = config.d * PT_PER_INCH;

    let mut document = Pdf::new(output)?;
    let content_oid = document.write_stream(|canvas| {
        canvas.set_fill_color(config.color)?;
        for x in FloatIterator::new(margin, d, config.num_x + 1) {
            for y in FloatIterator::new(margin, d, config.num_y + 1) {
                canvas.circle(x, y, d / 10.0)?;
            }
        }
        Ok(())
    })?;
    document.write_page_with_obj(width, height, content_oid, config.num_pages)?;
    document.finish()
}

pub fn grid<T: Write + Seek>(output: T, config: Config) -> io::Result<()> {
    let width = config.width * PT_PER_INCH;
    let height = config.height * PT_PER_INCH;
    let margin = config.margin * PT_PER_INCH;
    let d = config.d * PT_PER_INCH;

    let tot_x: f32 = d * config.num_x as f32;
    let tot_y: f32 = d * config.num_y as f32;

    let mut document = Pdf::new(output)?;
    let content_oid = document.write_stream(|canvas| {
        canvas.set_stroke_color(config.color)?;

        for x in FloatIterator::new(margin, d, config.num_x + 1) {
            canvas.line(x, margin, x, margin + tot_y)?;
        }
        for y in FloatIterator::new(margin, d, config.num_y + 1) {
            canvas.line(margin, y, margin + tot_x, y)?;
        }
        Ok(())
    })?;
    document.write_page_with_obj(width, height, content_oid, config.num_pages)?;

    document.finish()
}

struct FloatIterator {
    current: f32,
    step: f32,
    num: u32,
}

impl FloatIterator {
    pub fn new(start: f32, step: f32, num: u32) -> Self {
        FloatIterator {
            current: start - step,
            step,
            num,
        }
    }
}

impl Iterator for FloatIterator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num > 0 {
            self.num -= 1;
            self.current += self.step;
            Some(self.current)
        } else {
            None
        }
    }
}
