use crate::Error;
use itext::itext::io::{FontProgramFactory, PdfEncodings};
use itext::itext::kernel::{PdfDocument, PdfFontFactory, PdfWriter};
use itext::itext::layout::{Document, ElementPropertyContainer};
use itext::java::ByteArrayOutputStream;
use jni::JNIEnv;

/// A target for rendering
pub struct RenderTarget<'a> {
    /// The byte output stream where the rendered result will be put
    byte_stream: ByteArrayOutputStream<'a>,
    /// The document being worked on
    pub document: Document<'a>,
}

/// Configurations for the entire document
pub struct DocumentConfiguration {
    /// The font family to use.
    /// If left to None, a default is used.
    pub font_family: Option<Vec<u8>>,
    /// The font size to use.
    /// If left to None, a default is used
    pub font_size: Option<f32>,
}

impl<'a> RenderTarget<'a> {
    /// Create a new render target.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    pub fn new(config: &DocumentConfiguration, env: &mut JNIEnv<'a>) -> Result<Self, Error> {
        let byte_stream = ByteArrayOutputStream::new(env)?;
        let document = Document::new(
            &PdfDocument::new(&PdfWriter::new(&byte_stream, env)?, env)?,
            env,
        )?;

        document
            .set_font_size(11.0, env)?
            .set_margins(40.0, 30.0, 40.0, 30.0, env)?;

        if let Some(font_family) = &config.font_family {
            Self::configure_font_family(&document, font_family, env)?;
        }

        if let Some(font_size) = config.font_size {
            document.set_font_size(font_size, env)?;
        }

        Ok(Self {
            byte_stream,
            document,
        })
    }

    /// Configure the font family for the current document
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn configure_font_family(
        document: &Document<'a>,
        family: &[u8],
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let font_program = FontProgramFactory::new_from_ttf(family, env)?;
        let pdf_font = PdfFontFactory::create_from_program_with_encoding_embedded(
            font_program,
            PdfEncodings::Winansi,
            env,
        )?;

        document.set_font(&pdf_font, env)?;

        Ok(())
    }

    /// Finish rendering and export to bytes.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    pub fn finish(self, env: &mut JNIEnv<'a>) -> Result<Vec<u8>, Error> {
        self.document.close(env)?;
        let out = self.byte_stream.to_byte_array(env)?;
        Ok(out)
    }
}
