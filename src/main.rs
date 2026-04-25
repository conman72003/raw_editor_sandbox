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
            out_buffer.push(pixel.r.clamp(0,255) as u8);
            out_buffer.push(pixel.g.clamp(0,255) as u8);
            out_buffer.push(pixel.b.clamp(0,255) as u8);
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

    fn crop(&self, start_x: usize, start_y: usize, crop_width: usize, crop_height: usize) -> Self {
        let mut new_pixels: Vec<Pixel> = Vec::new();

        for y in 0..crop_height {
            for x in 0..crop_width {

                let original_x = start_x + x;
                let original_y = start_y + y;
                
                let original_index = (original_y * self.width) + original_x;

                let old_pixel = &self.pixels[original_index];
                new_pixels.push(Pixel::new(old_pixel.r, old_pixel.g, old_pixel.b));
            }
        }

        Image { 
            width: crop_width, 
            height: crop_height, 
            pixels: new_pixels 
        }
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

// Our main processing function
fn demosaic(raw: &rawloader::RawImage) -> Vec<u8> {
    let width = raw.width;
    let height = raw.height;
    
    // We create a new Vector to hold our finished RGB image.
    // Why do we multiply by 3? (One byte each for R, G, and B)
    let mut rgb_data = vec![0u8; width * height * 3];

    for y in 0..height {
        for x in 0..width {
            // Inside our loop...
        let base_index = (y * width + x) * 3;

        if y % 2 == 0 && x % 2 == 0 {
            // We are on a RED pixel
            let r_val = get_raw_safe(&raw.data, x as isize, y as isize, width, height);
            
            // Green is the average of the 4 "cross" neighbors
            let g_sum = get_raw_safe(&raw.data, x as isize, y as isize - 1, width, height) as u32 +
                        get_raw_safe(&raw.data, x as isize, y as isize + 1, width, height) as u32 +
                        get_raw_safe(&raw.data, x as isize - 1, y as isize, width, height) as u32 +
                        get_raw_safe(&raw.data, x as isize + 1, y as isize, width, height) as u32;
            
            // Blue is the average of the 4 "diagonal" neighbors
            let b_sum = get_raw_safe(&raw.data, x as isize - 1, y as isize - 1, width, height) as u32 +
                        get_raw_safe(&raw.data, x as isize + 1, y as isize - 1, width, height) as u32 +
                        get_raw_safe(&raw.data, x as isize - 1, y as isize + 1, width, height) as u32 +
                        get_raw_safe(&raw.data, x as isize + 1, y as isize + 1, width, height) as u32;

            // Apply scaling (dividing by 64) to get 8-bit values
            rgb_data[base_index]     = (r_val / 64) as u8;
            rgb_data[base_index + 1] = (g_sum / (4 * 64)) as u8;
            rgb_data[base_index + 2] = (b_sum / (4 * 64)) as u8;
        }
        }
    }
    
    rgb_data
}

fn main() {

    let file_path = "C:\\Users\\Conner\\Pictures\\Instagram\\DJI_0173.jpg";
    let output_path = "C:\\Users\\Conner\\Pictures\\Instagram\\test_image_bright.jpg";
    let my_image = Image::from_file(file_path);
    
    // 2. Create a new cropped version (Start X: 500, Start Y: 500, Width: 1000, Height: 1000)
    let mut cropped_image = my_image.crop(500, 500, 1000, 1000);

    // 3. (Optional) We can still edit the cropped version!
    cropped_image.brighten_all(50);

    // 4. Save it
    cropped_image.save_to_file(output_path);

    println!("Cropped image successfully saved!");
}

