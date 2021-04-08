use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let mut data = Data::new();

    let cx = 210.0 / 2.0;
    let rows = 60;
    let rowsD = 4.0;
    for i in 0..rows {
        let y = 10.0 + rowsD * (i as f32);
        let dx = 100.0 * (0.02 + 0.98 * ((i as f32) / (rows as f32)).powf(2.0));
        let dy = 1.4 * dx * (1.0 - (i as f32) / (rows as f32));
        data = data.move_to((cx - dx, y + dy));
        data = data.line_to((cx, y));
        data = data.line_to((cx + dx, y + dy));
    }
    let cols = 8;
    for i in 0..cols {
        let dx = 3.0;
        let dy = 40.0;
        let x = cx - dx * ((i as f32) - (cols as f32) / 2.0);
        let y = 8.0 + rowsD * (rows as f32);
        data = data.move_to((x, y));
        data = data.line_to((x, y + dy));
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "white")
        .set("stroke-width", 0.5)
        .set("d", data);

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 210, 297))
        .set("width", "210mm")
        .set("height", "297mm")
        .set("style", "background:black")
        .add(path);

    svg::save("image.svg", &document).unwrap();
}
