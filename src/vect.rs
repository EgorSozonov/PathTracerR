pub struct Vect {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vect {
    pub fn new(_x: f64, _y: f64, _z: f64) -> Vect {
        Vect { x: _x, y: _y, z: _z, }
    }

    pub fn plus(&self, a: Vect) -> Vect {
        Vect::new(self.x + a.x, self.y + a.y, self.z + a.z)
    }

    pub fn plusM(&mut self, a: Vect) {
        self.x += a.x;
        self.y += a.y;
        self.z += a.z;
    }

    pub fn minus(&self, a: Vect) -> Vect {
        Vect::new(self.x - a.x, self.y - a.y, self.z - a.z)
    }

    pub fn minusM(&mut self, a: &Vect) {
        self.x -= a.x;
        self.y -= a.y;
        self.z -= a.z;
    }

    pub fn times(self, a: f64) -> Vect {
        Vect::new(self.x * a, self.y * a, self.z * a)
    }

    pub fn timesM(&mut self, a: f64) {
        self.x *= a;
        self.y *= a;
        self.z *= a;
    }
    
    pub fn plusAll(self, a: f64) -> Vect {
        Vect::new(self.x + a, self.y + a, self.z + a)
    }

    pub fn plusAllM(&mut self, a: f64) {
        self.x += a;
        self.y += a;
        self.z += a;
    }

    pub fn normalize(self) -> Vect {
        let len = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        if len == 0.0 {return self}
        Vect::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn normalizeM(&mut self) {
        let len = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        if len == 0.0 {return}
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn dot(self, a: Vect) -> f64 {
        return self.x*a.x + self.y*a.y + self.z*a.z
    }

    pub fn length(self) -> f64 {
        return (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
}