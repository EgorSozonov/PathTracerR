use crate::vect::Vect;
use rand::Rng;
use crate::output::createBMP;



pub fn pathTrace(w: usize, h: usize, sampleCount: usize, position: Vect, goal: Vect) {
    let dirLeft = Vect::new(goal.z, 0, -goal.x).times(1.0 / w).normalize();

    // Cross-product to get the up vector
    let dirUp = Vect::new(goal.y * dirLeft.z - goal.z * dirLeft.y,
        goal.z * dirLeft.x - goal.x * dirLeft.z,
        goal.x * dirLeft.y - goal.y * dirLeft.x);
    let mut rng = rand::thread_rng();

    let mut pixels = vec![0; 3*w*h];
    let mut countOutput = 0;
    let mut onceOnly = true;
    for y in (1..h+1).rev() {
        for x in (1..w+1).rev() {
            let color = Vect::new(0, 0, 0);
            for p in 0..sampleCount {
                let randomizedDir: Vect = goal.plus(dirLeft.times(x - w/2 + rng.gen::<f64>()))
                                              .plus(dirLeft.times(y - h/2 + rng.gen::<f64>()))
                                              .normalize();
                // !(goal + left * (x - w / 2 + randomVal()) + up * (y - h / 2 + randomVal()));
                let incr = trace(position, randomizedDir, onceOnly);
                if onceOnly { onceOnly = false }
                color.plusM(&incr);
            }

            // Reinhard tone mapping
            color.timesM(1.0/sampleCount);
            color.plusAllM(14.0/241);
            let o = color.plusAll(1.0);
            color = Vec::new(color.x / o.x, color.y / o.y, color.z / o.z) * 255;
            let index = 3*(w*y - w + x - 1);
            pixels[index    ] = color.x as u8;
            pixels[index + 1] = color.y as u8;
            pixels[index + 2] = color.z as u8;
        }
    }
    createBMP(pixels, w, h, "cardCPP.bmp");
}

static lightDirection = Vect::new(0.6, 0.6, 1.0).normalize():

pub fn trace(origin: Vect, direction: Vect, onceOnly: bool) {
    Vec sampledPosition, normal, color;
    let attenuation: f64 = 1.0;
    Vec lightDirection(!Vec(.6, .6, 1)); // Directional light
  
    for (int bounceCount = 3; bounceCount--;) {
      int hitType = RayMarching(origin, direction, sampledPosition, normal);
      if (hitType == HIT_NONE) break; // No hit. This is over, return color.
      if (hitType == HIT_LETTER) { // Specular bounce on a letter. No color acc.
        direction = direction + normal * ( normal % direction * -2);
        origin = sampledPosition + direction * 0.1;
        attenuation = attenuation * 0.2; // Attenuation via distance traveled.
      }
      if (hitType == HIT_WALL) { // Wall hit uses color yellow?
        float incidence = normal % lightDirection;
        float p = 6.283185 * randomVal();
        float c = randomVal();
        float s = sqrtf(1 - c);
        float g = normal.z < 0 ? -1 : 1;
        float u = -1 / (g + normal.z);
        float v = normal.x * normal.y * u;
        direction = Vec(v,
                        g + normal.y * normal.y * u,
                        -normal.y) * (cosf(p) * s)
                    +
                    Vec(1 + g * normal.x * normal.x * u,
                        g * v,
                        -g * normal.x) * (sinf(p) * s) + normal * sqrtf(c);
        if (onceOnly) {
          printf("newDirection %.2f %.2f %.2f\n", direction.x, direction.y, direction.z);
          printf("incidence %.2f p %.2f c %.2f s %.4f g %.4f u %.f v %.4f \n", incidence, p, c, s, g, u, v);
        }
        origin = sampledPosition + direction * .1;
        attenuation = attenuation * 0.2;
        if (incidence > 0) {
            Vec pos = sampledPosition + normal * .1;
            int temp = RayMarching(pos,
                        lightDirection,
                        sampledPosition,
                        normal);
            if (onceOnly) {
                    //printf("%.2f %.2f %.2f\n", sampledPosition.x, sampledPosition.y, sampledPosition.z);
  
                    //printf("%.2f %.2f %.2f temp = %d\n", pos.x, pos.y, pos.z, temp);
                    //printf("sampledPosition %.2f %.2f %.2f\n", sampledPosition.x, sampledPosition.y, sampledPosition.z, temp);
                }                      
            if (temp == HIT_SUN) {
                color = color + attenuation * Vec(500, 400, 100) * incidence;
                if (onceOnly) {
                    //printf("wall incrementation, attenuation = %.2f, incidence = %.2f\n", attenuation, incidence);
                }
            }
        }
      }
      if (hitType == HIT_SUN) { //
        color = color + attenuation * Vec(50, 80, 100); 
        if (onceOnly) {
            printf("Sun incrementation\n");
        }
        break; // Sun Color      
      }
    }
    return color;
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
                boxTest( // Ceiling "planks" spaced 8 units apart.
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