#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHB3LPENR {
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
#[doc = "Possible values of the field `FMCLPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCLPENR {
    #[doc = "Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl FMCLPENR {
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
            FMCLPENR::DISABLEDINSLEEP => false,
            FMCLPENR::ENABLEDINSLEEP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FMCLPENR {
        match value {
            false => FMCLPENR::DISABLEDINSLEEP,
            true => FMCLPENR::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FMCLPENR::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FMCLPENR::ENABLEDINSLEEP
    }
}
#[doc = "Possible values of the field `QSPILPEN`"]
pub type QSPILPENR = FMCLPENR;
#[doc = "Values that can be written to the field `FMCLPEN`"]
pub enum FMCLPENW {
    #[doc = "Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl FMCLPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FMCLPENW::DISABLEDINSLEEP => false,
            FMCLPENW::ENABLEDINSLEEP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FMCLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _FMCLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FMCLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `QSPILPEN`"]
pub type QSPILPENW = FMCLPENW;
#[doc = r" Proxy"]
pub struct _QSPILPENW<'a> {
    w: &'a mut W,
}
impl<'a> _QSPILPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QSPILPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline]
    pub fn fmclpen(&self) -> FMCLPENR {
        FMCLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Quand SPI memory controller clock enable during Sleep mode"]
    #[inline]
    pub fn qspilpen(&self) -> QSPILPENR {
        QSPILPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x01 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline]
    pub fn fmclpen(&mut self) -> _FMCLPENW {
        _FMCLPENW { w: self }
    }
    #[doc = "Bit 1 - Quand SPI memory controller clock enable during Sleep mode"]
    #[inline]
    pub fn qspilpen(&mut self) -> _QSPILPENW {
        _QSPILPENW { w: self }
    }
}
