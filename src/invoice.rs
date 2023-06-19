use crate::footer::Footer;
use crate::header::Header;
use crate::{Error, PdfRenderable, RenderTarget};
use itext::itext::layout::{Document, Table};
use jni::JNIEnv;

pub struct Invoice {
    pub header: Header,
    pub footer: Footer,

    pub reference_id: String,
    pub invoice_id: String,
    pub invoice_date: String,
    pub expiration_date: String,

    pub totals: InvoiceTotals,
    pub items: Vec<InvoiceItem>,
    pub note: Option<String>,

    pub currency: Currency,
}

pub enum Currency {
    Euro,
}

pub struct InvoiceItem {
    pub identifier: String,
    pub description: String,
    pub number: i32,
    pub price_per_unit: f32,
    pub discount_percentage: f32,
    pub subtotal_price_per_unit: f32,
    pub total_price: f32,
}

pub struct InvoiceTotals {
    pub total_excluding_vat: f32,
    pub total_vat: f32,
    pub total_including_vat: f32,
}

impl PdfRenderable for Invoice {
    fn render<'a>(&self, target: &RenderTarget<'a>, env: &mut JNIEnv<'a>) -> Result<(), Error> {
        self.header.render(target, env)?;

        // TODO render invoice information
        // TODO render items
        // TODO render totals

        self.footer.render(target, env)?;

        Ok(())
    }
}

impl Invoice {
    fn render_invoice_information<'a>(
        &self,
        document: &Document<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        todo!()
    }

    fn render_items_section<'a>(
        &self,
        document: &Document<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        todo!()
    }

    fn render_items_header<'a>(
        &self,
        table: &Table<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        todo!()
    }

    fn render_item<'a>(
        &self,
        table: &Table<'a>,
        item: &InvoiceItem,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        todo!()
    }

    fn render_invoice_totals<'a>(
        &self,
        document: &Document<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<(), Error> {
        todo!()
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
