use crate::Error;
use crate::JVM;
use colors_transform::{Color as _, Rgb};
use itext::itext::io::ImageData;
use itext::itext::kernel::{Color, ColorConstant, PdfDocument, PdfWriter};
use itext::itext::layout::{
    BlockElement, Border, Cell, Document, ElementPropertyContainer, HorizontalAlignment, Image,
    Paragraph, Table, TextAlignment,
};
use itext::java::{BufferedImage, ByteArrayInputStream, ByteArrayOutputStream};
use itext::javax::ImageInputStream;

pub struct Shipment {
    pub items: Vec<Article>,

    pub customer: CustomerInfo,
    pub seller: SellerInfo,
    pub date: String,
    pub reference_id: String,
    pub order_id: String,
    pub shipment_id: String,
    pub deliver_date: String,
}

pub struct CustomerInfo {
    pub company_name: String,
    pub customer_name: String,
    pub address1: String,
    pub address2: String,
}

pub struct SellerInfo {
    pub company_name: String,
    pub address: String,
    pub telephone: String,
    pub email: String,
    pub website: String,
    pub iban: String,
    pub bic: String,
    pub kvk: String,
    pub btw: String,
    pub mailbox: String,
    pub logo: Vec<u8>,
    pub footer_company_name_color_hex: String,
}

pub struct Article {
    pub identifier: String,
    pub description: String,
    pub quantity_ordered: u32,
    pub quantity_delivered: u32,
    pub quantity_backorder: u32,
}

const LOGO_HEIGHT: f32 = 100.0;

impl Shipment {
    pub fn render(&self, jvm: &JVM) -> Result<Vec<u8>, Error> {
        let mut env = jvm.attach()?;

        let bos = ByteArrayOutputStream::new(&mut env)?;
        let pdfwriter = PdfWriter::new(&bos, &mut env)?;
        let pdfdocument = PdfDocument::new(&pdfwriter, &mut env)?;
        let doc = Document::new(&pdfdocument, &mut env)?;

        doc.set_margins(40.0, 30.0, 40.0, 30.0, &mut env)?;

        // Calculate the scaled size of the seller logo
        let logo_iis = ImageInputStream::new_from_byte_stream(
            ByteArrayInputStream::new(&self.seller.logo, &mut env)?,
            &mut env,
        )?;
        let logo_bi = BufferedImage::new_from_image_input_stream(logo_iis, &mut env)?;
        let logo_height = logo_bi.get_height(&mut env)? as f32;
        let logo_width = logo_bi.get_width(&mut env)? as f32 / (logo_height / LOGO_HEIGHT);

        let document_info = Table::new(&[8.0, 2.0], &mut env)?;
        document_info
            .use_all_available_width(&mut env)?
            .start_new_row(&mut env)?
            // Customer information
            .add_cell(
                Cell::new(&mut env)?
                    .add(
                        Paragraph::new_with_text(&self.customer.company_name, &mut env)?
                            .set_bold(&mut env)?,
                        &mut env,
                    )?
                    .add(
                        &Paragraph::new_with_text(&self.customer.address1, &mut env)?,
                        &mut env,
                    )?
                    .add(
                        &Paragraph::new_with_text(&self.customer.address2, &mut env)?,
                        &mut env,
                    )?
                    .add(
                        &Paragraph::new_with_text(
                            &format!("{}, {}", self.seller.company_name, self.seller.address),
                            &mut env,
                        )?
                        .set_font_size(9.0, &mut env)?
                        .set_italic(&mut env)?,
                        &mut env,
                    )?
                    .set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            // Seller logo
            .add_cell(
                Cell::new(&mut env)?
                    .add_image(
                        Image::new(ImageData::new(&mut env, &self.seller.logo)?, &mut env)?
                            .set_width(logo_width, &mut env)?
                            .set_height(LOGO_HEIGHT, &mut env)?
                            .set_horizontal_alignment(HorizontalAlignment::Right, &mut env)?,
                        &mut env,
                    )?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_horizontal_alignment(HorizontalAlignment::Right, &mut env)?
                    .set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .start_new_row(&mut env)?
            // Empty cell
            .add_cell(
                Cell::new(&mut env)?
                    .set_height(1.0, &mut env)?
                    .set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            // 'Pakbon'
            .add_cell(
                Cell::new(&mut env)?
                    .add(
                        Paragraph::new_with_text("Pakbon", &mut env)?
                            .set_bold(&mut env)?
                            .set_text_alignment(TextAlignment::Right, &mut env)?
                            .set_font_size(24.0, &mut env)?,
                        &mut env,
                    )?
                    .set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?;
        doc.add(document_info, &mut env)?;

        let shipment_info = Table::new(&[1.0, 1.0, 1.0, 1.0, 1.0], &mut env)?;
        let black_border = Border::Solid {
            color: ColorConstant::Black,
            width: 1.0,
        };
        shipment_info
            .use_all_available_width(&mut env)?
            .start_new_row(&mut env)?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        Paragraph::new_with_text("Betreft: ", &mut env)?.set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        &Paragraph::new_with_text(
                            &format!("Bestelling #: {}", self.reference_id),
                            &mut env,
                        )?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?.set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?.set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?.set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .start_new_row(&mut env)?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_top(black_border.clone(), &mut env)?
                    .add(
                        Paragraph::new_with_text("Onze referentie", &mut env)?
                            .set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_top(black_border.clone(), &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_top(black_border.clone(), &mut env)?
                    .add(
                        &Paragraph::new_with_text("Afleverdatum", &mut env)?.set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_top(black_border.clone(), &mut env)?
                    .add(
                        &Paragraph::new_with_text("Ordernummer", &mut env)?.set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_top(black_border.clone(), &mut env)?
                    .add(
                        &Paragraph::new_with_text("Pakbonnummer", &mut env)?.set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .start_new_row(&mut env)?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?
                    .add(
                        &Paragraph::new_with_text(
                            &format!("Bestelling #: {}", self.reference_id),
                            &mut env,
                        )?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?
                    .add(
                        &Paragraph::new_with_text(&self.deliver_date, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?
                    .add(
                        &Paragraph::new_with_text(&self.order_id, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?
                    .add(
                        &Paragraph::new_with_text(&self.shipment_id, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            // Empty row to create some spacing
            .start_new_row(&mut env)?
            .add_cell(
                Cell::new(&mut env)?
                    .set_height(1.0, &mut env)?
                    .set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_height(1.0, &mut env)?
                    .set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_height(1.0, &mut env)?
                    .set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_height(1.0, &mut env)?
                    .set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_height(1.0, &mut env)?
                    .set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?;

        doc.add(shipment_info, &mut env)?;

        let articles = Table::new(&[1.0, 1.0, 2.0, 1.0, 1.0, 1.0], &mut env)?;
        articles
            .use_all_available_width(&mut env)?
            .start_new_row(&mut env)?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?
                    .add(
                        Paragraph::new_with_text("Order nummer", &mut env)?.set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?
                    .add(
                        Paragraph::new_with_text("Artikel", &mut env)?.set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?
                    .add(
                        Paragraph::new_with_text("Omschrijving", &mut env)?.set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?
                    .add(
                        Paragraph::new_with_text("Aantal geleverd", &mut env)?
                            .set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?
                    .add(
                        Paragraph::new_with_text("Aantal besteld", &mut env)?.set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .set_border_bottom(black_border.clone(), &mut env)?
                    .add(
                        Paragraph::new_with_text("Aantal backorder", &mut env)?
                            .set_bold(&mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?;
        for article in &self.items {
            articles
                .start_new_row(&mut env)?
                .add_cell(
                    Cell::new(&mut env)?
                        .set_border(Border::NoBorder, &mut env)?
                        .add(
                            &Paragraph::new_with_text(&self.order_id, &mut env)?,
                            &mut env,
                        )?,
                    &mut env,
                )?
                .add_cell(
                    Cell::new(&mut env)?
                        .set_border(Border::NoBorder, &mut env)?
                        .add(
                            &Paragraph::new_with_text(&article.identifier, &mut env)?,
                            &mut env,
                        )?,
                    &mut env,
                )?
                .add_cell(
                    Cell::new(&mut env)?
                        .set_border(Border::NoBorder, &mut env)?
                        .add(
                            &Paragraph::new_with_text(&article.description, &mut env)?,
                            &mut env,
                        )?,
                    &mut env,
                )?
                .add_cell(
                    Cell::new(&mut env)?
                        .set_border(Border::NoBorder, &mut env)?
                        .add(
                            &Paragraph::new_with_text(
                                &article.quantity_delivered.to_string(),
                                &mut env,
                            )?,
                            &mut env,
                        )?,
                    &mut env,
                )?
                .add_cell(
                    Cell::new(&mut env)?
                        .set_border(Border::NoBorder, &mut env)?
                        .add(
                            &Paragraph::new_with_text(
                                &article.quantity_ordered.to_string(),
                                &mut env,
                            )?,
                            &mut env,
                        )?,
                    &mut env,
                )?
                .add_cell(
                    Cell::new(&mut env)?
                        .set_border(Border::NoBorder, &mut env)?
                        .add(
                            &Paragraph::new_with_text(
                                &article.quantity_backorder.to_string(),
                                &mut env,
                            )?,
                            &mut env,
                        )?,
                    &mut env,
                )?;
        }

        doc.add(articles, &mut env)?;

        let footer = Table::new(&[1.0, 1.0, 1.0, 1.0], &mut env)?;
        const FOOTER_FONT_SIZE: f32 = 7.0;
        const FOOTER_FONT_COLOR: ColorConstant = ColorConstant::Gray;

        let company_name_color = Rgb::from_hex_str(&self.seller.footer_company_name_color_hex)
            .map_err(|e| Error::ColorTransform(e))?;

        let footer_font_color = Color::from_constant(FOOTER_FONT_COLOR, &mut env)?;

        footer
            .use_all_available_width(&mut env)?
            .set_fixed_position(
                doc.get_left_margin(&mut env)?,
                doc.get_bottom_margin(&mut env)?,
                pdfdocument
                    .get_default_page_size(&mut env)?
                    .get_width(&mut env)?
                    - doc.get_left_margin(&mut env)?
                    - doc.get_right_margin(&mut env)?,
                &mut env,
            )?
            .start_new_row(&mut env)?
            .add_cell(
                Cell::new(&mut env)?
                    // Company name row
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        Paragraph::new_with_text(&self.seller.company_name, &mut env)?
                            .set_font_size(FOOTER_FONT_SIZE + 2.0, &mut env)?
                            .set_font_color(
                                &Color::from_rgb(
                                    company_name_color.get_red() / 255.0,
                                    company_name_color.get_green() / 255.0,
                                    company_name_color.get_blue() / 255.0,
                                    &mut env,
                                )?,
                                &mut env,
                            )?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?.set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?.set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?.set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?.set_border(Border::NoBorder, &mut env)?,
                &mut env,
            )?
            // First row of company info
            .start_new_row(&mut env)?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        Paragraph::new_with_text(
                            &format!("Postbus {}", self.seller.mailbox),
                            &mut env,
                        )?
                        .set_font_size(FOOTER_FONT_SIZE, &mut env)?
                        .set_font_color(&footer_font_color, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        Paragraph::new_with_text(&self.seller.email, &mut env)?
                            .set_font_size(FOOTER_FONT_SIZE, &mut env)?
                            .set_font_color(&footer_font_color, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        Paragraph::new_with_text(&format!("IBAN {}", self.seller.iban), &mut env)?
                            .set_font_size(FOOTER_FONT_SIZE, &mut env)?
                            .set_font_color(&footer_font_color, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        Paragraph::new_with_text(&format!("KvK {}", self.seller.kvk), &mut env)?
                            .set_font_size(FOOTER_FONT_SIZE, &mut env)?
                            .set_font_color(&footer_font_color, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            // Second row of company info
            .start_new_row(&mut env)?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        Paragraph::new_with_text(
                            &format!("T {}", self.seller.telephone),
                            &mut env,
                        )?
                        .set_font_size(FOOTER_FONT_SIZE, &mut env)?
                        .set_font_color(&footer_font_color, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        Paragraph::new_with_text(&self.seller.website, &mut env)?
                            .set_font_size(FOOTER_FONT_SIZE, &mut env)?
                            .set_font_color(&footer_font_color, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        Paragraph::new_with_text(&format!("BIC {}", self.seller.bic), &mut env)?
                            .set_font_size(FOOTER_FONT_SIZE, &mut env)?
                            .set_font_color(&footer_font_color, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?
            .add_cell(
                Cell::new(&mut env)?
                    .set_border(Border::NoBorder, &mut env)?
                    .add(
                        Paragraph::new_with_text(&format!("BTW {}", self.seller.btw), &mut env)?
                            .set_font_size(FOOTER_FONT_SIZE, &mut env)?
                            .set_font_color(&footer_font_color, &mut env)?,
                        &mut env,
                    )?,
                &mut env,
            )?;

        doc.add(footer, &mut env)?;

        doc.close(&mut env)?;

        let out = bos.to_byte_array(&mut env)?;
        Ok(out)
    }
}
