#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MDIOS_WRFR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct WRFR {
    bits: u32,
}
impl WRFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Write flags for MDIO registers 0 to 31"]
    #[inline]
    pub fn wrf(&self) -> WRFR {
        let bits = {
            const MASK: u32 = 0xffff_ffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        WRFR { bits }
    }
}
