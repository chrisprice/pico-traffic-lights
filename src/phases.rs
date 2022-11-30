use cortex_m::delay::Delay;

pub enum Phase {
    Red,
    StartingRedAmber,
    Green,
    LeavingAmber,
}

const ONE_SECOND_MS: u32 = 1000;

impl Phase {
    pub fn new() -> Self {
        Self::Red
    }
    pub fn next(&mut self, delay: &mut Delay) {
        match self {
            Self::Red => {
                delay.delay_ms(10 * ONE_SECOND_MS);
                *self = Self::StartingRedAmber;
            }
            Self::StartingRedAmber => {
                delay.delay_ms(2 * ONE_SECOND_MS);
                *self = Self::Green;
            }
            Self::Green => {
                delay.delay_ms(10 * ONE_SECOND_MS);
                *self = Self::LeavingAmber;
            }
            Self::LeavingAmber => {
                delay.delay_ms(3 * ONE_SECOND_MS);
                *self = Self::Red;
            }
        }
    }
}
