#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB2ENR {
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
#[doc = "Possible values of the field `TIM1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1ENR {
    #[doc = "The selected clock is disabled"]
    DISABLED,
    #[doc = "The selected clock is enabled"]
    ENABLED,
}
impl TIM1ENR {
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
            TIM1ENR::DISABLED => false,
            TIM1ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM1ENR {
        match value {
            false => TIM1ENR::DISABLED,
            true => TIM1ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1ENR::ENABLED
    }
}
#[doc = "Possible values of the field `TIM8EN`"]
pub type TIM8ENR = TIM1ENR;
#[doc = "Possible values of the field `USART1EN`"]
pub type USART1ENR = TIM1ENR;
#[doc = "Possible values of the field `USART6EN`"]
pub type USART6ENR = TIM1ENR;
#[doc = "Possible values of the field `ADC1EN`"]
pub type ADC1ENR = TIM1ENR;
#[doc = "Possible values of the field `ADC2EN`"]
pub type ADC2ENR = TIM1ENR;
#[doc = "Possible values of the field `ADC3EN`"]
pub type ADC3ENR = TIM1ENR;
#[doc = "Possible values of the field `SPI1EN`"]
pub type SPI1ENR = TIM1ENR;
#[doc = r" Value of the field"]
pub struct SPI4ENRR {
    bits: bool,
}
impl SPI4ENRR {
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
#[doc = "Possible values of the field `SYSCFGEN`"]
pub type SYSCFGENR = TIM1ENR;
#[doc = "Possible values of the field `TIM9EN`"]
pub type TIM9ENR = TIM1ENR;
#[doc = "Possible values of the field `TIM10EN`"]
pub type TIM10ENR = TIM1ENR;
#[doc = "Possible values of the field `TIM11EN`"]
pub type TIM11ENR = TIM1ENR;
#[doc = r" Value of the field"]
pub struct SPI5ENRR {
    bits: bool,
}
impl SPI5ENRR {
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
pub struct SPI6ENRR {
    bits: bool,
}
impl SPI6ENRR {
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
#[doc = "Possible values of the field `SAI1EN`"]
pub type SAI1ENR = TIM1ENR;
#[doc = "Possible values of the field `LTDCEN`"]
pub type LTDCENR = TIM1ENR;
#[doc = "Possible values of the field `SAI2EN`"]
pub type SAI2ENR = TIM1ENR;
#[doc = "Possible values of the field `SDMMC1EN`"]
pub type SDMMC1ENR = TIM1ENR;
#[doc = "Possible values of the field `MDIOEN`"]
pub type MDIOENR = TIM1ENR;
#[doc = "Values that can be written to the field `TIM1EN`"]
pub enum TIM1ENW {
    #[doc = "The selected clock is disabled"]
    DISABLED,
    #[doc = "The selected clock is enabled"]
    ENABLED,
}
impl TIM1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM1ENW::DISABLED => false,
            TIM1ENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM8EN`"]
pub type TIM8ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _TIM8ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM8ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM8ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `USART1EN`"]
pub type USART1ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _USART1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `USART6EN`"]
pub type USART6ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _USART6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART6ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART6ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `ADC1EN`"]
pub type ADC1ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _ADC1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `ADC2EN`"]
pub type ADC2ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _ADC2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `ADC3EN`"]
pub type ADC3ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _ADC3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `SPI1EN`"]
pub type SPI1ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _SPI1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _SPI4ENRW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI4ENRW<'a> {
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
#[doc = "Values that can be written to the field `SYSCFGEN`"]
pub type SYSCFGENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _SYSCFGENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCFGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSCFGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM9EN`"]
pub type TIM9ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _TIM9ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM9ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM9ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM10EN`"]
pub type TIM10ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _TIM10ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM10ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM10ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM11EN`"]
pub type TIM11ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _TIM11ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM11ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM11ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _SPI5ENRW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI5ENRW<'a> {
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
#[doc = r" Proxy"]
pub struct _SPI6ENRW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI6ENRW<'a> {
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
#[doc = "Values that can be written to the field `SAI1EN`"]
pub type SAI1ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _SAI1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `LTDCEN`"]
pub type LTDCENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _LTDCENW<'a> {
    w: &'a mut W,
}
impl<'a> _LTDCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LTDCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `SAI2EN`"]
pub type SAI2ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _SAI2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAI2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `SDMMC1EN`"]
pub type SDMMC1ENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _SDMMC1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMMC1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMMC1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `MDIOEN`"]
pub type MDIOENW = TIM1ENW;
#[doc = r" Proxy"]
pub struct _MDIOENW<'a> {
    w: &'a mut W,
}
impl<'a> _MDIOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDIOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1ENW::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1ENW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline]
    pub fn tim1en(&self) -> TIM1ENR {
        TIM1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline]
    pub fn tim8en(&self) -> TIM8ENR {
        TIM8ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline]
    pub fn usart1en(&self) -> USART1ENR {
        USART1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline]
    pub fn usart6en(&self) -> USART6ENR {
        USART6ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline]
    pub fn adc1en(&self) -> ADC1ENR {
        ADC1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline]
    pub fn adc2en(&self) -> ADC2ENR {
        ADC2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline]
    pub fn adc3en(&self) -> ADC3ENR {
        ADC3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline]
    pub fn spi1en(&self) -> SPI1ENR {
        SPI1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline]
    pub fn spi4enr(&self) -> SPI4ENRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPI4ENRR { bits }
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline]
    pub fn syscfgen(&self) -> SYSCFGENR {
        SYSCFGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline]
    pub fn tim9en(&self) -> TIM9ENR {
        TIM9ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline]
    pub fn tim10en(&self) -> TIM10ENR {
        TIM10ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline]
    pub fn tim11en(&self) -> TIM11ENR {
        TIM11ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - SPI5 clock enable"]
    #[inline]
    pub fn spi5enr(&self) -> SPI5ENRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPI5ENRR { bits }
    }
    #[doc = "Bit 21 - SPI6 clock enable"]
    #[inline]
    pub fn spi6enr(&self) -> SPI6ENRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPI6ENRR { bits }
    }
    #[doc = "Bit 22 - SAI1 clock enable"]
    #[inline]
    pub fn sai1en(&self) -> SAI1ENR {
        SAI1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - LTDC clock enable"]
    #[inline]
    pub fn ltdcen(&self) -> LTDCENR {
        LTDCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - SAI2 clock enable"]
    #[inline]
    pub fn sai2en(&self) -> SAI2ENR {
        SAI2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - SDMMC1 clock enable"]
    #[inline]
    pub fn sdmmc1en(&self) -> SDMMC1ENR {
        SDMMC1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - MDIO clock enable"]
    #[inline]
    pub fn mdioen(&self) -> MDIOENR {
        MDIOENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline]
    pub fn tim1en(&mut self) -> _TIM1ENW {
        _TIM1ENW { w: self }
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline]
    pub fn tim8en(&mut self) -> _TIM8ENW {
        _TIM8ENW { w: self }
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline]
    pub fn usart1en(&mut self) -> _USART1ENW {
        _USART1ENW { w: self }
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline]
    pub fn usart6en(&mut self) -> _USART6ENW {
        _USART6ENW { w: self }
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline]
    pub fn adc1en(&mut self) -> _ADC1ENW {
        _ADC1ENW { w: self }
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline]
    pub fn adc2en(&mut self) -> _ADC2ENW {
        _ADC2ENW { w: self }
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline]
    pub fn adc3en(&mut self) -> _ADC3ENW {
        _ADC3ENW { w: self }
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline]
    pub fn spi1en(&mut self) -> _SPI1ENW {
        _SPI1ENW { w: self }
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline]
    pub fn spi4enr(&mut self) -> _SPI4ENRW {
        _SPI4ENRW { w: self }
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline]
    pub fn syscfgen(&mut self) -> _SYSCFGENW {
        _SYSCFGENW { w: self }
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline]
    pub fn tim9en(&mut self) -> _TIM9ENW {
        _TIM9ENW { w: self }
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline]
    pub fn tim10en(&mut self) -> _TIM10ENW {
        _TIM10ENW { w: self }
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline]
    pub fn tim11en(&mut self) -> _TIM11ENW {
        _TIM11ENW { w: self }
    }
    #[doc = "Bit 20 - SPI5 clock enable"]
    #[inline]
    pub fn spi5enr(&mut self) -> _SPI5ENRW {
        _SPI5ENRW { w: self }
    }
    #[doc = "Bit 21 - SPI6 clock enable"]
    #[inline]
    pub fn spi6enr(&mut self) -> _SPI6ENRW {
        _SPI6ENRW { w: self }
    }
    #[doc = "Bit 22 - SAI1 clock enable"]
    #[inline]
    pub fn sai1en(&mut self) -> _SAI1ENW {
        _SAI1ENW { w: self }
    }
    #[doc = "Bit 26 - LTDC clock enable"]
    #[inline]
    pub fn ltdcen(&mut self) -> _LTDCENW {
        _LTDCENW { w: self }
    }
    #[doc = "Bit 23 - SAI2 clock enable"]
    #[inline]
    pub fn sai2en(&mut self) -> _SAI2ENW {
        _SAI2ENW { w: self }
    }
    #[doc = "Bit 11 - SDMMC1 clock enable"]
    #[inline]
    pub fn sdmmc1en(&mut self) -> _SDMMC1ENW {
        _SDMMC1ENW { w: self }
    }
    #[doc = "Bit 30 - MDIO clock enable"]
    #[inline]
    pub fn mdioen(&mut self) -> _MDIOENW {
        _MDIOENW { w: self }
    }
}
