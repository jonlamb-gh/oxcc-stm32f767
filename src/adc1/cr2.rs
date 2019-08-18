#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR2 {
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
#[doc = "Possible values of the field `SWSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSTARTR {
    #[doc = "Starts conversion of regular channels"]
    START,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SWSTARTR {
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
            SWSTARTR::START => true,
            SWSTARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWSTARTR {
        match value {
            true => SWSTARTR::START,
            i => SWSTARTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == SWSTARTR::START
    }
}
#[doc = "Possible values of the field `EXTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTENR {
    #[doc = "Trigger detection disabled"]
    DISABLED,
    #[doc = "Trigger detection on the rising edge"]
    RISINGEDGE,
    #[doc = "Trigger detection on the falling edge"]
    FALLINGEDGE,
    #[doc = "Trigger detection on both the rising and falling edges"]
    BOTHEDGES,
}
impl EXTENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTENR::DISABLED => 0,
            EXTENR::RISINGEDGE => 0x01,
            EXTENR::FALLINGEDGE => 0x02,
            EXTENR::BOTHEDGES => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTENR {
        match value {
            0 => EXTENR::DISABLED,
            1 => EXTENR::RISINGEDGE,
            2 => EXTENR::FALLINGEDGE,
            3 => EXTENR::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EXTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTENR::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTENR::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTENR::BOTHEDGES
    }
}
#[doc = "Possible values of the field `EXTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSELR {
    #[doc = "Timer 1 CC1 event"]
    TIM1CC1,
    #[doc = "Timer 1 CC2 event"]
    TIM1CC2,
    #[doc = "Timer 1 CC3 event"]
    TIM1CC3,
    #[doc = "Timer 2 CC2 event"]
    TIM2CC2,
    #[doc = "Timer 2 CC3 event"]
    TIM2CC3,
    #[doc = "Timer 2 CC4 event"]
    TIM2CC4,
    #[doc = "Timer 2 TRGO event"]
    TIM2TRGO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTSELR::TIM1CC1 => 0,
            EXTSELR::TIM1CC2 => 0x01,
            EXTSELR::TIM1CC3 => 0x02,
            EXTSELR::TIM2CC2 => 0x03,
            EXTSELR::TIM2CC3 => 0x04,
            EXTSELR::TIM2CC4 => 0x05,
            EXTSELR::TIM2TRGO => 0x06,
            EXTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTSELR {
        match value {
            0 => EXTSELR::TIM1CC1,
            1 => EXTSELR::TIM1CC2,
            2 => EXTSELR::TIM1CC3,
            3 => EXTSELR::TIM2CC2,
            4 => EXTSELR::TIM2CC3,
            5 => EXTSELR::TIM2CC4,
            6 => EXTSELR::TIM2TRGO,
            i => EXTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM1CC1`"]
    #[inline]
    pub fn is_tim1cc1(&self) -> bool {
        *self == EXTSELR::TIM1CC1
    }
    #[doc = "Checks if the value of the field is `TIM1CC2`"]
    #[inline]
    pub fn is_tim1cc2(&self) -> bool {
        *self == EXTSELR::TIM1CC2
    }
    #[doc = "Checks if the value of the field is `TIM1CC3`"]
    #[inline]
    pub fn is_tim1cc3(&self) -> bool {
        *self == EXTSELR::TIM1CC3
    }
    #[doc = "Checks if the value of the field is `TIM2CC2`"]
    #[inline]
    pub fn is_tim2cc2(&self) -> bool {
        *self == EXTSELR::TIM2CC2
    }
    #[doc = "Checks if the value of the field is `TIM2CC3`"]
    #[inline]
    pub fn is_tim2cc3(&self) -> bool {
        *self == EXTSELR::TIM2CC3
    }
    #[doc = "Checks if the value of the field is `TIM2CC4`"]
    #[inline]
    pub fn is_tim2cc4(&self) -> bool {
        *self == EXTSELR::TIM2CC4
    }
    #[doc = "Checks if the value of the field is `TIM2TRGO`"]
    #[inline]
    pub fn is_tim2trgo(&self) -> bool {
        *self == EXTSELR::TIM2TRGO
    }
}
#[doc = "Possible values of the field `JSWSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSWSTARTR {
    #[doc = "Starts conversion of injected channels"]
    START,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl JSWSTARTR {
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
            JSWSTARTR::START => true,
            JSWSTARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JSWSTARTR {
        match value {
            true => JSWSTARTR::START,
            i => JSWSTARTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == JSWSTARTR::START
    }
}
#[doc = "Possible values of the field `JEXTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEXTENR {
    #[doc = "Trigger detection disabled"]
    DISABLED,
    #[doc = "Trigger detection on the rising edge"]
    RISINGEDGE,
    #[doc = "Trigger detection on the falling edge"]
    FALLINGEDGE,
    #[doc = "Trigger detection on both the rising and falling edges"]
    BOTHEDGES,
}
impl JEXTENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            JEXTENR::DISABLED => 0,
            JEXTENR::RISINGEDGE => 0x01,
            JEXTENR::FALLINGEDGE => 0x02,
            JEXTENR::BOTHEDGES => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> JEXTENR {
        match value {
            0 => JEXTENR::DISABLED,
            1 => JEXTENR::RISINGEDGE,
            2 => JEXTENR::FALLINGEDGE,
            3 => JEXTENR::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTENR::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTENR::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTENR::BOTHEDGES
    }
}
#[doc = "Possible values of the field `JEXTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEXTSELR {
    #[doc = "Timer 1 TRGO event"]
    TIM1TRGO,
    #[doc = "Timer 1 CC4 event"]
    TIM1CC4,
    #[doc = "Timer 2 TRGO event"]
    TIM2TRGO,
    #[doc = "Timer 2 CC1 event"]
    TIM2CC1,
    #[doc = "Timer 3 CC4 event"]
    TIM3CC4,
    #[doc = "Timer 4 TRGO event"]
    TIM4TRGO,
    #[doc = "Timer 8 CC4 event"]
    TIM8CC4,
    #[doc = "Timer 1 TRGO(2) event"]
    TIM1TRGO2,
    #[doc = "Timer 8 TRGO event"]
    TIM8TRGO,
    #[doc = "Timer 8 TRGO(2) event"]
    TIM8TRGO2,
    #[doc = "Timer 3 CC3 event"]
    TIM3CC3,
    #[doc = "Timer 5 TRGO event"]
    TIM5TRGO,
    #[doc = "Timer 3 CC1 event"]
    TIM3CC1,
    #[doc = "Timer 6 TRGO event"]
    TIM6TRGO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl JEXTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            JEXTSELR::TIM1TRGO => 0,
            JEXTSELR::TIM1CC4 => 0x01,
            JEXTSELR::TIM2TRGO => 0x02,
            JEXTSELR::TIM2CC1 => 0x03,
            JEXTSELR::TIM3CC4 => 0x04,
            JEXTSELR::TIM4TRGO => 0x05,
            JEXTSELR::TIM8CC4 => 0x07,
            JEXTSELR::TIM1TRGO2 => 0x08,
            JEXTSELR::TIM8TRGO => 0x09,
            JEXTSELR::TIM8TRGO2 => 0x0a,
            JEXTSELR::TIM3CC3 => 0x0b,
            JEXTSELR::TIM5TRGO => 0x0c,
            JEXTSELR::TIM3CC1 => 0x0d,
            JEXTSELR::TIM6TRGO => 0x0e,
            JEXTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> JEXTSELR {
        match value {
            0 => JEXTSELR::TIM1TRGO,
            1 => JEXTSELR::TIM1CC4,
            2 => JEXTSELR::TIM2TRGO,
            3 => JEXTSELR::TIM2CC1,
            4 => JEXTSELR::TIM3CC4,
            5 => JEXTSELR::TIM4TRGO,
            7 => JEXTSELR::TIM8CC4,
            8 => JEXTSELR::TIM1TRGO2,
            9 => JEXTSELR::TIM8TRGO,
            10 => JEXTSELR::TIM8TRGO2,
            11 => JEXTSELR::TIM3CC3,
            12 => JEXTSELR::TIM5TRGO,
            13 => JEXTSELR::TIM3CC1,
            14 => JEXTSELR::TIM6TRGO,
            i => JEXTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM1TRGO`"]
    #[inline]
    pub fn is_tim1trgo(&self) -> bool {
        *self == JEXTSELR::TIM1TRGO
    }
    #[doc = "Checks if the value of the field is `TIM1CC4`"]
    #[inline]
    pub fn is_tim1cc4(&self) -> bool {
        *self == JEXTSELR::TIM1CC4
    }
    #[doc = "Checks if the value of the field is `TIM2TRGO`"]
    #[inline]
    pub fn is_tim2trgo(&self) -> bool {
        *self == JEXTSELR::TIM2TRGO
    }
    #[doc = "Checks if the value of the field is `TIM2CC1`"]
    #[inline]
    pub fn is_tim2cc1(&self) -> bool {
        *self == JEXTSELR::TIM2CC1
    }
    #[doc = "Checks if the value of the field is `TIM3CC4`"]
    #[inline]
    pub fn is_tim3cc4(&self) -> bool {
        *self == JEXTSELR::TIM3CC4
    }
    #[doc = "Checks if the value of the field is `TIM4TRGO`"]
    #[inline]
    pub fn is_tim4trgo(&self) -> bool {
        *self == JEXTSELR::TIM4TRGO
    }
    #[doc = "Checks if the value of the field is `TIM8CC4`"]
    #[inline]
    pub fn is_tim8cc4(&self) -> bool {
        *self == JEXTSELR::TIM8CC4
    }
    #[doc = "Checks if the value of the field is `TIM1TRGO2`"]
    #[inline]
    pub fn is_tim1trgo2(&self) -> bool {
        *self == JEXTSELR::TIM1TRGO2
    }
    #[doc = "Checks if the value of the field is `TIM8TRGO`"]
    #[inline]
    pub fn is_tim8trgo(&self) -> bool {
        *self == JEXTSELR::TIM8TRGO
    }
    #[doc = "Checks if the value of the field is `TIM8TRGO2`"]
    #[inline]
    pub fn is_tim8trgo2(&self) -> bool {
        *self == JEXTSELR::TIM8TRGO2
    }
    #[doc = "Checks if the value of the field is `TIM3CC3`"]
    #[inline]
    pub fn is_tim3cc3(&self) -> bool {
        *self == JEXTSELR::TIM3CC3
    }
    #[doc = "Checks if the value of the field is `TIM5TRGO`"]
    #[inline]
    pub fn is_tim5trgo(&self) -> bool {
        *self == JEXTSELR::TIM5TRGO
    }
    #[doc = "Checks if the value of the field is `TIM3CC1`"]
    #[inline]
    pub fn is_tim3cc1(&self) -> bool {
        *self == JEXTSELR::TIM3CC1
    }
    #[doc = "Checks if the value of the field is `TIM6TRGO`"]
    #[inline]
    pub fn is_tim6trgo(&self) -> bool {
        *self == JEXTSELR::TIM6TRGO
    }
}
#[doc = "Possible values of the field `ALIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGNR {
    #[doc = "Right alignment"]
    RIGHT,
    #[doc = "Left alignment"]
    LEFT,
}
impl ALIGNR {
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
            ALIGNR::RIGHT => false,
            ALIGNR::LEFT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALIGNR {
        match value {
            false => ALIGNR::RIGHT,
            true => ALIGNR::LEFT,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline]
    pub fn is_right(&self) -> bool {
        *self == ALIGNR::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline]
    pub fn is_left(&self) -> bool {
        *self == ALIGNR::LEFT
    }
}
#[doc = "Possible values of the field `EOCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCSR {
    #[doc = "The EOC bit is set at the end of each sequence of regular conversions"]
    EACHSEQUENCE,
    #[doc = "The EOC bit is set at the end of each regular conversion"]
    EACHCONVERSION,
}
impl EOCSR {
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
            EOCSR::EACHSEQUENCE => false,
            EOCSR::EACHCONVERSION => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOCSR {
        match value {
            false => EOCSR::EACHSEQUENCE,
            true => EOCSR::EACHCONVERSION,
        }
    }
    #[doc = "Checks if the value of the field is `EACHSEQUENCE`"]
    #[inline]
    pub fn is_each_sequence(&self) -> bool {
        *self == EOCSR::EACHSEQUENCE
    }
    #[doc = "Checks if the value of the field is `EACHCONVERSION`"]
    #[inline]
    pub fn is_each_conversion(&self) -> bool {
        *self == EOCSR::EACHCONVERSION
    }
}
#[doc = "Possible values of the field `DDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDSR {
    #[doc = "No new DMA request is issued after the last transfer"]
    SINGLE,
    #[doc = "DMA requests are issued as long as data are converted and DMA=1"]
    CONTINUOUS,
}
impl DDSR {
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
            DDSR::SINGLE => false,
            DDSR::CONTINUOUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DDSR {
        match value {
            false => DDSR::SINGLE,
            true => DDSR::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == DDSR::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == DDSR::CONTINUOUS
    }
}
#[doc = "Possible values of the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR {
    #[doc = "DMA mode disabled"]
    DISABLED,
    #[doc = "DMA mode enabled"]
    ENABLED,
}
impl DMAR {
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
            DMAR::DISABLED => false,
            DMAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAR {
        match value {
            false => DMAR::DISABLED,
            true => DMAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR::ENABLED
    }
}
#[doc = "Possible values of the field `CONT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTR {
    #[doc = "Single conversion mode"]
    SINGLE,
    #[doc = "Continuous conversion mode"]
    CONTINUOUS,
}
impl CONTR {
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
            CONTR::SINGLE => false,
            CONTR::CONTINUOUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTR {
        match value {
            false => CONTR::SINGLE,
            true => CONTR::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == CONTR::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == CONTR::CONTINUOUS
    }
}
#[doc = "Possible values of the field `ADON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADONR {
    #[doc = "Disable ADC conversion and go to power down mode"]
    DISABLED,
    #[doc = "Enable ADC"]
    ENABLED,
}
impl ADONR {
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
            ADONR::DISABLED => false,
            ADONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADONR {
        match value {
            false => ADONR::DISABLED,
            true => ADONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADONR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SWSTART`"]
pub enum SWSTARTW {
    #[doc = "Starts conversion of regular channels"]
    START,
}
impl SWSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWSTARTW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWSTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Starts conversion of regular channels"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(SWSTARTW::START)
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
#[doc = "Values that can be written to the field `EXTEN`"]
pub enum EXTENW {
    #[doc = "Trigger detection disabled"]
    DISABLED,
    #[doc = "Trigger detection on the rising edge"]
    RISINGEDGE,
    #[doc = "Trigger detection on the falling edge"]
    FALLINGEDGE,
    #[doc = "Trigger detection on both the rising and falling edges"]
    BOTHEDGES,
}
impl EXTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTENW::DISABLED => 0,
            EXTENW::RISINGEDGE => 1,
            EXTENW::FALLINGEDGE => 2,
            EXTENW::BOTHEDGES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTENW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger detection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTENW::DISABLED)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTENW::RISINGEDGE)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTENW::FALLINGEDGE)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTENW::BOTHEDGES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTSEL`"]
pub enum EXTSELW {
    #[doc = "Timer 1 CC1 event"]
    TIM1CC1,
    #[doc = "Timer 1 CC2 event"]
    TIM1CC2,
    #[doc = "Timer 1 CC3 event"]
    TIM1CC3,
    #[doc = "Timer 2 CC2 event"]
    TIM2CC2,
    #[doc = "Timer 2 CC3 event"]
    TIM2CC3,
    #[doc = "Timer 2 CC4 event"]
    TIM2CC4,
    #[doc = "Timer 2 TRGO event"]
    TIM2TRGO,
}
impl EXTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTSELW::TIM1CC1 => 0,
            EXTSELW::TIM1CC2 => 1,
            EXTSELW::TIM1CC3 => 2,
            EXTSELW::TIM2CC2 => 3,
            EXTSELW::TIM2CC3 => 4,
            EXTSELW::TIM2CC4 => 5,
            EXTSELW::TIM2TRGO => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer 1 CC1 event"]
    #[inline]
    pub fn tim1cc1(self) -> &'a mut W {
        self.variant(EXTSELW::TIM1CC1)
    }
    #[doc = "Timer 1 CC2 event"]
    #[inline]
    pub fn tim1cc2(self) -> &'a mut W {
        self.variant(EXTSELW::TIM1CC2)
    }
    #[doc = "Timer 1 CC3 event"]
    #[inline]
    pub fn tim1cc3(self) -> &'a mut W {
        self.variant(EXTSELW::TIM1CC3)
    }
    #[doc = "Timer 2 CC2 event"]
    #[inline]
    pub fn tim2cc2(self) -> &'a mut W {
        self.variant(EXTSELW::TIM2CC2)
    }
    #[doc = "Timer 2 CC3 event"]
    #[inline]
    pub fn tim2cc3(self) -> &'a mut W {
        self.variant(EXTSELW::TIM2CC3)
    }
    #[doc = "Timer 2 CC4 event"]
    #[inline]
    pub fn tim2cc4(self) -> &'a mut W {
        self.variant(EXTSELW::TIM2CC4)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline]
    pub fn tim2trgo(self) -> &'a mut W {
        self.variant(EXTSELW::TIM2TRGO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `JSWSTART`"]
pub enum JSWSTARTW {
    #[doc = "Starts conversion of injected channels"]
    START,
}
impl JSWSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JSWSTARTW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JSWSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _JSWSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JSWSTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Starts conversion of injected channels"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(JSWSTARTW::START)
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
#[doc = "Values that can be written to the field `JEXTEN`"]
pub enum JEXTENW {
    #[doc = "Trigger detection disabled"]
    DISABLED,
    #[doc = "Trigger detection on the rising edge"]
    RISINGEDGE,
    #[doc = "Trigger detection on the falling edge"]
    FALLINGEDGE,
    #[doc = "Trigger detection on both the rising and falling edges"]
    BOTHEDGES,
}
impl JEXTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            JEXTENW::DISABLED => 0,
            JEXTENW::RISINGEDGE => 1,
            JEXTENW::FALLINGEDGE => 2,
            JEXTENW::BOTHEDGES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JEXTENW<'a> {
    w: &'a mut W,
}
impl<'a> _JEXTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JEXTENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger detection disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTENW::DISABLED)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(JEXTENW::RISINGEDGE)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(JEXTENW::FALLINGEDGE)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(JEXTENW::BOTHEDGES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `JEXTSEL`"]
pub enum JEXTSELW {
    #[doc = "Timer 1 TRGO event"]
    TIM1TRGO,
    #[doc = "Timer 1 CC4 event"]
    TIM1CC4,
    #[doc = "Timer 2 TRGO event"]
    TIM2TRGO,
    #[doc = "Timer 2 CC1 event"]
    TIM2CC1,
    #[doc = "Timer 3 CC4 event"]
    TIM3CC4,
    #[doc = "Timer 4 TRGO event"]
    TIM4TRGO,
    #[doc = "Timer 8 CC4 event"]
    TIM8CC4,
    #[doc = "Timer 1 TRGO(2) event"]
    TIM1TRGO2,
    #[doc = "Timer 8 TRGO event"]
    TIM8TRGO,
    #[doc = "Timer 8 TRGO(2) event"]
    TIM8TRGO2,
    #[doc = "Timer 3 CC3 event"]
    TIM3CC3,
    #[doc = "Timer 5 TRGO event"]
    TIM5TRGO,
    #[doc = "Timer 3 CC1 event"]
    TIM3CC1,
    #[doc = "Timer 6 TRGO event"]
    TIM6TRGO,
}
impl JEXTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            JEXTSELW::TIM1TRGO => 0,
            JEXTSELW::TIM1CC4 => 1,
            JEXTSELW::TIM2TRGO => 2,
            JEXTSELW::TIM2CC1 => 3,
            JEXTSELW::TIM3CC4 => 4,
            JEXTSELW::TIM4TRGO => 5,
            JEXTSELW::TIM8CC4 => 7,
            JEXTSELW::TIM1TRGO2 => 8,
            JEXTSELW::TIM8TRGO => 9,
            JEXTSELW::TIM8TRGO2 => 10,
            JEXTSELW::TIM3CC3 => 11,
            JEXTSELW::TIM5TRGO => 12,
            JEXTSELW::TIM3CC1 => 13,
            JEXTSELW::TIM6TRGO => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JEXTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _JEXTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JEXTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline]
    pub fn tim1trgo(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM1TRGO)
    }
    #[doc = "Timer 1 CC4 event"]
    #[inline]
    pub fn tim1cc4(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM1CC4)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline]
    pub fn tim2trgo(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM2TRGO)
    }
    #[doc = "Timer 2 CC1 event"]
    #[inline]
    pub fn tim2cc1(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM2CC1)
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline]
    pub fn tim3cc4(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM3CC4)
    }
    #[doc = "Timer 4 TRGO event"]
    #[inline]
    pub fn tim4trgo(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM4TRGO)
    }
    #[doc = "Timer 8 CC4 event"]
    #[inline]
    pub fn tim8cc4(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM8CC4)
    }
    #[doc = "Timer 1 TRGO(2) event"]
    #[inline]
    pub fn tim1trgo2(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM1TRGO2)
    }
    #[doc = "Timer 8 TRGO event"]
    #[inline]
    pub fn tim8trgo(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM8TRGO)
    }
    #[doc = "Timer 8 TRGO(2) event"]
    #[inline]
    pub fn tim8trgo2(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM8TRGO2)
    }
    #[doc = "Timer 3 CC3 event"]
    #[inline]
    pub fn tim3cc3(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM3CC3)
    }
    #[doc = "Timer 5 TRGO event"]
    #[inline]
    pub fn tim5trgo(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM5TRGO)
    }
    #[doc = "Timer 3 CC1 event"]
    #[inline]
    pub fn tim3cc1(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM3CC1)
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline]
    pub fn tim6trgo(self) -> &'a mut W {
        self.variant(JEXTSELW::TIM6TRGO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALIGN`"]
pub enum ALIGNW {
    #[doc = "Right alignment"]
    RIGHT,
    #[doc = "Left alignment"]
    LEFT,
}
impl ALIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALIGNW::RIGHT => false,
            ALIGNW::LEFT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _ALIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Right alignment"]
    #[inline]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGNW::RIGHT)
    }
    #[doc = "Left alignment"]
    #[inline]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGNW::LEFT)
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
#[doc = "Values that can be written to the field `EOCS`"]
pub enum EOCSW {
    #[doc = "The EOC bit is set at the end of each sequence of regular conversions"]
    EACHSEQUENCE,
    #[doc = "The EOC bit is set at the end of each regular conversion"]
    EACHCONVERSION,
}
impl EOCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOCSW::EACHSEQUENCE => false,
            EOCSW::EACHCONVERSION => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOCSW<'a> {
    w: &'a mut W,
}
impl<'a> _EOCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The EOC bit is set at the end of each sequence of regular conversions"]
    #[inline]
    pub fn each_sequence(self) -> &'a mut W {
        self.variant(EOCSW::EACHSEQUENCE)
    }
    #[doc = "The EOC bit is set at the end of each regular conversion"]
    #[inline]
    pub fn each_conversion(self) -> &'a mut W {
        self.variant(EOCSW::EACHCONVERSION)
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
#[doc = "Values that can be written to the field `DDS`"]
pub enum DDSW {
    #[doc = "No new DMA request is issued after the last transfer"]
    SINGLE,
    #[doc = "DMA requests are issued as long as data are converted and DMA=1"]
    CONTINUOUS,
}
impl DDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DDSW::SINGLE => false,
            DDSW::CONTINUOUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DDSW<'a> {
    w: &'a mut W,
}
impl<'a> _DDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DDSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No new DMA request is issued after the last transfer"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(DDSW::SINGLE)
    }
    #[doc = "DMA requests are issued as long as data are converted and DMA=1"]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DDSW::CONTINUOUS)
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
#[doc = "Values that can be written to the field `DMA`"]
pub enum DMAW {
    #[doc = "DMA mode disabled"]
    DISABLED,
    #[doc = "DMA mode enabled"]
    ENABLED,
}
impl DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAW::DISABLED => false,
            DMAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAW::DISABLED)
    }
    #[doc = "DMA mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAW::ENABLED)
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
#[doc = "Values that can be written to the field `CONT`"]
pub enum CONTW {
    #[doc = "Single conversion mode"]
    SINGLE,
    #[doc = "Continuous conversion mode"]
    CONTINUOUS,
}
impl CONTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTW::SINGLE => false,
            CONTW::CONTINUOUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single conversion mode"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(CONTW::SINGLE)
    }
    #[doc = "Continuous conversion mode"]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONTW::CONTINUOUS)
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
#[doc = "Values that can be written to the field `ADON`"]
pub enum ADONW {
    #[doc = "Disable ADC conversion and go to power down mode"]
    DISABLED,
    #[doc = "Enable ADC"]
    ENABLED,
}
impl ADONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADONW::DISABLED => false,
            ADONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADONW<'a> {
    w: &'a mut W,
}
impl<'a> _ADONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable ADC conversion and go to power down mode"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADONW::DISABLED)
    }
    #[doc = "Enable ADC"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADONW::ENABLED)
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
    #[doc = "Bit 30 - Start conversion of regular channels"]
    #[inline]
    pub fn swstart(&self) -> SWSTARTR {
        SWSTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:29 - External trigger enable for regular channels"]
    #[inline]
    pub fn exten(&self) -> EXTENR {
        EXTENR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - External event select for regular group"]
    #[inline]
    pub fn extsel(&self) -> EXTSELR {
        EXTSELR::_from({
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Start conversion of injected channels"]
    #[inline]
    pub fn jswstart(&self) -> JSWSTARTR {
        JSWSTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:21 - External trigger enable for injected channels"]
    #[inline]
    pub fn jexten(&self) -> JEXTENR {
        JEXTENR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - External event select for injected group"]
    #[inline]
    pub fn jextsel(&self) -> JEXTSELR {
        JEXTSELR::_from({
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline]
    pub fn align(&self) -> ALIGNR {
        ALIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - End of conversion selection"]
    #[inline]
    pub fn eocs(&self) -> EOCSR {
        EOCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - DMA disable selection (for single ADC mode)"]
    #[inline]
    pub fn dds(&self) -> DDSR {
        DDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Direct memory access mode (for single ADC mode)"]
    #[inline]
    pub fn dma(&self) -> DMAR {
        DMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline]
    pub fn cont(&self) -> CONTR {
        CONTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - A/D Converter ON / OFF"]
    #[inline]
    pub fn adon(&self) -> ADONR {
        ADONR::_from({
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
    #[doc = "Bit 30 - Start conversion of regular channels"]
    #[inline]
    pub fn swstart(&mut self) -> _SWSTARTW {
        _SWSTARTW { w: self }
    }
    #[doc = "Bits 28:29 - External trigger enable for regular channels"]
    #[inline]
    pub fn exten(&mut self) -> _EXTENW {
        _EXTENW { w: self }
    }
    #[doc = "Bits 24:27 - External event select for regular group"]
    #[inline]
    pub fn extsel(&mut self) -> _EXTSELW {
        _EXTSELW { w: self }
    }
    #[doc = "Bit 22 - Start conversion of injected channels"]
    #[inline]
    pub fn jswstart(&mut self) -> _JSWSTARTW {
        _JSWSTARTW { w: self }
    }
    #[doc = "Bits 20:21 - External trigger enable for injected channels"]
    #[inline]
    pub fn jexten(&mut self) -> _JEXTENW {
        _JEXTENW { w: self }
    }
    #[doc = "Bits 16:19 - External event select for injected group"]
    #[inline]
    pub fn jextsel(&mut self) -> _JEXTSELW {
        _JEXTSELW { w: self }
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline]
    pub fn align(&mut self) -> _ALIGNW {
        _ALIGNW { w: self }
    }
    #[doc = "Bit 10 - End of conversion selection"]
    #[inline]
    pub fn eocs(&mut self) -> _EOCSW {
        _EOCSW { w: self }
    }
    #[doc = "Bit 9 - DMA disable selection (for single ADC mode)"]
    #[inline]
    pub fn dds(&mut self) -> _DDSW {
        _DDSW { w: self }
    }
    #[doc = "Bit 8 - Direct memory access mode (for single ADC mode)"]
    #[inline]
    pub fn dma(&mut self) -> _DMAW {
        _DMAW { w: self }
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
    #[doc = "Bit 0 - A/D Converter ON / OFF"]
    #[inline]
    pub fn adon(&mut self) -> _ADONW {
        _ADONW { w: self }
    }
}
