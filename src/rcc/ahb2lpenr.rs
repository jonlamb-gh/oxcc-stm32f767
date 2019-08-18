#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHB2LPENR {
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
#[doc = "Possible values of the field `OTGFSLPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGFSLPENR {
    #[doc = "Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl OTGFSLPENR {
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
            OTGFSLPENR::DISABLEDINSLEEP => false,
            OTGFSLPENR::ENABLEDINSLEEP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTGFSLPENR {
        match value {
            false => OTGFSLPENR::DISABLEDINSLEEP,
            true => OTGFSLPENR::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == OTGFSLPENR::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == OTGFSLPENR::ENABLEDINSLEEP
    }
}
#[doc = "Possible values of the field `RNGLPEN`"]
pub type RNGLPENR = OTGFSLPENR;
#[doc = "Possible values of the field `HASHLPEN`"]
pub type HASHLPENR = OTGFSLPENR;
#[doc = "Possible values of the field `CRYPLPEN`"]
pub type CRYPLPENR = OTGFSLPENR;
#[doc = "Possible values of the field `DCMILPEN`"]
pub type DCMILPENR = OTGFSLPENR;
#[doc = "Values that can be written to the field `OTGFSLPEN`"]
pub enum OTGFSLPENW {
    #[doc = "Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl OTGFSLPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTGFSLPENW::DISABLEDINSLEEP => false,
            OTGFSLPENW::ENABLEDINSLEEP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTGFSLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGFSLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGFSLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RNGLPEN`"]
pub type RNGLPENW = OTGFSLPENW;
#[doc = r" Proxy"]
pub struct _RNGLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _RNGLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNGLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HASHLPEN`"]
pub type HASHLPENW = OTGFSLPENW;
#[doc = r" Proxy"]
pub struct _HASHLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _HASHLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HASHLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRYPLPEN`"]
pub type CRYPLPENW = OTGFSLPENW;
#[doc = r" Proxy"]
pub struct _CRYPLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRYPLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRYPLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCMILPEN`"]
pub type DCMILPENW = OTGFSLPENW;
#[doc = r" Proxy"]
pub struct _DCMILPENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCMILPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCMILPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPENW::ENABLEDINSLEEP)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline]
    pub fn otgfslpen(&self) -> OTGFSLPENR {
        OTGFSLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Random number generator clock enable during Sleep mode"]
    #[inline]
    pub fn rnglpen(&self) -> RNGLPENR {
        RNGLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Hash modules clock enable during Sleep mode"]
    #[inline]
    pub fn hashlpen(&self) -> HASHLPENR {
        HASHLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Cryptography modules clock enable during Sleep mode"]
    #[inline]
    pub fn cryplpen(&self) -> CRYPLPENR {
        CRYPLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Camera interface enable during Sleep mode"]
    #[inline]
    pub fn dcmilpen(&self) -> DCMILPENR {
        DCMILPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0xf1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline]
    pub fn otgfslpen(&mut self) -> _OTGFSLPENW {
        _OTGFSLPENW { w: self }
    }
    #[doc = "Bit 6 - Random number generator clock enable during Sleep mode"]
    #[inline]
    pub fn rnglpen(&mut self) -> _RNGLPENW {
        _RNGLPENW { w: self }
    }
    #[doc = "Bit 5 - Hash modules clock enable during Sleep mode"]
    #[inline]
    pub fn hashlpen(&mut self) -> _HASHLPENW {
        _HASHLPENW { w: self }
    }
    #[doc = "Bit 4 - Cryptography modules clock enable during Sleep mode"]
    #[inline]
    pub fn cryplpen(&mut self) -> _CRYPLPENW {
        _CRYPLPENW { w: self }
    }
    #[doc = "Bit 0 - Camera interface enable during Sleep mode"]
    #[inline]
    pub fn dcmilpen(&mut self) -> _DCMILPENW {
        _DCMILPENW { w: self }
    }
}
