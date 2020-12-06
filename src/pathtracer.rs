use crate::vect::Vect;
use rand::Rng;
use crate::output::createBMP;
static lightDirection: Vect = Vect::new(0.6, 0.6, 1.0).normalize();
static rng: rand::ThreadRng = rand::thread_rng();

static colorSun: Vect = Vect::new(50.0, 80.0, 100.0);

pub fn pathTrace(w: usize, h: usize, sampleCount: usize, position: Vect, goal: Vect) {
    let dirLeft = Vect::new(goal.z, 0.0, -goal.x).times(1.0 / (w as f64)).normalize();

    // Cross-product to get the up vector
    let dirUp = Vect::new(goal.y * dirLeft.z - goal.z * dirLeft.y,
        goal.z * dirLeft.x - goal.x * dirLeft.z,
        goal.x * dirLeft.y - goal.y * dirLeft.x);
    let mut pixels = vec![0; 3*w*h];
    let mut countOutput = 0;
    let mut onceOnly = true;
    for y in (1..h+1).rev() {
        for x in (1..w+1).rev() {
            let color = Vect::new(0.0, 0.0, 0.0);
            for p in 0..sampleCount {
                let randomizedDir: Vect = goal.plus(dirLeft.times(((x - w/2) as f64) + rng.gen::<f64>()))
                                              .plus(dirLeft.times(((y - h/2) as f64) + rng.gen::<f64>()))
                                              .normalize();
                // !(goal + left * (x - w / 2 + randomVal()) + up * (y - h / 2 + randomVal()));
                let incr = trace(position, randomizedDir, onceOnly);
                if onceOnly { onceOnly = false }
                color.plusM(incr);
            }

            // Reinhard tone mapping
            color.timesM(1.0/(sampleCount as f64));
            color.plusAllM(14.0/(241 as f64));
            let o = color.plusAll(1.0);
            color = Vect::new(color.x / o.x, color.y / o.y, color.z / o.z).times(255.0);
            let index = 3*(w*y - w + x - 1);
            pixels[index    ] = color.x as u8;
            pixels[index + 1] = color.y as u8;
            pixels[index + 2] = color.z as u8;
        }
    }
    createBMP(pixels, w, h, String::from("cardCPP.bmp"));
}



pub fn trace(origin: Vect, marchDirection: Vect, onceOnly: bool) -> Vect {
    let mut color = Vect::new(0.0, 0.0, 0.0);
    let mut sampledPosition = Vect::new(0.0, 0.0, 0.0);
    let mut normal = Vect::new(0.0, 0.0, 0.0);
    let attenuation: f64 = 1.0;
  
    for bounceCount in 0..3 {
        let mut hitType: Hit = rayMarching(origin, marchDirection, sampledPosition, normal);
        match hitType {
            NoHit => break,
            Wall => {
                let incidence = normal.dot(marchDirection);
                let p = 6.283185 * rng.gen::<f64>();
                let c = rng.gen::<f64>();
                let s = (1.0 - c).sqrt();
                let g = if normal.z < 0.0 { -1.0 } else { 1.0 };
                let u = -1.0/(g + normal.z);
                let v = normal.x*normal.y*u;
                let a = Vect::new(v,  g + normal.y * normal.y * u, -normal.y).times(p.cos() * s);
                let b = Vect::new(1.0 + g * normal.x * normal.x * u, g * v, -g * normal.x).times(p.sin() * s);
                let direction = a.plus(b).plus(normal.times(c.sqrt()));
                if onceOnly {
                    //println("newDirection %.2f %.2f %.2f\n", direction.x, direction.y, direction.z);
                    //println("incidence %.2f p %.2f c %.2f s %.4f g %.4f u %.f v %.4f \n", incidence, p, c, s, g, u, v);
                }
                origin = sampledPosition.plus(direction.times(0.1));
                attenuation *= 0.2;
                if incidence > 0.0 {
                    let pos: Vect = sampledPosition.plus(normal.times(0.1));
                    let temp: Hit = rayMarching(pos, marchDirection, sampledPosition, normal);
                    if onceOnly {
                        //printf("%.2f %.2f %.2f\n", sampledPosition.x, sampledPosition.y, sampledPosition.z);
    
                        //printf("%.2f %.2f %.2f temp = %d\n", pos.x, pos.y, pos.z, temp);
                        //printf("sampledPosition %.2f %.2f %.2f\n", sampledPosition.x, sampledPosition.y, sampledPosition.z, temp);
                    }                      
                    if let temp = Hit::Sun {                         
                        color.plusM(colorSun.times(attenuation * incidence));
                        if onceOnly {
                            //println!("wall incrementation, attenuation = %.2f, incidence = %.2f\n", attenuation, incidence);
                        }                     
                    }
                }
            },
            Figure => {
                lightDirection.plusM(normal.times(-2.0*normal.dot(lightDirection)));
                origin = sampledPosition.plus(lightDirection.times(0.1));
                attenuation *= 0.2;
            },
            Sun => {
                color.plusM(colorSun.times(attenuation)); 
                if onceOnly {
                    println!("Sun incrementation\n");
                }
                break; // Sun Color
            },
        }
    }
    return color;
}

// Perform signed sphere marching
// Returns hitType 0, 1, 2, or 3 and update hit position/normal
pub fn rayMarching(origin: Vect, direction: Vect, hitPos: &Vect, hitNorm: &Vect) -> Hit {
    let hitType = Hit::NoHit;
    let noHitCount: i64 = 0;
    let d: f64 = 0.0; // distance from closest object in world.
  
    // Signed distance marching
    for (float totalD = 0; totalD < 100; totalD += d) {
        d = QueryDatabase(hitPos = origin + direction * total_d, hitType);
        if (d < 0.01 || ++noHitCount > 99);
        hitNorm =
           !Vec(QueryDatabase(hitPos + Vec(0.01, 0.0, 0.0), noHitCount) - d,
                QueryDatabase(hitPos + Vec(0.0, 0.01, 0.0), noHitCount) - d,
                QueryDatabase(hitPos + Vec(0.0, 0.0, 0.01), noHitCount) - d);
        return hitType; // Weird return statement where a variable is also updated.
    }
    return Hit::NoHit;
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

static firstBox0: Vect = Vect::new(0.0, 5.0, 0.0);
static firstBox1: Vect = Vect::new(5.0, 10.0, 5.0);
static lowerRoom0: Vect = Vect::new(-30.0, -0.5, -30.0);
static lowerRoom1: Vect = Vect::new(30.0, 18.0, 30.0);
static upperRoom0: Vect = Vect::new(-25.0, 17.0, -25.0);
static upperRoom1: Vect = Vect::new(25.0, 20.0, 25.0);
static plank0: Vect = Vect::new(1.5, 18.5, -25.0);
static plank1: Vect = Vect::new(6.5, 20.0, 25.0);

pub fn queryDatabase(position: Vect, hitType: &mut Hit) -> f64 {
    let mut distance: f64 = 1e9;
    distance = min(distance, 
        boxTest(position, firstBox0, firstBox1)
        );

    let mut roomDist: f64 = 0.0;
    roomDist = min(// min(A,B) = Union with Constructive solid geometry
               //-min carves an empty space
                -min(// Lower room
                     boxTest(position, lowerRoom0, lowerRoom1),
                     // Upper room
                     boxTest(position, upperRoom0, upperRoom1)
                ),
                boxTest( // Ceiling "planks" spaced 8 units apart.
                  Vect::new((position.x).abs()%8.0,
                      position.y,
                      position.z),
                  plank0, plank1)
    );  
    if roomDist < distance {
        distance = roomDist;
        *hitType = Hit::Wall;
    }
    let sunDist = 19.9 - position.y; // Everything above 19.9 is light source.
    if sunDist < distance {
        distance = sunDist;
        *hitType = Hit::Sun;
    }
  
    return distance;
}



pub fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

pub enum Hit {
    NoHit,
    Wall,
    Figure,
    Sun,
}