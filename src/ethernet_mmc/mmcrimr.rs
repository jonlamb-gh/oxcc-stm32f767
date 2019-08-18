#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMCRIMR {
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
#[doc = "Possible values of the field `RFCEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCEMR {
    #[doc = "Received-crc-error counter half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Received-crc-error counter half-full interrupt disabled"]
    MASKED,
}
impl RFCEMR {
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
            RFCEMR::UNMASKED => false,
            RFCEMR::MASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFCEMR {
        match value {
            false => RFCEMR::UNMASKED,
            true => RFCEMR::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline]
    pub fn is_unmasked(&self) -> bool {
        *self == RFCEMR::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == RFCEMR::MASKED
    }
}
#[doc = "Possible values of the field `RFAEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFAEMR {
    #[doc = "Received-alignment-error counter half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Received-alignment-error counter half-full interrupt disabled"]
    MASKED,
}
impl RFAEMR {
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
            RFAEMR::UNMASKED => false,
            RFAEMR::MASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFAEMR {
        match value {
            false => RFAEMR::UNMASKED,
            true => RFAEMR::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline]
    pub fn is_unmasked(&self) -> bool {
        *self == RFAEMR::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == RFAEMR::MASKED
    }
}
#[doc = "Possible values of the field `RGUFM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGUFMR {
    #[doc = "Received-good-unicast counter half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Received-good-unicast counter half-full interrupt disabled"]
    MASKED,
}
impl RGUFMR {
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
            RGUFMR::UNMASKED => false,
            RGUFMR::MASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGUFMR {
        match value {
            false => RGUFMR::UNMASKED,
            true => RGUFMR::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline]
    pub fn is_unmasked(&self) -> bool {
        *self == RGUFMR::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == RGUFMR::MASKED
    }
}
#[doc = "Values that can be written to the field `RFCEM`"]
pub enum RFCEMW {
    #[doc = "Received-crc-error counter half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Received-crc-error counter half-full interrupt disabled"]
    MASKED,
}
impl RFCEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFCEMW::UNMASKED => false,
            RFCEMW::MASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFCEMW<'a> {
    w: &'a mut W,
}
impl<'a> _RFCEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFCEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received-crc-error counter half-full interrupt enabled"]
    #[inline]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RFCEMW::UNMASKED)
    }
    #[doc = "Received-crc-error counter half-full interrupt disabled"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(RFCEMW::MASKED)
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
#[doc = "Values that can be written to the field `RFAEM`"]
pub enum RFAEMW {
    #[doc = "Received-alignment-error counter half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Received-alignment-error counter half-full interrupt disabled"]
    MASKED,
}
impl RFAEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFAEMW::UNMASKED => false,
            RFAEMW::MASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFAEMW<'a> {
    w: &'a mut W,
}
impl<'a> _RFAEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFAEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received-alignment-error counter half-full interrupt enabled"]
    #[inline]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RFAEMW::UNMASKED)
    }
    #[doc = "Received-alignment-error counter half-full interrupt disabled"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(RFAEMW::MASKED)
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
#[doc = "Values that can be written to the field `RGUFM`"]
pub enum RGUFMW {
    #[doc = "Received-good-unicast counter half-full interrupt enabled"]
    UNMASKED,
    #[doc = "Received-good-unicast counter half-full interrupt disabled"]
    MASKED,
}
impl RGUFMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGUFMW::UNMASKED => false,
            RGUFMW::MASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGUFMW<'a> {
    w: &'a mut W,
}
impl<'a> _RGUFMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGUFMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received-good-unicast counter half-full interrupt enabled"]
    #[inline]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RGUFMW::UNMASKED)
    }
    #[doc = "Received-good-unicast counter half-full interrupt disabled"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(RGUFMW::MASKED)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 5 - RFCEM"]
    #[inline]
    pub fn rfcem(&self) -> RFCEMR {
        RFCEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RFAEM"]
    #[inline]
    pub fn rfaem(&self) -> RFAEMR {
        RFAEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - RGUFM"]
    #[inline]
    pub fn rgufm(&self) -> RGUFMR {
        RGUFMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
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
    #[doc = "Bit 5 - RFCEM"]
    #[inline]
    pub fn rfcem(&mut self) -> _RFCEMW {
        _RFCEMW { w: self }
    }
    #[doc = "Bit 6 - RFAEM"]
    #[inline]
    pub fn rfaem(&mut self) -> _RFAEMW {
        _RFAEMW { w: self }
    }
    #[doc = "Bit 17 - RGUFM"]
    #[inline]
    pub fn rgufm(&mut self) -> _RGUFMW {
        _RGUFMW { w: self }
    }
}
