#[doc = "Reader of register DKCFGR1"]
pub type R = crate::R<u32, super::DKCFGR1>;
#[doc = "Writer for register DKCFGR1"]
pub type W = crate::W<u32, super::DKCFGR1>;
#[doc = "Register DKCFGR1 `reset()`'s with value 0x2000_3000"]
impl crate::ResetValue for super::DKCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_3000
    }
}
#[doc = "Reader of field `PLLI2SDIV`"]
pub type PLLI2SDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLI2SDIV`"]
pub struct PLLI2SDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PLLSAIDIVQ`"]
pub type PLLSAIDIVQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAIDIVQ`"]
pub struct PLLSAIDIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIDIVQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PLLSAIDIVR`"]
pub type PLLSAIDIVR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAIDIVR`"]
pub struct PLLSAIDIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIDIVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SAI1SEL`"]
pub type SAI1SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAI1SEL`"]
pub struct SAI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `SAI2SEL`"]
pub type SAI2SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAI2SEL`"]
pub struct SAI2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `TIMPRE`"]
pub type TIMPRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMPRE`"]
pub struct TIMPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sdiv(&self) -> PLLI2SDIV_R {
        PLLI2SDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaidivq(&self) -> PLLSAIDIVQ_R {
        PLLSAIDIVQ_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline(always)]
    pub fn pllsaidivr(&self) -> PLLSAIDIVR_R {
        PLLSAIDIVR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sdiv(&mut self) -> PLLI2SDIV_W {
        PLLI2SDIV_W { w: self }
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaidivq(&mut self) -> PLLSAIDIVQ_W {
        PLLSAIDIVQ_W { w: self }
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline(always)]
    pub fn pllsaidivr(&mut self) -> PLLSAIDIVR_W {
        PLLSAIDIVR_W { w: self }
    }
    #[doc = "Bits 20:21 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W {
        SAI1SEL_W { w: self }
    }
    #[doc = "Bits 22:23 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W {
        SAI2SEL_W { w: self }
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W {
        TIMPRE_W { w: self }
    }
}
