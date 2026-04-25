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

// 1. We change the first argument to accept the RawImageData type from the crate
fn get_raw_safe(raw_data: &rawloader::RawImageData, x: isize, y: isize, width: usize, height: usize) -> u16 {
    if x < 0 || x >= width as isize || y < 0 || y >= height as isize {
        return 0;
    }

    // 2. We use a 'match' to look inside the data. 
    // Most RAW files will fall into the "Integer" category.
    match raw_data {
        rawloader::RawImageData::Integer(data) => {
            let index = (y as usize * width) + x as usize;
            data[index]
        },
        _ => 0, // If the data is in a format we don't support yet, return 0
    }
}

// Our main processing function
fn demosaic(raw: &rawloader::RawImage) -> Vec<u8> {
    let width = raw.width;
    let height = raw.height;
    let mut rgb_data = vec![0u8; width * height * 3];

    for y in 0..height {
        for x in 0..width {
            let base_index = (y * width + x) * 3;
            let xi = x as isize;
            let yi = y as isize;

            if y % 2 == 0 && x % 2 == 0 {
                // CASE 1: RED PIXEL 🟥
                let r_val = get_raw_safe(&raw.data, xi, yi, width, height);
                let g_sum = get_raw_safe(&raw.data, xi, yi-1, width, height) as u32 +
                            get_raw_safe(&raw.data, xi, yi+1, width, height) as u32 +
                            get_raw_safe(&raw.data, xi-1, yi, width, height) as u32 +
                            get_raw_safe(&raw.data, xi+1, yi, width, height) as u32;
                let b_sum = get_raw_safe(&raw.data, xi-1, yi-1, width, height) as u32 +
                            get_raw_safe(&raw.data, xi+1, yi-1, width, height) as u32 +
                            get_raw_safe(&raw.data, xi-1, yi+1, width, height) as u32 +
                            get_raw_safe(&raw.data, xi+1, yi+1, width, height) as u32;

                rgb_data[base_index]     = (r_val / 64) as u8;
                rgb_data[base_index + 1] = (g_sum / (4 * 64)) as u8;
                rgb_data[base_index + 2] = (b_sum / (4 * 64)) as u8;

            } else if y % 2 != 0 && x % 2 != 0 {
                // CASE 2: BLUE PIXEL 🟦
                let b_val = get_raw_safe(&raw.data, xi, yi, width, height);
                let g_sum = get_raw_safe(&raw.data, xi, yi-1, width, height) as u32 +
                            get_raw_safe(&raw.data, xi, yi+1, width, height) as u32 +
                            get_raw_safe(&raw.data, xi-1, yi, width, height) as u32 +
                            get_raw_safe(&raw.data, xi+1, yi, width, height) as u32;
                let r_sum = get_raw_safe(&raw.data, xi-1, yi-1, width, height) as u32 +
                            get_raw_safe(&raw.data, xi+1, yi-1, width, height) as u32 +
                            get_raw_safe(&raw.data, xi-1, yi+1, width, height) as u32 +
                            get_raw_safe(&raw.data, xi+1, yi+1, width, height) as u32;

                rgb_data[base_index]     = (r_sum / (4 * 64)) as u8;
                rgb_data[base_index + 1] = (g_sum / (4 * 64)) as u8;
                rgb_data[base_index + 2] = (b_val / 64) as u8;

            } else {
                // CASE 3 & 4: GREEN PIXELS 🟩
                let g_val = get_raw_safe(&raw.data, xi, yi, width, height);
                let mut r_sum: u32 = 0;
                let mut b_sum: u32 = 0;

                if y % 2 == 0 { // Green on a Red row
                    r_sum = get_raw_safe(&raw.data, xi-1, yi, width, height) as u32 + 
                            get_raw_safe(&raw.data, xi+1, yi, width, height) as u32;
                    b_sum = get_raw_safe(&raw.data, xi, yi-1, width, height) as u32 + 
                            get_raw_safe(&raw.data, xi, yi+1, width, height) as u32;
                } else { // Green on a Blue row
                    r_sum = get_raw_safe(&raw.data, xi, yi-1, width, height) as u32 + 
                            get_raw_safe(&raw.data, xi, yi+1, width, height) as u32;
                    b_sum = get_raw_safe(&raw.data, xi-1, yi, width, height) as u32 + 
                            get_raw_safe(&raw.data, xi+1, yi, width, height) as u32;
                }

                rgb_data[base_index]     = (r_sum / (2 * 64)) as u8;
                rgb_data[base_index + 1] = (g_val / 64) as u8;
                rgb_data[base_index + 2] = (b_sum / (2 * 64)) as u8;
            }
        }
    }
    rgb_data
}

fn main() {

    let file_path = "D:\\Pictures\\A7r3\\10350614\\DSC00471.ARW";
    let output_path = "C:\\Users\\Conner\\Pictures\\Instagram\\test_image_bright.tiff";
    let raw_image = rawloader::decode_file(file_path).expect("Failed to open RAW file");

    // Run our custom demosaicing math
    let processed_data = demosaic(&raw_image);

    // Use the image crate to save our processed buffer
    image::save_buffer(
        output_path,
        &processed_data,
        raw_image.width as u32,
        raw_image.height as u32,
        image::ColorType::Rgb8,
    ).expect("Failed to save image");

    println!("RAW processing complete!");
}

