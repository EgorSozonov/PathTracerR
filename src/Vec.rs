struct Vec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec {
    pub fn new(_x: f64, _y: f64, _z: f64): Vec {
        Vec { x: _x, y: _y, z: _z, }
    }
}

trait Vec3D {
    pub fn plus(&self, a: &Vec): Vec;
    pub fn plusM(&mut self, a: &Vec);
    pub fn minus(&self, a: &Vec): Vec;
    pub fn minusM(&mut self, a: &Vec);
    pub fn plusM(&mut self, a: &Vec);
    pub fn times(&self, a: &Vec): Vec;
    pub fn timesM(&mut self, a: &Vec);
    pub fn plusAll(&self, a: f64): Vec;
    pub fn plusAllM(&mut self, a: f64);
    pub fn normalize(&self): Vec;
    pub fn normalizeM(&mut self);
    pub fn dot(&self, a: &Vec): f64;
    pub fn length(&self): f64;

}

impl Vec3D for Vec {
    fn plus(&self, a: &Vec): Vec {
        Vec::new(self.x + a.x, self.y + a.y, self.z + a.z);
    }

    fn plusM(&mut self, a: &Vec) {
        self.x += a.x;
        self.y += a.y;
        self.z += a.z;
    }

    fn minus(&self, a: &Vec): Vec;

    fn minusM(&mut self, a: &Vec) {
        self.x -= a.x;
        self.y -= a.y;
        self.z -= a.z;
    }

    fn times(&self, a: &Vec): Vec;
    fn timesM(&self, a: &Vec) {
        self.x *= a.x;
        self.y *= a.y;
        self.z *= a.z;
    }
    
    fn plusAll(&self, a: f64): Vec;
    fn plusAllM(&self, a: f64);
    fn normalize(&self): Vec;
    fn normalizeM(&self);
    fn dot(&self, a: &Vec): f64;
    fn length(&self): f64;
}