use bevy::prelude::*;

static TILE_SIZE: i32 = 16;

#[derive(Copy, Clone)]
pub struct Hex {
    q: f32,
    r: f32,
}

impl Hex {
    pub fn new(q: f32, r: f32) -> Self {
        Self { q, r }
    }

    pub fn from_pixel_coords(coords: &Vec2) -> Self {
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

    pub fn distance_to(&self, other: &Hex) -> u32 {
        let ac = self.to_cube_coords();
        let bc = other.to_cube_coords();

        ac.distance_to(&bc)
    }

    fn get_size_w() -> f32 {
        TILE_SIZE as f32 / 2.0
    }

    fn get_size_h() -> f32 {
        TILE_SIZE as f32 / 3.0_f32.sqrt()
    }
}

impl PartialEq for Hex {
    fn eq(&self, other: &Self) -> bool {
        let ar = self.r as i32;
        let aq = self.q as i32;
        let br = other.r as i32;
        let bq = other.q as i32;

        ar == br && aq == bq
    }
}

impl Eq for Hex  {}

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

    pub fn distance_to(&self, other: &Cube) -> u32 {
        (((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2.0) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::*;

    #[test]
    fn test_hex_to_pixel_coords() {
        let pixel_coords = Vec2::new(156., -24.);
        let hex_coords = Hex::from_pixel_coords(&pixel_coords);
        let converted_pixel_coords = hex_coords.to_pixel_coords();
        assert!(approx_eq!(f32, pixel_coords.x, converted_pixel_coords.x));
    }
}
