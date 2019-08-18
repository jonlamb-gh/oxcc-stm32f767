#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHB2RSTR {
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
#[doc = "Possible values of the field `OTGFSRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGFSRSTR {
    #[doc = "Reset the selected module"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OTGFSRSTR {
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
            OTGFSRSTR::RESET => true,
            OTGFSRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTGFSRSTR {
        match value {
            true => OTGFSRSTR::RESET,
            i => OTGFSRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == OTGFSRSTR::RESET
    }
}
#[doc = "Possible values of the field `RNGRST`"]
pub type RNGRSTR = OTGFSRSTR;
#[doc = "Possible values of the field `HSAHRST`"]
pub type HSAHRSTR = OTGFSRSTR;
#[doc = "Possible values of the field `CRYPRST`"]
pub type CRYPRSTR = OTGFSRSTR;
#[doc = "Possible values of the field `DCMIRST`"]
pub type DCMIRSTR = OTGFSRSTR;
#[doc = "Values that can be written to the field `OTGFSRST`"]
pub enum OTGFSRSTW {
    #[doc = "Reset the selected module"]
    RESET,
}
impl OTGFSRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTGFSRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTGFSRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGFSRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGFSRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRSTW::RESET)
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
#[doc = "Values that can be written to the field `RNGRST`"]
pub type RNGRSTW = OTGFSRSTW;
#[doc = r" Proxy"]
pub struct _RNGRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RNGRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNGRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRSTW::RESET)
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
#[doc = "Values that can be written to the field `HSAHRST`"]
pub type HSAHRSTW = OTGFSRSTW;
#[doc = r" Proxy"]
pub struct _HSAHRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HSAHRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSAHRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRSTW::RESET)
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
#[doc = "Values that can be written to the field `CRYPRST`"]
pub type CRYPRSTW = OTGFSRSTW;
#[doc = r" Proxy"]
pub struct _CRYPRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRYPRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRYPRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRSTW::RESET)
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
#[doc = "Values that can be written to the field `DCMIRST`"]
pub type DCMIRSTW = OTGFSRSTW;
#[doc = r" Proxy"]
pub struct _DCMIRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DCMIRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCMIRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRSTW::RESET)
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
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline]
    pub fn otgfsrst(&self) -> OTGFSRSTR {
        OTGFSRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Random number generator module reset"]
    #[inline]
    pub fn rngrst(&self) -> RNGRSTR {
        RNGRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Hash module reset"]
    #[inline]
    pub fn hsahrst(&self) -> HSAHRSTR {
        HSAHRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Cryptographic module reset"]
    #[inline]
    pub fn cryprst(&self) -> CRYPRSTR {
        CRYPRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Camera interface reset"]
    #[inline]
    pub fn dcmirst(&self) -> DCMIRSTR {
        DCMIRSTR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline]
    pub fn otgfsrst(&mut self) -> _OTGFSRSTW {
        _OTGFSRSTW { w: self }
    }
    #[doc = "Bit 6 - Random number generator module reset"]
    #[inline]
    pub fn rngrst(&mut self) -> _RNGRSTW {
        _RNGRSTW { w: self }
    }
    #[doc = "Bit 5 - Hash module reset"]
    #[inline]
    pub fn hsahrst(&mut self) -> _HSAHRSTW {
        _HSAHRSTW { w: self }
    }
    #[doc = "Bit 4 - Cryptographic module reset"]
    #[inline]
    pub fn cryprst(&mut self) -> _CRYPRSTW {
        _CRYPRSTW { w: self }
    }
    #[doc = "Bit 0 - Camera interface reset"]
    #[inline]
    pub fn dcmirst(&mut self) -> _DCMIRSTW {
        _DCMIRSTW { w: self }
    }
}
