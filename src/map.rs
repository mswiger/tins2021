use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::prelude::*;

static TILE_SIZE: i32 = 16;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_map);
    }
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let width = 40;
    let height = 40;
    let water_level = 0.27;
    let height_map = HeightMap::new(width, height);

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let value = height_map.get(x, y);
            let texture_handle = if value < water_level {
                asset_server.load("water.png")
            } else {
                asset_server.load("grass.png")
            };

            let q = x as f32;
            let r = y as f32 - (x as f32 /2.0).floor();

            // axial translate
            let root3 = 3.0_f32.sqrt();
            let translate_x = TILE_SIZE as f32 / 2.0 * (3.0 / 2.0 * q);
            let translate_y = TILE_SIZE as f32 / root3 * (root3 / 2.0 * q + root3 * r);
           
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(texture_handle.into()),
                transform: Transform::from_translation(Vec3::new(translate_x, translate_y, 0.0)),
                ..Default::default()
            });
        }
    }
}

pub struct HeightMap {
    noise: OpenSimplex,
    width: u32,
    height: u32,
    octave_scale: f64,
    octave_count: usize,
    octave_persistence: f64,
    map: Vec<f64>,
}

impl HeightMap {
    fn new(width: u32, height: u32) -> Self {
        let mut rng = rand::thread_rng();
        let seed: u32 = rng.gen::<u32>();
        let noise = OpenSimplex::new().set_seed(seed);
        let mut height_map = Self {
            noise,
            width,
            height,
            octave_scale: 0.025,
            octave_count: 8,
            octave_persistence: 0.015,
            map: vec![0.0; width as usize * height as usize],
        };

        height_map.generate();

        height_map
    }

    fn generate(&mut self) {
        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                let index = (y * self.width + x) as usize;
                self.map[index] = self.sum_octave(x, y) * self.get_base_value(x, y);
            }
        }
    }

    fn get(&self, x: u32, y: u32) -> f64 {
        let index = (y * self.width + x) as usize;
        self.map[index]
    }

    fn get_base_value(&self, x: u32, y: u32) -> f64 {
        let center_x = self.width as f64 / 2.0;
        let center_y = self.height as f64 / 2.0;

        let max_dist = (center_x * center_y + center_y * center_y).sqrt();

        let dist_x = center_x - x as f64;
        let dist_y = center_y - y as f64;

        return 1.0 - (dist_x * dist_x + dist_y * dist_y).sqrt() / max_dist;
    }

    fn sum_octave(&self, x: u32, y: u32) -> f64 {
        let noise_min = 0.0;
        let noise_max = 1.0;
        let mut max_amp = 0.0;
        let mut amp = 1.0;
        let mut freq = self.octave_scale;
        let mut value = 0.0;

        for _ in 0..self.octave_count {
            value += self.noise.get([x as f64 * freq, y as f64 * freq]) * amp;
            max_amp += amp;
            amp *= self.octave_persistence;
            freq *= 2.0;
        }

        // Take the average noise value of the iterations
        value /= max_amp;

        // Normalize the result
        value = value * (noise_max - noise_min) / 2.0 + (noise_max + noise_min) / 2.0;

        value
    }
}
