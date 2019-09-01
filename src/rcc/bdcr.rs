#[doc = "Reader of register BDCR"]
pub type R = crate::R<u32, super::BDCR>;
#[doc = "Writer for register BDCR"]
pub type W = crate::W<u32, super::BDCR>;
#[doc = "Register BDCR `reset()`'s with value 0"]
impl crate::ResetValue for super::BDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Backup domain software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDRST_A {
    #[doc = "1: Resets the entire Backup domain"]
    RESET,
}
impl From<BDRST_A> for bool {
    #[inline(always)]
    fn from(variant: BDRST_A) -> Self {
        match variant {
            BDRST_A::RESET => true,
        }
    }
}
#[doc = "Reader of field `BDRST`"]
pub type BDRST_R = crate::R<bool, BDRST_A>;
impl BDRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, BDRST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BDRST_A::RESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BDRST_A::RESET
    }
}
#[doc = "Write proxy for field `BDRST`"]
pub struct BDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> BDRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resets the entire Backup domain"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BDRST_A::RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "RTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEN_A {
    #[doc = "0: RTC clock disabled"]
    DISABLED,
    #[doc = "1: RTC clock enabled"]
    ENABLED,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        match variant {
            RTCEN_A::DISABLED => false,
            RTCEN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `RTCEN`"]
pub type RTCEN_R = crate::R<bool, RTCEN_A>;
impl RTCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::DISABLED,
            true => RTCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `RTCEN`"]
pub struct RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::DISABLED)
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCEN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "External low-speed oscillator bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEBYP_A {
    #[doc = "0: LSE oscillator not bypassed"]
    NOTBYPASSED,
    #[doc = "1: LSE oscillator bypassed"]
    BYPASSED,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        match variant {
            LSEBYP_A::NOTBYPASSED => false,
            LSEBYP_A::BYPASSED => true,
        }
    }
}
#[doc = "Reader of field `LSEBYP`"]
pub type LSEBYP_R = crate::R<bool, LSEBYP_A>;
impl LSEBYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::NOTBYPASSED,
            true => LSEBYP_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBYPASSED`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYP_A::NOTBYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYP_A::BYPASSED
    }
}
#[doc = "Write proxy for field `LSEBYP`"]
pub struct LSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::NOTBYPASSED)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::BYPASSED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "External low-speed oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDY_A {
    #[doc = "0: LSE clock not ready"]
    NOTREADY,
    #[doc = "1: LSE clock ready"]
    READY,
}
impl From<LSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDY_A) -> Self {
        match variant {
            LSERDY_A::NOTREADY => false,
            LSERDY_A::READY => true,
        }
    }
}
#[doc = "Reader of field `LSERDY`"]
pub type LSERDY_R = crate::R<bool, LSERDY_A>;
impl LSERDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDY_A {
        match self.bits {
            false => LSERDY_A::NOTREADY,
            true => LSERDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDY_A::READY
    }
}
#[doc = "External low-speed oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEON_A {
    #[doc = "0: LSE clock OFF"]
    DISABLED,
    #[doc = "1: LSE clock ON"]
    ENABLED,
}
impl From<LSEON_A> for bool {
    #[inline(always)]
    fn from(variant: LSEON_A) -> Self {
        match variant {
            LSEON_A::DISABLED => false,
            LSEON_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `LSEON`"]
pub type LSEON_R = crate::R<bool, LSEON_A>;
impl LSEON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEON_A {
        match self.bits {
            false => LSEON_A::DISABLED,
            true => LSEON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSEON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSEON_A::ENABLED
    }
}
#[doc = "Write proxy for field `LSEON`"]
pub struct LSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSE clock OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSEON_A::DISABLED)
    }
    #[doc = "LSE clock ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSEON_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `LSEDRV`"]
pub type LSEDRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSEDRV`"]
pub struct LSEDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "RTC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSEL_A {
    #[doc = "0: No clock"]
    NOCLOCK,
    #[doc = "1: LSE oscillator clock used as the RTC clock"]
    LSE,
    #[doc = "2: LSI oscillator clock used as the RTC clock"]
    LSI,
    #[doc = "3: HSE oscillator clock divided by a programmable prescaler used as the RTC clock"]
    HSE,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        match variant {
            RTCSEL_A::NOCLOCK => 0,
            RTCSEL_A::LSE => 1,
            RTCSEL_A::LSI => 2,
            RTCSEL_A::HSE => 3,
        }
    }
}
#[doc = "Reader of field `RTCSEL`"]
pub type RTCSEL_R = crate::R<u8, RTCSEL_A>;
impl RTCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            0 => RTCSEL_A::NOCLOCK,
            1 => RTCSEL_A::LSE,
            2 => RTCSEL_A::LSI,
            3 => RTCSEL_A::HSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL_A::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSEL_A::HSE
    }
}
#[doc = "Write proxy for field `RTCSEL`"]
pub struct RTCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSEL_A::NOCLOCK)
    }
    #[doc = "LSE oscillator clock used as the RTC clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSEL_A::LSE)
    }
    #[doc = "LSI oscillator clock used as the RTC clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSEL_A::LSI)
    }
    #[doc = "HSE oscillator clock divided by a programmable prescaler used as the RTC clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(RTCSEL_A::HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W {
        BDRST_W { w: self }
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W { w: self }
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W {
        LSEBYP_W { w: self }
    }
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W {
        LSEON_W { w: self }
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W {
        LSEDRV_W { w: self }
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W {
        RTCSEL_W { w: self }
    }
}
