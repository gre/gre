use reqwest::Error;
use serde::Deserialize;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

#[derive(Deserialize)]
struct Datapoint(
    u64,    // open_time
    String, // open
    String, // high
    String, // low
    String, // close
    String, // volume
    u64,    // close_time
    String, // quote_asset_volume
    u64,    // trades
    String, // taker_buy_base_volume
    String, // taker_buy_quote_volume
    String,
);

async fn fetch_latest_binance_klines(
    symbol: &str,
    interval: &str,
    limit: u32,
) -> Result<Vec<Datapoint>, Error> {
    let url =
        format!(
        "https://api.binance.com/api/v3/klines?symbol={symbol}&interval={interval}&limit={limit}",
        symbol=symbol, interval=interval, limit=limit
    );
    let response = reqwest::get(&url).await?;
    let datapoints: Vec<Datapoint> = response.json().await?;
    return Ok(datapoints);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let datapoints = fetch_latest_binance_klines("BTCUSDT", "1d", 1000).await?;
    let size = datapoints.len();
    assert_ne!(size, 0);
    let values = datapoints
        .iter()
        .map(|d| d.4.parse().unwrap())
        .collect::<Vec<f32>>();
    let first = values[0];
    let mut min_value = first;
    let mut max_value = first;
    for v in values.clone() {
        if v < min_value {
            min_value = v;
        }
        if v > max_value {
            max_value = v;
        }
    }
    // min & max are bounded to better rounded values
    let mag = (10.0 as f32).powf(max_value.log10().floor() - 1.0);
    min_value = ((min_value - mag / 2.0) / mag).floor() * mag;
    max_value = ((max_value + mag / 2.0) / mag).ceil() * mag;

    // ~2cm margins on A4
    let project = |value, index| {
        (
            20.0 + 260.0 * (index as f32) / (size as f32),
            20.0 + 170.0 * (1.0 - (value - min_value) / (max_value - min_value)),
        )
    };

    let mut data = Data::new().move_to(project(first, 0));
    for (i, v) in values.iter().enumerate() {
        data = data.line_to(project(*v, i));
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.5)
        .set("d", data);

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(
            Line::new()
                .set("x1", 20)
                .set("x2", 20)
                .set("y1", 20)
                .set("y2", 190)
                .set("stroke", "black")
                .set("stroke-width", 0.5),
        )
        .add(path)
        .add(
            Text::new()
                .set("x", 280)
                .set("y", 190)
                .set("font-family", "serif")
                .set("text-anchor", "end")
                .set("font-size", "6")
                .add(svg::node::Text::new("BTC USD (3 last years)")),
        )
        .add(
            Text::new()
                .set("x", 22)
                .set("y", 26)
                .set("font-family", "serif")
                .set("font-size", "6")
                .add(svg::node::Text::new(format!("{}", max_value))),
        )
        .add(
            Text::new()
                .set("x", 22)
                .set("y", 188)
                .set("font-family", "serif")
                .set("font-size", "6")
                .add(svg::node::Text::new(format!("{}", min_value))),
        );

    svg::save("image.svg", &document).unwrap();

    return Ok(());
}
