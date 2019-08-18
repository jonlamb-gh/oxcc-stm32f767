#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMCTIMR {
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
#[doc = "Possible values of the field `TGFSCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFSCMR {
    #[doc = "Transmitted-good-single-collision half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Transmitted-good-single-collision half-full interrupt disabled"]
    MASKED,
}
impl TGFSCMR {
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
            TGFSCMR::UNMASKED => false,
            TGFSCMR::MASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TGFSCMR {
        match value {
            false => TGFSCMR::UNMASKED,
            true => TGFSCMR::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFSCMR::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == TGFSCMR::MASKED
    }
}
#[doc = "Possible values of the field `TGFMSCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFMSCMR {
    #[doc = "Transmitted-good-multiple-collision half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Transmitted-good-multiple-collision half-full interrupt disabled"]
    MASKED,
}
impl TGFMSCMR {
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
            TGFMSCMR::UNMASKED => false,
            TGFMSCMR::MASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TGFMSCMR {
        match value {
            false => TGFMSCMR::UNMASKED,
            true => TGFMSCMR::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFMSCMR::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == TGFMSCMR::MASKED
    }
}
#[doc = "Possible values of the field `TGFM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFMR {
    #[doc = "Transmitted-good counter half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Transmitted-good counter half-full interrupt disabled"]
    MASKED,
}
impl TGFMR {
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
            TGFMR::UNMASKED => false,
            TGFMR::MASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TGFMR {
        match value {
            false => TGFMR::UNMASKED,
            true => TGFMR::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFMR::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == TGFMR::MASKED
    }
}
#[doc = "Values that can be written to the field `TGFSCM`"]
pub enum TGFSCMW {
    #[doc = "Transmitted-good-single-collision half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Transmitted-good-single-collision half-full interrupt disabled"]
    MASKED,
}
impl TGFSCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TGFSCMW::UNMASKED => false,
            TGFSCMW::MASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TGFSCMW<'a> {
    w: &'a mut W,
}
impl<'a> _TGFSCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TGFSCMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmitted-good-single-collision half-full interrupt enabled"]
    #[inline]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFSCMW::UNMASKED)
    }
    #[doc = "Transmitted-good-single-collision half-full interrupt disabled"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFSCMW::MASKED)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TGFMSCM`"]
pub enum TGFMSCMW {
    #[doc = "Transmitted-good-multiple-collision half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Transmitted-good-multiple-collision half-full interrupt disabled"]
    MASKED,
}
impl TGFMSCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TGFMSCMW::UNMASKED => false,
            TGFMSCMW::MASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TGFMSCMW<'a> {
    w: &'a mut W,
}
impl<'a> _TGFMSCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TGFMSCMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmitted-good-multiple-collision half-full interrupt enabled"]
    #[inline]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFMSCMW::UNMASKED)
    }
    #[doc = "Transmitted-good-multiple-collision half-full interrupt disabled"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFMSCMW::MASKED)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TGFM`"]
pub enum TGFMW {
    #[doc = "Transmitted-good counter half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Transmitted-good counter half-full interrupt disabled"]
    MASKED,
}
impl TGFMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TGFMW::UNMASKED => false,
            TGFMW::MASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TGFMW<'a> {
    w: &'a mut W,
}
impl<'a> _TGFMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TGFMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmitted-good counter half-full interrupt enabled"]
    #[inline]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFMW::UNMASKED)
    }
    #[doc = "Transmitted-good counter half-full interrupt disabled"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFMW::MASKED)
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
    #[doc = "Bit 14 - TGFSCM"]
    #[inline]
    pub fn tgfscm(&self) -> TGFSCMR {
        TGFSCMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - TGFMSCM"]
    #[inline]
    pub fn tgfmscm(&self) -> TGFMSCMR {
        TGFMSCMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - TGFM"]
    #[inline]
    pub fn tgfm(&self) -> TGFMR {
        TGFMR::_from({
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
    #[doc = "Bit 14 - TGFSCM"]
    #[inline]
    pub fn tgfscm(&mut self) -> _TGFSCMW {
        _TGFSCMW { w: self }
    }
    #[doc = "Bit 15 - TGFMSCM"]
    #[inline]
    pub fn tgfmscm(&mut self) -> _TGFMSCMW {
        _TGFMSCMW { w: self }
    }
    #[doc = "Bit 16 - TGFM"]
    #[inline]
    pub fn tgfm(&mut self) -> _TGFMW {
        _TGFMW { w: self }
    }
}
