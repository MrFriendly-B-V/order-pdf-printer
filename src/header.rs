use crate::{Error, PdfRenderable, RenderTarget};
use itext::itext::io::ImageData;
use itext::itext::layout::{
    BlockElement, Border, Cell, ElementPropertyContainer, HorizontalAlignment, Image, Paragraph,
    Table, TextAlignment,
};
use itext::java::{BufferedImage, ByteArrayInputStream};
use itext::javax::ImageInputStream;
use jni::JNIEnv;

/// The header of the document
#[derive(Debug, Clone)]
pub struct Header {
    /// The logo's bytes
    pub logo: Vec<u8>,
    /// The type of document, e.g. 'Invoice'
    pub label: String,
    /// The addressee of the document
    pub addressed_to: AddressableParty,
    /// The seller
    pub seller: AddressableParty,
}

/// An entity which has an address
#[derive(Debug, Clone)]
pub struct AddressableParty {
    /// The company name
    pub name: String,
    /// The department
    pub department: Option<String>,
    /// The address of the entity
    pub address: Address,
    /// The VAT number of the company
    pub vat_number: Option<String>,
}

/// An address
#[derive(Debug, Clone)]
pub struct Address {
    /// Street name
    pub street: String,
    /// House number and optional extension
    pub number: String,
    /// The ZIP/Postal code
    pub zipcode: String,
    /// The city
    pub city: String,
    /// The country
    pub country: String,
}

impl PdfRenderable for Header {
    fn render<'a>(&self, target: &RenderTarget<'a>, env: &mut JNIEnv<'a>) -> Result<(), Error> {
        let table = Table::new(&[8.0, 2.0], env)?;
        table.use_all_available_width(env)?.start_new_row(env)?;

        self.render_document_info(&table, env)?;
        self.render_logo(&table, env)?;

        table.start_new_row(env)?;
        self.render_document_label(&table, env)?;

        target.document.add(table, env)?;
        Ok(())
    }
}

/// The height of the logo on the document.
/// The width will automatically be adjusted to
/// preserve aspect ration.
const LOGO_HEIGHT: f32 = 100.0;

/// The prefix to put before the name of the department.
const DEPARTMENT_PREFIX: &str = "T.a.v";

impl Header {
    /// Calculate the width and height of the logo on the document.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn calculate_logo_size(&self, env: &mut JNIEnv<'_>) -> Result<(f32, f32), Error> {
        let logo_iis = ImageInputStream::new_from_byte_stream(
            ByteArrayInputStream::new(&self.logo, env)?,
            env,
        )?;
        let logo_bi = BufferedImage::new_from_image_input_stream(logo_iis, env)?;
        let logo_height = logo_bi.get_height(env)? as f32;
        let logo_width = logo_bi.get_width(env)? as f32 / (logo_height / LOGO_HEIGHT);

        Ok((logo_width, LOGO_HEIGHT))
    }

    /// Render the document label to the header table
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_document_label<'a>(
        &self,
        table: &Table<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        table
            // Empty cell
            .add_cell(
                Cell::new(env)?
                    .set_height(1.0, env)?
                    .set_border(Border::NoBorder, env)?,
                env,
            )?
            // 'Pakbon'
            .add_cell(
                Cell::new(env)?
                    .add(
                        Paragraph::new_with_text(&self.label, env)?
                            .set_bold(env)?
                            .set_text_alignment(TextAlignment::Right, env)?
                            .set_font_size(24.0, env)?,
                        env,
                    )?
                    .set_border(Border::NoBorder, env)?,
                env,
            )?;
        Ok(())
    }

    /// Render the company logo to the header table.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_logo<'a>(&self, table: &Table<'a>, env: &mut JNIEnv<'a>) -> Result<(), Error> {
        let (logo_width, logo_height) = self.calculate_logo_size(env)?;

        table.add_cell(
            Cell::new(env)?
                .add_image(
                    Image::new(ImageData::new(env, &self.logo)?, env)?
                        .set_width(logo_width, env)?
                        .set_height(logo_height, env)?
                        .set_horizontal_alignment(HorizontalAlignment::Right, env)?,
                    env,
                )?
                .set_border(Border::NoBorder, env)?
                .set_horizontal_alignment(HorizontalAlignment::Right, env)?
                .set_border(Border::NoBorder, env)?,
            env,
        )?;

        Ok(())
    }

    /// Render the document information to the header table
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_document_info<'a>(
        &self,
        table: &Table<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let cell = Cell::new(env)?;
        cell.set_border(Border::NoBorder, env)?;

        // Name
        cell.add(
            Paragraph::new_with_text(&self.addressed_to.name, env)?.set_bold(env)?,
            env,
        )?;

        // Department
        if let Some(department) = &self.addressed_to.department {
            cell.add(
                &Paragraph::new_with_text(&format!("{DEPARTMENT_PREFIX} {department}"), env)?,
                env,
            )?;
        }

        // Street + house number
        cell.add(
            &Paragraph::new_with_text(
                &format!(
                    "{} {}",
                    self.addressed_to.address.street, self.addressed_to.address.number
                ),
                env,
            )?,
            env,
        )?;

        // ZIP + city
        cell.add(
            &Paragraph::new_with_text(
                &format!(
                    "{} {}",
                    self.addressed_to.address.zipcode, self.addressed_to.address.city
                ),
                env,
            )?,
            env,
        )?;

        // Country
        cell.add(
            &Paragraph::new_with_text(&self.addressed_to.address.country, env)?,
            env,
        )?;

        // VAT number
        if let Some(vat) = &self.addressed_to.vat_number {
            cell.add(&Paragraph::new_with_text(vat, env)?, env)?;
        }

        // Seller information
        cell.add(
            &Paragraph::new_with_text(
                &format!(
                    "{}, {} {}, {}, {}",
                    self.seller.name,
                    self.seller.address.street,
                    self.seller.address.number,
                    self.seller.address.zipcode,
                    self.seller.address.city
                ),
                env,
            )?
            .set_font_size(9.0, env)?
            .set_italic(env)?,
            env,
        )?;

        table.add_cell(&cell, env)?;
        Ok(())
    }
}
