const CAN_WIDTH: u8 = 15;
const CAN_POLY: u16 = 0x4599;
const CAN_INIT: u16 = 0x0000;

pub struct CRC {
    data: [u16; 256],
}

impl CRC {
    pub fn new() -> Self {
        Self {
            data: Self::generate_table(CAN_WIDTH, CAN_POLY),
        }
    }

    const fn generate_table(width: u8, poly: u16) -> [u16; 256] {
        let poly = poly << (16u8 - width);
    
        let mut table = [0u16; 256];
        let mut i = 0;
        while i < table.len() {
            table[i] = Self::crc16(poly, i as u16);
            i += 1;
        }
        table
    }

    const fn crc16(poly: u16, mut value: u16) -> u16 {
        value <<= 8;
    
        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> 15) & 1) * poly);
            i += 1;
        }
        
        value
    }

    pub fn calculate(&self, crc_seq: &[u8]) -> u16 {
        let mut crc = Self::init(CAN_INIT);
        crc = self.update(crc, crc_seq);
        Self::finalize(crc)
    }

    const fn init(initial: u16) -> u16 {
        initial << (16u8 - CAN_WIDTH)
    }

    const fn update(&self, crc: u16, bytes: &[u8]) -> u16 {
        Self::update_bytewise(crc, &self.data, bytes)
    }

    const fn update_bytewise(mut crc: u16, table: &[u16; 256], bytes: &[u8]) -> u16 {
        let mut i = 0;
        
        while i < bytes.len() {
            let table_index = (((crc >> 8) ^ bytes[i] as u16) & 0xFF) as usize;
            crc = table[table_index] ^ (crc << 8);
            i += 1;
        }
        
        crc
    }

    const fn finalize(crc: u16) -> u16 {
        crc >> 16u8 - CAN_WIDTH
    }

    // OLD

    pub fn calculate_old(crc_seq: &[usize]) -> usize {
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