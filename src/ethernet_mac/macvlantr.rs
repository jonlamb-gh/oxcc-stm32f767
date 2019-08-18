#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACVLANTR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct VLANTIR {
    bits: u16,
}
impl VLANTIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `VLANTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLANTCR {
    #[doc = "Full 16 bit VLAN identifiers are used for comparison and filtering"]
    VLANTC16,
    #[doc = "12 bit VLAN identifies are used for comparison and filtering"]
    VLANTC12,
}
impl VLANTCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VLANTCR::VLANTC16 => false,
            VLANTCR::VLANTC12 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VLANTCR {
        match value {
            false => VLANTCR::VLANTC16,
            true => VLANTCR::VLANTC12,
        }
    }
    #[doc = "Checks if the value of the field is `VLANTC16`"]
    #[inline]
    pub fn is_vlantc16(&self) -> bool {
        *self == VLANTCR::VLANTC16
    }
    #[doc = "Checks if the value of the field is `VLANTC12`"]
    #[inline]
    pub fn is_vlantc12(&self) -> bool {
        *self == VLANTCR::VLANTC12
    }
}
#[doc = r" Proxy"]
pub struct _VLANTIW<'a> {
    w: &'a mut W,
}
impl<'a> _VLANTIW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0xffff;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VLANTC`"]
pub enum VLANTCW {
    #[doc = "Full 16 bit VLAN identifiers are used for comparison and filtering"]
    VLANTC16,
    #[doc = "12 bit VLAN identifies are used for comparison and filtering"]
    VLANTC12,
}
impl VLANTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VLANTCW::VLANTC16 => false,
            VLANTCW::VLANTC12 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLANTCW<'a> {
    w: &'a mut W,
}
impl<'a> _VLANTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLANTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full 16 bit VLAN identifiers are used for comparison and filtering"]
    #[inline]
    pub fn vlantc16(self) -> &'a mut W {
        self.variant(VLANTCW::VLANTC16)
    }
    #[doc = "12 bit VLAN identifies are used for comparison and filtering"]
    #[inline]
    pub fn vlantc12(self) -> &'a mut W {
        self.variant(VLANTCW::VLANTC12)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - VLANTI"]
    #[inline]
    pub fn vlanti(&self) -> VLANTIR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VLANTIR { bits }
    }
    #[doc = "Bit 16 - VLANTC"]
    #[inline]
    pub fn vlantc(&self) -> VLANTCR {
        VLANTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - VLANTI"]
    #[inline]
    pub fn vlanti(&mut self) -> _VLANTIW {
        _VLANTIW { w: self }
    }
    #[doc = "Bit 16 - VLANTC"]
    #[inline]
    pub fn vlantc(&mut self) -> _VLANTCW {
        _VLANTCW { w: self }
    }
}
