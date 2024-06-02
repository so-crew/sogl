pub struct Color {
    value: u32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { value:
            (u32::from(r) << 24) + 
            (u32::from(g) << 16) + 
            (u32::from(b) << 8) + 
            u32::from(a)
        }
    }

    pub fn red(&self) -> u8 {
        ((self.value & 0xFF000000) >> 24) as u8
    }

    pub fn green(&self) -> u8 {
        ((self.value & 0x00FF0000) >> 16) as u8
    }

    pub fn blue(&self) -> u8 {
        ((self.value & 0x0000FF00) >> 8) as u8
    }

    pub fn alpha(&self) -> u8 {
        (self.value & 0x000000FF) as u8
    }

    pub fn grayscale(&self) -> u8 {
        let temp: u16 = self.red() as u16 + self.green() as u16 + self.blue() as u16;
        (temp / 3) as u8
    }

    pub fn set_red(mut self, r: u8) {
        self.value = (self.value & 0x00FFFFFF) + (u32::from(r) << 24);
    }

    pub fn set_green(mut self, g: u8) {
        self.value = (self.value & 0xFF00FFFF) + (u32::from(g) << 16);
    }

    pub fn set_blue(mut self, b: u8) {
        self.value = (self.value & 0xFFFF00FF) + (u32::from(b) << 8);
    }

    pub fn set_alpha(mut self, a: u8) {
        self.value = (self.value & 0xFFFFFF00) + u32::from(a);
    }
}