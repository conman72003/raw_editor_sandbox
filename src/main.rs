struct Pixel{
    r: u16,
    g: u16,
    b: u16,
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
    let mut my_pixel = Pixel::new(100,200,150);
    my_pixel.brighten(50);

    println!("R: {}, G: {}, B: {}", my_pixel.r, my_pixel.g, my_pixel.b);
}
