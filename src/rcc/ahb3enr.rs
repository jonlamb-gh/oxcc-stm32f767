#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHB3ENR {
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
#[doc = "Possible values of the field `FMCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCENR {
    #[doc = "The selected clock is disabled"]
    DISABLED,
    #[doc = "The selected clock is enabled"]
    ENABLED,
}
impl FMCENR {
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
            FMCENR::DISABLED => false,
            FMCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FMCENR {
        match value {
            false => FMCENR::DISABLED,
            true => FMCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FMCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FMCENR::ENABLED
    }
}
#[doc = "Possible values of the field `QSPIEN`"]
pub type QSPIENR = FMCENR;
#[doc = "Values that can be written to the field `FMCEN`"]
pub enum FMCENW {
    #[doc = "The selected clock is disabled"]
    DISABLED,
    #[doc = "The selected clock is enabled"]
    ENABLED,
}
impl FMCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FMCENW::DISABLED => false,
            FMCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FMCENW<'a> {
    w: &'a mut W,
}
impl<'a> _FMCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FMCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMCENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMCENW::ENABLED)
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
#[doc = "Values that can be written to the field `QSPIEN`"]
pub type QSPIENW = FMCENW;
#[doc = r" Proxy"]
pub struct _QSPIENW<'a> {
    w: &'a mut W,
}
impl<'a> _QSPIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QSPIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMCENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMCENW::ENABLED)
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
    #[doc = "Bit 0 - Flexible memory controller module clock enable"]
    #[inline]
    pub fn fmcen(&self) -> FMCENR {
        FMCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Quad SPI memory controller clock enable"]
    #[inline]
    pub fn qspien(&self) -> QSPIENR {
        QSPIENR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Flexible memory controller module clock enable"]
    #[inline]
    pub fn fmcen(&mut self) -> _FMCENW {
        _FMCENW { w: self }
    }
    #[doc = "Bit 1 - Quad SPI memory controller clock enable"]
    #[inline]
    pub fn qspien(&mut self) -> _QSPIENW {
        _QSPIENW { w: self }
    }
}
