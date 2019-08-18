#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
#[doc = "Possible values of the field `DMAUDR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDR2R {
    #[doc = "No DMA underrun error condition occurred for DAC channel2"]
    NOUNDERRUN,
    #[doc = "DMA underrun error condition occurred for DAC channel2"]
    UNDERRUN,
}
impl DMAUDR2R {
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
            DMAUDR2R::NOUNDERRUN => false,
            DMAUDR2R::UNDERRUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAUDR2R {
        match value {
            false => DMAUDR2R::NOUNDERRUN,
            true => DMAUDR2R::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOUNDERRUN`"]
    #[inline]
    pub fn is_no_underrun(&self) -> bool {
        *self == DMAUDR2R::NOUNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline]
    pub fn is_underrun(&self) -> bool {
        *self == DMAUDR2R::UNDERRUN
    }
}
#[doc = "Possible values of the field `DMAUDR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDR1R {
    #[doc = "No DMA underrun error condition occurred for DAC channel1"]
    NOUNDERRUN,
    #[doc = "DMA underrun error condition occurred for DAC channel1"]
    UNDERRUN,
}
impl DMAUDR1R {
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
            DMAUDR1R::NOUNDERRUN => false,
            DMAUDR1R::UNDERRUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAUDR1R {
        match value {
            false => DMAUDR1R::NOUNDERRUN,
            true => DMAUDR1R::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOUNDERRUN`"]
    #[inline]
    pub fn is_no_underrun(&self) -> bool {
        *self == DMAUDR1R::NOUNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline]
    pub fn is_underrun(&self) -> bool {
        *self == DMAUDR1R::UNDERRUN
    }
}
#[doc = "Values that can be written to the field `DMAUDR2`"]
pub enum DMAUDR2W {
    #[doc = "No DMA underrun error condition occurred for DAC channel2"]
    NOUNDERRUN,
    #[doc = "DMA underrun error condition occurred for DAC channel2"]
    UNDERRUN,
}
impl DMAUDR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAUDR2W::NOUNDERRUN => false,
            DMAUDR2W::UNDERRUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAUDR2W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAUDR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAUDR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel2"]
    #[inline]
    pub fn no_underrun(self) -> &'a mut W {
        self.variant(DMAUDR2W::NOUNDERRUN)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel2"]
    #[inline]
    pub fn underrun(self) -> &'a mut W {
        self.variant(DMAUDR2W::UNDERRUN)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAUDR1`"]
pub enum DMAUDR1W {
    #[doc = "No DMA underrun error condition occurred for DAC channel1"]
    NOUNDERRUN,
    #[doc = "DMA underrun error condition occurred for DAC channel1"]
    UNDERRUN,
}
impl DMAUDR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAUDR1W::NOUNDERRUN => false,
            DMAUDR1W::UNDERRUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAUDR1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAUDR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAUDR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel1"]
    #[inline]
    pub fn no_underrun(self) -> &'a mut W {
        self.variant(DMAUDR1W::NOUNDERRUN)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel1"]
    #[inline]
    pub fn underrun(self) -> &'a mut W {
        self.variant(DMAUDR1W::UNDERRUN)
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag"]
    #[inline]
    pub fn dmaudr2(&self) -> DMAUDR2R {
        DMAUDR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline]
    pub fn dmaudr1(&self) -> DMAUDR1R {
        DMAUDR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag"]
    #[inline]
    pub fn dmaudr2(&mut self) -> _DMAUDR2W {
        _DMAUDR2W { w: self }
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline]
    pub fn dmaudr1(&mut self) -> _DMAUDR1W {
        _DMAUDR1W { w: self }
    }
}
