pub fn crc16_ccitt_false(content: &str) -> String{
    let chars = content.chars();
    let mut crc:u16 = 0xFFFF;
    chars.for_each(|c | {
        let c_u16 = c as u16;
        crc ^= c_u16<< 8;
        for _ in 0..8 {
            if crc & 0x8000 == 0{
                crc = crc << 1
            }else{
                crc = (crc << 1)^0x1021

            }
        };
    });
    format!("{:0>4X}",crc & 0xFFFF)
}