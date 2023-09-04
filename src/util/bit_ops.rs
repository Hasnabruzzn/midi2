use crate::*;

pub trait BitOps {
    fn bit(&self, index: usize) -> bool;
    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self;
    fn nibble(&self, index: usize) -> u4;
    fn set_nibble(&mut self, index: usize, v: u4) -> &mut Self;
    fn octet(&self, index: usize) -> u8;
    fn set_octet(&mut self, index: usize, v: u8) -> &mut Self;
    fn word(&self, index: usize) -> u16;
    fn set_word(&mut self, index: usize, v: u16) -> &mut Self;
}

impl BitOps for u32 {
    fn bit(&self, index: usize) -> bool {
        assert!(index < 32);
        self >> (31 - index) & 0b1 != 0
    }

    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self {
        assert!(index < 32);
        let v = u32::from(v);
        let shift = 31 - index;
        *self &= !(0b1 << shift);
        *self |= v << shift;
        self
    }

    fn nibble(&self, index: usize) -> u4 {
        assert!(index < 8);
        ((self >> (28 - index * 4)) & 0xF).try_into().unwrap()
    }

    fn set_nibble(&mut self, index: usize, v: u4) -> &mut Self {
        assert!(index < 8);
        let shift = 28 - index * 4;
        *self &= !(0xF << shift);
        *self |= u32::from(v) << shift;
        self
    }

    fn octet(&self, index: usize) -> u8 {
        assert!(index < 4);
        (self >> (24 - index * 8) & 0xFF).try_into().unwrap()
    }

    fn set_octet(&mut self, index: usize, v: u8) -> &mut Self {
        assert!(index < 4);
        let shift = 24 - index * 8;
        *self &= !(0xFF << shift);
        *self |= (v as u32) << shift;
        self
    }

    fn word(&self, index: usize) -> u16 {
        assert!(index < 2);
        (self >> (16 - index * 16) & 0xFFFF).try_into().unwrap()
    }

    fn set_word(&mut self, index: usize, v: u16) -> &mut Self {
        assert!(index < 2);
        let shift = 16 - index * 16;
        *self &= !(0xFFFF << shift);
        *self |= (v as u32) << shift;
        self
    }
}

impl BitOps for u8 {
    fn bit(&self, index: usize) -> bool {
        assert!(index < 8);
        self >> (7 - index) & 0b1 != 0
    }
    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self {
        assert!(index < 8);
        let v = u8::from(v);
        let shift = 7 - index;
        *self &= !(0b1 << shift);
        *self |= v << shift;
        self
    }
    fn nibble(&self, index: usize) -> u4 {
        assert!(index < 2);
        ((self >> (4 - index * 4)) & 0xF).try_into().unwrap()
    }
    fn set_nibble(&mut self, index: usize, v: u4) -> &mut Self {
        assert!(index < 2);
        let shift = 4 - index * 4;
        *self &= !(0xF << shift);
        *self |= u8::from(v) << shift;
        self
    }
    fn octet(&self, _index: usize) -> u8 {
        *self
    }
    fn set_octet(&mut self, index: usize, v: u8) -> &mut Self {
        assert!(index == 0);
        *self = v;
        self
    }
    fn word(&self, _index: usize) -> u16 {
        panic!()
    }
    fn set_word(&mut self, _index: usize, _v: u16) -> &mut Self {
        panic!()
    }
}

#[cfg(test)]
mod tests_u8 {
    use super::*;

    #[test]
    fn bit() {
        let p = 0b1000_0010_u8;
        assert!(p.bit(0));
        assert!(!p.bit(1));
        assert!(p.bit(6));
        assert!(!p.bit(7));
    }

    #[test]
    fn set_bit() {
        assert_eq!(0b1000_0000_u8.set_bit(0, false), &0x0,);
        assert_eq!(0x0_u8.set_bit(0, true), &0b1000_0000,);
        assert_eq!(0x0_u8.set_bit(4, true), &0b0000_1000,);
    }

    #[test]
    fn nibble() {
        let p = 0xAB_u8;
        assert_eq!(p.nibble(0), u4::new(0xA));
        assert_eq!(p.nibble(1), u4::new(0xB));
    }

    #[test]
    fn set_nibble() {
        assert_eq!(0x1_u8.set_nibble(1, u4::new(6)), &0x6,);
        assert_eq!(0x0_u8.set_nibble(0, u4::new(0xB)), &0xB0,);
        assert_eq!(0x0_u8.set_nibble(1, u4::new(0xB)), &0x0B,);
    }

    #[test]
    fn octet() {
        let p = 0xFC_u8;
        assert_eq!(p.octet(0), 0xFC);
    }

    #[test]
    fn set_octet() {
        assert_eq!(0x1_u8.set_octet(0, 0x6), &0x6);
        assert_eq!(0x0_u8.set_octet(0, 0xBE), &0xBE);
    }
}

#[cfg(test)]
mod tests_u32 {
    use super::*;

    #[test]
    fn bit() {
        let p = 0b1000_0000_0000_0000_0000_0000_0000_0010_u32;
        assert!(p.bit(0));
        assert!(!p.bit(1));
        assert!(p.bit(30));
        assert!(!p.bit(31));
    }

    #[test]
    fn set_bit() {
        assert_eq!(
            0x0_u32.set_bit(0, true),
            &0b1000_0000_0000_0000_0000_0000_0000_0000_u32,
        );
        assert_eq!(
            0x0_u32.set_bit(10, true),
            &0b0000_0000_0010_0000_0000_0000_0000_0000,
        );
    }

    #[test]
    fn nibble() {
        let p = 0x2468_ACE0_u32;
        assert_eq!(p.nibble(0), u4::new(0x2));
        assert_eq!(p.nibble(3), u4::new(0x8));
        assert_eq!(p.nibble(5), u4::new(0xC));
        assert_eq!(p.nibble(7), u4::new(0x0));
    }

    #[test]
    fn set_nibble() {
        assert_eq!(0x5A21_C612_u32.set_nibble(3, u4::new(6)), &0x5A26_C612,);
        assert_eq!(0x0_u32.set_nibble(0, u4::new(0xB)), &0xB000_0000,);
        assert_eq!(0x0_u32.set_nibble(5, u4::new(0xB)), &0x0000_0B00,);
        assert_eq!(0x0_u32.set_nibble(7, u4::new(0x4)), &0x0000_0004,);
    }

    #[test]
    fn octet() {
        let p = 0x0123_4567_u32;
        assert_eq!(p.octet(0), 0x01);
        assert_eq!(p.octet(1), 0x23);
        assert_eq!(p.octet(2), 0x45);
        assert_eq!(p.octet(3), 0x67);
    }

    #[test]
    fn set_octet() {
        assert_eq!(0x0_u32.set_octet(0, 0xBE), &0xBE00_0000,);
        assert_eq!(0x0_u32.set_octet(2, 0xBE), &0x0000_BE00,);
        assert_eq!(0x0_u32.set_octet(3, 0xBE), &0x0000_00BE,);
    }

    #[test]
    fn word() {
        let p = 0x0123_4567_u32;
        assert_eq!(p.word(0), 0x0123);
        assert_eq!(p.word(1), 0x4567);
    }

    #[test]
    fn set_word() {
        assert_eq!(0x0_u32.set_word(0, 0x0ABE), &0x0ABE_0000,);
        assert_eq!(0x0_u32.set_word(1, 0x0ABE), &0x0000_0ABE,);
    }
}