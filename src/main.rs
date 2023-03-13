use core::fmt;

use gfx_maths::Vec3;
use image::{DynamicImage, GenericImage, Rgba};

#[derive(Debug, Clone, PartialEq)]
struct Canvas {
    width: u32,
    height: u32,
    image: DynamicImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            image: DynamicImage::new_rgb8(width, height),
        }
    }

    pub fn draw(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            panic!(
                "drawing outside of canvas: point ({}, {}) outside dimensions ({}/{})",
                x, y, self.width, self.height
            );
        }

        let pixel = color.to_rgba();
        self.image.put_pixel(x, y, pixel);
    }

    pub fn draw_area(&mut self, x: u32, y: u32, width: u32, colors: &[Color]) {
        if x + width > self.width || y + colors.len() as u32 / width > self.height {
            panic!(
                "drawing outside of canvas: drawing area ({}-{}, {}-{}) outside dimensions ({}/{})",
                x,
                width,
                y,
                colors.len() as u32 / width,
                self.width,
                self.height
            );
        }

        for (i, color) in colors.iter().enumerate() {
            let x = x + (i as u32 % width);
            let y = y + (i as u32 / width);
            self.draw(x, y, *color);
        }
    }

    pub fn save(self, filename: &str) {
        self.image.save(filename).unwrap();
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Default for Canvas {
    fn default() -> Canvas {
        Canvas::new(800, 600)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    fn to_rgba(self) -> Rgba<u8> {
        Rgba([self.red, self.green, self.blue, 0])
    }
}

#[derive(Debug, Default)]
struct Raytracer {
    scene: Scene,
}

impl Raytracer {
    pub fn new(scene: Scene) -> Raytracer {
        Raytracer { scene }
    }

    pub fn render(self, canvas: &mut Canvas) {
        let color = Color::new(0, 255, 0);
        let (width, height) = canvas.get_dimensions();
        for x in 0..width {
            for y in 0..height {
                for object in &self.scene.renderables {
                    let ray =
                        Ray::new(Vec3::new(x as f32, y as f32, 0.0), Vec3::new(0.0, 0.0, 1.0));
                    if let Some(t) = object.intersect(&ray) {
                        // println!("{} {} {:?}", x, y, object);
                        canvas.draw(x, y, color);
                    }
                }
            }
        }
    }
}

trait Renderable: fmt::Debug {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}

#[derive(Default, Debug)]
struct Scene {
    pub renderables: Vec<Box<dyn Renderable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene::default()
    }

    pub fn add<T: Renderable + 'static>(&mut self, renderable: T) {
        self.renderables.push(Box::new(renderable));
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Renderable for Sphere {
    /// Returns the distance along the ray at which the sphere is intersected
    // https://www.cs.princeton.edu/courses/archive/fall00/cs426/lectures/raycast/raycast.pdf (page 6-7)
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        // distance vector between the center of the sphere and the ray origin
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html
        let center_dir = self.center - ray.origin; // `L = O - P_0`

        // project the distance vector onto the ray direction
        // gives the distance from the viewpoint to the center of the sphere projected onto the ray
        let ray_to_center = center_dir.dot(ray.direction); // `t_ca = L . V`

        // dot product = 1 angles match, 0: angles are perpendicular, -1: angles are opposite
        // if the cosine of the angle between the ray and the center of the sphere is negative, the ray is pointing away from the sphere
        if ray_to_center < 0.0 {
            return None;
        }

        // projects the center of the sphere onto the ray, so that `d` would be the (shortest) distance between the ray and the center of the sphere
        // dot product of vector by itself is the square of it's magnitude (length)
        let proj_dist2 = center_dir.dot(center_dir) - ray_to_center.powi(2); // `d^2 = L . L - t_ca^2`
        let radius2 = self.radius.powi(2); // `r^2`

        // if the projected distance is greater than the radius, the ray misses the sphere
        if proj_dist2 > radius2 {
            return None;
        }

        // the distance between the hit point and center of the sphere projected onto the ray
        // (basically how deep the ray would have to penetrate the sphere to reach the center (halfway through))
        let penetration_halfway = (radius2 - proj_dist2).sqrt(); // `t_hc = sqrt(r^2 - d^2)`

        // both intersections: the ray enters the sphere at distance `t_0` and exits at distance `t_1`
        let hit_in = ray_to_center - penetration_halfway; // `t_0 = t_ca - t_hc`
        let hit_out = ray_to_center + penetration_halfway; // `t_1 = t_ca + t_hc`

        // if both `t0` and `t1` are negative, the ray is pointing away from the sphere (or the sphere is behind the camera/ray origin)
        if hit_in < 0.0 && hit_out < 0.0 {
            return None;
        }

        // return the closest intersection
        Some(if hit_in < hit_out { hit_in } else { hit_out })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
}

fn main() {
    let mut scene = Scene::new();

    // add a bunch of spheres
    scene.add(Sphere {
        center: Vec3::new(150.0, 150.0, 0.0),
        radius: 100.0,
    });
    scene.add(Sphere {
        center: Vec3::new(300.0, 300.0, 0.0),
        radius: 32.0,
    });
    scene.add(Sphere {
        center: Vec3::new(550.0, 450.0, 0.0),
        radius: 50.0,
    });
    scene.add(Sphere {
        center: Vec3::new(600.0, -20.0, 0.0),
        radius: 300.0,
    });

    let raytracer = Raytracer::new(scene);
    let mut canvas = Canvas::default();

    // render a nice backdrop
    let mut backdrop = vec![Color::new(0, 0, 0); (canvas.width * canvas.height) as usize];
    for x in 0..canvas.width {
        for y in 0..canvas.height {
            let t = y as f32 / canvas.height as f32;

            let color = Color::new(
                (122.0 * (1.0 - t) + 220.0 * t) as u8,
                (170.0 * (1.0 - t) + 220.0 * t) as u8,
                (255.0 * (1.0 - t) + 230.0 * t) as u8,
            );

            backdrop[(y * canvas.width + x) as usize] = color;
        }
    }
    canvas.draw_area(0, 0, canvas.width, &backdrop);

    raytracer.render(&mut canvas);
    canvas.save("render.png");
}
