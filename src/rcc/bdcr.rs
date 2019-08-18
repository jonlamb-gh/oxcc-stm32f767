#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BDCR {
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
#[doc = "Possible values of the field `BDRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDRSTR {
    #[doc = "Resets the entire Backup domain"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BDRSTR {
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
            BDRSTR::RESET => true,
            BDRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BDRSTR {
        match value {
            true => BDRSTR::RESET,
            i => BDRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == BDRSTR::RESET
    }
}
#[doc = "Possible values of the field `RTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCENR {
    #[doc = "RTC clock disabled"]
    DISABLED,
    #[doc = "RTC clock enabled"]
    ENABLED,
}
impl RTCENR {
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
            RTCENR::DISABLED => false,
            RTCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCENR {
        match value {
            false => RTCENR::DISABLED,
            true => RTCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RTCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RTCENR::ENABLED
    }
}
#[doc = "Possible values of the field `LSEBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEBYPR {
    #[doc = "LSE oscillator not bypassed"]
    NOTBYPASSED,
    #[doc = "LSE oscillator bypassed"]
    BYPASSED,
}
impl LSEBYPR {
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
            LSEBYPR::NOTBYPASSED => false,
            LSEBYPR::BYPASSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSEBYPR {
        match value {
            false => LSEBYPR::NOTBYPASSED,
            true => LSEBYPR::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBYPASSED`"]
    #[inline]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYPR::NOTBYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYPR::BYPASSED
    }
}
#[doc = "Possible values of the field `LSERDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDYR {
    #[doc = "LSE clock not ready"]
    NOTREADY,
    #[doc = "LSE clock ready"]
    READY,
}
impl LSERDYR {
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
            LSERDYR::NOTREADY => false,
            LSERDYR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSERDYR {
        match value {
            false => LSERDYR::NOTREADY,
            true => LSERDYR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDYR::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == LSERDYR::READY
    }
}
#[doc = "Possible values of the field `LSEON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEONR {
    #[doc = "LSE clock OFF"]
    DISABLED,
    #[doc = "LSE clock ON"]
    ENABLED,
}
impl LSEONR {
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
            LSEONR::DISABLED => false,
            LSEONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSEONR {
        match value {
            false => LSEONR::DISABLED,
            true => LSEONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LSEONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LSEONR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct LSEDRVR {
    bits: u8,
}
impl LSEDRVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RTCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSELR {
    #[doc = "No clock"]
    NOCLOCK,
    #[doc = "LSE oscillator clock used as the RTC clock"]
    LSE,
    #[doc = "LSI oscillator clock used as the RTC clock"]
    LSI,
    #[doc = "HSE oscillator clock divided by a programmable prescaler used as the RTC clock"]
    HSE,
}
impl RTCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCSELR::NOCLOCK => 0,
            RTCSELR::LSE => 0x01,
            RTCSELR::LSI => 0x02,
            RTCSELR::HSE => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCSELR {
        match value {
            0 => RTCSELR::NOCLOCK,
            1 => RTCSELR::LSE,
            2 => RTCSELR::LSI,
            3 => RTCSELR::HSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSELR::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline]
    pub fn is_lse(&self) -> bool {
        *self == RTCSELR::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSELR::LSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == RTCSELR::HSE
    }
}
#[doc = "Values that can be written to the field `BDRST`"]
pub enum BDRSTW {
    #[doc = "Resets the entire Backup domain"]
    RESET,
}
impl BDRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BDRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BDRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BDRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BDRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resets the entire Backup domain"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BDRSTW::RESET)
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
#[doc = "Values that can be written to the field `RTCEN`"]
pub enum RTCENW {
    #[doc = "RTC clock disabled"]
    DISABLED,
    #[doc = "RTC clock enabled"]
    ENABLED,
}
impl RTCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCENW::DISABLED => false,
            RTCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTC clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCENW::DISABLED)
    }
    #[doc = "RTC clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCENW::ENABLED)
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
#[doc = "Values that can be written to the field `LSEBYP`"]
pub enum LSEBYPW {
    #[doc = "LSE oscillator not bypassed"]
    NOTBYPASSED,
    #[doc = "LSE oscillator bypassed"]
    BYPASSED,
}
impl LSEBYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSEBYPW::NOTBYPASSED => false,
            LSEBYPW::BYPASSED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSEBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _LSEBYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSEBYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSE oscillator not bypassed"]
    #[inline]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LSEBYPW::NOTBYPASSED)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LSEBYPW::BYPASSED)
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
#[doc = "Values that can be written to the field `LSEON`"]
pub enum LSEONW {
    #[doc = "LSE clock OFF"]
    DISABLED,
    #[doc = "LSE clock ON"]
    ENABLED,
}
impl LSEONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSEONW::DISABLED => false,
            LSEONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSEONW<'a> {
    w: &'a mut W,
}
impl<'a> _LSEONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSEONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSE clock OFF"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSEONW::DISABLED)
    }
    #[doc = "LSE clock ON"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSEONW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _LSEDRVW<'a> {
    w: &'a mut W,
}
impl<'a> _LSEDRVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCSEL`"]
pub enum RTCSELW {
    #[doc = "No clock"]
    NOCLOCK,
    #[doc = "LSE oscillator clock used as the RTC clock"]
    LSE,
    #[doc = "LSI oscillator clock used as the RTC clock"]
    LSI,
    #[doc = "HSE oscillator clock divided by a programmable prescaler used as the RTC clock"]
    HSE,
}
impl RTCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCSELW::NOCLOCK => 0,
            RTCSELW::LSE => 1,
            RTCSELW::LSI => 2,
            RTCSELW::HSE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No clock"]
    #[inline]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSELW::NOCLOCK)
    }
    #[doc = "LSE oscillator clock used as the RTC clock"]
    #[inline]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSELW::LSE)
    }
    #[doc = "LSI oscillator clock used as the RTC clock"]
    #[inline]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSELW::LSI)
    }
    #[doc = "HSE oscillator clock divided by a programmable prescaler used as the RTC clock"]
    #[inline]
    pub fn hse(self) -> &'a mut W {
        self.variant(RTCSELW::HSE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline]
    pub fn bdrst(&self) -> BDRSTR {
        BDRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline]
    pub fn rtcen(&self) -> RTCENR {
        RTCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline]
    pub fn lsebyp(&self) -> LSEBYPR {
        LSEBYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - External low-speed oscillator ready"]
    #[inline]
    pub fn lserdy(&self) -> LSERDYR {
        LSERDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline]
    pub fn lseon(&self) -> LSEONR {
        LSEONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline]
    pub fn lsedrv(&self) -> LSEDRVR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LSEDRVR { bits }
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline]
    pub fn rtcsel(&self) -> RTCSELR {
        RTCSELR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline]
    pub fn bdrst(&mut self) -> _BDRSTW {
        _BDRSTW { w: self }
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline]
    pub fn rtcen(&mut self) -> _RTCENW {
        _RTCENW { w: self }
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline]
    pub fn lsebyp(&mut self) -> _LSEBYPW {
        _LSEBYPW { w: self }
    }
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline]
    pub fn lseon(&mut self) -> _LSEONW {
        _LSEONW { w: self }
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline]
    pub fn lsedrv(&mut self) -> _LSEDRVW {
        _LSEDRVW { w: self }
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline]
    pub fn rtcsel(&mut self) -> _RTCSELW {
        _RTCSELW { w: self }
    }
}
