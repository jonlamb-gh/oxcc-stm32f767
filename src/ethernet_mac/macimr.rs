#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACIMR {
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
#[doc = "Possible values of the field `PMTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMTIMR {
    #[doc = "PMT Status interrupt generation enabled"]
    UNMASKED,
    #[doc = "PMT Status interrupt generation disabled"]
    MASKED,
}
impl PMTIMR {
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
            PMTIMR::UNMASKED => false,
            PMTIMR::MASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMTIMR {
        match value {
            false => PMTIMR::UNMASKED,
            true => PMTIMR::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline]
    pub fn is_unmasked(&self) -> bool {
        *self == PMTIMR::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == PMTIMR::MASKED
    }
}
#[doc = "Possible values of the field `TSTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTIMR {
    #[doc = "Time stamp interrupt generation enabled"]
    UNMASKED,
    #[doc = "Time stamp interrupt generation disabled"]
    MASKED,
}
impl TSTIMR {
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
            TSTIMR::UNMASKED => false,
            TSTIMR::MASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSTIMR {
        match value {
            false => TSTIMR::UNMASKED,
            true => TSTIMR::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline]
    pub fn is_unmasked(&self) -> bool {
        *self == TSTIMR::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == TSTIMR::MASKED
    }
}
#[doc = "Values that can be written to the field `PMTIM`"]
pub enum PMTIMW {
    #[doc = "PMT Status interrupt generation enabled"]
    UNMASKED,
    #[doc = "PMT Status interrupt generation disabled"]
    MASKED,
}
impl PMTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMTIMW::UNMASKED => false,
            PMTIMW::MASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMTIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMTIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PMT Status interrupt generation enabled"]
    #[inline]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(PMTIMW::UNMASKED)
    }
    #[doc = "PMT Status interrupt generation disabled"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMTIMW::MASKED)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSTIM`"]
pub enum TSTIMW {
    #[doc = "Time stamp interrupt generation enabled"]
    UNMASKED,
    #[doc = "Time stamp interrupt generation disabled"]
    MASKED,
}
impl TSTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSTIMW::UNMASKED => false,
            TSTIMW::MASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Time stamp interrupt generation enabled"]
    #[inline]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TSTIMW::UNMASKED)
    }
    #[doc = "Time stamp interrupt generation disabled"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(TSTIMW::MASKED)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 3 - PMTIM"]
    #[inline]
    pub fn pmtim(&self) -> PMTIMR {
        PMTIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - TSTIM"]
    #[inline]
    pub fn tstim(&self) -> TSTIMR {
        TSTIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 3 - PMTIM"]
    #[inline]
    pub fn pmtim(&mut self) -> _PMTIMW {
        _PMTIMW { w: self }
    }
    #[doc = "Bit 9 - TSTIM"]
    #[inline]
    pub fn tstim(&mut self) -> _TSTIMW {
        _TSTIMW { w: self }
    }
}
