#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB2RSTR {
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
#[doc = "Possible values of the field `TIM1RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1RSTR {
    #[doc = "Reset the selected module"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TIM1RSTR {
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
            TIM1RSTR::RESET => true,
            TIM1RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM1RSTR {
        match value {
            true => TIM1RSTR::RESET,
            i => TIM1RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RSTR::RESET
    }
}
#[doc = "Possible values of the field `TIM8RST`"]
pub type TIM8RSTR = TIM1RSTR;
#[doc = "Possible values of the field `USART1RST`"]
pub type USART1RSTR = TIM1RSTR;
#[doc = "Possible values of the field `USART6RST`"]
pub type USART6RSTR = TIM1RSTR;
#[doc = "Possible values of the field `ADCRST`"]
pub type ADCRSTR = TIM1RSTR;
#[doc = "Possible values of the field `SPI1RST`"]
pub type SPI1RSTR = TIM1RSTR;
#[doc = "Possible values of the field `SPI4RST`"]
pub type SPI4RSTR = TIM1RSTR;
#[doc = "Possible values of the field `SYSCFGRST`"]
pub type SYSCFGRSTR = TIM1RSTR;
#[doc = "Possible values of the field `TIM9RST`"]
pub type TIM9RSTR = TIM1RSTR;
#[doc = "Possible values of the field `TIM10RST`"]
pub type TIM10RSTR = TIM1RSTR;
#[doc = "Possible values of the field `TIM11RST`"]
pub type TIM11RSTR = TIM1RSTR;
#[doc = "Possible values of the field `SPI5RST`"]
pub type SPI5RSTR = TIM1RSTR;
#[doc = "Possible values of the field `SPI6RST`"]
pub type SPI6RSTR = TIM1RSTR;
#[doc = "Possible values of the field `SAI1RST`"]
pub type SAI1RSTR = TIM1RSTR;
#[doc = "Possible values of the field `LTDCRST`"]
pub type LTDCRSTR = TIM1RSTR;
#[doc = "Possible values of the field `SAI2RST`"]
pub type SAI2RSTR = TIM1RSTR;
#[doc = "Possible values of the field `SDMMC1RST`"]
pub type SDMMC1RSTR = TIM1RSTR;
#[doc = "Values that can be written to the field `TIM1RST`"]
pub enum TIM1RSTW {
    #[doc = "Reset the selected module"]
    RESET,
}
impl TIM1RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM1RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM8RST`"]
pub type TIM8RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _TIM8RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM8RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM8RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `USART1RST`"]
pub type USART1RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _USART1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `USART6RST`"]
pub type USART6RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _USART6RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USART6RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART6RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `ADCRST`"]
pub type ADCRSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _ADCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `SPI1RST`"]
pub type SPI1RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _SPI1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `SPI4RST`"]
pub type SPI4RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _SPI4RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI4RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI4RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `SYSCFGRST`"]
pub type SYSCFGRSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _SYSCFGRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCFGRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSCFGRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM9RST`"]
pub type TIM9RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _TIM9RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM9RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM9RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM10RST`"]
pub type TIM10RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _TIM10RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM10RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM10RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM11RST`"]
pub type TIM11RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _TIM11RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM11RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM11RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI5RST`"]
pub type SPI5RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _SPI5RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI5RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI5RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI6RST`"]
pub type SPI6RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _SPI6RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI6RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI6RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `SAI1RST`"]
pub type SAI1RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _SAI1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `LTDCRST`"]
pub type LTDCRSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _LTDCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _LTDCRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LTDCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAI2RST`"]
pub type SAI2RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _SAI2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `SDMMC1RST`"]
pub type SDMMC1RSTW = TIM1RSTW;
#[doc = r" Proxy"]
pub struct _SDMMC1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMMC1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMMC1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - TIM1 reset"]
    #[inline]
    pub fn tim1rst(&self) -> TIM1RSTR {
        TIM1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TIM8 reset"]
    #[inline]
    pub fn tim8rst(&self) -> TIM8RSTR {
        TIM8RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline]
    pub fn usart1rst(&self) -> USART1RSTR {
        USART1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline]
    pub fn usart6rst(&self) -> USART6RSTR {
        USART6RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - ADC interface reset (common to all ADCs)"]
    #[inline]
    pub fn adcrst(&self) -> ADCRSTR {
        ADCRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline]
    pub fn spi1rst(&self) -> SPI1RSTR {
        SPI1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline]
    pub fn spi4rst(&self) -> SPI4RSTR {
        SPI4RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - System configuration controller reset"]
    #[inline]
    pub fn syscfgrst(&self) -> SYSCFGRSTR {
        SYSCFGRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - TIM9 reset"]
    #[inline]
    pub fn tim9rst(&self) -> TIM9RSTR {
        TIM9RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TIM10 reset"]
    #[inline]
    pub fn tim10rst(&self) -> TIM10RSTR {
        TIM10RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - TIM11 reset"]
    #[inline]
    pub fn tim11rst(&self) -> TIM11RSTR {
        TIM11RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - SPI5 reset"]
    #[inline]
    pub fn spi5rst(&self) -> SPI5RSTR {
        SPI5RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - SPI6 reset"]
    #[inline]
    pub fn spi6rst(&self) -> SPI6RSTR {
        SPI6RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - SAI1 reset"]
    #[inline]
    pub fn sai1rst(&self) -> SAI1RSTR {
        SAI1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - LTDC reset"]
    #[inline]
    pub fn ltdcrst(&self) -> LTDCRSTR {
        LTDCRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - SAI2 reset"]
    #[inline]
    pub fn sai2rst(&self) -> SAI2RSTR {
        SAI2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - SDMMC1 reset"]
    #[inline]
    pub fn sdmmc1rst(&self) -> SDMMC1RSTR {
        SDMMC1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - TIM1 reset"]
    #[inline]
    pub fn tim1rst(&mut self) -> _TIM1RSTW {
        _TIM1RSTW { w: self }
    }
    #[doc = "Bit 1 - TIM8 reset"]
    #[inline]
    pub fn tim8rst(&mut self) -> _TIM8RSTW {
        _TIM8RSTW { w: self }
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline]
    pub fn usart1rst(&mut self) -> _USART1RSTW {
        _USART1RSTW { w: self }
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline]
    pub fn usart6rst(&mut self) -> _USART6RSTW {
        _USART6RSTW { w: self }
    }
    #[doc = "Bit 8 - ADC interface reset (common to all ADCs)"]
    #[inline]
    pub fn adcrst(&mut self) -> _ADCRSTW {
        _ADCRSTW { w: self }
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline]
    pub fn spi1rst(&mut self) -> _SPI1RSTW {
        _SPI1RSTW { w: self }
    }
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline]
    pub fn spi4rst(&mut self) -> _SPI4RSTW {
        _SPI4RSTW { w: self }
    }
    #[doc = "Bit 14 - System configuration controller reset"]
    #[inline]
    pub fn syscfgrst(&mut self) -> _SYSCFGRSTW {
        _SYSCFGRSTW { w: self }
    }
    #[doc = "Bit 16 - TIM9 reset"]
    #[inline]
    pub fn tim9rst(&mut self) -> _TIM9RSTW {
        _TIM9RSTW { w: self }
    }
    #[doc = "Bit 17 - TIM10 reset"]
    #[inline]
    pub fn tim10rst(&mut self) -> _TIM10RSTW {
        _TIM10RSTW { w: self }
    }
    #[doc = "Bit 18 - TIM11 reset"]
    #[inline]
    pub fn tim11rst(&mut self) -> _TIM11RSTW {
        _TIM11RSTW { w: self }
    }
    #[doc = "Bit 20 - SPI5 reset"]
    #[inline]
    pub fn spi5rst(&mut self) -> _SPI5RSTW {
        _SPI5RSTW { w: self }
    }
    #[doc = "Bit 21 - SPI6 reset"]
    #[inline]
    pub fn spi6rst(&mut self) -> _SPI6RSTW {
        _SPI6RSTW { w: self }
    }
    #[doc = "Bit 22 - SAI1 reset"]
    #[inline]
    pub fn sai1rst(&mut self) -> _SAI1RSTW {
        _SAI1RSTW { w: self }
    }
    #[doc = "Bit 26 - LTDC reset"]
    #[inline]
    pub fn ltdcrst(&mut self) -> _LTDCRSTW {
        _LTDCRSTW { w: self }
    }
    #[doc = "Bit 23 - SAI2 reset"]
    #[inline]
    pub fn sai2rst(&mut self) -> _SAI2RSTW {
        _SAI2RSTW { w: self }
    }
    #[doc = "Bit 11 - SDMMC1 reset"]
    #[inline]
    pub fn sdmmc1rst(&mut self) -> _SDMMC1RSTW {
        _SDMMC1RSTW { w: self }
    }
}
