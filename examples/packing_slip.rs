use color_eyre::Result;
use order_pdf_printer::packing_slip::{ArticlePackingInfo, PackingSlip};
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

    let packing_slip = PackingSlip {
        header: info::get_header("Pakbon"),
        footer: info::get_footer(),
        reference_id: "10273".into(),
        order_id: "23138".to_string(),
        packing_slip_id: "880".to_string(),
        delivery_date: "7-2-2023".to_string(),
        items: vec![
            ArticlePackingInfo {
                identifier: "15001A".to_string(),
                description: "Filter falcon (set van 3)".into(),
                quantity_delivered: 1,
                quantity_ordered: 1,
                quantity_backorder: 0,
            },
            ArticlePackingInfo {
                identifier: "15020".to_string(),
                description: "Liquid 'Waterless' 1L".into(),
                quantity_delivered: 1,
                quantity_ordered: 1,
                quantity_backorder: 0,
            },
            ArticlePackingInfo {
                identifier: "16502".to_string(),
                description: "Verzendkosten".into(),
                quantity_delivered: 1,
                quantity_ordered: 1,
                quantity_backorder: 0,
            },
        ],
    };

    packing_slip.render(&render_target, &mut env)?;

    let pdf_bytes = render_target.finish(&mut env)?;
    let mut f = File::create("out.pdf")?;
    f.write_all(&pdf_bytes)?;

    Ok(())
}
