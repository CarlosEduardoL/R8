/// Represents the display of the Chip8 system.
/// The display is a 64x32 monochrome display.
///
/// # Fields
///
/// * `vram` - A 2D array of booleans representing the video RAM of the display.
/// * `updated` - Indicates whether the display has been updated. (to avoid redrawing the display when it hasn't changed)
pub struct Display {
    /// The video RAM of the display.
    vram: [[bool; crate::constants::HEIGHT]; crate::constants::WIDTH],
    /// Indicates whether the display has been updated.
    pub updated: bool,
}

impl Display {
    /// Creates a new display.
    ///
    /// # Returns
    ///
    /// * `Display` - The display created.
    pub(super) fn new() -> Self {
        Self {
            vram: [[false; crate::constants::HEIGHT]; crate::constants::WIDTH],
            updated: false,
        }
    }

    /// Clears the display.
    ///
    /// Sets all pixels to false.
    pub(super) fn clear(&mut self) {
        self.updated = true;
        self.vram = [[false; crate::constants::HEIGHT]; crate::constants::WIDTH];
    }

    /// Returns an iterator over the grid of the display.
    pub fn grid(&self) -> impl Iterator<Item = bool> + '_ {
        InvertIterator {
            display: self,
            current: (0, 0),
        }
    }

    /// Sets 8 pixels on the display.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    /// * `value` - The value to set the pixels to, represented as 8 bit-encoded pixels.
    ///
    /// # Returns
    ///
    /// * `u8` - Returns 1 if a pixel was erased, otherwise returns 0.
    pub fn set(&mut self, x: u8, mut y: u8, value: u8) -> u8 {
        self.updated = true;
        let mut result = 0;
        y %= crate::constants::HEIGHT as u8;
        let y_usize = y as usize;
        for bit_index in 0..u8::BITS as u8 {
            let x_usize = (x + bit_index) as usize % crate::constants::WIDTH;
            let pixel = (value & (0x80 >> bit_index)) != 0;
            if !(self.vram[x_usize][y_usize] ^ pixel) && !pixel {
                result = 1
            }
            self.vram[x_usize][y_usize] ^= pixel;
        }
        result
    }

    /// Returns the value of a pixel.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    ///
    /// # Returns
    ///
    /// * `bool` - The value of the pixel.
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.vram[x][y]
    }
}

struct InvertIterator<'a> {
    display: &'a Display,
    current: (usize, usize),
}

impl Iterator for InvertIterator<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.current;
        if x >= crate::constants::WIDTH {
            self.current = (0, y + 1);
        }
        let (x, y) = self.current;
        if y >= crate::constants::HEIGHT {
            return None;
        }
        let result = self.display.get(x, y);
        self.current = (x + 1, y);
        Some(result)
    }
}