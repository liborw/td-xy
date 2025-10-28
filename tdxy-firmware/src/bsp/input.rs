use embedded_hal::digital::InputPin;

pub struct Encoder<A, B> {
    pin_a: A,
    pin_b: B,
    callback: Option<&'static (dyn Fn(i32) + Send + Sync)>,
}

impl<A: InputPin, B: InputPin> Encoder<A, B> {
    pub fn new(pin_a: A, pin_b: B) -> Self {
        Self { pin_a, pin_b, callback: None }
    }

    pub fn set_callback(&mut self, cb: &'static (dyn Fn(i32) + Send + Sync)) {
        self.callback = Some(cb);
    }

    pub fn poll(&mut self, last_state: &mut u8) {
        let a = self.pin_a.is_high().unwrap_or(false) as u8;
        let b = self.pin_b.is_high().unwrap_or(false) as u8;
        let state = (a << 1) | b;
        let delta = match (*last_state, state) {
            (0b00, 0b01) | (0b01, 0b11) | (0b11, 0b10) | (0b10, 0b00) => 1,
            (0b00, 0b10) | (0b10, 0b11) | (0b11, 0b01) | (0b01, 0b00) => -1,
            _ => 0,
        };
        if delta != 0 {
            if let Some(cb) = self.callback {
                cb(delta);
            }
        }
        *last_state = state;
    }
}

pub struct Button<B> {
    pin: B,
    callback: Option<&'static (dyn Fn(bool) + Send + Sync)>,
}

impl<B: InputPin> Button<B> {
    pub fn new(pin: B) -> Self {
        Self { pin, callback: None }
    }

    pub fn set_callback(&mut self, cb: &'static (dyn Fn(bool) + Send + Sync)) {
        self.callback = Some(cb);
    }

    pub fn poll(&mut self, last_state: &mut bool) {
        let pressed = self.pin.is_low().unwrap_or(false);
        if pressed != *last_state {
            if let Some(cb) = self.callback {
                cb(pressed);
            }
            *last_state = pressed;
        }
    }
}
