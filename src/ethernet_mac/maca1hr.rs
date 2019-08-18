#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACA1HR {
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
pub struct MACA1HR {
    bits: u16,
}
impl MACA1HR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MBCR {
    bits: u8,
}
impl MBCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAR {
    #[doc = "This address is used for comparison with DA fields of the received frame"]
    DESTINATION,
    #[doc = "This address is used for comparison with SA fields of received frames"]
    SOURCE,
}
impl SAR {
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
            SAR::DESTINATION => false,
            SAR::SOURCE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAR {
        match value {
            false => SAR::DESTINATION,
            true => SAR::SOURCE,
        }
    }
    #[doc = "Checks if the value of the field is `DESTINATION`"]
    #[inline]
    pub fn is_destination(&self) -> bool {
        *self == SAR::DESTINATION
    }
    #[doc = "Checks if the value of the field is `SOURCE`"]
    #[inline]
    pub fn is_source(&self) -> bool {
        *self == SAR::SOURCE
    }
}
#[doc = "Possible values of the field `AE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AER {
    #[doc = "Address filters ignore this address"]
    DISABLED,
    #[doc = "Address filters use this address"]
    ENABLED,
}
impl AER {
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
            AER::DISABLED => false,
            AER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AER {
        match value {
            false => AER::DISABLED,
            true => AER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == AER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == AER::ENABLED
    }
}
#[doc = r" Proxy"]
pub struct _MACA1HW<'a> {
    w: &'a mut W,
}
impl<'a> _MACA1HW<'a> {
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
#[doc = r" Proxy"]
pub struct _MBCW<'a> {
    w: &'a mut W,
}
impl<'a> _MBCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x3f;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SA`"]
pub enum SAW {
    #[doc = "This address is used for comparison with DA fields of the received frame"]
    DESTINATION,
    #[doc = "This address is used for comparison with SA fields of received frames"]
    SOURCE,
}
impl SAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAW::DESTINATION => false,
            SAW::SOURCE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAW<'a> {
    w: &'a mut W,
}
impl<'a> _SAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This address is used for comparison with DA fields of the received frame"]
    #[inline]
    pub fn destination(self) -> &'a mut W {
        self.variant(SAW::DESTINATION)
    }
    #[doc = "This address is used for comparison with SA fields of received frames"]
    #[inline]
    pub fn source(self) -> &'a mut W {
        self.variant(SAW::SOURCE)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AE`"]
pub enum AEW {
    #[doc = "Address filters ignore this address"]
    DISABLED,
    #[doc = "Address filters use this address"]
    ENABLED,
}
impl AEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AEW::DISABLED => false,
            AEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AEW<'a> {
    w: &'a mut W,
}
impl<'a> _AEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Address filters ignore this address"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AEW::DISABLED)
    }
    #[doc = "Address filters use this address"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AEW::ENABLED)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:15 - MACA1H"]
    #[inline]
    pub fn maca1h(&self) -> MACA1HR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MACA1HR { bits }
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline]
    pub fn mbc(&self) -> MBCR {
        let bits = {
            const MASK: u8 = 0x3f;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MBCR { bits }
    }
    #[doc = "Bit 30 - SA"]
    #[inline]
    pub fn sa(&self) -> SAR {
        SAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - AE"]
    #[inline]
    pub fn ae(&self) -> AER {
        AER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0xffff }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - MACA1H"]
    #[inline]
    pub fn maca1h(&mut self) -> _MACA1HW {
        _MACA1HW { w: self }
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline]
    pub fn mbc(&mut self) -> _MBCW {
        _MBCW { w: self }
    }
    #[doc = "Bit 30 - SA"]
    #[inline]
    pub fn sa(&mut self) -> _SAW {
        _SAW { w: self }
    }
    #[doc = "Bit 31 - AE"]
    #[inline]
    pub fn ae(&mut self) -> _AEW {
        _AEW { w: self }
    }
}
