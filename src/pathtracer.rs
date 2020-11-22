use rand::Rng;

pub fn pathTrace(w: usize, h: usize, sampleCount: usize) {

}

// Rectangle CSG equation. Returns minimum signed distance from
// space carved by
// lowerLeft vertex and opposite rectangle vertex upperRight.
float BoxTest(Vec position, Vec lowerLeft, Vec upperRight) {
    lowerLeft = position + lowerLeft * -1;
    upperRight = upperRight + position * -1;
    return -min(
            min(
                    min(lowerLeft.x, upperRight.x),
                    min(lowerLeft.y, upperRight.y)),
            min(lowerLeft.z, upperRight.z));
  }



pub fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}