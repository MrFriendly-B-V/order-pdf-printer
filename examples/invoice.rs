use color_eyre::Result;
use order_pdf_printer::invoice::{Currency, Invoice, InvoiceItem, InvoiceTotals};
use order_pdf_printer::{DocumentConfiguration, PdfRenderable, RenderTarget, JVM};
use std::fs::File;
use std::io::Write;

mod info;

fn main() -> Result<()> {
    color_eyre::install()?;

    let jvm = JVM::new()?;
    let mut env = jvm.attach()?;

    let render_target = RenderTarget::new(
        &DocumentConfiguration {
            font_family: Some(include_bytes!("OpenSans-Regular.ttf").to_vec()),
            font_size: Some(11.0),
        },
        &mut env,
    )?;

    let invoice = Invoice {
        header: info::get_header("Factuur"),
        footer: info::get_footer(),
        reference_id: "10315".into(),
        invoice_id: "230307".into(),
        invoice_date: "19-06-2023".to_string(),
        currency: Currency::Euro,
        note: Some("LET OP : REEDS BETAALD".into()),
        totals: InvoiceTotals {
            total_excluding_vat: 242.95,
            total_vat: 51.02,
            total_including_vat: 293.97,
        },

        expiration_date: "19-07-2023".to_string(),
        items: vec![
            InvoiceItem {
                identifier: "16005-3".to_string(),
                description: "Set 3 stuks Mr.Friendly Filter Globe (nat)".to_string(),
                quantity: 1,
                price_per_unit: 59.95,
                discount_percentage: 10.0,
                subtotal_price_per_unit: 59.95,
                total_price: 59.95,
            };
            3
        ],
    };

    invoice.render(&render_target, &mut env)?;

    let pdf_bytes = render_target.finish(&mut env)?;
    let mut f = File::create("out.pdf")?;
    f.write_all(&pdf_bytes)?;

    Ok(())
}
