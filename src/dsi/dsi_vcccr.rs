#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DSI_VCCCR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NUMCR {
    bits: u16,
}
impl NUMCR {
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
    #[doc = "Bits 0:12 - Number of Chunks"]
    #[inline]
    pub fn numc(&self) -> NUMCR {
        let bits = {
            const MASK: u16 = 0x1fff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        NUMCR { bits }
    }
}
