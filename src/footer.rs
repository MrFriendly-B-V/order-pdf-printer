use crate::{Error, PdfRenderable, RenderTarget};
use colors_transform::{Color as _, Rgb};
use itext::itext::kernel::{Color, ColorConstant};
use itext::itext::layout::{Border, Cell, ElementPropertyContainer, Paragraph, Table};
use jni::JNIEnv;

/// A document's footer
pub struct Footer {
    /// The fields to be included in the footer
    pub fields: Vec<String>,
    /// The name of the company
    pub company_name: String,
    /// Hexadecimal color for the company name
    pub company_name_color: String,
}

/// The maximum amount of fields per row in the footer
const MAX_FIELD_H: usize = 4;

impl PdfRenderable for Footer {
    fn render<'a>(&self, target: &RenderTarget<'a>, env: &mut JNIEnv<'a>) -> Result<(), Error> {
        let footer = Table::new(&[1.0; MAX_FIELD_H], env)?;
        let doc = &target.document;

        footer
            .set_font_size(7.0, env)?
            .set_font_color(&Color::from_constant(ColorConstant::LightGray, env)?, env)?
            .set_fixed_position(
                doc.get_left_margin(env)?,
                doc.get_bottom_margin(env)?,
                doc.get_pdf_document(env)?
                    .get_default_page_size(env)?
                    .get_width(env)?
                    - doc.get_left_margin(env)?
                    - doc.get_right_margin(env)?,
                env,
            )?
            .use_all_available_width(env)?;

        self.render_company_name(&footer, env)?;
        self.render_footer_fields(&footer, env)?;

        target.document.add(footer, env)?;
        Ok(())
    }
}

impl Footer {
    /// Render all fields of the footer to the table.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_footer_fields<'a>(
        &self,
        footer: &Table<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let rows = (self.fields.len() as f32 / MAX_FIELD_H as f32).ceil() as usize;
        let empty_cells_last_row = (rows * MAX_FIELD_H) - self.fields.len();

        for (row_idx, cells) in self.fields.chunks(MAX_FIELD_H).enumerate() {
            footer.start_new_row(env)?;
            for cell in cells {
                footer.add_cell(
                    Cell::new(env)?
                        .set_border(Border::NoBorder, env)?
                        .add(&Paragraph::new_with_text(cell, env)?, env)?,
                    env,
                )?;
            }

            if row_idx == self.fields.len() - 1 {
                for _ in 0..empty_cells_last_row {
                    footer.add_cell(Cell::new(env)?.set_border(Border::NoBorder, env)?, env)?;
                }
            }
        }

        Ok(())
    }

    /// Render the company's name in the correct color to the footer table.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_company_name<'a>(
        &self,
        footer: &Table<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let company_name_color =
            Rgb::from_hex_str(&self.company_name_color).map_err(|e| Error::ColorTransform(e))?;

        footer.start_new_row(env)?;
        footer.add_cell(
            Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                Paragraph::new_with_text(&self.company_name, env)?
                    .set_font_size(9.0, env)?
                    .set_font_color(
                        &Color::from_rgb(
                            company_name_color.get_red() / 255.0,
                            company_name_color.get_green() / 255.0,
                            company_name_color.get_blue() / 255.0,
                            env,
                        )?,
                        env,
                    )?,
                env,
            )?,
            env,
        )?;

        for _ in 0..MAX_FIELD_H - 1 {
            footer.add_cell(Cell::new(env)?.set_border(Border::NoBorder, env)?, env)?;
        }

        Ok(())
    }
}
