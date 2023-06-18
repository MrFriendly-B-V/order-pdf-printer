use color_eyre::Result;
use invgen::shipment::{Article, CustomerInfo, SellerInfo, Shipment};
use invgen::*;

fn main() -> Result<()> {
    color_eyre::install()?;

    let jvm = JVM::new()?;

    let invoice = Shipment {
        seller: SellerInfo {
            logo: include_bytes!("logo.png").to_vec(),
            address: "De Boomgaard 11-07, 1243 HV, 's-Graveland".into(),
            company_name: "MrFriendly".into(),
            website: "www.mrfriendly.nl".into(),
            telephone: "+31 (0)88 39 29 26 0".into(),
            iban: "NL75 INGB 0007 5383 83".into(),
            kvk: "67223370".into(),
            btw: "NL856883566B01".into(),
            mailbox: "1128; 1400 BC Bussum".into(),
            email: "info@mrfriendly.nl".into(),
            bic: "INGBNL2A".into(),
            footer_company_name_color_hex: "#3BAF29".into(),
        },
        date: "".into(),
        customer: CustomerInfo {
            company_name: "Sportfondsen B.V.".into(),
            address1: "Struikheiweg 14".into(),
            address2: "1406 TK, Bussum".into(),
            customer_name: "Tobias de Bruijn".into(),
        },
        reference_id: "10273".into(),
        order_id: "23138".to_string(),
        shipment_id: "880".to_string(),
        deliver_date: "7-2-2023".to_string(),
        items: vec![
            Article {
                identifier: "15001A".to_string(),
                description: "Filter falcon (set van 3)".into(),
                quantity_delivered: 1,
                quantity_ordered: 1,
                quantity_backorder: 0,
            },
            Article {
                identifier: "15020".to_string(),
                description: "Liquid 'Waterless' 1L".into(),
                quantity_delivered: 1,
                quantity_ordered: 1,
                quantity_backorder: 0,
            },
            Article {
                identifier: "16502".to_string(),
                description: "Verzendkosten".into(),
                quantity_delivered: 1,
                quantity_ordered: 1,
                quantity_backorder: 0,
            },
        ],
    };

    invoice.render(&jvm)?;
    Ok(())
}
