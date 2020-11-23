
use crate::vect::Vect;
use rand::Rng;

pub fn pathTrace(w: usize, h: usize, sampleCount: usize, position: Vect, goal: Vect) {
    let dirLeft = Vect::new(goal.z, 0, -goal.x).times(1.0 / w).normalize();

    // Cross-product to get the up vector
    let dirUp = Vect::new(goal.y * left.z - goal.z * left.y,
        goal.z * left.x - goal.x * left.z,
        goal.x * left.y - goal.y * left.x);

    let mut pixels = vec![u8; 3*w*h];
    let mut countOutput = 0;
    //let onceOnly = true;
    for (int y = h; y > 0; --y) {
        for (int x = w; x > 0; --x) {
        Vec color;
        for (int p = samplesCount; p--;) {

            Vec incr = Trace(position, !(goal + left * (x - w / 2 + randomVal()) + up * (y - h / 2 + randomVal())), onceOnly);
            if (onceOnly) onceOnly = false;
            color = color + incr;
        }

        // Reinhard tone mapping
        color = color * (1. / samplesCount) + 14. / 241;
        Vec o = color + 1;
        color = Vec(color.x / o.x, color.y / o.y, color.z / o.z) * 255;
        int index = 3*(w*y - w + x - 1);
        pixels[index    ] = (byte)color.x;
        pixels[index + 1] = (byte)color.y;
        pixels[index + 2] = (byte)color.z;
        
        //if (x < w/2 && y < h/2) printf("%d %d \n", x, y);
        //printf("%c%c%c", (int) color.x, (int) color.y, (int) color.z);
        }
  }
  createBMP(pixels, w, h, "cardCPP.bmp");
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

pub fn queryDatabase(position: Vec, &mut hitType: Hit) -> f64 {
    let mut distance: f64 = 1e9;
    distance = min(distance, 
        boxTest(position, Vec(0, 5, 0), Vec(5, 10, 5))
        );

    let mut roomDist: f64 = 0.0;
    roomDist = min(// min(A,B) = Union with Constructive solid geometry
               //-min carves an empty space
                -min(// Lower room
                     boxTest(position, Vec(-30, -.5, -30), Vec(30, 18, 30)),
                     // Upper room
                     boxTest(position, Vec(-25, 17, -25), Vec(25, 20, 25))
                ),
                BoxTest( // Ceiling "planks" spaced 8 units apart.
                  Vec(fmodf(fabsf(position.x), 8),
                      position.y,
                      position.z),
                  Vec(1.5, 18.5, -25),
                  Vec(6.5, 20, 25)
                )
  );  
    if roomDist < distance {
        distance = roomDist;
        hitType = Hit::Wall;
    }
    let sunDist = 19.9 - position.y; // Everything above 19.9 is light source.
    if sunDist < distance {
        distance = sunDist;
        hitType = Hit::Sun;
    }
  
    return distance;
}



pub fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

pub enum Hit {
    None,
    Wall,
    Figure,
    Sun,
}