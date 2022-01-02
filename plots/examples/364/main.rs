use clap::Clap;
use gre::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let mut rng = rng_from_seed(opts.seed);
    let get_color = image_get_color("/Users/grenaudeau/Desktop/photo.png").unwrap();

    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(ci, color)| {
            let dim = 1200;
            let samples = 50000;
            let f = |p| {
                let rgb = get_color(p);
                let c = grayscale(rgb);
                let dist = euclidian_dist(p, (0.5, 0.5));
                (1. - c).powf(2.0) * smoothstep(0.49, 0.15, dist)
            };
            let mut samples = sample_2d_candidates_f64(&f, dim, samples, &mut rng);
            // pre-tsp
            samples = tsp(samples, time::Duration::seconds(opts.tsp_it_seconds_limit));
            // split samples into chunks
            let chunk_size = 200;
            let mut chunks = Vec::new();
            let mut chunk = Vec::new();
            for p in samples {
                if chunk.len() >= chunk_size {
                    chunks.push(chunk);
                    chunk = Vec::new();
                }
                chunk.push(p);
            }
            chunks.push(chunk);

            // run tsp in parallel on each chunk
            chunks = chunks.par_iter().map(|chunk| {
                tsp(chunk.clone(), time::Duration::seconds(opts.tsp_it_seconds_limit))
            }).collect();
            
            let pad = 20.0;
            let height = 297.0;
            let width = 210.0;
            let boundaries = (pad, pad, width - pad, height - pad);
            let stroke_dist = 1.0;

            let mut l = layer(color);
            let mut data = Data::new();
            for chunk in chunks {
                for p in chunk {
                    let a = project_in_boundaries(p, boundaries);
                    let b = follow_angle(a, (p.0-0.5).atan2(p.1-0.5), stroke_dist);
                    data = data.move_to(a).line_to(b);
                }
            }
            l = l.add(base_path(color, 0.35, data));
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
    #[clap(short, long, default_value = "5")]
    tsp_it_seconds_limit: i64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
