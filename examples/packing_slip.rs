use color_eyre::Result;
use order_pdf_printer::footer::Footer;
use order_pdf_printer::header::{Address, AddressableParty, Header};
use order_pdf_printer::packing_slip::{ArticlePackingInfo, PackingSlip};
use order_pdf_printer::{DocumentConfiguration, PdfRenderable, RenderTarget, JVM};
use std::fs::File;
use std::io::Write;

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
        header: Header {
            label: "Pakbon".into(),
            logo: include_bytes!("logo.png").to_vec(),
            seller: AddressableParty {
                name: "Mr.Friendly B.V.".into(),
                address: Address {
                    city: "'s-Graveland".into(),
                    zipcode: "1243 HV".into(),
                    number: "11-07".into(),
                    street: "De Boomgaard".into(),
                    country: "Nederland".into(),
                },
                vat_number: None,
                department: None,
            },
            addressed_to: AddressableParty {
                name: "Sportfondsen B.V.".into(),
                address: Address {
                    city: "Bussum".into(),
                    zipcode: "1406 TK".into(),
                    number: "14".into(),
                    street: "Struikheiweg".into(),
                    country: "Nederland".into(),
                },
                vat_number: None,
                department: None,
            },
        },
        footer: Footer {
            company_name: "Mr.Friendly B.V.".into(),
            company_name_color: "#3BAF29".into(),
            fields: vec![
                "Postbus 1128; 1400 BC Bussum".into(),
                "info@mrfriendly.nl".into(),
                "IBAN NL75 INGB 0007 5383 83".into(),
                "KvK 67223370".into(),
                "T +31 (0)88 39 29 26 0".into(),
                "www.mrfriendly.nl".into(),
                "BIC INGBNL2A".into(),
                "BTW NL856883566B01".into(),
            ],
        },
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
