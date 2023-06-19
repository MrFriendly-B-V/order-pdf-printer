use crate::footer::Footer;
use crate::header::Header;
use crate::RenderTarget;
use crate::{Error, PdfRenderable};
use itext::itext::kernel::ColorConstant;
use itext::itext::layout::{
    BlockElement, Border, Cell, Document, ElementPropertyContainer, Paragraph, Table,
};
use jni::JNIEnv;

pub struct PackingSlip {
    /// Items to be included in the shipment
    pub items: Vec<ArticlePackingInfo>,
    /// The document header
    pub header: Header,
    /// The document footer
    pub footer: Footer,
    /// The seller's reference
    pub reference_id: String,
    /// The order ID
    pub order_id: String,
    /// The ID of this packing slip
    pub packing_slip_id: String,
    /// The expected delivery date
    pub delivery_date: String,
}

/// Information about an article included in the shipment
pub struct ArticlePackingInfo {
    /// The article ID
    pub identifier: String,
    /// Short description of the item (the item's name)
    pub description: String,
    /// The amount ordered by the customer
    pub quantity_ordered: u32,
    /// The amount that will be delivered in this shipment
    pub quantity_delivered: u32,
    /// The amount on backorder
    pub quantity_backorder: u32,
}

impl PdfRenderable for PackingSlip {
    fn render<'a>(&self, target: &RenderTarget<'a>, env: &mut JNIEnv<'a>) -> Result<(), Error> {
        self.header.render(target, env)?;
        self.render_document_info(&target.document, env)?;
        self.render_articles_section(&target.document, env)?;
        self.footer.render(target, env)?;

        Ok(())
    }
}

const NUMBER_BACKORDER_LABEL: &str = "Aantal backorder";
const NUMBER_DELIVERED_LABEL: &str = "Aantal geleverd";
const NUMBER_ORDERED_LABEL: &str = "Aantal besteld";
const ORDER_ID_LABEL: &str = "Order nummer";
const ARTICLE_LABEL: &str = "Artikel";
const DESCRIPTION_LABEL: &str = "Omschrijving";
const PACKING_SLIP_ID_LABEL: &str = "Pakbonnummer";
const OUR_REFERENCE_LABEL: &str = "Onze referentie";
const CONCERNING_LABEL: &str = "Betreft";
const ORDER_ID_PREFIX: &str = "Bestelling #:";
const DELIVERY_DATE_LABEL: &str = "Afleverdatum";

impl PackingSlip {
    /// Render the packing slip information.
    /// This includes:
    /// - Our reference
    /// - Delivery date
    /// - Order ID
    /// - Packing slip ID
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_document_info<'a>(
        &self,
        doc: &Document<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let document_info = Table::new(&[1.0, 1.0, 1.0, 1.0, 1.0], env)?;
        let black_border = Border::Solid {
            color: ColorConstant::Black,
            width: 1.0,
        };
        document_info
            .use_all_available_width(env)?
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    Paragraph::new_with_text(&format!("{CONCERNING_LABEL}: "), env)?
                        .set_bold(env)?,
                    env,
                )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    &Paragraph::new_with_text(
                        &format!("{ORDER_ID_PREFIX} {}", self.reference_id),
                        env,
                    )?,
                    env,
                )?,
                env,
            )?
            .add_cell(Cell::new(env)?.set_border(Border::NoBorder, env)?, env)?
            .add_cell(Cell::new(env)?.set_border(Border::NoBorder, env)?, env)?
            .add_cell(Cell::new(env)?.set_border(Border::NoBorder, env)?, env)?
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_top(black_border.clone(), env)?
                    .add(
                        Paragraph::new_with_text(OUR_REFERENCE_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_top(black_border.clone(), env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_top(black_border.clone(), env)?
                    .add(
                        &Paragraph::new_with_text(DELIVERY_DATE_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_top(black_border.clone(), env)?
                    .add(
                        &Paragraph::new_with_text(ORDER_ID_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_top(black_border.clone(), env)?
                    .add(
                        &Paragraph::new_with_text(PACKING_SLIP_ID_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?
                    .add(
                        &Paragraph::new_with_text(
                            &format!("{ORDER_ID_PREFIX} : {}", self.reference_id),
                            env,
                        )?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?
                    .add(&Paragraph::new_with_text(&self.delivery_date, env)?, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?
                    .add(&Paragraph::new_with_text(&self.order_id, env)?, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?
                    .add(&Paragraph::new_with_text(&self.packing_slip_id, env)?, env)?,
                env,
            )?
            // Empty row to create some spacing
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?
                    .set_height(1.0, env)?
                    .set_border(Border::NoBorder, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_height(1.0, env)?
                    .set_border(Border::NoBorder, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_height(1.0, env)?
                    .set_border(Border::NoBorder, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_height(1.0, env)?
                    .set_border(Border::NoBorder, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_height(1.0, env)?
                    .set_border(Border::NoBorder, env)?,
                env,
            )?;

        doc.add(document_info, env)?;

        Ok(())
    }

    /// Render the article section of the packing slip
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_articles_section<'a>(
        &self,
        doc: &Document<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let articles = Table::new(&[1.0, 1.0, 2.0, 1.0, 1.0, 1.0], env)?;
        articles.use_all_available_width(env)?;

        self.render_articles_header(&articles, env)?;

        for article in &self.items {
            self.render_article(article, &articles, env)?;
        }

        doc.add(articles, env)?;

        Ok(())
    }

    /// Render the header of the articles section.
    /// I.e. the table header
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_articles_header<'a>(
        &self,
        table: &Table<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let black_border = Border::Solid {
            width: 1.0,
            color: ColorConstant::Black,
        };

        table
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?
                    .add(
                        Paragraph::new_with_text(ORDER_ID_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?
                    .add(
                        Paragraph::new_with_text(ARTICLE_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?
                    .add(
                        Paragraph::new_with_text(DESCRIPTION_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?
                    .add(
                        Paragraph::new_with_text(NUMBER_DELIVERED_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?
                    .add(
                        Paragraph::new_with_text(NUMBER_ORDERED_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(black_border.clone(), env)?
                    .add(
                        Paragraph::new_with_text(NUMBER_BACKORDER_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?;

        Ok(())
    }

    /// Render a single article to the table.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_article<'a>(
        &self,
        article: &ArticlePackingInfo,
        table: &Table<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        table
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .add(&Paragraph::new_with_text(&self.order_id, env)?, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .add(&Paragraph::new_with_text(&article.identifier, env)?, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .add(&Paragraph::new_with_text(&article.description, env)?, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    &Paragraph::new_with_text(&article.quantity_delivered.to_string(), env)?,
                    env,
                )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    &Paragraph::new_with_text(&article.quantity_ordered.to_string(), env)?,
                    env,
                )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    &Paragraph::new_with_text(&article.quantity_backorder.to_string(), env)?,
                    env,
                )?,
                env,
            )?;

        Ok(())
    }
}
