fn radion_from_degree(degree: f64) -> f64 {
  0.01745329251994329577f64 * degree
}

fn square(x: f64) -> f64 {
  x * x
}

pub fn haversine(x0: f64, x1: f64, y0: f64, y1: f64, earth_radius: f64) -> f64 {
  let lat1 = y0;
  let lat2 = y1;
  let lon1 = x0;
  let lon2 = x1;
  let d_lat = radion_from_degree(lat2 - lat1);
  let d_lon = radion_from_degree(lon2 - lon1);

  let d_lat1 = radion_from_degree(lat1);
  let d_lat2 = radion_from_degree(lat2);
  let a = square(f64::sin(d_lat / 2f64))
      + (f64::cos(d_lat1) * f64::cos(d_lat2) * square(f64::sin(d_lon / 2f64)));
  let c = 2f64 * f64::asin(f64::sqrt(a));

  earth_radius * c
}
