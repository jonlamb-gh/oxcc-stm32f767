#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MDIOS_DINR22 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DIN22R {
    bits: u16,
}
impl DIN22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline]
    pub fn din22(&self) -> DIN22R {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DIN22R { bits }
    }
}
