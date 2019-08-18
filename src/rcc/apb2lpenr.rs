#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB2LPENR {
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
#[doc = "Possible values of the field `TIM1LPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1LPENR {
    #[doc = "Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl TIM1LPENR {
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
            TIM1LPENR::DISABLEDINSLEEP => false,
            TIM1LPENR::ENABLEDINSLEEP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM1LPENR {
        match value {
            false => TIM1LPENR::DISABLEDINSLEEP,
            true => TIM1LPENR::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM1LPENR::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM1LPENR::ENABLEDINSLEEP
    }
}
#[doc = "Possible values of the field `TIM8LPEN`"]
pub type TIM8LPENR = TIM1LPENR;
#[doc = "Possible values of the field `USART1LPEN`"]
pub type USART1LPENR = TIM1LPENR;
#[doc = "Possible values of the field `USART6LPEN`"]
pub type USART6LPENR = TIM1LPENR;
#[doc = "Possible values of the field `ADC1LPEN`"]
pub type ADC1LPENR = TIM1LPENR;
#[doc = "Possible values of the field `ADC2LPEN`"]
pub type ADC2LPENR = TIM1LPENR;
#[doc = "Possible values of the field `ADC3LPEN`"]
pub type ADC3LPENR = TIM1LPENR;
#[doc = "Possible values of the field `SPI1LPEN`"]
pub type SPI1LPENR = TIM1LPENR;
#[doc = "Possible values of the field `SPI4LPEN`"]
pub type SPI4LPENR = TIM1LPENR;
#[doc = "Possible values of the field `SYSCFGLPEN`"]
pub type SYSCFGLPENR = TIM1LPENR;
#[doc = "Possible values of the field `TIM9LPEN`"]
pub type TIM9LPENR = TIM1LPENR;
#[doc = "Possible values of the field `TIM10LPEN`"]
pub type TIM10LPENR = TIM1LPENR;
#[doc = "Possible values of the field `TIM11LPEN`"]
pub type TIM11LPENR = TIM1LPENR;
#[doc = "Possible values of the field `SPI5LPEN`"]
pub type SPI5LPENR = TIM1LPENR;
#[doc = "Possible values of the field `SPI6LPEN`"]
pub type SPI6LPENR = TIM1LPENR;
#[doc = "Possible values of the field `SAI1LPEN`"]
pub type SAI1LPENR = TIM1LPENR;
#[doc = "Possible values of the field `LTDCLPEN`"]
pub type LTDCLPENR = TIM1LPENR;
#[doc = "Possible values of the field `SAI2LPEN`"]
pub type SAI2LPENR = TIM1LPENR;
#[doc = "Possible values of the field `SDMMC1LPEN`"]
pub type SDMMC1LPENR = TIM1LPENR;
#[doc = "Values that can be written to the field `TIM1LPEN`"]
pub enum TIM1LPENW {
    #[doc = "Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl TIM1LPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM1LPENW::DISABLEDINSLEEP => false,
            TIM1LPENW::ENABLEDINSLEEP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM8LPEN`"]
pub type TIM8LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _TIM8LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM8LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM8LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `USART1LPEN`"]
pub type USART1LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _USART1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `USART6LPEN`"]
pub type USART6LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _USART6LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART6LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART6LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `ADC1LPEN`"]
pub type ADC1LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _ADC1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `ADC2LPEN`"]
pub type ADC2LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _ADC2LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC2LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC2LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `ADC3LPEN`"]
pub type ADC3LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _ADC3LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC3LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC3LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SPI1LPEN`"]
pub type SPI1LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _SPI1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SPI4LPEN`"]
pub type SPI4LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _SPI4LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI4LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI4LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SYSCFGLPEN`"]
pub type SYSCFGLPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _SYSCFGLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCFGLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSCFGLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM9LPEN`"]
pub type TIM9LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _TIM9LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM9LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM9LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM10LPEN`"]
pub type TIM10LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _TIM10LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM10LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM10LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM11LPEN`"]
pub type TIM11LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _TIM11LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM11LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM11LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SPI5LPEN`"]
pub type SPI5LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _SPI5LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI5LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI5LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SPI6LPEN`"]
pub type SPI6LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _SPI6LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI6LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI6LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SAI1LPEN`"]
pub type SAI1LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _SAI1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `LTDCLPEN`"]
pub type LTDCLPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _LTDCLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _LTDCLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LTDCLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SAI2LPEN`"]
pub type SAI2LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _SAI2LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI2LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI2LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SDMMC1LPEN`"]
pub type SDMMC1LPENW = TIM1LPENW;
#[doc = r" Proxy"]
pub struct _SDMMC1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMMC1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMMC1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPENW::ENABLEDINSLEEP)
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
    #[doc = "Bit 0 - TIM1 clock enable during Sleep mode"]
    #[inline]
    pub fn tim1lpen(&self) -> TIM1LPENR {
        TIM1LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TIM8 clock enable during Sleep mode"]
    #[inline]
    pub fn tim8lpen(&self) -> TIM8LPENR {
        TIM8LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - USART1 clock enable during Sleep mode"]
    #[inline]
    pub fn usart1lpen(&self) -> USART1LPENR {
        USART1LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - USART6 clock enable during Sleep mode"]
    #[inline]
    pub fn usart6lpen(&self) -> USART6LPENR {
        USART6LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - ADC1 clock enable during Sleep mode"]
    #[inline]
    pub fn adc1lpen(&self) -> ADC1LPENR {
        ADC1LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ADC2 clock enable during Sleep mode"]
    #[inline]
    pub fn adc2lpen(&self) -> ADC2LPENR {
        ADC2LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - ADC 3 clock enable during Sleep mode"]
    #[inline]
    pub fn adc3lpen(&self) -> ADC3LPENR {
        ADC3LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline]
    pub fn spi1lpen(&self) -> SPI1LPENR {
        SPI1LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - SPI 4 clock enable during Sleep mode"]
    #[inline]
    pub fn spi4lpen(&self) -> SPI4LPENR {
        SPI4LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - System configuration controller clock enable during Sleep mode"]
    #[inline]
    pub fn syscfglpen(&self) -> SYSCFGLPENR {
        SYSCFGLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - TIM9 clock enable during sleep mode"]
    #[inline]
    pub fn tim9lpen(&self) -> TIM9LPENR {
        TIM9LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TIM10 clock enable during Sleep mode"]
    #[inline]
    pub fn tim10lpen(&self) -> TIM10LPENR {
        TIM10LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - TIM11 clock enable during Sleep mode"]
    #[inline]
    pub fn tim11lpen(&self) -> TIM11LPENR {
        TIM11LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - SPI 5 clock enable during Sleep mode"]
    #[inline]
    pub fn spi5lpen(&self) -> SPI5LPENR {
        SPI5LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - SPI 6 clock enable during Sleep mode"]
    #[inline]
    pub fn spi6lpen(&self) -> SPI6LPENR {
        SPI6LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - SAI1 clock enable during sleep mode"]
    #[inline]
    pub fn sai1lpen(&self) -> SAI1LPENR {
        SAI1LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - LTDC clock enable during sleep mode"]
    #[inline]
    pub fn ltdclpen(&self) -> LTDCLPENR {
        LTDCLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - SAI2 clock enable during sleep mode"]
    #[inline]
    pub fn sai2lpen(&self) -> SAI2LPENR {
        SAI2LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - SDMMC1 clock enable during Sleep mode"]
    #[inline]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPENR {
        SDMMC1LPENR::_from({
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
        W { bits: 0x0007_5f33 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - TIM1 clock enable during Sleep mode"]
    #[inline]
    pub fn tim1lpen(&mut self) -> _TIM1LPENW {
        _TIM1LPENW { w: self }
    }
    #[doc = "Bit 1 - TIM8 clock enable during Sleep mode"]
    #[inline]
    pub fn tim8lpen(&mut self) -> _TIM8LPENW {
        _TIM8LPENW { w: self }
    }
    #[doc = "Bit 4 - USART1 clock enable during Sleep mode"]
    #[inline]
    pub fn usart1lpen(&mut self) -> _USART1LPENW {
        _USART1LPENW { w: self }
    }
    #[doc = "Bit 5 - USART6 clock enable during Sleep mode"]
    #[inline]
    pub fn usart6lpen(&mut self) -> _USART6LPENW {
        _USART6LPENW { w: self }
    }
    #[doc = "Bit 8 - ADC1 clock enable during Sleep mode"]
    #[inline]
    pub fn adc1lpen(&mut self) -> _ADC1LPENW {
        _ADC1LPENW { w: self }
    }
    #[doc = "Bit 9 - ADC2 clock enable during Sleep mode"]
    #[inline]
    pub fn adc2lpen(&mut self) -> _ADC2LPENW {
        _ADC2LPENW { w: self }
    }
    #[doc = "Bit 10 - ADC 3 clock enable during Sleep mode"]
    #[inline]
    pub fn adc3lpen(&mut self) -> _ADC3LPENW {
        _ADC3LPENW { w: self }
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline]
    pub fn spi1lpen(&mut self) -> _SPI1LPENW {
        _SPI1LPENW { w: self }
    }
    #[doc = "Bit 13 - SPI 4 clock enable during Sleep mode"]
    #[inline]
    pub fn spi4lpen(&mut self) -> _SPI4LPENW {
        _SPI4LPENW { w: self }
    }
    #[doc = "Bit 14 - System configuration controller clock enable during Sleep mode"]
    #[inline]
    pub fn syscfglpen(&mut self) -> _SYSCFGLPENW {
        _SYSCFGLPENW { w: self }
    }
    #[doc = "Bit 16 - TIM9 clock enable during sleep mode"]
    #[inline]
    pub fn tim9lpen(&mut self) -> _TIM9LPENW {
        _TIM9LPENW { w: self }
    }
    #[doc = "Bit 17 - TIM10 clock enable during Sleep mode"]
    #[inline]
    pub fn tim10lpen(&mut self) -> _TIM10LPENW {
        _TIM10LPENW { w: self }
    }
    #[doc = "Bit 18 - TIM11 clock enable during Sleep mode"]
    #[inline]
    pub fn tim11lpen(&mut self) -> _TIM11LPENW {
        _TIM11LPENW { w: self }
    }
    #[doc = "Bit 20 - SPI 5 clock enable during Sleep mode"]
    #[inline]
    pub fn spi5lpen(&mut self) -> _SPI5LPENW {
        _SPI5LPENW { w: self }
    }
    #[doc = "Bit 21 - SPI 6 clock enable during Sleep mode"]
    #[inline]
    pub fn spi6lpen(&mut self) -> _SPI6LPENW {
        _SPI6LPENW { w: self }
    }
    #[doc = "Bit 22 - SAI1 clock enable during sleep mode"]
    #[inline]
    pub fn sai1lpen(&mut self) -> _SAI1LPENW {
        _SAI1LPENW { w: self }
    }
    #[doc = "Bit 26 - LTDC clock enable during sleep mode"]
    #[inline]
    pub fn ltdclpen(&mut self) -> _LTDCLPENW {
        _LTDCLPENW { w: self }
    }
    #[doc = "Bit 23 - SAI2 clock enable during sleep mode"]
    #[inline]
    pub fn sai2lpen(&mut self) -> _SAI2LPENW {
        _SAI2LPENW { w: self }
    }
    #[doc = "Bit 11 - SDMMC1 clock enable during Sleep mode"]
    #[inline]
    pub fn sdmmc1lpen(&mut self) -> _SDMMC1LPENW {
        _SDMMC1LPENW { w: self }
    }
}
