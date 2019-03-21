use crate::lcd;

pub enum Keys {
    Down,
    Up,
    Left,
    Right,
    Start,
    Select,
    B,
    A,
}

pub struct Joypad {
    p1: u8,
}

impl Joypad {
    pub fn new()  -> Joypad {
        Joypad {
            p1: 0,
        }
    }

    pub fn read(&self) -> u8 {
        self.p1
    }

    pub fn write(&mut self, p1: u8) {
        self.p1 = p1 & 0xF8;
    }

    pub fn do_cycle(&mut self, input_lcd: &lcd::LCD) {
        if self.is_selected_button_keys() {
            self.set_key(Keys::A, input_lcd.get_key(Keys::A));
            self.set_key(Keys::B, input_lcd.get_key(Keys::B));
            self.set_key(Keys::Select, input_lcd.get_key(Keys::Select));
            self.set_key(Keys::Start, input_lcd.get_key(Keys::Start));
        }

        if self.is_selected_direction_keys() {
            self.set_key(Keys::Down, input_lcd.get_key(Keys::Down));
            self.set_key(Keys::Up, input_lcd.get_key(Keys::Up));
            self.set_key(Keys::Left, input_lcd.get_key(Keys::Left));
            self.set_key(Keys::Right, input_lcd.get_key(Keys::Right));
        }

        eprintln!("p1: {:8b}", self.p1);
    }

    fn is_selected_button_keys(&self) -> bool {
        self.p1 & (1 << 5) == 0
    }

    fn is_selected_direction_keys(&self) -> bool {
        self.p1 & (1 << 4) == 0
    }

    fn set_key(&mut self, key: Keys, pressed: bool) {
        let bit = match key {
            Keys::Down  | Keys::Start  => 3,
            Keys::Up    | Keys::Select => 2,
            Keys::Left  | Keys::B      => 1,
            Keys::Right | Keys::A      => 0,
        };

        self.p1 &= !(1 << bit);
    }
}
