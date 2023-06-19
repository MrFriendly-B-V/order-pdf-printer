use crate::footer::Footer;
use crate::header::Header;
use crate::{Error, PdfRenderable, RenderTarget};
use itext::itext::kernel::ColorConstant;
use itext::itext::layout::{
    BlockElement, Border, Cell, Document, ElementPropertyContainer, Paragraph, Table, TextAlignment,
};
use jni::JNIEnv;

#[derive(Debug, Clone)]
pub struct Invoice {
    /// The invoice header
    pub header: Header,
    /// The invoice footer
    pub footer: Footer,
    /// The reference ID
    pub reference_id: String,
    /// The invoice ID
    pub invoice_id: String,
    /// The invoice date
    pub invoice_date: String,
    /// Date at which the invoice expires
    pub expiration_date: String,
    /// Invoice totals
    pub totals: InvoiceTotals,
    /// Items to be invoiced
    pub items: Vec<InvoiceItem>,
    /// Optional note to customer
    pub note: Option<String>,
    /// Currency of the invoice
    pub currency: Currency,
}

#[derive(Debug, Clone)]
pub enum Currency {
    Euro,
}

#[derive(Debug, Clone)]
pub struct InvoiceItem {
    /// The item ID
    pub identifier: String,
    /// A short description of the item
    pub description: String,
    /// The item quantity
    pub quantity: i32,
    /// The price per unit
    pub price_per_unit: f32,
    /// The discount percentage.
    /// Omitted if all articles lack a discount.
    pub discount_percentage: f32,
    /// The price per unit taking into account the discount
    pub subtotal_price_per_unit: f32,
    /// Total price of the item.
    pub total_price: f32,
}

/// Invoice totals.
/// Displayed at the bottom of the invoice
#[derive(Debug, Clone)]
pub struct InvoiceTotals {
    /// The total price excluding VAT.
    pub total_excluding_vat: f32,
    /// The total amount of VAT.
    pub total_vat: f32,
    /// The total price to be paid including VAT.
    pub total_including_vat: f32,
}

impl PdfRenderable for Invoice {
    fn render<'a>(&self, target: &RenderTarget<'a>, env: &mut JNIEnv<'a>) -> Result<(), Error> {
        self.header.render(target, env)?;

        self.render_invoice_information(&target.document, env)?;
        self.render_items_section(&target.document, env)?;

        if let Some(note) = &self.note {
            let paragraph = Paragraph::new_with_text(note, env)?;
            paragraph
                .set_margin_top(30.0, env)?
                .set_margin_bottom(30.0, env)?;
            target.document.add(paragraph, env)?;
        }

        self.render_invoice_totals(&target.document, env)?;

        self.footer.render(target, env)?;

        Ok(())
    }
}

const CONCERNING_LABEL: &str = "Betreft";
const OUR_REFERENCE_LABEL: &str = "Onze referentie";
const ORDER_ID_PREFIX: &str = "Bestelling #:";
const INVOICE_DATE_LABEL: &str = "Factuurdatum";
const EXPIRY_DATE_LABEL: &str = "Vervaldatum";
const INVOICE_ID_LABEL: &str = "Factuurnummer";

const ARTICLE_ID_LABEL: &str = "Artikelnummer";
const DESCRIPTION_LABEL: &str = "Omschrijving";
const QUANTITY_LABEL: &str = "Aantal";
const PRICE_LABEL: &str = "Prijs";
const DISCOUNT_LABEL: &str = "Korting";
const SUBTOTAL_ITEM_PRICE_LABEL: &str = "Bedrag";
const TOTAL_ITEM_LABEL: &str = "Totaalbedrag";

const TOTAL_EXCLUDING_VAT_LABEL: &str = "Totaal excl. BTW";
const TOTAL_VAT_LABEL: &str = "BTW";
const TOTAL_PRICE_LABEL: &str = "Totaal te voldoen";

impl Invoice {
    /// Render information about the invoice.
    /// This includes:
    /// - Our reference
    /// - Invoice date
    /// - Expiry date
    /// - Invoice ID
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_invoice_information<'a>(
        &self,
        document: &Document<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let border = Border::Solid {
            color: ColorConstant::Black,
            width: 1.0,
        };

        let table = Table::new(&[1.0, 1.0, 1.0, 1.0, 1.0], env)?;
        table
            .use_all_available_width(env)?
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    Paragraph::new_with_text(CONCERNING_LABEL, env)?.set_bold(env)?,
                    env,
                )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    &Paragraph::new_with_text(
                        &format!("{} {}", ORDER_ID_PREFIX, self.reference_id),
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
                    .set_border_top(border.clone(), env)?
                    .add(
                        &Paragraph::new_with_text(OUR_REFERENCE_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_top(border.clone(), env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_top(border.clone(), env)?
                    .add(
                        Paragraph::new_with_text(INVOICE_DATE_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_top(border.clone(), env)?
                    .add(
                        Paragraph::new_with_text(EXPIRY_DATE_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_top(border.clone(), env)?
                    .add(
                        Paragraph::new_with_text(INVOICE_ID_LABEL, env)?.set_bold(env)?,
                        env,
                    )?,
                env,
            )?
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(border.clone(), env)?
                    .add(
                        &Paragraph::new_with_text(
                            &format!("{} {}", ORDER_ID_PREFIX, self.reference_id),
                            env,
                        )?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(border.clone(), env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(border.clone(), env)?
                    .add(&Paragraph::new_with_text(&self.invoice_date, env)?, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(border.clone(), env)?
                    .add(&Paragraph::new_with_text(&self.expiration_date, env)?, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(border.clone(), env)?
                    .add(&Paragraph::new_with_text(&self.invoice_id, env)?, env)?,
                env,
            )?;

        document.add(table, env)?;

        Ok(())
    }

    /// Render the items section of the invoice
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_items_section<'a>(
        &self,
        document: &Document<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let columns = if self.any_item_has_discount() {
            [1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0].to_vec()
        } else {
            [1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0].to_vec()
        };

        let table = Table::new(&columns, env)?;
        table.use_all_available_width(env)?;

        table.start_new_row(env)?;
        self.render_items_header(&table, env)?;

        for item in &self.items {
            table.start_new_row(env)?;
            self.render_item(&table, &item, env)?;
        }

        document.add(table, env)?;
        Ok(())
    }

    /// Whether any of the items on the invoice has a discount.
    fn any_item_has_discount(&self) -> bool {
        self.items
            .iter()
            .find(|item| item.discount_percentage > 0.00)
            .is_some()
    }

    /// Render the items section table header.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_items_header<'a>(
        &self,
        table: &Table<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let column_headers = if self.any_item_has_discount() {
            vec![
                Some(ARTICLE_ID_LABEL),
                Some(DESCRIPTION_LABEL),
                Some(QUANTITY_LABEL),
                None,
                Some(PRICE_LABEL),
                Some(DISCOUNT_LABEL),
                None,
                Some(SUBTOTAL_ITEM_PRICE_LABEL),
                None,
                Some(TOTAL_ITEM_LABEL),
            ]
        } else {
            vec![
                Some(ARTICLE_ID_LABEL),
                Some(DESCRIPTION_LABEL),
                Some(QUANTITY_LABEL),
                None,
                Some(PRICE_LABEL),
                None,
                Some(TOTAL_ITEM_LABEL),
            ]
        };

        for header in column_headers {
            if let Some(label) = header {
                table.add_cell(
                    Cell::new(env)?
                        .set_border(Border::NoBorder, env)?
                        .add(Paragraph::new_with_text(label, env)?.set_bold(env)?, env)?,
                    env,
                )?;
            } else {
                table.add_cell(Cell::new(env)?.set_border(Border::NoBorder, env)?, env)?;
            }
        }

        Ok(())
    }

    /// Render a signle item on the invoice.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_item<'a>(
        &self,
        table: &Table<'a>,
        item: &InvoiceItem,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let values = vec![
            Some((item.identifier.clone(), false)),
            Some((item.description.clone(), false)),
            Some((format!("{:.2}", item.quantity as f32), false)),
            Some((self.currency.to_string(), false)),
            Some((format!("{:.2}", item.price_per_unit), true)),
            self.any_item_has_discount().then_some((format!("{:.2}%", item.discount_percentage), true)),
            self.any_item_has_discount().then_some((self.currency.to_string(), true)),
            self.any_item_has_discount().then_some((format!("{:.2}", item.subtotal_price_per_unit), true)),
            Some((self.currency.to_string(), false)),
            Some((format!("{:.2}", item.total_price), true)),
        ];

        for value in values {
            if let Some((value, right_aligned)) = value {
                let paragraph = Paragraph::new_with_text(&value, env)?;

                if right_aligned {
                    paragraph.set_text_alignment(TextAlignment::Right, env)?;
                }

                table.add_cell(
                    Cell::new(env)?
                        .set_border(Border::NoBorder, env)?
                        .add(&paragraph, env)?,
                    env,
                )?;
            }
        }

        Ok(())
    }

    /// Render the invoice totals at the bottom of the last page.
    ///
    /// # Errors
    ///
    /// If a JNI error occurs
    fn render_invoice_totals<'a>(
        &self,
        document: &Document<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        let border = Border::Solid {
            color: ColorConstant::Black,
            width: 1.0,
        };

        let table = Table::new(&[1.0, 1.0, 1.0], env)?;

        let page_width = document
            .get_pdf_document(env)?
            .get_default_page_size(env)?
            .get_width(env)?;

        const BOTTOM_OFFSET: f32 = 100.0;

        table
            .set_fixed_position(
                document.get_left_margin(env)? + document.get_right_margin(env)? + page_width / 2.0,
                document.get_bottom_margin(env)? + BOTTOM_OFFSET,
                page_width - document.get_left_margin(env)? - document.get_right_margin(env)?,
                env,
            )?
            .use_all_available_width(env)?
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    &Paragraph::new_with_text(TOTAL_EXCLUDING_VAT_LABEL, env)?,
                    env,
                )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    &Paragraph::new_with_text(&self.currency.to_string(), env)?,
                    env,
                )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    &Paragraph::new_with_text(
                        &format!("{:.2}", self.totals.total_excluding_vat),
                        env,
                    )?
                    .set_text_alignment(TextAlignment::Right, env)?,
                    env,
                )?,
                env,
            )?
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(border.clone(), env)?
                    .add(&Paragraph::new_with_text(TOTAL_VAT_LABEL, env)?, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(border.clone(), env)?
                    .add(
                        &Paragraph::new_with_text(&self.currency.to_string(), env)?,
                        env,
                    )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .set_border_bottom(border.clone(), env)?
                    .add(
                        &Paragraph::new_with_text(&format!("{:.2}", self.totals.total_vat), env)?
                            .set_text_alignment(TextAlignment::Right, env)?,
                        env,
                    )?,
                env,
            )?
            .start_new_row(env)?
            .add_cell(
                Cell::new(env)?
                    .set_border(Border::NoBorder, env)?
                    .add(&Paragraph::new_with_text(TOTAL_PRICE_LABEL, env)?, env)?,
                env,
            )?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    &Paragraph::new_with_text(&self.currency.to_string(), env)?,
                    env,
                )?,
                env,
            )?
            .add_cell(
                Cell::new(env)?.set_border(Border::NoBorder, env)?.add(
                    &Paragraph::new_with_text(
                        &format!("{:.2}", self.totals.total_including_vat),
                        env,
                    )?
                    .set_text_alignment(TextAlignment::Right, env)?,
                    env,
                )?,
                env,
            )?;

        document.add(table, env)?;
        Ok(())
    }
}

impl ToString for Currency {
    fn to_string(&self) -> String {
        match self {
            Self::Euro => "€",
        }
        .to_string()
    }
}
