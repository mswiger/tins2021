use bevy::prelude::*;

static TILE_SIZE: i32 = 16;

pub struct Hex {
    q: f32,
    r: f32,
}

impl Hex {
    pub fn new(q: f32, r: f32) -> Self {
        Self { q, r }
    }

    pub fn from_pixel_coords(coords: Vec2) -> Self {
        let root3 = 3.0_f32.sqrt();
        let q = (2.0 / 3.0 * coords.x) / Self::get_size_w();
        let r = (coords.y / Self::get_size_h() - root3 / 2. * q) / root3;

        Self::new(q, r).rounded()
    }

    pub fn to_cube_coords(&self) -> Cube {
        let x = self.q;
        let z = self.r;
        let y = -x - z;

        Cube::new(x, y, z)
    }

    pub fn to_pixel_coords(&self) -> Vec2 {
        let root3 = 3.0_f32.sqrt();
        let translate_x = Self::get_size_w() * (3.0 / 2.0 * self.q);
        let translate_y = Self::get_size_h() * (root3 / 2.0 * self.q + root3 * self.r);
        return Vec2::new(translate_x, translate_y);
    }

    pub fn rounded(&self) -> Hex {
        self.to_cube_coords().rounded().to_axial_coords()
    }

    fn get_size_w() -> f32 {
        TILE_SIZE as f32 / 2.0
    }

    fn get_size_h() -> f32 {
        TILE_SIZE as f32 / 3.0_f32.sqrt()
    }
}

pub struct Cube {
    x: f32,
    y: f32,
    z: f32,
}

impl Cube {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_axial_coords(&self) -> Hex {
        let q = self.x;
        let r = self.z;

        Hex::new(q, r)
    }

    pub fn rounded(&self) -> Cube {
        let mut rx = self.x.round();
        let mut ry = self.y.round();
        let mut rz = self.z.round();

        let x_diff = (rx - self.x).abs();
        let y_diff = (ry - self.y).abs();
        let z_diff = (rz - self.z).abs();

        if x_diff > y_diff && x_diff > z_diff {
            rx = -ry - rz;
        } else if y_diff > z_diff {
            ry = -rx - rz;
        } else {
            rz = -rx - ry;
        }

        Cube::new(rx, ry, rz)
    }
}
