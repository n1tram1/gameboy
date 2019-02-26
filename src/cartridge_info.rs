pub struct CartridgeInfo {
    pub title: String,
}

impl CartridgeInfo {
    pub fn new(rom_data: &Vec<u8>) -> CartridgeInfo {
        CartridgeInfo {
            title: CartridgeInfo::extract_title(rom_data),
        }
    }

    fn extract_title(rom_data: &Vec<u8>) -> String {
        let mut title = String::with_capacity(0x143 - 0x134);

        for c in rom_data[0x134..0x143].iter() {
            title.push(*c as char);
        }

        title
    }
}
