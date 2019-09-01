#[doc = "Reader of register AHB2LPENR"]
pub type R = crate::R<u32, super::AHB2LPENR>;
#[doc = "Writer for register AHB2LPENR"]
pub type W = crate::W<u32, super::AHB2LPENR>;
#[doc = "Register AHB2LPENR `reset()`'s with value 0xf1"]
impl crate::ResetValue for super::AHB2LPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf1
    }
}
#[doc = "USB OTG FS clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGFSLPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl From<OTGFSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFSLPEN_A) -> Self {
        match variant {
            OTGFSLPEN_A::DISABLEDINSLEEP => false,
            OTGFSLPEN_A::ENABLEDINSLEEP => true,
        }
    }
}
#[doc = "Reader of field `OTGFSLPEN`"]
pub type OTGFSLPEN_R = crate::R<bool, OTGFSLPEN_A>;
impl OTGFSLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGFSLPEN_A {
        match self.bits {
            false => OTGFSLPEN_A::DISABLEDINSLEEP,
            true => OTGFSLPEN_A::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == OTGFSLPEN_A::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == OTGFSLPEN_A::ENABLEDINSLEEP
    }
}
#[doc = "Write proxy for field `OTGFSLPEN`"]
pub struct OTGFSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGFSLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::ENABLEDINSLEEP)
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
#[doc = "Random number generator clock enable during Sleep mode"]
pub type RNGLPEN_A = OTGFSLPEN_A;
#[doc = "Reader of field `RNGLPEN`"]
pub type RNGLPEN_R = crate::R<bool, OTGFSLPEN_A>;
#[doc = "Write proxy for field `RNGLPEN`"]
pub struct RNGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Hash modules clock enable during Sleep mode"]
pub type HASHLPEN_A = OTGFSLPEN_A;
#[doc = "Reader of field `HASHLPEN`"]
pub type HASHLPEN_R = crate::R<bool, OTGFSLPEN_A>;
#[doc = "Write proxy for field `HASHLPEN`"]
pub struct HASHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASHLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Cryptography modules clock enable during Sleep mode"]
pub type CRYPLPEN_A = OTGFSLPEN_A;
#[doc = "Reader of field `CRYPLPEN`"]
pub type CRYPLPEN_R = crate::R<bool, OTGFSLPEN_A>;
#[doc = "Write proxy for field `CRYPLPEN`"]
pub struct CRYPLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Camera interface enable during Sleep mode"]
pub type DCMILPEN_A = OTGFSLPEN_A;
#[doc = "Reader of field `DCMILPEN`"]
pub type DCMILPEN_R = crate::R<bool, OTGFSLPEN_A>;
#[doc = "Write proxy for field `DCMILPEN`"]
pub struct DCMILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMILPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::ENABLEDINSLEEP)
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
impl R {
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otgfslpen(&self) -> OTGFSLPEN_R {
        OTGFSLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Random number generator clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hash modules clock enable during Sleep mode"]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Cryptography modules clock enable during Sleep mode"]
    #[inline(always)]
    pub fn cryplpen(&self) -> CRYPLPEN_R {
        CRYPLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Camera interface enable during Sleep mode"]
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otgfslpen(&mut self) -> OTGFSLPEN_W {
        OTGFSLPEN_W { w: self }
    }
    #[doc = "Bit 6 - Random number generator clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W {
        RNGLPEN_W { w: self }
    }
    #[doc = "Bit 5 - Hash modules clock enable during Sleep mode"]
    #[inline(always)]
    pub fn hashlpen(&mut self) -> HASHLPEN_W {
        HASHLPEN_W { w: self }
    }
    #[doc = "Bit 4 - Cryptography modules clock enable during Sleep mode"]
    #[inline(always)]
    pub fn cryplpen(&mut self) -> CRYPLPEN_W {
        CRYPLPEN_W { w: self }
    }
    #[doc = "Bit 0 - Camera interface enable during Sleep mode"]
    #[inline(always)]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W {
        DCMILPEN_W { w: self }
    }
}
