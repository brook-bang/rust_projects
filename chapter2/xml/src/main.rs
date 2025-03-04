use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Default)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Debug, Default)]
struct Sale {
    id: String,
    product_id: u32,
    date: i64,
    quantity: f64,
    unit: String,
}

enum LocationItem {
    Other,
    InProduct,
    InSale,
}
enum LocationProduct {
    Other,
    InId,
    InCategory,
    InName,
}
enum LocationSale {
    Other,
    InId,
    InProductId,
    InDate,
    InQuantity,
    InUnit,
}
fn main() {
    let mut location_item = LocationItem::Other;
    let mut location_product = LocationProduct::Other;
    let mut location_sale = LocationSale::Other;

    let pathname = std::env::args().nth(1).unwrap();

    let mut product: Product = Default::default();
    let mut sale: Sale = Default::default();

    let file = std::fs::File::open(pathname).unwrap();
    let file = std::io::BufReader::new(file);

    let parser = EventReader::new(file);

    for event in parser {
        match &location_item {
            LocationItem::Other => match event {
                Ok(XmlEvent::StartElement { ref name, ..}) if name.local_name == "product" => {
                    location_item = LocationItem::InProduct;
                    location_product = LocationProduct::Other;
                    product = Default::default();
                }
                Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "sale" => {
                    location_item = LocationItem::InSale;
                    location_sale = LocationSale::Other;
                    sale = Default::default();
                }
                _ => {}
            },
            LocationItem::InProduct => match &location_product {
                LocationProduct::Other => match event {
                    Ok(XmlEvent::StartElement { ref name, attributes, namespace })
                }
            }
            
        }
    }









}
