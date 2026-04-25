use std::vec;
use image::GenericImageView;

#[allow(dead_code)]
struct Pixel{
    r: u16,
    g: u16,
    b: u16,
}

struct Image{
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::new();

        for _ in 0..(width * height) {
            pixels.push(Pixel::new(0, 0, 0)); 
        }
        Image { width, height, pixels }
    }

    fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        let index = (y * self.width) + x;
        &mut self.pixels[index]
    }

    fn from_file(file_path: &str) -> Self {
        // 1. Open the image right here inside the constructor
        let img = image::open(file_path).expect("Failed to open image!");
        
        // 2. Grab the dimensions (casting to usize for our struct)
        let width = img.width() as usize;
        let height = img.height() as usize;

        // 3. Create the empty Vector
        let mut pixels: Vec<Pixel> = Vec::new();

        // 4. The Loop! (This is where you come in)
        for (x, y, rgba) in img.pixels() {
            let red = rgba[0] as u16;
            let green = rgba[1] as u16;
            let blue = rgba[2] as u16;

            pixels.push(Pixel::new(red, green, blue));
        }

        // 5. Return the built Image
        Image { width, height, pixels }
    }

    fn brighten_all(&mut self, amount: u16) {
        for pixel in self.pixels.iter_mut() {
            pixel.brighten(amount);
        }
    }

    fn save_to_file(&self, output_path: &str) {
        // 1. Create the flat buffer for the image crate
        let mut out_buffer: Vec<u8> = Vec::new();

        // 2. The Extraction Loop! (Your turn)
        for pixel in self.pixels.iter() {
            out_buffer.push(pixel.r as u8);
            out_buffer.push(pixel.g as u8);
            out_buffer.push(pixel.b as u8);
        }

        // 3. Save it to disk
        image::save_buffer(
            output_path, 
            &out_buffer, 
            self.width as u32, 
            self.height as u32, 
            image::ColorType::Rgb8
        ).expect("Failed to save image!");
    }

}

impl Pixel {
    fn new(r: u16, g:u16, b:u16) -> Self{
        Pixel { r, g, b }
    }

    fn brighten(&mut self, amount: u16) {
        self.r = self.r.saturating_add(amount);
        self.g = self.g.saturating_add(amount);
        self.b = self.b.saturating_add(amount);
    }
}

fn main() {

    let file_path = "C:\\Users\\Conner\\Pictures\\Instagram\\DJI_0173.jpg";
    let output_path = "C:\\Users\\Conner\\Pictures\\Instagram\\test_image_bright.jpg";
    let mut my_image = Image::from_file(file_path);
    
    // Brighten the entire image!
    my_image.brighten_all(50);
    my_image.save_to_file(output_path);
    println!("Image successfully edited and saved!");
}

