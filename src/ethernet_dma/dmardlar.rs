#[doc = "Reader of register DMARDLAR"]
pub type R = crate::R<u32, super::DMARDLAR>;
#[doc = "Writer for register DMARDLAR"]
pub type W = crate::W<u32, super::DMARDLAR>;
#[doc = "Register DMARDLAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMARDLAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRL`"]
pub type SRL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SRL`"]
pub struct SRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SRL"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SRL"]
    #[inline(always)]
    pub fn srl(&mut self) -> SRL_W {
        SRL_W { w: self }
    }
}
