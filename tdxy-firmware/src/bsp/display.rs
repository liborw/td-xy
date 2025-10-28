
use smart_leds::{RGB8, SmartLedsWrite};
use ws2812_spi::{self, Ws2812};

pub type Display8x8<SPI> = Display<SPI, 8, 8, 64>;

pub struct Display<SPI, const W: usize, const H: usize, const LEN: usize> {
    pixels: [RGB8; LEN],
    driver: Ws2812<SPI>,
    zigzag: bool,
}

impl<SPI, const W: usize, const H: usize, const LEN: usize> Display<SPI, W, H, LEN>
where
    SPI: embedded_hal::spi::SpiBus<u8>,
{
    pub fn new(spi: SPI, zigzag: bool) -> Self {
        let driver = Ws2812::new(spi);
        Self {
            pixels: [RGB8::default(); LEN],
            driver,
            zigzag,
        }
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= W || y >= H {
            return None;
        }
        let row_offset = y * W;
        let idx = if self.zigzag && (y % 2 == 1) {
            row_offset + (W - 1 - x)
        } else {
            row_offset + x
        };
        Some(idx)
    }

    pub fn set(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if let Some(i) = self.index(x, y) {
            self.pixels[i] = RGB8 { r, g, b };
        }
    }

    pub fn clear(&mut self) {
        for p in &mut self.pixels {
            *p = RGB8::default();
        }
    }

    pub fn show(&mut self) {
        let _ = self.driver.write(self.pixels.iter().cloned());
    }
}
