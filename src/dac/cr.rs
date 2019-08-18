#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `DMAUDRIE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDRIE2R {
    #[doc = "DAC channel2 DMA Underrun Interrupt disabled"]
    DISABLED,
    #[doc = "DAC channel2 DMA Underrun Interrupt enabled"]
    ENABLED,
}
impl DMAUDRIE2R {
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
            DMAUDRIE2R::DISABLED => false,
            DMAUDRIE2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAUDRIE2R {
        match value {
            false => DMAUDRIE2R::DISABLED,
            true => DMAUDRIE2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMAUDRIE2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMAUDRIE2R::ENABLED
    }
}
#[doc = "Possible values of the field `DMAEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN2R {
    #[doc = "DAC channel2 DMA mode disabled"]
    DISABLED,
    #[doc = "DAC channel2 DMA mode enabled"]
    ENABLED,
}
impl DMAEN2R {
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
            DMAEN2R::DISABLED => false,
            DMAEN2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAEN2R {
        match value {
            false => DMAEN2R::DISABLED,
            true => DMAEN2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN2R::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct MAMP2R {
    bits: u8,
}
impl MAMP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WAVE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVE2R {
    #[doc = "Wave generation disabled"]
    DISABLED,
    #[doc = "Noise wave generation enabled"]
    NOISE,
    #[doc = "Triangle wave generation enabled"]
    TRIANGLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WAVE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAVE2R::DISABLED => 0,
            WAVE2R::NOISE => 0x01,
            WAVE2R::TRIANGLE => 0x02,
            WAVE2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAVE2R {
        match value {
            0 => WAVE2R::DISABLED,
            1 => WAVE2R::NOISE,
            2 => WAVE2R::TRIANGLE,
            i => WAVE2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WAVE2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `NOISE`"]
    #[inline]
    pub fn is_noise(&self) -> bool {
        *self == WAVE2R::NOISE
    }
    #[doc = "Checks if the value of the field is `TRIANGLE`"]
    #[inline]
    pub fn is_triangle(&self) -> bool {
        *self == WAVE2R::TRIANGLE
    }
}
#[doc = "Possible values of the field `TSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEL2R {
    #[doc = "Timer 6 TRGO event"]
    TIM6_TRGO,
    #[doc = "Timer 8 TRGO event"]
    TIM8_TRGO,
    #[doc = "Timer 7 TRGO event"]
    TIM7_TRGO,
    #[doc = "Timer 5 TRGO event"]
    TIM5_TRGO,
    #[doc = "Timer 2 TRGO event"]
    TIM2_TRGO,
    #[doc = "Timer 4 TRGO event"]
    TIM4_TRGO,
    #[doc = "EXTI line9"]
    EXTI9,
    #[doc = "Software trigger"]
    SOFTWARE,
}
impl TSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSEL2R::TIM6_TRGO => 0,
            TSEL2R::TIM8_TRGO => 0x01,
            TSEL2R::TIM7_TRGO => 0x02,
            TSEL2R::TIM5_TRGO => 0x03,
            TSEL2R::TIM2_TRGO => 0x04,
            TSEL2R::TIM4_TRGO => 0x05,
            TSEL2R::EXTI9 => 0x06,
            TSEL2R::SOFTWARE => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSEL2R {
        match value {
            0 => TSEL2R::TIM6_TRGO,
            1 => TSEL2R::TIM8_TRGO,
            2 => TSEL2R::TIM7_TRGO,
            3 => TSEL2R::TIM5_TRGO,
            4 => TSEL2R::TIM2_TRGO,
            5 => TSEL2R::TIM4_TRGO,
            6 => TSEL2R::EXTI9,
            7 => TSEL2R::SOFTWARE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIM6_TRGO`"]
    #[inline]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == TSEL2R::TIM6_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM8_TRGO`"]
    #[inline]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == TSEL2R::TIM8_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM7_TRGO`"]
    #[inline]
    pub fn is_tim7_trgo(&self) -> bool {
        *self == TSEL2R::TIM7_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM5_TRGO`"]
    #[inline]
    pub fn is_tim5_trgo(&self) -> bool {
        *self == TSEL2R::TIM5_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM2_TRGO`"]
    #[inline]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL2R::TIM2_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM4_TRGO`"]
    #[inline]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == TSEL2R::TIM4_TRGO
    }
    #[doc = "Checks if the value of the field is `EXTI9`"]
    #[inline]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL2R::EXTI9
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline]
    pub fn is_software(&self) -> bool {
        *self == TSEL2R::SOFTWARE
    }
}
#[doc = "Possible values of the field `TEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN2R {
    #[doc = "DAC channel2 trigger disabled"]
    DISABLED,
    #[doc = "DAC channel2 trigger enabled"]
    ENABLED,
}
impl TEN2R {
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
            TEN2R::DISABLED => false,
            TEN2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEN2R {
        match value {
            false => TEN2R::DISABLED,
            true => TEN2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TEN2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TEN2R::ENABLED
    }
}
#[doc = "Possible values of the field `BOFF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFF2R {
    #[doc = "DAC channel2 output buffer enabled"]
    ENABLED,
    #[doc = "DAC channel2 output buffer disabled"]
    DISABLED,
}
impl BOFF2R {
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
            BOFF2R::ENABLED => false,
            BOFF2R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFF2R {
        match value {
            false => BOFF2R::ENABLED,
            true => BOFF2R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BOFF2R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BOFF2R::DISABLED
    }
}
#[doc = "Possible values of the field `EN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN2R {
    #[doc = "DAC channel2 disabled"]
    DISABLED,
    #[doc = "DAC channel2 enabled"]
    ENABLED,
}
impl EN2R {
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
            EN2R::DISABLED => false,
            EN2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN2R {
        match value {
            false => EN2R::DISABLED,
            true => EN2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EN2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EN2R::ENABLED
    }
}
#[doc = "Possible values of the field `DMAUDRIE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDRIE1R {
    #[doc = "DAC channel1 DMA Underrun Interrupt disabled"]
    DISABLED,
    #[doc = "DAC channel1 DMA Underrun Interrupt enabled"]
    ENABLED,
}
impl DMAUDRIE1R {
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
            DMAUDRIE1R::DISABLED => false,
            DMAUDRIE1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAUDRIE1R {
        match value {
            false => DMAUDRIE1R::DISABLED,
            true => DMAUDRIE1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMAUDRIE1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMAUDRIE1R::ENABLED
    }
}
#[doc = "Possible values of the field `DMAEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN1R {
    #[doc = "DAC channel1 DMA mode disabled"]
    DISABLED,
    #[doc = "DAC channel1 DMA mode enabled"]
    ENABLED,
}
impl DMAEN1R {
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
            DMAEN1R::DISABLED => false,
            DMAEN1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAEN1R {
        match value {
            false => DMAEN1R::DISABLED,
            true => DMAEN1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN1R::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct MAMP1R {
    bits: u8,
}
impl MAMP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WAVE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVE1R {
    #[doc = "Wave generation disabled"]
    DISABLED,
    #[doc = "Noise wave generation enabled"]
    NOISE,
    #[doc = "Triangle wave generation enabled"]
    TRIANGLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WAVE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAVE1R::DISABLED => 0,
            WAVE1R::NOISE => 0x01,
            WAVE1R::TRIANGLE => 0x02,
            WAVE1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAVE1R {
        match value {
            0 => WAVE1R::DISABLED,
            1 => WAVE1R::NOISE,
            2 => WAVE1R::TRIANGLE,
            i => WAVE1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WAVE1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `NOISE`"]
    #[inline]
    pub fn is_noise(&self) -> bool {
        *self == WAVE1R::NOISE
    }
    #[doc = "Checks if the value of the field is `TRIANGLE`"]
    #[inline]
    pub fn is_triangle(&self) -> bool {
        *self == WAVE1R::TRIANGLE
    }
}
#[doc = "Possible values of the field `TSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEL1R {
    #[doc = "Timer 6 TRGO event"]
    TIM6_TRGO,
    #[doc = "Timer 3 TRGO event"]
    TIM3_TRGO,
    #[doc = "Timer 7 TRGO event"]
    TIM7_TRGO,
    #[doc = "Timer 15 TRGO event"]
    TIM15_TRGO,
    #[doc = "Timer 2 TRGO event"]
    TIM2_TRGO,
    #[doc = "EXTI line9"]
    EXTI9,
    #[doc = "Software trigger"]
    SOFTWARE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSEL1R::TIM6_TRGO => 0,
            TSEL1R::TIM3_TRGO => 0x01,
            TSEL1R::TIM7_TRGO => 0x02,
            TSEL1R::TIM15_TRGO => 0x03,
            TSEL1R::TIM2_TRGO => 0x04,
            TSEL1R::EXTI9 => 0x06,
            TSEL1R::SOFTWARE => 0x07,
            TSEL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSEL1R {
        match value {
            0 => TSEL1R::TIM6_TRGO,
            1 => TSEL1R::TIM3_TRGO,
            2 => TSEL1R::TIM7_TRGO,
            3 => TSEL1R::TIM15_TRGO,
            4 => TSEL1R::TIM2_TRGO,
            6 => TSEL1R::EXTI9,
            7 => TSEL1R::SOFTWARE,
            i => TSEL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM6_TRGO`"]
    #[inline]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == TSEL1R::TIM6_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM3_TRGO`"]
    #[inline]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == TSEL1R::TIM3_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM7_TRGO`"]
    #[inline]
    pub fn is_tim7_trgo(&self) -> bool {
        *self == TSEL1R::TIM7_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM15_TRGO`"]
    #[inline]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == TSEL1R::TIM15_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM2_TRGO`"]
    #[inline]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL1R::TIM2_TRGO
    }
    #[doc = "Checks if the value of the field is `EXTI9`"]
    #[inline]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL1R::EXTI9
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline]
    pub fn is_software(&self) -> bool {
        *self == TSEL1R::SOFTWARE
    }
}
#[doc = "Possible values of the field `TEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN1R {
    #[doc = "DAC channel1 trigger disabled"]
    DISABLED,
    #[doc = "DAC channel1 trigger enabled"]
    ENABLED,
}
impl TEN1R {
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
            TEN1R::DISABLED => false,
            TEN1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEN1R {
        match value {
            false => TEN1R::DISABLED,
            true => TEN1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TEN1R::ENABLED
    }
}
#[doc = "Possible values of the field `BOFF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFF1R {
    #[doc = "DAC channel1 output buffer enabled"]
    ENABLED,
    #[doc = "DAC channel1 output buffer disabled"]
    DISABLED,
}
impl BOFF1R {
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
            BOFF1R::ENABLED => false,
            BOFF1R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFF1R {
        match value {
            false => BOFF1R::ENABLED,
            true => BOFF1R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BOFF1R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BOFF1R::DISABLED
    }
}
#[doc = "Possible values of the field `EN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1R {
    #[doc = "DAC channel1 disabled"]
    DISABLED,
    #[doc = "DAC channel1 enabled"]
    ENABLED,
}
impl EN1R {
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
            EN1R::DISABLED => false,
            EN1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN1R {
        match value {
            false => EN1R::DISABLED,
            true => EN1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EN1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EN1R::ENABLED
    }
}
#[doc = "Values that can be written to the field `DMAUDRIE2`"]
pub enum DMAUDRIE2W {
    #[doc = "DAC channel2 DMA Underrun Interrupt disabled"]
    DISABLED,
    #[doc = "DAC channel2 DMA Underrun Interrupt enabled"]
    ENABLED,
}
impl DMAUDRIE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAUDRIE2W::DISABLED => false,
            DMAUDRIE2W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAUDRIE2W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAUDRIE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAUDRIE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel2 DMA Underrun Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAUDRIE2W::DISABLED)
    }
    #[doc = "DAC channel2 DMA Underrun Interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAUDRIE2W::ENABLED)
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
#[doc = "Values that can be written to the field `DMAEN2`"]
pub enum DMAEN2W {
    #[doc = "DAC channel2 DMA mode disabled"]
    DISABLED,
    #[doc = "DAC channel2 DMA mode enabled"]
    ENABLED,
}
impl DMAEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAEN2W::DISABLED => false,
            DMAEN2W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel2 DMA mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN2W::DISABLED)
    }
    #[doc = "DAC channel2 DMA mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN2W::ENABLED)
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
#[doc = r" Proxy"]
pub struct _MAMP2W<'a> {
    w: &'a mut W,
}
impl<'a> _MAMP2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAVE2`"]
pub enum WAVE2W {
    #[doc = "Wave generation disabled"]
    DISABLED,
    #[doc = "Noise wave generation enabled"]
    NOISE,
    #[doc = "Triangle wave generation enabled"]
    TRIANGLE,
}
impl WAVE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAVE2W::DISABLED => 0,
            WAVE2W::NOISE => 1,
            WAVE2W::TRIANGLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAVE2W<'a> {
    w: &'a mut W,
}
impl<'a> _WAVE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAVE2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Wave generation disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAVE2W::DISABLED)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline]
    pub fn noise(self) -> &'a mut W {
        self.variant(WAVE2W::NOISE)
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline]
    pub fn triangle(self) -> &'a mut W {
        self.variant(WAVE2W::TRIANGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSEL2`"]
pub enum TSEL2W {
    #[doc = "Timer 6 TRGO event"]
    TIM6_TRGO,
    #[doc = "Timer 8 TRGO event"]
    TIM8_TRGO,
    #[doc = "Timer 7 TRGO event"]
    TIM7_TRGO,
    #[doc = "Timer 5 TRGO event"]
    TIM5_TRGO,
    #[doc = "Timer 2 TRGO event"]
    TIM2_TRGO,
    #[doc = "Timer 4 TRGO event"]
    TIM4_TRGO,
    #[doc = "EXTI line9"]
    EXTI9,
    #[doc = "Software trigger"]
    SOFTWARE,
}
impl TSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSEL2W::TIM6_TRGO => 0,
            TSEL2W::TIM8_TRGO => 1,
            TSEL2W::TIM7_TRGO => 2,
            TSEL2W::TIM5_TRGO => 3,
            TSEL2W::TIM2_TRGO => 4,
            TSEL2W::TIM4_TRGO => 5,
            TSEL2W::EXTI9 => 6,
            TSEL2W::SOFTWARE => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _TSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSEL2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(TSEL2W::TIM6_TRGO)
    }
    #[doc = "Timer 8 TRGO event"]
    #[inline]
    pub fn tim8_trgo(self) -> &'a mut W {
        self.variant(TSEL2W::TIM8_TRGO)
    }
    #[doc = "Timer 7 TRGO event"]
    #[inline]
    pub fn tim7_trgo(self) -> &'a mut W {
        self.variant(TSEL2W::TIM7_TRGO)
    }
    #[doc = "Timer 5 TRGO event"]
    #[inline]
    pub fn tim5_trgo(self) -> &'a mut W {
        self.variant(TSEL2W::TIM5_TRGO)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL2W::TIM2_TRGO)
    }
    #[doc = "Timer 4 TRGO event"]
    #[inline]
    pub fn tim4_trgo(self) -> &'a mut W {
        self.variant(TSEL2W::TIM4_TRGO)
    }
    #[doc = "EXTI line9"]
    #[inline]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL2W::EXTI9)
    }
    #[doc = "Software trigger"]
    #[inline]
    pub fn software(self) -> &'a mut W {
        self.variant(TSEL2W::SOFTWARE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TEN2`"]
pub enum TEN2W {
    #[doc = "DAC channel2 trigger disabled"]
    DISABLED,
    #[doc = "DAC channel2 trigger enabled"]
    ENABLED,
}
impl TEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEN2W::DISABLED => false,
            TEN2W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _TEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel2 trigger disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN2W::DISABLED)
    }
    #[doc = "DAC channel2 trigger enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN2W::ENABLED)
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
#[doc = "Values that can be written to the field `BOFF2`"]
pub enum BOFF2W {
    #[doc = "DAC channel2 output buffer enabled"]
    ENABLED,
    #[doc = "DAC channel2 output buffer disabled"]
    DISABLED,
}
impl BOFF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFF2W::ENABLED => false,
            BOFF2W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFF2W<'a> {
    w: &'a mut W,
}
impl<'a> _BOFF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel2 output buffer enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOFF2W::ENABLED)
    }
    #[doc = "DAC channel2 output buffer disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOFF2W::DISABLED)
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
#[doc = "Values that can be written to the field `EN2`"]
pub enum EN2W {
    #[doc = "DAC channel2 disabled"]
    DISABLED,
    #[doc = "DAC channel2 enabled"]
    ENABLED,
}
impl EN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN2W::DISABLED => false,
            EN2W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN2W<'a> {
    w: &'a mut W,
}
impl<'a> _EN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel2 disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN2W::DISABLED)
    }
    #[doc = "DAC channel2 enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN2W::ENABLED)
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
#[doc = "Values that can be written to the field `DMAUDRIE1`"]
pub enum DMAUDRIE1W {
    #[doc = "DAC channel1 DMA Underrun Interrupt disabled"]
    DISABLED,
    #[doc = "DAC channel1 DMA Underrun Interrupt enabled"]
    ENABLED,
}
impl DMAUDRIE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAUDRIE1W::DISABLED => false,
            DMAUDRIE1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAUDRIE1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAUDRIE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAUDRIE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1W::DISABLED)
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1W::ENABLED)
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
#[doc = "Values that can be written to the field `DMAEN1`"]
pub enum DMAEN1W {
    #[doc = "DAC channel1 DMA mode disabled"]
    DISABLED,
    #[doc = "DAC channel1 DMA mode enabled"]
    ENABLED,
}
impl DMAEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAEN1W::DISABLED => false,
            DMAEN1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel1 DMA mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN1W::DISABLED)
    }
    #[doc = "DAC channel1 DMA mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN1W::ENABLED)
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
pub struct _MAMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _MAMP1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAVE1`"]
pub enum WAVE1W {
    #[doc = "Wave generation disabled"]
    DISABLED,
    #[doc = "Noise wave generation enabled"]
    NOISE,
    #[doc = "Triangle wave generation enabled"]
    TRIANGLE,
}
impl WAVE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAVE1W::DISABLED => 0,
            WAVE1W::NOISE => 1,
            WAVE1W::TRIANGLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAVE1W<'a> {
    w: &'a mut W,
}
impl<'a> _WAVE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAVE1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Wave generation disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAVE1W::DISABLED)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline]
    pub fn noise(self) -> &'a mut W {
        self.variant(WAVE1W::NOISE)
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline]
    pub fn triangle(self) -> &'a mut W {
        self.variant(WAVE1W::TRIANGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSEL1`"]
pub enum TSEL1W {
    #[doc = "Timer 6 TRGO event"]
    TIM6_TRGO,
    #[doc = "Timer 3 TRGO event"]
    TIM3_TRGO,
    #[doc = "Timer 7 TRGO event"]
    TIM7_TRGO,
    #[doc = "Timer 15 TRGO event"]
    TIM15_TRGO,
    #[doc = "Timer 2 TRGO event"]
    TIM2_TRGO,
    #[doc = "EXTI line9"]
    EXTI9,
    #[doc = "Software trigger"]
    SOFTWARE,
}
impl TSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSEL1W::TIM6_TRGO => 0,
            TSEL1W::TIM3_TRGO => 1,
            TSEL1W::TIM7_TRGO => 2,
            TSEL1W::TIM15_TRGO => 3,
            TSEL1W::TIM2_TRGO => 4,
            TSEL1W::EXTI9 => 6,
            TSEL1W::SOFTWARE => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _TSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSEL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(TSEL1W::TIM6_TRGO)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(TSEL1W::TIM3_TRGO)
    }
    #[doc = "Timer 7 TRGO event"]
    #[inline]
    pub fn tim7_trgo(self) -> &'a mut W {
        self.variant(TSEL1W::TIM7_TRGO)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(TSEL1W::TIM15_TRGO)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL1W::TIM2_TRGO)
    }
    #[doc = "EXTI line9"]
    #[inline]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL1W::EXTI9)
    }
    #[doc = "Software trigger"]
    #[inline]
    pub fn software(self) -> &'a mut W {
        self.variant(TSEL1W::SOFTWARE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TEN1`"]
pub enum TEN1W {
    #[doc = "DAC channel1 trigger disabled"]
    DISABLED,
    #[doc = "DAC channel1 trigger enabled"]
    ENABLED,
}
impl TEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEN1W::DISABLED => false,
            TEN1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _TEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel1 trigger disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN1W::DISABLED)
    }
    #[doc = "DAC channel1 trigger enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN1W::ENABLED)
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
#[doc = "Values that can be written to the field `BOFF1`"]
pub enum BOFF1W {
    #[doc = "DAC channel1 output buffer enabled"]
    ENABLED,
    #[doc = "DAC channel1 output buffer disabled"]
    DISABLED,
}
impl BOFF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFF1W::ENABLED => false,
            BOFF1W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFF1W<'a> {
    w: &'a mut W,
}
impl<'a> _BOFF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel1 output buffer enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOFF1W::ENABLED)
    }
    #[doc = "DAC channel1 output buffer disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOFF1W::DISABLED)
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
#[doc = "Values that can be written to the field `EN1`"]
pub enum EN1W {
    #[doc = "DAC channel1 disabled"]
    DISABLED,
    #[doc = "DAC channel1 enabled"]
    ENABLED,
}
impl EN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN1W::DISABLED => false,
            EN1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN1W<'a> {
    w: &'a mut W,
}
impl<'a> _EN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel1 disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN1W::DISABLED)
    }
    #[doc = "DAC channel1 enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN1W::ENABLED)
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
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline]
    pub fn dmaudrie2(&self) -> DMAUDRIE2R {
        DMAUDRIE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline]
    pub fn dmaen2(&self) -> DMAEN2R {
        DMAEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline]
    pub fn mamp2(&self) -> MAMP2R {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAMP2R { bits }
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline]
    pub fn wave2(&self) -> WAVE2R {
        WAVE2R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - DAC channel2 trigger selection"]
    #[inline]
    pub fn tsel2(&self) -> TSEL2R {
        TSEL2R::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - DAC channel2 trigger enable"]
    #[inline]
    pub fn ten2(&self) -> TEN2R {
        TEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - DAC channel2 output buffer disable"]
    #[inline]
    pub fn boff2(&self) -> BOFF2R {
        BOFF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline]
    pub fn en2(&self) -> EN2R {
        EN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline]
    pub fn dmaudrie1(&self) -> DMAUDRIE1R {
        DMAUDRIE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline]
    pub fn dmaen1(&self) -> DMAEN1R {
        DMAEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline]
    pub fn mamp1(&self) -> MAMP1R {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAMP1R { bits }
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline]
    pub fn wave1(&self) -> WAVE1R {
        WAVE1R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline]
    pub fn tsel1(&self) -> TSEL1R {
        TSEL1R::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline]
    pub fn ten1(&self) -> TEN1R {
        TEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline]
    pub fn boff1(&self) -> BOFF1R {
        BOFF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline]
    pub fn en1(&self) -> EN1R {
        EN1R::_from({
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
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline]
    pub fn dmaudrie2(&mut self) -> _DMAUDRIE2W {
        _DMAUDRIE2W { w: self }
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline]
    pub fn dmaen2(&mut self) -> _DMAEN2W {
        _DMAEN2W { w: self }
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline]
    pub fn mamp2(&mut self) -> _MAMP2W {
        _MAMP2W { w: self }
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline]
    pub fn wave2(&mut self) -> _WAVE2W {
        _WAVE2W { w: self }
    }
    #[doc = "Bits 19:21 - DAC channel2 trigger selection"]
    #[inline]
    pub fn tsel2(&mut self) -> _TSEL2W {
        _TSEL2W { w: self }
    }
    #[doc = "Bit 18 - DAC channel2 trigger enable"]
    #[inline]
    pub fn ten2(&mut self) -> _TEN2W {
        _TEN2W { w: self }
    }
    #[doc = "Bit 17 - DAC channel2 output buffer disable"]
    #[inline]
    pub fn boff2(&mut self) -> _BOFF2W {
        _BOFF2W { w: self }
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline]
    pub fn en2(&mut self) -> _EN2W {
        _EN2W { w: self }
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline]
    pub fn dmaudrie1(&mut self) -> _DMAUDRIE1W {
        _DMAUDRIE1W { w: self }
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline]
    pub fn dmaen1(&mut self) -> _DMAEN1W {
        _DMAEN1W { w: self }
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline]
    pub fn mamp1(&mut self) -> _MAMP1W {
        _MAMP1W { w: self }
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline]
    pub fn wave1(&mut self) -> _WAVE1W {
        _WAVE1W { w: self }
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline]
    pub fn tsel1(&mut self) -> _TSEL1W {
        _TSEL1W { w: self }
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline]
    pub fn ten1(&mut self) -> _TEN1W {
        _TEN1W { w: self }
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline]
    pub fn boff1(&mut self) -> _BOFF1W {
        _BOFF1W { w: self }
    }
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline]
    pub fn en1(&mut self) -> _EN1W {
        _EN1W { w: self }
    }
}
