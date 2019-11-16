use flate2::{write::ZlibEncoder, Compression};
use std::io::{self, Seek, SeekFrom, Write};

use crate::canvas::Canvas;

pub struct Pdf<S: Write + Seek> {
    output: S,
    object_offsets: Vec<i64>,
    page_objects_ids: Vec<usize>,
}

const ROOT_OBJECT_ID: usize = 1;
const PAGES_OBJECT_ID: usize = 2;

impl<S: Write + Seek> Pdf<S> {
    // Create a new PDF document, writing to `output`.
    pub fn new(mut output: S) -> io::Result<Pdf<S>> {
        output.write_all(b"%PDF-1.2\n%\xB5\xED\xAE\xFB\n")?;
        Ok(Pdf {
            output,
            // Object ID 0 is special in PDF.
            // We reserve IDs 1 and 2 for the catalog and page tree.
            object_offsets: vec![-1, -1, -1],
            page_objects_ids: Vec::new(),
        })
    }

    // Return the current read/write position in the output file.
    fn tell(&mut self) -> io::Result<u64> {
        self.output.seek(SeekFrom::Current(0))
    }

    pub fn write_stream<F>(&mut self, render_contents: F) -> io::Result<usize>
    where
        F: FnOnce(&mut Canvas<ZlibEncoder<&mut S>>) -> io::Result<()>,
    {
        let (contents_object_id, content_length) =
            self.write_new_object(move |contents_object_id, pdf| {
                // Guess the ID of the next object. (We’ll assert it below.)
                writeln!(
                    pdf.output,
                    "<< /Length {} 0 R /Filter /FlateDecode>>\n\
                     stream",
                    contents_object_id + 1,
                )?;

                let start = pdf.tell()?;
                let mut encoder = ZlibEncoder::new(&mut pdf.output, Compression::default());

                writeln!(encoder, "/DeviceRGB cs /DeviceRGB CS")?;
                render_contents(&mut Canvas::new(&mut encoder))?;
                drop(encoder);
                let end = pdf.tell()?;

                writeln!(pdf.output, "\nendstream")?;
                Ok((contents_object_id, end - start))
            })?;
        self.write_new_object(|length_object_id, pdf| {
            assert!(length_object_id == contents_object_id + 1);
            writeln!(pdf.output, "{}", content_length)
        })?;
        Ok(contents_object_id)
    }

    /// Create a new page in the PDF document.
    ///
    /// The page will be `width` x `height` points large, and the
    /// actual content of the page will be created by the function
    /// `render_contents` by applying drawing methods on the Canvas.
    pub fn write_page_with_obj(
        &mut self,
        width: f32,
        height: f32,
        content_oid: usize,
        repeat: usize,
    ) -> io::Result<()> {
        let page_oid = self.write_new_object(|page_oid, pdf| {
            writeln!(
                pdf.output,
                "<< /Type /Page\n   \
                 /Parent {parent} 0 R\n   \
                 /MediaBox [ 0 0 {width} {height} ]\n   \
                 /Contents {c_oid} 0 R\n\
                 >>",
                parent = PAGES_OBJECT_ID,
                width = width,
                height = height,
                c_oid = content_oid,
            )
            .map(|_| page_oid)
        })?;
        for _ in 0..repeat {
            self.page_objects_ids.push(page_oid);
        }
        Ok(())
    }

    fn write_new_object<F, T>(&mut self, write_content: F) -> io::Result<T>
    where
        F: FnOnce(usize, &mut Pdf<S>) -> io::Result<T>,
    {
        let id = self.object_offsets.len();
        let (result, offset) = self.write_object(id, |pdf| write_content(id, pdf))?;
        self.object_offsets.push(offset);
        Ok(result)
    }

    fn write_object_with_id<F, T>(&mut self, id: usize, write_content: F) -> io::Result<T>
    where
        F: FnOnce(&mut Pdf<S>) -> io::Result<T>,
    {
        assert!(self.object_offsets[id] == -1);
        let (result, offset) = self.write_object(id, write_content)?;
        self.object_offsets[id] = offset;
        Ok(result)
    }

    fn write_object<F, T>(&mut self, id: usize, write_content: F) -> io::Result<(T, i64)>
    where
        F: FnOnce(&mut Pdf<S>) -> io::Result<T>,
    {
        let offset = self.tell()? as i64;
        writeln!(self.output, "{} 0 obj", id)?;
        let result = write_content(self)?;
        writeln!(self.output, "endobj")?;
        Ok((result, offset))
    }

    // Write out the document trailer.
    // The trailer consists of the pages object, the root object,
    // the xref list, the trailer object and the startxref position.
    pub fn finish(mut self) -> io::Result<()> {
        self.write_object_with_id(PAGES_OBJECT_ID, |pdf| {
            write!(
                pdf.output,
                "<< /Type /Pages\n   \
                 /Count {}\n   ",
                pdf.page_objects_ids.len()
            )?;
            write!(pdf.output, "/Kids [ ")?;
            for id in &pdf.page_objects_ids {
                write!(pdf.output, "{} 0 R ", id)?;
            }
            writeln!(pdf.output, "]\n>>")
        })?;

        self.write_object_with_id(ROOT_OBJECT_ID, |pdf| {
            writeln!(
                pdf.output,
                "<< /Type /Catalog\n   \
                 /Pages {} 0 R",
                PAGES_OBJECT_ID,
            )?;
            writeln!(pdf.output, ">>")
        })?;
        let startxref = self.tell()?;
        writeln!(
            self.output,
            "xref\n\
             0 {}\n\
             0000000000 65535 f ",
            self.object_offsets.len(),
        )?;
        // Object 0 (above) is special
        // Use [1..] to skip object 0 in self.object_offsets.
        for &offset in &self.object_offsets[1..] {
            assert!(offset >= 0);
            writeln!(self.output, "{:010} 00000 n ", offset)?;
        }
        writeln!(
            self.output,
            "trailer\n\
             << /Size {size}\n   \
             /Root {root} 0 R",
            size = self.object_offsets.len(),
            root = ROOT_OBJECT_ID,
        )?;
        writeln!(
            self.output,
            ">>\n\
             startxref\n\
             {}\n\
             %%EOF",
            startxref,
        )
    }
}
