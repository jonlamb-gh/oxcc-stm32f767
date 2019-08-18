#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
pub struct LPDSR {
    bits: bool,
}
impl LPDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `PDDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDDSR {
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    STOP_MODE,
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    STANDBY_MODE,
}
impl PDDSR {
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
            PDDSR::STOP_MODE => false,
            PDDSR::STANDBY_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDDSR {
        match value {
            false => PDDSR::STOP_MODE,
            true => PDDSR::STANDBY_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_MODE`"]
    #[inline]
    pub fn is_stop_mode(&self) -> bool {
        *self == PDDSR::STOP_MODE
    }
    #[doc = "Checks if the value of the field is `STANDBY_MODE`"]
    #[inline]
    pub fn is_standby_mode(&self) -> bool {
        *self == PDDSR::STANDBY_MODE
    }
}
#[doc = r" Value of the field"]
pub struct CSBFR {
    bits: bool,
}
impl CSBFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PVDER {
    bits: bool,
}
impl PVDER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PLSR {
    bits: u8,
}
impl PLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DBPR {
    bits: bool,
}
impl DBPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct FPDSR {
    bits: bool,
}
impl FPDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct LPUDSR {
    bits: bool,
}
impl LPUDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct MRUDSR {
    bits: bool,
}
impl MRUDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ADCDC1R {
    bits: bool,
}
impl ADCDC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `VOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOSR {
    #[doc = "Scale 1 mode (reset value)"]
    SCALE1,
    #[doc = "Scale 2 mode"]
    SCALE2,
    #[doc = "Scale 3 mode"]
    SCALE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VOSR::SCALE1 => 0x03,
            VOSR::SCALE2 => 0x02,
            VOSR::SCALE3 => 0x01,
            VOSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VOSR {
        match value {
            3 => VOSR::SCALE1,
            2 => VOSR::SCALE2,
            1 => VOSR::SCALE3,
            i => VOSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCALE1`"]
    #[inline]
    pub fn is_scale1(&self) -> bool {
        *self == VOSR::SCALE1
    }
    #[doc = "Checks if the value of the field is `SCALE2`"]
    #[inline]
    pub fn is_scale2(&self) -> bool {
        *self == VOSR::SCALE2
    }
    #[doc = "Checks if the value of the field is `SCALE3`"]
    #[inline]
    pub fn is_scale3(&self) -> bool {
        *self == VOSR::SCALE3
    }
}
#[doc = r" Value of the field"]
pub struct ODENR {
    bits: bool,
}
impl ODENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ODSWENR {
    bits: bool,
}
impl ODSWENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct UDENR {
    bits: u8,
}
impl UDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LPDSW<'a> {
    w: &'a mut W,
}
impl<'a> _LPDSW<'a> {
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
#[doc = "Values that can be written to the field `PDDS`"]
pub enum PDDSW {
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    STOP_MODE,
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    STANDBY_MODE,
}
impl PDDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDDSW::STOP_MODE => false,
            PDDSW::STANDBY_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDDSW<'a> {
    w: &'a mut W,
}
impl<'a> _PDDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDDSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    #[inline]
    pub fn stop_mode(self) -> &'a mut W {
        self.variant(PDDSW::STOP_MODE)
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline]
    pub fn standby_mode(self) -> &'a mut W {
        self.variant(PDDSW::STANDBY_MODE)
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
#[doc = r" Proxy"]
pub struct _CSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _CSBFW<'a> {
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
#[doc = r" Proxy"]
pub struct _PVDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PVDEW<'a> {
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
#[doc = r" Proxy"]
pub struct _PLSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DBPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBPW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FPDSW<'a> {
    w: &'a mut W,
}
impl<'a> _FPDSW<'a> {
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
#[doc = r" Proxy"]
pub struct _LPUDSW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUDSW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MRUDSW<'a> {
    w: &'a mut W,
}
impl<'a> _MRUDSW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADCDC1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCDC1W<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VOS`"]
pub enum VOSW {
    #[doc = "Scale 1 mode (reset value)"]
    SCALE1,
    #[doc = "Scale 2 mode"]
    SCALE2,
    #[doc = "Scale 3 mode"]
    SCALE3,
}
impl VOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VOSW::SCALE1 => 3,
            VOSW::SCALE2 => 2,
            VOSW::SCALE3 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VOSW<'a> {
    w: &'a mut W,
}
impl<'a> _VOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VOSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Scale 1 mode (reset value)"]
    #[inline]
    pub fn scale1(self) -> &'a mut W {
        self.variant(VOSW::SCALE1)
    }
    #[doc = "Scale 2 mode"]
    #[inline]
    pub fn scale2(self) -> &'a mut W {
        self.variant(VOSW::SCALE2)
    }
    #[doc = "Scale 3 mode"]
    #[inline]
    pub fn scale3(self) -> &'a mut W {
        self.variant(VOSW::SCALE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ODENW<'a> {
    w: &'a mut W,
}
impl<'a> _ODENW<'a> {
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
#[doc = r" Proxy"]
pub struct _ODSWENW<'a> {
    w: &'a mut W,
}
impl<'a> _ODSWENW<'a> {
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
#[doc = r" Proxy"]
pub struct _UDENW<'a> {
    w: &'a mut W,
}
impl<'a> _UDENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Low-power deep sleep"]
    #[inline]
    pub fn lpds(&self) -> LPDSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPDSR { bits }
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline]
    pub fn pdds(&self) -> PDDSR {
        PDDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline]
    pub fn csbf(&self) -> CSBFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSBFR { bits }
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline]
    pub fn pvde(&self) -> PVDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PVDER { bits }
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline]
    pub fn pls(&self) -> PLSR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLSR { bits }
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline]
    pub fn dbp(&self) -> DBPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBPR { bits }
    }
    #[doc = "Bit 9 - Flash power down in Stop mode"]
    #[inline]
    pub fn fpds(&self) -> FPDSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FPDSR { bits }
    }
    #[doc = "Bit 10 - Low-power regulator in deepsleep under-drive mode"]
    #[inline]
    pub fn lpuds(&self) -> LPUDSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPUDSR { bits }
    }
    #[doc = "Bit 11 - Main regulator in deepsleep under-drive mode"]
    #[inline]
    pub fn mruds(&self) -> MRUDSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MRUDSR { bits }
    }
    #[doc = "Bit 13 - ADCDC1"]
    #[inline]
    pub fn adcdc1(&self) -> ADCDC1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADCDC1R { bits }
    }
    #[doc = "Bits 14:15 - Regulator voltage scaling output selection"]
    #[inline]
    pub fn vos(&self) -> VOSR {
        VOSR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Over-drive enable"]
    #[inline]
    pub fn oden(&self) -> ODENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ODENR { bits }
    }
    #[doc = "Bit 17 - Over-drive switching enabled"]
    #[inline]
    pub fn odswen(&self) -> ODSWENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ODSWENR { bits }
    }
    #[doc = "Bits 18:19 - Under-drive enable in stop mode"]
    #[inline]
    pub fn uden(&self) -> UDENR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UDENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0xc000 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Low-power deep sleep"]
    #[inline]
    pub fn lpds(&mut self) -> _LPDSW {
        _LPDSW { w: self }
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline]
    pub fn pdds(&mut self) -> _PDDSW {
        _PDDSW { w: self }
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline]
    pub fn csbf(&mut self) -> _CSBFW {
        _CSBFW { w: self }
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline]
    pub fn pvde(&mut self) -> _PVDEW {
        _PVDEW { w: self }
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline]
    pub fn pls(&mut self) -> _PLSW {
        _PLSW { w: self }
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline]
    pub fn dbp(&mut self) -> _DBPW {
        _DBPW { w: self }
    }
    #[doc = "Bit 9 - Flash power down in Stop mode"]
    #[inline]
    pub fn fpds(&mut self) -> _FPDSW {
        _FPDSW { w: self }
    }
    #[doc = "Bit 10 - Low-power regulator in deepsleep under-drive mode"]
    #[inline]
    pub fn lpuds(&mut self) -> _LPUDSW {
        _LPUDSW { w: self }
    }
    #[doc = "Bit 11 - Main regulator in deepsleep under-drive mode"]
    #[inline]
    pub fn mruds(&mut self) -> _MRUDSW {
        _MRUDSW { w: self }
    }
    #[doc = "Bit 13 - ADCDC1"]
    #[inline]
    pub fn adcdc1(&mut self) -> _ADCDC1W {
        _ADCDC1W { w: self }
    }
    #[doc = "Bits 14:15 - Regulator voltage scaling output selection"]
    #[inline]
    pub fn vos(&mut self) -> _VOSW {
        _VOSW { w: self }
    }
    #[doc = "Bit 16 - Over-drive enable"]
    #[inline]
    pub fn oden(&mut self) -> _ODENW {
        _ODENW { w: self }
    }
    #[doc = "Bit 17 - Over-drive switching enabled"]
    #[inline]
    pub fn odswen(&mut self) -> _ODSWENW {
        _ODSWENW { w: self }
    }
    #[doc = "Bits 18:19 - Under-drive enable in stop mode"]
    #[inline]
    pub fn uden(&mut self) -> _UDENW {
        _UDENW { w: self }
    }
}
