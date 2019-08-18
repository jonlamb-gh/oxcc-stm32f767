#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB1LPENR {
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
#[doc = "Possible values of the field `TIM2LPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2LPENR {
    #[doc = "Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl TIM2LPENR {
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
            TIM2LPENR::DISABLEDINSLEEP => false,
            TIM2LPENR::ENABLEDINSLEEP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM2LPENR {
        match value {
            false => TIM2LPENR::DISABLEDINSLEEP,
            true => TIM2LPENR::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM2LPENR::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM2LPENR::ENABLEDINSLEEP
    }
}
#[doc = "Possible values of the field `TIM3LPEN`"]
pub type TIM3LPENR = TIM2LPENR;
#[doc = "Possible values of the field `TIM4LPEN`"]
pub type TIM4LPENR = TIM2LPENR;
#[doc = "Possible values of the field `TIM5LPEN`"]
pub type TIM5LPENR = TIM2LPENR;
#[doc = "Possible values of the field `TIM6LPEN`"]
pub type TIM6LPENR = TIM2LPENR;
#[doc = "Possible values of the field `TIM7LPEN`"]
pub type TIM7LPENR = TIM2LPENR;
#[doc = "Possible values of the field `TIM12LPEN`"]
pub type TIM12LPENR = TIM2LPENR;
#[doc = "Possible values of the field `TIM13LPEN`"]
pub type TIM13LPENR = TIM2LPENR;
#[doc = "Possible values of the field `TIM14LPEN`"]
pub type TIM14LPENR = TIM2LPENR;
#[doc = "Possible values of the field `WWDGLPEN`"]
pub type WWDGLPENR = TIM2LPENR;
#[doc = "Possible values of the field `SPI2LPEN`"]
pub type SPI2LPENR = TIM2LPENR;
#[doc = "Possible values of the field `SPI3LPEN`"]
pub type SPI3LPENR = TIM2LPENR;
#[doc = "Possible values of the field `USART2LPEN`"]
pub type USART2LPENR = TIM2LPENR;
#[doc = "Possible values of the field `USART3LPEN`"]
pub type USART3LPENR = TIM2LPENR;
#[doc = "Possible values of the field `UART4LPEN`"]
pub type UART4LPENR = TIM2LPENR;
#[doc = "Possible values of the field `UART5LPEN`"]
pub type UART5LPENR = TIM2LPENR;
#[doc = "Possible values of the field `I2C1LPEN`"]
pub type I2C1LPENR = TIM2LPENR;
#[doc = "Possible values of the field `I2C2LPEN`"]
pub type I2C2LPENR = TIM2LPENR;
#[doc = "Possible values of the field `I2C3LPEN`"]
pub type I2C3LPENR = TIM2LPENR;
#[doc = "Possible values of the field `CAN1LPEN`"]
pub type CAN1LPENR = TIM2LPENR;
#[doc = "Possible values of the field `CAN2LPEN`"]
pub type CAN2LPENR = TIM2LPENR;
#[doc = "Possible values of the field `PWRLPEN`"]
pub type PWRLPENR = TIM2LPENR;
#[doc = "Possible values of the field `DACLPEN`"]
pub type DACLPENR = TIM2LPENR;
#[doc = "Possible values of the field `UART7LPEN`"]
pub type UART7LPENR = TIM2LPENR;
#[doc = "Possible values of the field `UART8LPEN`"]
pub type UART8LPENR = TIM2LPENR;
#[doc = "Possible values of the field `SPDIFRXLPEN`"]
pub type SPDIFRXLPENR = TIM2LPENR;
#[doc = "Possible values of the field `CECLPEN`"]
pub type CECLPENR = TIM2LPENR;
#[doc = "Possible values of the field `LPTIM1LPEN`"]
pub type LPTIM1LPENR = TIM2LPENR;
#[doc = "Possible values of the field `I2C4LPEN`"]
pub type I2C4LPENR = TIM2LPENR;
#[doc = "Values that can be written to the field `TIM2LPEN`"]
pub enum TIM2LPENW {
    #[doc = "Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl TIM2LPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM2LPENW::DISABLEDINSLEEP => false,
            TIM2LPENW::ENABLEDINSLEEP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM2LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM2LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM3LPEN`"]
pub type TIM3LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _TIM3LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM3LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM3LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM4LPEN`"]
pub type TIM4LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _TIM4LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM4LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM4LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM5LPEN`"]
pub type TIM5LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _TIM5LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM5LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM5LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM6LPEN`"]
pub type TIM6LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _TIM6LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM6LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM7LPEN`"]
pub type TIM7LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _TIM7LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM7LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM7LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM12LPEN`"]
pub type TIM12LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _TIM12LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM12LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM12LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM13LPEN`"]
pub type TIM13LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _TIM13LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM13LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM13LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `TIM14LPEN`"]
pub type TIM14LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _TIM14LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM14LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `WWDGLPEN`"]
pub type WWDGLPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _WWDGLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDGLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDGLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SPI2LPEN`"]
pub type SPI2LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _SPI2LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI2LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI2LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SPI3LPEN`"]
pub type SPI3LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _SPI3LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI3LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI3LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USART2LPEN`"]
pub type USART2LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _USART2LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART2LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `USART3LPEN`"]
pub type USART3LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _USART3LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART3LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `UART4LPEN`"]
pub type UART4LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _UART4LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART4LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART4LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART5LPEN`"]
pub type UART5LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _UART5LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART5LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART5LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `I2C1LPEN`"]
pub type I2C1LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _I2C1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `I2C2LPEN`"]
pub type I2C2LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _I2C2LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C2LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `I2C3LPEN`"]
pub type I2C3LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _I2C3LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C3LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C3LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `CAN1LPEN`"]
pub type CAN1LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _CAN1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `CAN2LPEN`"]
pub type CAN2LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _CAN2LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN2LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN2LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `PWRLPEN`"]
pub type PWRLPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _PWRLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DACLPEN`"]
pub type DACLPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _DACLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `UART7LPEN`"]
pub type UART7LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _UART7LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART7LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART7LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART8LPEN`"]
pub type UART8LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _UART8LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART8LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART8LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPDIFRXLPEN`"]
pub type SPDIFRXLPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _SPDIFRXLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDIFRXLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDIFRXLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `CECLPEN`"]
pub type CECLPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _CECLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CECLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CECLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPTIM1LPEN`"]
pub type LPTIM1LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _LPTIM1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPTIM1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPTIM1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `I2C4LPEN`"]
pub type I2C4LPENW = TIM2LPENW;
#[doc = r" Proxy"]
pub struct _I2C4LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C4LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C4LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline]
    pub fn tim2lpen(&self) -> TIM2LPENR {
        TIM2LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline]
    pub fn tim3lpen(&self) -> TIM3LPENR {
        TIM3LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline]
    pub fn tim4lpen(&self) -> TIM4LPENR {
        TIM4LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline]
    pub fn tim5lpen(&self) -> TIM5LPENR {
        TIM5LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - TIM6 clock enable during Sleep mode"]
    #[inline]
    pub fn tim6lpen(&self) -> TIM6LPENR {
        TIM6LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - TIM7 clock enable during Sleep mode"]
    #[inline]
    pub fn tim7lpen(&self) -> TIM7LPENR {
        TIM7LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - TIM12 clock enable during Sleep mode"]
    #[inline]
    pub fn tim12lpen(&self) -> TIM12LPENR {
        TIM12LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - TIM13 clock enable during Sleep mode"]
    #[inline]
    pub fn tim13lpen(&self) -> TIM13LPENR {
        TIM13LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - TIM14 clock enable during Sleep mode"]
    #[inline]
    pub fn tim14lpen(&self) -> TIM14LPENR {
        TIM14LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline]
    pub fn wwdglpen(&self) -> WWDGLPENR {
        WWDGLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline]
    pub fn spi2lpen(&self) -> SPI2LPENR {
        SPI2LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline]
    pub fn spi3lpen(&self) -> SPI3LPENR {
        SPI3LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline]
    pub fn usart2lpen(&self) -> USART2LPENR {
        USART2LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline]
    pub fn usart3lpen(&self) -> USART3LPENR {
        USART3LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - UART4 clock enable during Sleep mode"]
    #[inline]
    pub fn uart4lpen(&self) -> UART4LPENR {
        UART4LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - UART5 clock enable during Sleep mode"]
    #[inline]
    pub fn uart5lpen(&self) -> UART5LPENR {
        UART5LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline]
    pub fn i2c1lpen(&self) -> I2C1LPENR {
        I2C1LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline]
    pub fn i2c2lpen(&self) -> I2C2LPENR {
        I2C2LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline]
    pub fn i2c3lpen(&self) -> I2C3LPENR {
        I2C3LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CAN 1 clock enable during Sleep mode"]
    #[inline]
    pub fn can1lpen(&self) -> CAN1LPENR {
        CAN1LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - CAN 2 clock enable during Sleep mode"]
    #[inline]
    pub fn can2lpen(&self) -> CAN2LPENR {
        CAN2LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline]
    pub fn pwrlpen(&self) -> PWRLPENR {
        PWRLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline]
    pub fn daclpen(&self) -> DACLPENR {
        DACLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - UART7 clock enable during Sleep mode"]
    #[inline]
    pub fn uart7lpen(&self) -> UART7LPENR {
        UART7LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - UART8 clock enable during Sleep mode"]
    #[inline]
    pub fn uart8lpen(&self) -> UART8LPENR {
        UART8LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - SPDIF-RX clock enable during sleep mode"]
    #[inline]
    pub fn spdifrxlpen(&self) -> SPDIFRXLPENR {
        SPDIFRXLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - HDMI-CEN clock enable during Sleep mode"]
    #[inline]
    pub fn ceclpen(&self) -> CECLPENR {
        CECLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - low power timer 1 clock enable during Sleep mode"]
    #[inline]
    pub fn lptim1lpen(&self) -> LPTIM1LPENR {
        LPTIM1LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - I2C4 clock enable during Sleep mode"]
    #[inline]
    pub fn i2c4lpen(&self) -> I2C4LPENR {
        I2C4LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x36fe_c9ff }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline]
    pub fn tim2lpen(&mut self) -> _TIM2LPENW {
        _TIM2LPENW { w: self }
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline]
    pub fn tim3lpen(&mut self) -> _TIM3LPENW {
        _TIM3LPENW { w: self }
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline]
    pub fn tim4lpen(&mut self) -> _TIM4LPENW {
        _TIM4LPENW { w: self }
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline]
    pub fn tim5lpen(&mut self) -> _TIM5LPENW {
        _TIM5LPENW { w: self }
    }
    #[doc = "Bit 4 - TIM6 clock enable during Sleep mode"]
    #[inline]
    pub fn tim6lpen(&mut self) -> _TIM6LPENW {
        _TIM6LPENW { w: self }
    }
    #[doc = "Bit 5 - TIM7 clock enable during Sleep mode"]
    #[inline]
    pub fn tim7lpen(&mut self) -> _TIM7LPENW {
        _TIM7LPENW { w: self }
    }
    #[doc = "Bit 6 - TIM12 clock enable during Sleep mode"]
    #[inline]
    pub fn tim12lpen(&mut self) -> _TIM12LPENW {
        _TIM12LPENW { w: self }
    }
    #[doc = "Bit 7 - TIM13 clock enable during Sleep mode"]
    #[inline]
    pub fn tim13lpen(&mut self) -> _TIM13LPENW {
        _TIM13LPENW { w: self }
    }
    #[doc = "Bit 8 - TIM14 clock enable during Sleep mode"]
    #[inline]
    pub fn tim14lpen(&mut self) -> _TIM14LPENW {
        _TIM14LPENW { w: self }
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline]
    pub fn wwdglpen(&mut self) -> _WWDGLPENW {
        _WWDGLPENW { w: self }
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline]
    pub fn spi2lpen(&mut self) -> _SPI2LPENW {
        _SPI2LPENW { w: self }
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline]
    pub fn spi3lpen(&mut self) -> _SPI3LPENW {
        _SPI3LPENW { w: self }
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline]
    pub fn usart2lpen(&mut self) -> _USART2LPENW {
        _USART2LPENW { w: self }
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline]
    pub fn usart3lpen(&mut self) -> _USART3LPENW {
        _USART3LPENW { w: self }
    }
    #[doc = "Bit 19 - UART4 clock enable during Sleep mode"]
    #[inline]
    pub fn uart4lpen(&mut self) -> _UART4LPENW {
        _UART4LPENW { w: self }
    }
    #[doc = "Bit 20 - UART5 clock enable during Sleep mode"]
    #[inline]
    pub fn uart5lpen(&mut self) -> _UART5LPENW {
        _UART5LPENW { w: self }
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline]
    pub fn i2c1lpen(&mut self) -> _I2C1LPENW {
        _I2C1LPENW { w: self }
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline]
    pub fn i2c2lpen(&mut self) -> _I2C2LPENW {
        _I2C2LPENW { w: self }
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline]
    pub fn i2c3lpen(&mut self) -> _I2C3LPENW {
        _I2C3LPENW { w: self }
    }
    #[doc = "Bit 25 - CAN 1 clock enable during Sleep mode"]
    #[inline]
    pub fn can1lpen(&mut self) -> _CAN1LPENW {
        _CAN1LPENW { w: self }
    }
    #[doc = "Bit 26 - CAN 2 clock enable during Sleep mode"]
    #[inline]
    pub fn can2lpen(&mut self) -> _CAN2LPENW {
        _CAN2LPENW { w: self }
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline]
    pub fn pwrlpen(&mut self) -> _PWRLPENW {
        _PWRLPENW { w: self }
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline]
    pub fn daclpen(&mut self) -> _DACLPENW {
        _DACLPENW { w: self }
    }
    #[doc = "Bit 30 - UART7 clock enable during Sleep mode"]
    #[inline]
    pub fn uart7lpen(&mut self) -> _UART7LPENW {
        _UART7LPENW { w: self }
    }
    #[doc = "Bit 31 - UART8 clock enable during Sleep mode"]
    #[inline]
    pub fn uart8lpen(&mut self) -> _UART8LPENW {
        _UART8LPENW { w: self }
    }
    #[doc = "Bit 16 - SPDIF-RX clock enable during sleep mode"]
    #[inline]
    pub fn spdifrxlpen(&mut self) -> _SPDIFRXLPENW {
        _SPDIFRXLPENW { w: self }
    }
    #[doc = "Bit 27 - HDMI-CEN clock enable during Sleep mode"]
    #[inline]
    pub fn ceclpen(&mut self) -> _CECLPENW {
        _CECLPENW { w: self }
    }
    #[doc = "Bit 9 - low power timer 1 clock enable during Sleep mode"]
    #[inline]
    pub fn lptim1lpen(&mut self) -> _LPTIM1LPENW {
        _LPTIM1LPENW { w: self }
    }
    #[doc = "Bit 24 - I2C4 clock enable during Sleep mode"]
    #[inline]
    pub fn i2c4lpen(&mut self) -> _I2C4LPENW {
        _I2C4LPENW { w: self }
    }
}
