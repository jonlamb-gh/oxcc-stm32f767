#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::S7FCR {
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
#[doc = "Possible values of the field `FEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIER {
    #[doc = "FE interrupt disabled"]
    DISABLED,
    #[doc = "FE interrupt enabled"]
    ENABLED,
}
impl FEIER {
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
            FEIER::DISABLED => false,
            FEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEIER {
        match value {
            false => FEIER::DISABLED,
            true => FEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FEIER::ENABLED
    }
}
#[doc = "Possible values of the field `FS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSR {
    #[doc = "0 < fifo_level < 1/4"]
    FIRSTQUARTER,
    #[doc = "1/4 <= fifo_level < 1/2"]
    SECONDQUARTER,
    #[doc = "1/2 <= fifo_level < 3/4"]
    THIRDQUARTER,
    #[doc = "3/4 <= fifo_level < full"]
    FOURTHQUARTER,
    #[doc = "FIFO is empty"]
    EMPTY,
    #[doc = "FIFO is full"]
    FULL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSR::FIRSTQUARTER => 0,
            FSR::SECONDQUARTER => 0x01,
            FSR::THIRDQUARTER => 0x02,
            FSR::FOURTHQUARTER => 0x03,
            FSR::EMPTY => 0x04,
            FSR::FULL => 0x05,
            FSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSR {
        match value {
            0 => FSR::FIRSTQUARTER,
            1 => FSR::SECONDQUARTER,
            2 => FSR::THIRDQUARTER,
            3 => FSR::FOURTHQUARTER,
            4 => FSR::EMPTY,
            5 => FSR::FULL,
            i => FSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIRSTQUARTER`"]
    #[inline]
    pub fn is_first_quarter(&self) -> bool {
        *self == FSR::FIRSTQUARTER
    }
    #[doc = "Checks if the value of the field is `SECONDQUARTER`"]
    #[inline]
    pub fn is_second_quarter(&self) -> bool {
        *self == FSR::SECONDQUARTER
    }
    #[doc = "Checks if the value of the field is `THIRDQUARTER`"]
    #[inline]
    pub fn is_third_quarter(&self) -> bool {
        *self == FSR::THIRDQUARTER
    }
    #[doc = "Checks if the value of the field is `FOURTHQUARTER`"]
    #[inline]
    pub fn is_fourth_quarter(&self) -> bool {
        *self == FSR::FOURTHQUARTER
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == FSR::EMPTY
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == FSR::FULL
    }
}
#[doc = "Possible values of the field `DMDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMDISR {
    #[doc = "Direct mode is enabled"]
    ENABLED,
    #[doc = "Direct mode is disabled"]
    DISABLED,
}
impl DMDISR {
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
            DMDISR::ENABLED => false,
            DMDISR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMDISR {
        match value {
            false => DMDISR::ENABLED,
            true => DMDISR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMDISR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMDISR::DISABLED
    }
}
#[doc = "Possible values of the field `FTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTHR {
    #[doc = "1/4 full FIFO"]
    QUARTER,
    #[doc = "1/2 full FIFO"]
    HALF,
    #[doc = "3/4 full FIFO"]
    THREEQUARTERS,
    #[doc = "Full FIFO"]
    FULL,
}
impl FTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTHR::QUARTER => 0,
            FTHR::HALF => 0x01,
            FTHR::THREEQUARTERS => 0x02,
            FTHR::FULL => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTHR {
        match value {
            0 => FTHR::QUARTER,
            1 => FTHR::HALF,
            2 => FTHR::THREEQUARTERS,
            3 => FTHR::FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `QUARTER`"]
    #[inline]
    pub fn is_quarter(&self) -> bool {
        *self == FTHR::QUARTER
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == FTHR::HALF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERS`"]
    #[inline]
    pub fn is_three_quarters(&self) -> bool {
        *self == FTHR::THREEQUARTERS
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == FTHR::FULL
    }
}
#[doc = "Values that can be written to the field `FEIE`"]
pub enum FEIEW {
    #[doc = "FE interrupt disabled"]
    DISABLED,
    #[doc = "FE interrupt enabled"]
    ENABLED,
}
impl FEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEIEW::DISABLED => false,
            FEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FE interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FEIEW::DISABLED)
    }
    #[doc = "FE interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `DMDIS`"]
pub enum DMDISW {
    #[doc = "Direct mode is enabled"]
    ENABLED,
    #[doc = "Direct mode is disabled"]
    DISABLED,
}
impl DMDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMDISW::ENABLED => false,
            DMDISW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMDISW<'a> {
    w: &'a mut W,
}
impl<'a> _DMDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Direct mode is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMDISW::ENABLED)
    }
    #[doc = "Direct mode is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMDISW::DISABLED)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTH`"]
pub enum FTHW {
    #[doc = "1/4 full FIFO"]
    QUARTER,
    #[doc = "1/2 full FIFO"]
    HALF,
    #[doc = "3/4 full FIFO"]
    THREEQUARTERS,
    #[doc = "Full FIFO"]
    FULL,
}
impl FTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTHW::QUARTER => 0,
            FTHW::HALF => 1,
            FTHW::THREEQUARTERS => 2,
            FTHW::FULL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTHW<'a> {
    w: &'a mut W,
}
impl<'a> _FTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1/4 full FIFO"]
    #[inline]
    pub fn quarter(self) -> &'a mut W {
        self.variant(FTHW::QUARTER)
    }
    #[doc = "1/2 full FIFO"]
    #[inline]
    pub fn half(self) -> &'a mut W {
        self.variant(FTHW::HALF)
    }
    #[doc = "3/4 full FIFO"]
    #[inline]
    pub fn three_quarters(self) -> &'a mut W {
        self.variant(FTHW::THREEQUARTERS)
    }
    #[doc = "Full FIFO"]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(FTHW::FULL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
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
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline]
    pub fn feie(&self) -> FEIER {
        FEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline]
    pub fn fs(&self) -> FSR {
        FSR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline]
    pub fn dmdis(&self) -> DMDISR {
        DMDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline]
    pub fn fth(&self) -> FTHR {
        FTHR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x21 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline]
    pub fn feie(&mut self) -> _FEIEW {
        _FEIEW { w: self }
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline]
    pub fn dmdis(&mut self) -> _DMDISW {
        _DMDISW { w: self }
    }
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline]
    pub fn fth(&mut self) -> _FTHW {
        _FTHW { w: self }
    }
}
