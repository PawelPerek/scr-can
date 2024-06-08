pub struct CRC;

impl CRC {
    pub fn calculate(crc_seq: &[usize]) -> usize {
        let mut crc_rg: usize = 0;
        let polynomial: usize = 0x4599;
    
        for byte in crc_seq {
            for bit in 0..8 {
                let next_bit = (byte >> (7 - bit)) & 1;
                let crc_next = (crc_rg >> 14) ^ next_bit;
                crc_rg = (crc_rg << 1) & 0x7FFF;
    
                if crc_next == 1 {
                    crc_rg ^= polynomial;
                }
            }
        }
    
        crc_rg
    }
}