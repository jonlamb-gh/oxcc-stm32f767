#[doc = "Reader of register MMCTIMR"]
pub type R = crate::R<u32, super::MMCTIMR>;
#[doc = "Writer for register MMCTIMR"]
pub type W = crate::W<u32, super::MMCTIMR>;
#[doc = "Register MMCTIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MMCTIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TGFSCM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFSCM_A {
    #[doc = "0: Transmitted-good-single-collision half-full interrupt enabled"]
    UNMASKED,
    #[doc = "1: Transmitted-good-single-collision half-full interrupt disabled"]
    MASKED,
}
impl From<TGFSCM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFSCM_A) -> Self {
        match variant {
            TGFSCM_A::UNMASKED => false,
            TGFSCM_A::MASKED => true,
        }
    }
}
#[doc = "Reader of field `TGFSCM`"]
pub type TGFSCM_R = crate::R<bool, TGFSCM_A>;
impl TGFSCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGFSCM_A {
        match self.bits {
            false => TGFSCM_A::UNMASKED,
            true => TGFSCM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFSCM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFSCM_A::MASKED
    }
}
#[doc = "Write proxy for field `TGFSCM`"]
pub struct TGFSCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFSCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGFSCM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmitted-good-single-collision half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFSCM_A::UNMASKED)
    }
    #[doc = "Transmitted-good-single-collision half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFSCM_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "TGFMSCM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFMSCM_A {
    #[doc = "0: Transmitted-good-multiple-collision half-full interrupt enabled"]
    UNMASKED,
    #[doc = "1: Transmitted-good-multiple-collision half-full interrupt disabled"]
    MASKED,
}
impl From<TGFMSCM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFMSCM_A) -> Self {
        match variant {
            TGFMSCM_A::UNMASKED => false,
            TGFMSCM_A::MASKED => true,
        }
    }
}
#[doc = "Reader of field `TGFMSCM`"]
pub type TGFMSCM_R = crate::R<bool, TGFMSCM_A>;
impl TGFMSCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGFMSCM_A {
        match self.bits {
            false => TGFMSCM_A::UNMASKED,
            true => TGFMSCM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFMSCM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFMSCM_A::MASKED
    }
}
#[doc = "Write proxy for field `TGFMSCM`"]
pub struct TGFMSCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFMSCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGFMSCM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmitted-good-multiple-collision half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFMSCM_A::UNMASKED)
    }
    #[doc = "Transmitted-good-multiple-collision half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFMSCM_A::MASKED)
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
#[doc = "TGFM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFM_A {
    #[doc = "0: Transmitted-good counter half-full interrupt enabled"]
    UNMASKED,
    #[doc = "1: Transmitted-good counter half-full interrupt disabled"]
    MASKED,
}
impl From<TGFM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFM_A) -> Self {
        match variant {
            TGFM_A::UNMASKED => false,
            TGFM_A::MASKED => true,
        }
    }
}
#[doc = "Reader of field `TGFM`"]
pub type TGFM_R = crate::R<bool, TGFM_A>;
impl TGFM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGFM_A {
        match self.bits {
            false => TGFM_A::UNMASKED,
            true => TGFM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFM_A::MASKED
    }
}
#[doc = "Write proxy for field `TGFM`"]
pub struct TGFM_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGFM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmitted-good counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFM_A::UNMASKED)
    }
    #[doc = "Transmitted-good counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFM_A::MASKED)
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
impl R {
    #[doc = "Bit 14 - TGFSCM"]
    #[inline(always)]
    pub fn tgfscm(&self) -> TGFSCM_R {
        TGFSCM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TGFMSCM"]
    #[inline(always)]
    pub fn tgfmscm(&self) -> TGFMSCM_R {
        TGFMSCM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TGFM"]
    #[inline(always)]
    pub fn tgfm(&self) -> TGFM_R {
        TGFM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - TGFSCM"]
    #[inline(always)]
    pub fn tgfscm(&mut self) -> TGFSCM_W {
        TGFSCM_W { w: self }
    }
    #[doc = "Bit 15 - TGFMSCM"]
    #[inline(always)]
    pub fn tgfmscm(&mut self) -> TGFMSCM_W {
        TGFMSCM_W { w: self }
    }
    #[doc = "Bit 16 - TGFM"]
    #[inline(always)]
    pub fn tgfm(&mut self) -> TGFM_W {
        TGFM_W { w: self }
    }
}
