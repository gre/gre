use geo::algorithm::centroid::Centroid;
use geo::algorithm::euclidean_length::*;
use geo::*;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

// generate a svg path data that will fill a convex polygon
// NB: this is an unfinished version, rendering is pretty cool to make this an art
fn wip_spiral_fill_convex_polygon(polygon: Polygon<f32>, offset: f32) -> Data {
    let mut data = Data::new();
    let mut points: Vec<Point<f32>> = polygon.exterior().points_iter().collect();
    let mut i = 0;
    loop {
        let l = points.len();
        if l < 2 {
            break;
        }
        let next_i = (i + 1) % l;
        let from = points[i];
        let to = points[next_i];
        let length = Line::new(from, to).euclidean_length();
        if length <= offset + 0.01 {
            points.remove(next_i);
            if next_i < i {
                i -= 1;
            }
            continue;
        }

        let m = (from - to) * (offset / length);
        let to_next = to + m;
        points[i] = to_next;

        // todo: move the next "line", not just point
        // any point that is "eated" by the line must disappear

        data = data.move_to(from.x_y());
        data = data.line_to(to_next.x_y());

        i = next_i;
    }

    return data;
}

fn main() {
    let data = wip_spiral_fill_convex_polygon(
        Polygon::new(
            vec![(50.0, 30.0), (150.0, 60.0), (180.0, 140.0), (30.0, 180.0)].into(),
            vec![],
        ),
        4.0,
    );

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "white")
        .set("stroke-width", 0.2)
        .set("d", data);

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background:black")
        .set("viewBox", (0, 0, 210, 210))
        .set("width", "210mm")
        .set("height", "210mm")
        .add(path)
        .add(gre::signature(1.0, (180.0, 195.0), "white"));

    svg::save("image.svg", &document).unwrap();
}
