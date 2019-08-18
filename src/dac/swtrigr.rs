#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SWTRIGR {
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
}
#[doc = "Values that can be written to the field `SWTRIG2`"]
pub enum SWTRIG2W {
    #[doc = "DAC channel2 software trigger disabled"]
    DISABLED,
    #[doc = "DAC channel2 software trigger enabled"]
    ENABLED,
}
impl SWTRIG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWTRIG2W::DISABLED => false,
            SWTRIG2W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWTRIG2W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWTRIG2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel2 software trigger disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWTRIG2W::DISABLED)
    }
    #[doc = "DAC channel2 software trigger enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWTRIG2W::ENABLED)
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
#[doc = "Values that can be written to the field `SWTRIG1`"]
pub enum SWTRIG1W {
    #[doc = "DAC channel1 software trigger disabled"]
    DISABLED,
    #[doc = "DAC channel1 software trigger enabled"]
    ENABLED,
}
impl SWTRIG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWTRIG1W::DISABLED => false,
            SWTRIG1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWTRIG1W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWTRIG1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel1 software trigger disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWTRIG1W::DISABLED)
    }
    #[doc = "DAC channel1 software trigger enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWTRIG1W::ENABLED)
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
    #[doc = "Bit 1 - DAC channel2 software trigger"]
    #[inline]
    pub fn swtrig2(&mut self) -> _SWTRIG2W {
        _SWTRIG2W { w: self }
    }
    #[doc = "Bit 0 - DAC channel1 software trigger"]
    #[inline]
    pub fn swtrig1(&mut self) -> _SWTRIG1W {
        _SWTRIG1W { w: self }
    }
}
