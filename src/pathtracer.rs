
use crate::vect::Vect;
use rand::Rng;

pub fn pathTrace(w: usize, h: usize, sampleCount: usize) {

}

// Rectangle CSG equation. Returns minimum signed distance from
// space carved by
// lowerLeft vertex and opposite rectangle vertex upperRight.
pub fn boxTest(position: Vect, lowerLeft: Vect, upperRight: Vect) -> f64 {
    let diff0 = position.minus(lowerLeft);
    let diff1 = upperRight.minus(position);
    -min(min(min(diff0.x, diff1.x), 
                    min(diff0.y, diff1.y)),
                min(diff0.z, diff1.z))
}



pub fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}