#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB1ENR {
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
#[doc = "Possible values of the field `TIM2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2ENR {
    #[doc = "The selected clock is disabled"]
    DISABLED,
    #[doc = "The selected clock is enabled"]
    ENABLED,
}
impl TIM2ENR {
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
            TIM2ENR::DISABLED => false,
            TIM2ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM2ENR {
        match value {
            false => TIM2ENR::DISABLED,
            true => TIM2ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2ENR::ENABLED
    }
}
#[doc = "Possible values of the field `TIM3EN`"]
pub type TIM3ENR = TIM2ENR;
#[doc = "Possible values of the field `TIM4EN`"]
pub type TIM4ENR = TIM2ENR;
#[doc = "Possible values of the field `TIM5EN`"]
pub type TIM5ENR = TIM2ENR;
#[doc = "Possible values of the field `TIM6EN`"]
pub type TIM6ENR = TIM2ENR;
#[doc = "Possible values of the field `TIM7EN`"]
pub type TIM7ENR = TIM2ENR;
#[doc = "Possible values of the field `TIM12EN`"]
pub type TIM12ENR = TIM2ENR;
#[doc = "Possible values of the field `TIM13EN`"]
pub type TIM13ENR = TIM2ENR;
#[doc = "Possible values of the field `TIM14EN`"]
pub type TIM14ENR = TIM2ENR;
#[doc = "Possible values of the field `WWDGEN`"]
pub type WWDGENR = TIM2ENR;
#[doc = "Possible values of the field `SPI2EN`"]
pub type SPI2ENR = TIM2ENR;
#[doc = "Possible values of the field `SPI3EN`"]
pub type SPI3ENR = TIM2ENR;
#[doc = "Possible values of the field `USART2EN`"]
pub type USART2ENR = TIM2ENR;
#[doc = "Possible values of the field `USART3EN`"]
pub type USART3ENR = TIM2ENR;
#[doc = "Possible values of the field `UART4EN`"]
pub type UART4ENR = TIM2ENR;
#[doc = "Possible values of the field `UART5EN`"]
pub type UART5ENR = TIM2ENR;
#[doc = "Possible values of the field `I2C1EN`"]
pub type I2C1ENR = TIM2ENR;
#[doc = "Possible values of the field `I2C2EN`"]
pub type I2C2ENR = TIM2ENR;
#[doc = "Possible values of the field `I2C3EN`"]
pub type I2C3ENR = TIM2ENR;
#[doc = "Possible values of the field `CAN1EN`"]
pub type CAN1ENR = TIM2ENR;
#[doc = "Possible values of the field `CAN2EN`"]
pub type CAN2ENR = TIM2ENR;
#[doc = "Possible values of the field `PWREN`"]
pub type PWRENR = TIM2ENR;
#[doc = "Possible values of the field `DACEN`"]
pub type DACENR = TIM2ENR;
#[doc = r" Value of the field"]
pub struct UART7ENRR {
    bits: bool,
}
impl UART7ENRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct UART8ENRR {
    bits: bool,
}
impl UART8ENRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `SPDIFRXEN`"]
pub type SPDIFRXENR = TIM2ENR;
#[doc = "Possible values of the field `CECEN`"]
pub type CECENR = TIM2ENR;
#[doc = "Possible values of the field `LPTMI1EN`"]
pub type LPTMI1ENR = TIM2ENR;
#[doc = "Possible values of the field `I2C4EN`"]
pub type I2C4ENR = TIM2ENR;
#[doc = "Values that can be written to the field `TIM2EN`"]
pub enum TIM2ENW {
    #[doc = "The selected clock is disabled"]
    DISABLED,
    #[doc = "The selected clock is enabled"]
    ENABLED,
}
impl TIM2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM2ENW::DISABLED => false,
            TIM2ENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM3EN`"]
pub type TIM3ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _TIM3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM4EN`"]
pub type TIM4ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _TIM4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM4ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM4ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM5EN`"]
pub type TIM5ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _TIM5ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM5ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM5ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM6EN`"]
pub type TIM6ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _TIM6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM6ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM7EN`"]
pub type TIM7ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _TIM7ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM7ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM7ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM12EN`"]
pub type TIM12ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _TIM12ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM12ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM12ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM13EN`"]
pub type TIM13ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _TIM13ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM13ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM13ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM14EN`"]
pub type TIM14ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _TIM14ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM14ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `WWDGEN`"]
pub type WWDGENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _WWDGENW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `SPI2EN`"]
pub type SPI2ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _SPI2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `SPI3EN`"]
pub type SPI3ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _SPI3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `USART2EN`"]
pub type USART2ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _USART2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `USART3EN`"]
pub type USART3ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _USART3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `UART4EN`"]
pub type UART4ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _UART4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART4ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART4ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `UART5EN`"]
pub type UART5ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _UART5ENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART5ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART5ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `I2C1EN`"]
pub type I2C1ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _I2C1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `I2C2EN`"]
pub type I2C2ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _I2C2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `I2C3EN`"]
pub type I2C3ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _I2C3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `CAN1EN`"]
pub type CAN1ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _CAN1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `CAN2EN`"]
pub type CAN2ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _CAN2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `PWREN`"]
pub type PWRENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _PWRENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `DACEN`"]
pub type DACENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _DACENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _UART7ENRW<'a> {
    w: &'a mut W,
}
impl<'a> _UART7ENRW<'a> {
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
#[doc = r" Proxy"]
pub struct _UART8ENRW<'a> {
    w: &'a mut W,
}
impl<'a> _UART8ENRW<'a> {
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
#[doc = "Values that can be written to the field `SPDIFRXEN`"]
pub type SPDIFRXENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _SPDIFRXENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDIFRXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDIFRXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `CECEN`"]
pub type CECENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _CECENW<'a> {
    w: &'a mut W,
}
impl<'a> _CECENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CECENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `LPTMI1EN`"]
pub type LPTMI1ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _LPTMI1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPTMI1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPTMI1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
#[doc = "Values that can be written to the field `I2C4EN`"]
pub type I2C4ENW = TIM2ENW;
#[doc = r" Proxy"]
pub struct _I2C4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C4ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C4ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2ENW::ENABLED)
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
    #[doc = "Bit 0 - TIM2 clock enable"]
    #[inline]
    pub fn tim2en(&self) -> TIM2ENR {
        TIM2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TIM3 clock enable"]
    #[inline]
    pub fn tim3en(&self) -> TIM3ENR {
        TIM3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - TIM4 clock enable"]
    #[inline]
    pub fn tim4en(&self) -> TIM4ENR {
        TIM4ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - TIM5 clock enable"]
    #[inline]
    pub fn tim5en(&self) -> TIM5ENR {
        TIM5ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - TIM6 clock enable"]
    #[inline]
    pub fn tim6en(&self) -> TIM6ENR {
        TIM6ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - TIM7 clock enable"]
    #[inline]
    pub fn tim7en(&self) -> TIM7ENR {
        TIM7ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - TIM12 clock enable"]
    #[inline]
    pub fn tim12en(&self) -> TIM12ENR {
        TIM12ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - TIM13 clock enable"]
    #[inline]
    pub fn tim13en(&self) -> TIM13ENR {
        TIM13ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - TIM14 clock enable"]
    #[inline]
    pub fn tim14en(&self) -> TIM14ENR {
        TIM14ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline]
    pub fn wwdgen(&self) -> WWDGENR {
        WWDGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline]
    pub fn spi2en(&self) -> SPI2ENR {
        SPI2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline]
    pub fn spi3en(&self) -> SPI3ENR {
        SPI3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline]
    pub fn usart2en(&self) -> USART2ENR {
        USART2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline]
    pub fn usart3en(&self) -> USART3ENR {
        USART3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline]
    pub fn uart4en(&self) -> UART4ENR {
        UART4ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline]
    pub fn uart5en(&self) -> UART5ENR {
        UART5ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline]
    pub fn i2c1en(&self) -> I2C1ENR {
        I2C1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline]
    pub fn i2c2en(&self) -> I2C2ENR {
        I2C2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline]
    pub fn i2c3en(&self) -> I2C3ENR {
        I2C3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CAN 1 clock enable"]
    #[inline]
    pub fn can1en(&self) -> CAN1ENR {
        CAN1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - CAN 2 clock enable"]
    #[inline]
    pub fn can2en(&self) -> CAN2ENR {
        CAN2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline]
    pub fn pwren(&self) -> PWRENR {
        PWRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline]
    pub fn dacen(&self) -> DACENR {
        DACENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - UART7 clock enable"]
    #[inline]
    pub fn uart7enr(&self) -> UART7ENRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UART7ENRR { bits }
    }
    #[doc = "Bit 31 - UART8 clock enable"]
    #[inline]
    pub fn uart8enr(&self) -> UART8ENRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UART8ENRR { bits }
    }
    #[doc = "Bit 16 - SPDIF-RX clock enable"]
    #[inline]
    pub fn spdifrxen(&self) -> SPDIFRXENR {
        SPDIFRXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - HDMI-CEN clock enable"]
    #[inline]
    pub fn cecen(&self) -> CECENR {
        CECENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Low power timer 1 clock enable"]
    #[inline]
    pub fn lptmi1en(&self) -> LPTMI1ENR {
        LPTMI1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - I2C4 clock enable"]
    #[inline]
    pub fn i2c4en(&self) -> I2C4ENR {
        I2C4ENR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - TIM2 clock enable"]
    #[inline]
    pub fn tim2en(&mut self) -> _TIM2ENW {
        _TIM2ENW { w: self }
    }
    #[doc = "Bit 1 - TIM3 clock enable"]
    #[inline]
    pub fn tim3en(&mut self) -> _TIM3ENW {
        _TIM3ENW { w: self }
    }
    #[doc = "Bit 2 - TIM4 clock enable"]
    #[inline]
    pub fn tim4en(&mut self) -> _TIM4ENW {
        _TIM4ENW { w: self }
    }
    #[doc = "Bit 3 - TIM5 clock enable"]
    #[inline]
    pub fn tim5en(&mut self) -> _TIM5ENW {
        _TIM5ENW { w: self }
    }
    #[doc = "Bit 4 - TIM6 clock enable"]
    #[inline]
    pub fn tim6en(&mut self) -> _TIM6ENW {
        _TIM6ENW { w: self }
    }
    #[doc = "Bit 5 - TIM7 clock enable"]
    #[inline]
    pub fn tim7en(&mut self) -> _TIM7ENW {
        _TIM7ENW { w: self }
    }
    #[doc = "Bit 6 - TIM12 clock enable"]
    #[inline]
    pub fn tim12en(&mut self) -> _TIM12ENW {
        _TIM12ENW { w: self }
    }
    #[doc = "Bit 7 - TIM13 clock enable"]
    #[inline]
    pub fn tim13en(&mut self) -> _TIM13ENW {
        _TIM13ENW { w: self }
    }
    #[doc = "Bit 8 - TIM14 clock enable"]
    #[inline]
    pub fn tim14en(&mut self) -> _TIM14ENW {
        _TIM14ENW { w: self }
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline]
    pub fn wwdgen(&mut self) -> _WWDGENW {
        _WWDGENW { w: self }
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline]
    pub fn spi2en(&mut self) -> _SPI2ENW {
        _SPI2ENW { w: self }
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline]
    pub fn spi3en(&mut self) -> _SPI3ENW {
        _SPI3ENW { w: self }
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline]
    pub fn usart2en(&mut self) -> _USART2ENW {
        _USART2ENW { w: self }
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline]
    pub fn usart3en(&mut self) -> _USART3ENW {
        _USART3ENW { w: self }
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline]
    pub fn uart4en(&mut self) -> _UART4ENW {
        _UART4ENW { w: self }
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline]
    pub fn uart5en(&mut self) -> _UART5ENW {
        _UART5ENW { w: self }
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline]
    pub fn i2c1en(&mut self) -> _I2C1ENW {
        _I2C1ENW { w: self }
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline]
    pub fn i2c2en(&mut self) -> _I2C2ENW {
        _I2C2ENW { w: self }
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline]
    pub fn i2c3en(&mut self) -> _I2C3ENW {
        _I2C3ENW { w: self }
    }
    #[doc = "Bit 25 - CAN 1 clock enable"]
    #[inline]
    pub fn can1en(&mut self) -> _CAN1ENW {
        _CAN1ENW { w: self }
    }
    #[doc = "Bit 26 - CAN 2 clock enable"]
    #[inline]
    pub fn can2en(&mut self) -> _CAN2ENW {
        _CAN2ENW { w: self }
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline]
    pub fn pwren(&mut self) -> _PWRENW {
        _PWRENW { w: self }
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline]
    pub fn dacen(&mut self) -> _DACENW {
        _DACENW { w: self }
    }
    #[doc = "Bit 30 - UART7 clock enable"]
    #[inline]
    pub fn uart7enr(&mut self) -> _UART7ENRW {
        _UART7ENRW { w: self }
    }
    #[doc = "Bit 31 - UART8 clock enable"]
    #[inline]
    pub fn uart8enr(&mut self) -> _UART8ENRW {
        _UART8ENRW { w: self }
    }
    #[doc = "Bit 16 - SPDIF-RX clock enable"]
    #[inline]
    pub fn spdifrxen(&mut self) -> _SPDIFRXENW {
        _SPDIFRXENW { w: self }
    }
    #[doc = "Bit 27 - HDMI-CEN clock enable"]
    #[inline]
    pub fn cecen(&mut self) -> _CECENW {
        _CECENW { w: self }
    }
    #[doc = "Bit 9 - Low power timer 1 clock enable"]
    #[inline]
    pub fn lptmi1en(&mut self) -> _LPTMI1ENW {
        _LPTMI1ENW { w: self }
    }
    #[doc = "Bit 24 - I2C4 clock enable"]
    #[inline]
    pub fn i2c4en(&mut self) -> _I2C4ENW {
        _I2C4ENW { w: self }
    }
}
