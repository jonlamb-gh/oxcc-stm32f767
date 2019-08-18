#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHB1RSTR {
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
#[doc = "Possible values of the field `OTGHSRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGHSRSTR {
    #[doc = "Reset the selected module"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OTGHSRSTR {
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
            OTGHSRSTR::RESET => true,
            OTGHSRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTGHSRSTR {
        match value {
            true => OTGHSRSTR::RESET,
            i => OTGHSRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == OTGHSRSTR::RESET
    }
}
#[doc = "Possible values of the field `ETHMACRST`"]
pub type ETHMACRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `DMA2DRST`"]
pub type DMA2DRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `DMA2RST`"]
pub type DMA2RSTR = OTGHSRSTR;
#[doc = "Possible values of the field `DMA1RST`"]
pub type DMA1RSTR = OTGHSRSTR;
#[doc = "Possible values of the field `CRCRST`"]
pub type CRCRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIOKRST`"]
pub type GPIOKRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIOJRST`"]
pub type GPIOJRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIOIRST`"]
pub type GPIOIRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIOHRST`"]
pub type GPIOHRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIOGRST`"]
pub type GPIOGRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIOFRST`"]
pub type GPIOFRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIOERST`"]
pub type GPIOERSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIODRST`"]
pub type GPIODRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIOCRST`"]
pub type GPIOCRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIOBRST`"]
pub type GPIOBRSTR = OTGHSRSTR;
#[doc = "Possible values of the field `GPIOARST`"]
pub type GPIOARSTR = OTGHSRSTR;
#[doc = "Values that can be written to the field `OTGHSRST`"]
pub enum OTGHSRSTW {
    #[doc = "Reset the selected module"]
    RESET,
}
impl OTGHSRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTGHSRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTGHSRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGHSRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGHSRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
#[doc = "Values that can be written to the field `ETHMACRST`"]
pub type ETHMACRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _ETHMACRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHMACRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHMACRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA2DRST`"]
pub type DMA2DRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _DMA2DRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2DRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA2DRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA2RST`"]
pub type DMA2RSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _DMA2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA1RST`"]
pub type DMA1RSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _DMA1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRCRST`"]
pub type CRCRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _CRCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOKRST`"]
pub type GPIOKRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIOKRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOKRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOKRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOJRST`"]
pub type GPIOJRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIOJRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOJRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOJRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOIRST`"]
pub type GPIOIRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIOIRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOIRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOIRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOHRST`"]
pub type GPIOHRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIOHRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOHRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOHRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOGRST`"]
pub type GPIOGRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIOGRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOGRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOGRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOFRST`"]
pub type GPIOFRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIOFRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOFRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOFRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOERST`"]
pub type GPIOERSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIOERSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOERSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOERSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIODRST`"]
pub type GPIODRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIODRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIODRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIODRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOCRST`"]
pub type GPIOCRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIOCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOCRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
#[doc = "Values that can be written to the field `GPIOBRST`"]
pub type GPIOBRSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIOBRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOBRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOBRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
#[doc = "Values that can be written to the field `GPIOARST`"]
pub type GPIOARSTW = OTGHSRSTW;
#[doc = r" Proxy"]
pub struct _GPIOARSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOARSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOARSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGHSRSTW::RESET)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 29 - USB OTG HS module reset"]
    #[inline]
    pub fn otghsrst(&self) -> OTGHSRSTR {
        OTGHSRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Ethernet MAC reset"]
    #[inline]
    pub fn ethmacrst(&self) -> ETHMACRSTR {
        ETHMACRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - DMA2D reset"]
    #[inline]
    pub fn dma2drst(&self) -> DMA2DRSTR {
        DMA2DRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - DMA2 reset"]
    #[inline]
    pub fn dma2rst(&self) -> DMA2RSTR {
        DMA2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - DMA2 reset"]
    #[inline]
    pub fn dma1rst(&self) -> DMA1RSTR {
        DMA1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline]
    pub fn crcrst(&self) -> CRCRSTR {
        CRCRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - IO port K reset"]
    #[inline]
    pub fn gpiokrst(&self) -> GPIOKRSTR {
        GPIOKRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - IO port J reset"]
    #[inline]
    pub fn gpiojrst(&self) -> GPIOJRSTR {
        GPIOJRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - IO port I reset"]
    #[inline]
    pub fn gpioirst(&self) -> GPIOIRSTR {
        GPIOIRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline]
    pub fn gpiohrst(&self) -> GPIOHRSTR {
        GPIOHRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline]
    pub fn gpiogrst(&self) -> GPIOGRSTR {
        GPIOGRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline]
    pub fn gpiofrst(&self) -> GPIOFRSTR {
        GPIOFRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline]
    pub fn gpioerst(&self) -> GPIOERSTR {
        GPIOERSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline]
    pub fn gpiodrst(&self) -> GPIODRSTR {
        GPIODRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline]
    pub fn gpiocrst(&self) -> GPIOCRSTR {
        GPIOCRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline]
    pub fn gpiobrst(&self) -> GPIOBRSTR {
        GPIOBRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - IO port A reset"]
    #[inline]
    pub fn gpioarst(&self) -> GPIOARSTR {
        GPIOARSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 29 - USB OTG HS module reset"]
    #[inline]
    pub fn otghsrst(&mut self) -> _OTGHSRSTW {
        _OTGHSRSTW { w: self }
    }
    #[doc = "Bit 25 - Ethernet MAC reset"]
    #[inline]
    pub fn ethmacrst(&mut self) -> _ETHMACRSTW {
        _ETHMACRSTW { w: self }
    }
    #[doc = "Bit 23 - DMA2D reset"]
    #[inline]
    pub fn dma2drst(&mut self) -> _DMA2DRSTW {
        _DMA2DRSTW { w: self }
    }
    #[doc = "Bit 22 - DMA2 reset"]
    #[inline]
    pub fn dma2rst(&mut self) -> _DMA2RSTW {
        _DMA2RSTW { w: self }
    }
    #[doc = "Bit 21 - DMA2 reset"]
    #[inline]
    pub fn dma1rst(&mut self) -> _DMA1RSTW {
        _DMA1RSTW { w: self }
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline]
    pub fn crcrst(&mut self) -> _CRCRSTW {
        _CRCRSTW { w: self }
    }
    #[doc = "Bit 10 - IO port K reset"]
    #[inline]
    pub fn gpiokrst(&mut self) -> _GPIOKRSTW {
        _GPIOKRSTW { w: self }
    }
    #[doc = "Bit 9 - IO port J reset"]
    #[inline]
    pub fn gpiojrst(&mut self) -> _GPIOJRSTW {
        _GPIOJRSTW { w: self }
    }
    #[doc = "Bit 8 - IO port I reset"]
    #[inline]
    pub fn gpioirst(&mut self) -> _GPIOIRSTW {
        _GPIOIRSTW { w: self }
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline]
    pub fn gpiohrst(&mut self) -> _GPIOHRSTW {
        _GPIOHRSTW { w: self }
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline]
    pub fn gpiogrst(&mut self) -> _GPIOGRSTW {
        _GPIOGRSTW { w: self }
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline]
    pub fn gpiofrst(&mut self) -> _GPIOFRSTW {
        _GPIOFRSTW { w: self }
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline]
    pub fn gpioerst(&mut self) -> _GPIOERSTW {
        _GPIOERSTW { w: self }
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline]
    pub fn gpiodrst(&mut self) -> _GPIODRSTW {
        _GPIODRSTW { w: self }
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline]
    pub fn gpiocrst(&mut self) -> _GPIOCRSTW {
        _GPIOCRSTW { w: self }
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline]
    pub fn gpiobrst(&mut self) -> _GPIOBRSTW {
        _GPIOBRSTW { w: self }
    }
    #[doc = "Bit 0 - IO port A reset"]
    #[inline]
    pub fn gpioarst(&mut self) -> _GPIOARSTW {
        _GPIOARSTW { w: self }
    }
}
