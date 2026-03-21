use std::{convert::TryInto, io::BufWriter};
use stl::*;

use crate::algo::tri::Tri;

fn stl_tri(clr: &[f32; 3], triangle: &Tri) -> Triangle {
  let v1 = triangle.a;
  let v2 = triangle.b;
  let v3 = triangle.c;
  let normal = (v2 - v1).cross(&(v3 - v1)).normalize();

  /*
  The VisCAM and SolidView software packages use the two "attribute byte count" bytes at the end of every triangle to store a 15-bit RGB color:
  bits 0–4 are the intensity level for blue (0–31),
  bits 5–9 are the intensity level for green (0–31),
  bits 10–14 are the intensity level for red (0–31),
  bit 15 is 1 if the color is valid, or 0 if the color is not valid (as with normal STL files).
       */
  let mut attr_byte_count = 0u16;
  for i in 0..3 {
    let color = (clr[i] * 31.0) as u16;
    attr_byte_count |= color << (5 * (2 - i));
  }

  // Create the stl::Triangle struct using the normal vector and vertices
  Triangle {
    normal: [normal.x, normal.y, normal.z],
    v1: [v1.x, v1.y, v1.z],
    v2: [v2.x, v2.y, v2.z],
    v3: [v3.x, v3.y, v3.z],
    attr_byte_count,
  }
}

pub fn stl_export<T: std::io::Write>(
  bw: &mut BufWriter<T>,
  triangles: &Vec<(usize, Vec<Tri>)>,
  palette: &Vec<[f32; 3]>,
) {
  let header: [u8; 80] = vec![0u8; 80].as_slice().try_into().unwrap();
  let tris = triangles
    .iter()
    .flat_map(|(clr, tris)| tris.iter().map(move |t| (*clr, t)))
    .collect::<Vec<_>>();
  let stl = BinaryStlFile {
    header: BinaryStlHeader {
      header,
      num_triangles: tris.len() as u32,
    },
    triangles: tris
      .iter()
      .map(|(clr, t)| stl_tri(&palette[*clr], t))
      .collect(),
  };
  write_stl(bw, &stl).unwrap();
}
