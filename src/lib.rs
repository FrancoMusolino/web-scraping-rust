use charts_rs::BarChart;
use scraper::{ElementRef, Html, Selector};
use serde_json::json;
use std::ffi::OsStr;
use std::{error::Error, fs::File, io::Write, path::Path};

pub mod fetcher;
pub mod parser;
use crate::fetcher::{Fetcher, Methods, ResponseMethods};
use crate::parser::FloatParser;

const URL: &str = "https://www.cronista.com/MercadosOnline/dolar.html";

pub async fn run() -> Result<(), Box<dyn Error>> {
    let response = Fetcher::get(URL).await?;
    let response_as_text = ResponseMethods::parse_to_text(response).await?;

    let document = Html::parse_document(&response_as_text);
    let selector = Selector::parse(".piece.boxed.markets.standard table tbody tr").unwrap();
    let fragments = document.select(&selector);

    let mut dolar_names: Vec<String> = Vec::new();
    let mut dolar_buy_values: Vec<f32> = Vec::new();
    let mut dolar_sell_values: Vec<f32> = Vec::new();

    let mut last_update: Option<String> = None;

    for fragment in fragments {
        let name_selector = Selector::parse(".name").unwrap();
        let name_texts = get_texts_from_fragment(&fragment, &name_selector);
        dolar_names.push(name_texts.get(0).unwrap().to_uppercase().replace("�", "Ó"));

        let buy_selector = Selector::parse(".buy").unwrap();
        let buy_texts = get_texts_from_fragment(&fragment, &buy_selector); // ["Compra$1.371,20"] -> response
        let buy_price = split_text(&buy_texts[0], '$')[1];
        let buy_price_as_float = FloatParser::from_arg_price(buy_price).unwrap_or_default();
        dolar_buy_values.push(buy_price_as_float);

        let sell_selector = Selector::parse(".sell").unwrap();
        let sell_texts = get_texts_from_fragment(&fragment, &sell_selector); // ["Venta$1.371,20"] -> response
        let sell_price = split_text(&sell_texts[0], '$')[1];
        let sell_price_as_float = FloatParser::from_arg_price(sell_price).unwrap_or_default();
        dolar_sell_values.push(sell_price_as_float);

        if last_update.is_none() {
            let last_update_selector = Selector::parse(".date").unwrap();
            let last_update_texts = get_texts_from_fragment(&fragment, &last_update_selector);

            match last_update_texts.get(0) {
                Some(text) => last_update = Some(text.clone()),
                _ => (),
            }
        }
    }

    let chart = json!({
        "width": 1080,
        "height": 750,
        "margin": {
            "left": 10,
            "top": 5,
            "right": 10
        },
        "title_text": "Valor Dólar",
        "title_font_size": 24,
        "sub_title_text": last_update.unwrap_or("Sin última fecha de actualización".to_string()),
        "sub_title_font_size": 18,
        "title_align": "right",
        "legend_align": "left",
        "type": "horizontal_bar",
        "theme": "grafana",
        "series_list": [
            {
                "name": "Compra",
                "label_show": true,
                "data": dolar_buy_values
            },
            {
                "name": "Venta",
                "label_show": true,
                "data": dolar_sell_values
            }
        ],
        "x_axis_data": dolar_names
    });

    let bar_chart = BarChart::from_json(&chart.to_string()).unwrap();

    let mut file = create_file(OsStr::new("dolar.svg"))?;
    file.write(&bar_chart.svg().unwrap().to_string().as_bytes())?;

    Ok(())
}

fn get_texts_from_fragment(fragment: &ElementRef, selector: &Selector) -> Vec<String> {
    fragment
        .select(selector)
        .map(|node| node.text().collect())
        .collect()
}

fn split_text(text: &String, splitter: char) -> Vec<&str> {
    text.split(splitter).collect::<Vec<&str>>()
}

fn create_file(path: &OsStr) -> Result<File, Box<dyn Error>> {
    let path = Path::new(path);
    let file = File::create(path)?;

    Ok(file)
}
