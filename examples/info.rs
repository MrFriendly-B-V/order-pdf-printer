use order_pdf_printer::footer::Footer;
use order_pdf_printer::header::{Address, AddressableParty, Header};

pub fn get_header(label: &str) -> Header {
    Header {
        label: label.into(),
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
            department: Some("Henk Knakworst".to_string()),
        },
    }
}

pub fn get_footer() -> Footer {
    Footer {
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
    }
}

#[allow(unused)]
fn main() {}
