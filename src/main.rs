use image::{DynamicImage, Rgba, GenericImage};

#[derive(Debug, Clone, PartialEq)]
struct Canvas {
    image: DynamicImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas { image: DynamicImage::new_rgb8(width, height) }
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
}

impl Default for Canvas {
    fn default() -> Canvas {
        Canvas::new(800, 600)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    fn to_rgba(self) -> Rgba<u8> {
        Rgba([self.red, self.green, self.blue, 0])
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Raytracer {
}

impl Raytracer {

    pub fn new() -> Raytracer {
        Raytracer {}
    }

    pub fn render(self, canvas: &mut Canvas) {
        let color = Color::new(0, 255, 0);
        canvas.draw_area(0, 0, 800, 600, color);
    }
}

fn main() {
    let raytracer = Raytracer::default();
    let mut canvas = Canvas::default();
    raytracer.render(&mut canvas);
    canvas.save("render.png");
}
