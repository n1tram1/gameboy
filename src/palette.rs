use crate::lcd;

pub struct Palette {
    pub register: u8,
    colors: [lcd::Colors; 4],// HashMap<u8, LCD::Colors>,
}

impl Palette {
    pub fn new(palette_reg: u8) -> Palette {
        let color_0 = palette_reg & 0b11;
        let color_1 = (palette_reg >> 2) & 0b11;
        let color_2 = (palette_reg >> 4) & 0b11;
        let color_3 = palette_reg >> 6;

        Palette {
            register: palette_reg,
            colors: [
                Palette::to_grayshade(color_0),
                Palette::to_grayshade(color_1),
                Palette::to_grayshade(color_2),
                Palette::to_grayshade(color_3),
            ],
        }
    }

    fn to_grayshade(color: u8) -> lcd::Colors {
        match color {
            0 => lcd::Colors::White,
            1 => lcd::Colors::LightGray,
            2 => lcd::Colors::DarkGray,
            3 => lcd::Colors::Black,
            _ => panic!("Color value doesn't exists"),
        }
    }

    pub fn to_argb(&self, color: u8) -> u32 {
        /* TODO: makes this much better, I couldn't figure out another way to return the u32 held
         * in the num.
         */
        match self.colors[color as usize] {
            lcd::Colors::White => lcd::Colors::White as u32,
            lcd::Colors::LightGray => lcd::Colors::LightGray as u32,
            lcd::Colors::DarkGray => lcd::Colors::DarkGray as u32,
            lcd::Colors::Black => lcd::Colors::Black as u32,
        }
    }
}
