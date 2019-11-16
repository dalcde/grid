use std::io::{self, Write};

pub type Color = (u8, u8, u8);

/// A visual area where content can be drawn (a page).
///
/// Provides methods for defining and stroking or filling paths, as
/// well as placing text objects.
pub struct Canvas<'a, T: Write> {
    output: &'a mut T,
}

impl<'a, T: Write> Canvas<'a, T> {
    pub fn new(output: &'a mut T) -> Self {
        Canvas { output }
    }

    /// Set color for stroking operations.
    pub fn set_stroke_color(&mut self, color: Color) -> io::Result<()> {
        let norm = |c| f32::from(c) / 255.0;
        writeln!(
            self.output,
            "{} {} {} SC",
            norm(color.0),
            norm(color.1),
            norm(color.2),
        )
    }
    /// Set color for non-stroking operations.
    pub fn set_fill_color(&mut self, color: Color) -> io::Result<()> {
        let norm = |c| f32::from(c) / 255.0;
        writeln!(
            self.output,
            "{} {} {} sc",
            norm(color.0),
            norm(color.1),
            norm(color.2),
        )
    }

    /// Begin a new subpath at the point (x, y).
    pub fn line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) -> io::Result<()> {
        write!(self.output, "{} {} m ", x0, y0)?;
        write!(self.output, "{} {} l ", x1, y1)?;
        writeln!(self.output, "S")
    }

    /// Add a Bézier curve from the current point to (x3, y3) with
    /// (x1, y1) and (x2, y2) as Bézier controll points.
    pub fn curve_to(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> io::Result<()> {
        writeln!(self.output, "{} {} {} {} {} {} c", x1, y1, x2, y2, x3, y3)
    }
    /// Add a circle approximated by four cubic Bézier curves to the
    /// current path.  Based on
    /// http://spencermortensen.com/articles/bezier-circle/
    pub fn circle(&mut self, x: f32, y: f32, r: f32) -> io::Result<()> {
        let top = y - r;
        let bottom = y + r;
        let left = x - r;
        let right = x + r;
        #[allow(clippy::excessive_precision)]
        let c = 0.551_915_024_494;
        let dist = r * c;
        let up = y - dist;
        let down = y + dist;
        let leftp = x - dist;
        let rightp = x + dist;
        write!(self.output, "{} {} m ", x, top)?;
        self.curve_to(leftp, top, left, up, left, y)?;
        self.curve_to(left, down, leftp, bottom, x, bottom)?;
        self.curve_to(rightp, bottom, right, down, right, y)?;
        self.curve_to(right, up, rightp, top, x, top)?;
        writeln!(self.output, "f")
    }
}
