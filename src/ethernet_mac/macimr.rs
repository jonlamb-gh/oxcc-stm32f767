#[doc = "Reader of register MACIMR"]
pub type R = crate::R<u32, super::MACIMR>;
#[doc = "Writer for register MACIMR"]
pub type W = crate::W<u32, super::MACIMR>;
#[doc = "Register MACIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PMTIM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMTIM_A {
    #[doc = "0: PMT Status interrupt generation enabled"]
    UNMASKED,
    #[doc = "1: PMT Status interrupt generation disabled"]
    MASKED,
}
impl From<PMTIM_A> for bool {
    #[inline(always)]
    fn from(variant: PMTIM_A) -> Self {
        match variant {
            PMTIM_A::UNMASKED => false,
            PMTIM_A::MASKED => true,
        }
    }
}
#[doc = "Reader of field `PMTIM`"]
pub type PMTIM_R = crate::R<bool, PMTIM_A>;
impl PMTIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMTIM_A {
        match self.bits {
            false => PMTIM_A::UNMASKED,
            true => PMTIM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == PMTIM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMTIM_A::MASKED
    }
}
#[doc = "Write proxy for field `PMTIM`"]
pub struct PMTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PMTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMTIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PMT Status interrupt generation enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(PMTIM_A::UNMASKED)
    }
    #[doc = "PMT Status interrupt generation disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMTIM_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "TSTIM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTIM_A {
    #[doc = "0: Time stamp interrupt generation enabled"]
    UNMASKED,
    #[doc = "1: Time stamp interrupt generation disabled"]
    MASKED,
}
impl From<TSTIM_A> for bool {
    #[inline(always)]
    fn from(variant: TSTIM_A) -> Self {
        match variant {
            TSTIM_A::UNMASKED => false,
            TSTIM_A::MASKED => true,
        }
    }
}
#[doc = "Reader of field `TSTIM`"]
pub type TSTIM_R = crate::R<bool, TSTIM_A>;
impl TSTIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTIM_A {
        match self.bits {
            false => TSTIM_A::UNMASKED,
            true => TSTIM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TSTIM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TSTIM_A::MASKED
    }
}
#[doc = "Write proxy for field `TSTIM`"]
pub struct TSTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Time stamp interrupt generation enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TSTIM_A::UNMASKED)
    }
    #[doc = "Time stamp interrupt generation disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TSTIM_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - PMTIM"]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSTIM"]
    #[inline(always)]
    pub fn tstim(&self) -> TSTIM_R {
        TSTIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMTIM"]
    #[inline(always)]
    pub fn pmtim(&mut self) -> PMTIM_W {
        PMTIM_W { w: self }
    }
    #[doc = "Bit 9 - TSTIM"]
    #[inline(always)]
    pub fn tstim(&mut self) -> TSTIM_W {
        TSTIM_W { w: self }
    }
}
