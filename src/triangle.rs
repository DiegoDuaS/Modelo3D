use nalgebra_glm::{Vec3, dot};
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::line::line;
use crate::color::Color;

pub fn _triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
  let mut fragments = Vec::new();

  // Draw the three sides of the triangle
  fragments.extend(line(v1, v2));
  fragments.extend(line(v2, v3));
  fragments.extend(line(v3, v1));

  fragments
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
  let mut fragments = Vec::new();
  let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

  let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

  let light_dir = Vec3::new(0.0, 0.0, -1.0);

  let triangle_area = edge_function(&a, &b, &c);

  // Iterate over each pixel in the bounding box
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

      // Calculate barycentric coordinates
      let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

      // Check if the point is inside the triangle
      if w1 >= 0.0 && w1 <= 1.0 && 
         w2 >= 0.0 && w2 <= 1.0 &&
         w3 >= 0.0 && w3 <= 1.0 {
        // Interpolate normal
        // let normal = v1.transformed_normal * w1 + v2.transformed_normal * w2 + v3.transformed_normal * w3;
        let normal = v1.transformed_normal;
        let normal = normal.normalize();

        // Calculate lighting intensity
        let intensity = dot(&normal, &light_dir).max(0.0);

        // Create a gray color and apply lighting
        let base_color = Color::new(100, 100, 100); // Medium gray
        let lit_color = base_color * intensity;

        // Interpolate depth
        // let depth = a.z * w1 + b.z * w2 + c.z * w3;
        let depth = a.z;

        fragments.push(Fragment::new(x as f32, y as f32, lit_color, depth));
      }
    }
  }

  fragments
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32) {
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;

    (w1, w2, w3)
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}