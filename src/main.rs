use core::fmt;

use gfx_maths::Vec3;
use image::{DynamicImage, Rgba, GenericImage};

#[derive(Debug, Clone, PartialEq)]
struct Canvas {
    width: u32,
    height: u32,
    image: DynamicImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas { width, height, image: DynamicImage::new_rgb8(width, height) }
    }

    pub fn draw(&mut self, x: u32, y: u32, color: Color) {
        let pixel = color.to_rgba();
        self.image.put_pixel(x, y, pixel);
    }

    pub fn draw_area(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        let pixel = color.to_rgba();
        for x in x..x + width {
            for y in y..y + height {
                self.image.put_pixel(x, y, pixel);
            }
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
                    let ray = Ray::new(Vec3::new(x as f32, y as f32, 0.0), Vec3::new(0.0, 0.0, 1.0));
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
    // https://www.cs.princeton.edu/courses/archive/fall00/cs426/lectures/raycast/raycast.pdf (page 6-7)
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let l = self.center - ray.origin;
        let tca = l.dot(ray.direction);
        let d2 = l.dot(l) - tca * tca;
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }
        let t = if t0 < t1 { t0 } else { t1 };
        Some(t)
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
    scene.add(Sphere {
        center: Vec3::new(150.0, 150.0, 0.0),
        radius: 100.0,
    });
    let raytracer = Raytracer::new(scene);
    let mut canvas = Canvas::default();
    raytracer.render(&mut canvas);
    canvas.save("render.png");
}
