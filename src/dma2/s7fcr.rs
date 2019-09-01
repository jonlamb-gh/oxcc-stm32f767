#[doc = "Reader of register S7FCR"]
pub type R = crate::R<u32, super::S7FCR>;
#[doc = "Writer for register S7FCR"]
pub type W = crate::W<u32, super::S7FCR>;
#[doc = "Register S7FCR `reset()`'s with value 0x21"]
impl crate::ResetValue for super::S7FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x21
    }
}
#[doc = "FIFO error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: FE interrupt disabled"]
    DISABLED,
    #[doc = "1: FE interrupt enabled"]
    ENABLED,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        match variant {
            FEIE_A::DISABLED => false,
            FEIE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `FEIE`"]
pub type FEIE_R = crate::R<bool, FEIE_A>;
impl FEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::DISABLED,
            true => FEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FEIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `FEIE`"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FEIE_A::DISABLED)
    }
    #[doc = "FE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FEIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "FIFO status\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_A {
    #[doc = "0: 0 < fifo_level < 1/4"]
    FIRSTQUARTER,
    #[doc = "1: 1/4 <= fifo_level < 1/2"]
    SECONDQUARTER,
    #[doc = "2: 1/2 <= fifo_level < 3/4"]
    THIRDQUARTER,
    #[doc = "3: 3/4 <= fifo_level < full"]
    FOURTHQUARTER,
    #[doc = "4: FIFO is empty"]
    EMPTY,
    #[doc = "5: FIFO is full"]
    FULL,
}
impl From<FS_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        match variant {
            FS_A::FIRSTQUARTER => 0,
            FS_A::SECONDQUARTER => 1,
            FS_A::THIRDQUARTER => 2,
            FS_A::FOURTHQUARTER => 3,
            FS_A::EMPTY => 4,
            FS_A::FULL => 5,
        }
    }
}
#[doc = "Reader of field `FS`"]
pub type FS_R = crate::R<u8, FS_A>;
impl FS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FS_A::FIRSTQUARTER),
            1 => Val(FS_A::SECONDQUARTER),
            2 => Val(FS_A::THIRDQUARTER),
            3 => Val(FS_A::FOURTHQUARTER),
            4 => Val(FS_A::EMPTY),
            5 => Val(FS_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIRSTQUARTER`"]
    #[inline(always)]
    pub fn is_first_quarter(&self) -> bool {
        *self == FS_A::FIRSTQUARTER
    }
    #[doc = "Checks if the value of the field is `SECONDQUARTER`"]
    #[inline(always)]
    pub fn is_second_quarter(&self) -> bool {
        *self == FS_A::SECONDQUARTER
    }
    #[doc = "Checks if the value of the field is `THIRDQUARTER`"]
    #[inline(always)]
    pub fn is_third_quarter(&self) -> bool {
        *self == FS_A::THIRDQUARTER
    }
    #[doc = "Checks if the value of the field is `FOURTHQUARTER`"]
    #[inline(always)]
    pub fn is_fourth_quarter(&self) -> bool {
        *self == FS_A::FOURTHQUARTER
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FS_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FS_A::FULL
    }
}
#[doc = "Direct mode disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMDIS_A {
    #[doc = "0: Direct mode is enabled"]
    ENABLED,
    #[doc = "1: Direct mode is disabled"]
    DISABLED,
}
impl From<DMDIS_A> for bool {
    #[inline(always)]
    fn from(variant: DMDIS_A) -> Self {
        match variant {
            DMDIS_A::ENABLED => false,
            DMDIS_A::DISABLED => true,
        }
    }
}
#[doc = "Reader of field `DMDIS`"]
pub type DMDIS_R = crate::R<bool, DMDIS_A>;
impl DMDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMDIS_A {
        match self.bits {
            false => DMDIS_A::ENABLED,
            true => DMDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMDIS_A::DISABLED
    }
}
#[doc = "Write proxy for field `DMDIS`"]
pub struct DMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Direct mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMDIS_A::ENABLED)
    }
    #[doc = "Direct mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMDIS_A::DISABLED)
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
#[doc = "FIFO threshold selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTH_A {
    #[doc = "0: 1/4 full FIFO"]
    QUARTER,
    #[doc = "1: 1/2 full FIFO"]
    HALF,
    #[doc = "2: 3/4 full FIFO"]
    THREEQUARTERS,
    #[doc = "3: Full FIFO"]
    FULL,
}
impl From<FTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FTH_A) -> Self {
        match variant {
            FTH_A::QUARTER => 0,
            FTH_A::HALF => 1,
            FTH_A::THREEQUARTERS => 2,
            FTH_A::FULL => 3,
        }
    }
}
#[doc = "Reader of field `FTH`"]
pub type FTH_R = crate::R<u8, FTH_A>;
impl FTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTH_A {
        match self.bits {
            0 => FTH_A::QUARTER,
            1 => FTH_A::HALF,
            2 => FTH_A::THREEQUARTERS,
            3 => FTH_A::FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `QUARTER`"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FTH_A::QUARTER
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FTH_A::HALF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERS`"]
    #[inline(always)]
    pub fn is_three_quarters(&self) -> bool {
        *self == FTH_A::THREEQUARTERS
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTH_A::FULL
    }
}
#[doc = "Write proxy for field `FTH`"]
pub struct FTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1/4 full FIFO"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER)
    }
    #[doc = "1/2 full FIFO"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FTH_A::HALF)
    }
    #[doc = "3/4 full FIFO"]
    #[inline(always)]
    pub fn three_quarters(self) -> &'a mut W {
        self.variant(FTH_A::THREEQUARTERS)
    }
    #[doc = "Full FIFO"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(FTH_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W {
        DMDIS_W { w: self }
    }
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W {
        FTH_W { w: self }
    }
}
