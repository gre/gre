use geo::algorithm::centroid::Centroid;
use geo::algorithm::euclidean_length::*;
use geo::{Line, Point, Polygon};
use gre::line_intersection::*;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

fn length(p: Point<f64>) -> f64 {
    let x = p.x();
    let y = p.y();
    (x * x + y + y).sqrt()
}

fn move_offset(a: Point<f64>, b: Point<f64>, offset: f64) -> Option<Point<f64>> {
    let ab = b - a;
    let l = length(ab);
    if l < offset {
        None
    } else {
        Some(a + ab * (offset / l))
    }
}

// generate a svg path data that will fill a convex polygon
// NB: this is an unfinished version, rendering is pretty cool to make this an art
fn wip_spiral_fill_convex_polygon(polygon: Polygon<f64>, offset: f64) -> Option<Data> {
    let mut data = Data::new();
    let mut points: Vec<Point<f64>> = polygon.exterior().points_iter().collect();
    let l = points.len();
    if l < 3 {
        return None;
    }

    // arrange the initial points to be inside the polygon
    let start_edge = move_offset(points[0], points[l - 2], offset)?;
    points[l - 1] = start_edge;
    let start_dir = points[1] - points[0];
    let mut p = start_edge + start_dir * (offset / length(start_dir));
    points.remove(0);

    let mut dir = start_dir;

    data = data.move_to(p.x_y());

    loop {
        if points.len() < 2 {
            break;
        }
        let a = p;
        let b = points[0];
        let mut next = None;

        loop {
            // find a C where a projection of will intersect with BC
            let c = points[1];
            let intersection = LineInterval::ray(Line {
                start: a.into(),
                end: (a + dir).into(),
            })
            .relate(&LineInterval::line_segment(Line {
                start: b.into(),
                end: c.into(),
            }))
            .unique_intersection();
            match intersection {
                None => {
                    points.remove(1);
                    if points.len() < 2 {
                        break;
                    }
                }
                Some(point) => {
                    // todo: actually we want to do a bit more than just offset depending on BC angle..
                    next = move_offset(point, a, offset);
                    dir = c - b;
                    break;
                }
            };
        }
        if points.len() < 1 {
            break;
        }

        match next {
            None => {
                break;
            }
            Some(next) => {
                p = next;
                data = data.line_to(p.x_y());
                points.remove(0);
                points.push(p);
            }
        };
    }

    /*
    let mut i = 0;
    let mut dir = normalized(points[1] - points[0]) * offset;
    loop {
        let l = points.len();
        if l < 2 {
            break;
        }
        let next_i = (i + 1) % l;
        let from = points[i];
        let to = points[next_i];
        let l = length(from - to);
        if l <= offset + 0.01 {
            points.remove(next_i);
            if next_i < i {
                i -= 1;
            }
            continue;
        }

        let prev_from = from + dir;

        let to_next = to + (from - to) * offset / l;

        points[i] = to_next;

        // todo: move the next "line", not just point
        // any point that is "eated" by the line must disappear

        dir = normalized(to_next - from) * offset;

        data = data.move_to(from.x_y());
        data = data.line_to(to_next.x_y());

        i = next_i;
    }
    */

    return Some(data);
}

fn main() {
    let data = wip_spiral_fill_convex_polygon(
        Polygon::new(
            vec![(50.0, 30.0), (150.0, 60.0), (180.0, 140.0), (30.0, 180.0)].into(),
            vec![],
        ),
        1.0,
    )
    .unwrap();

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
