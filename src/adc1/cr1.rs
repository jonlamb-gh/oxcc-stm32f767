#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
#[doc = "Possible values of the field `OVRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRIER {
    #[doc = "Overrun interrupt disabled"]
    DISABLED,
    #[doc = "Overrun interrupt enabled"]
    ENABLED,
}
impl OVRIER {
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
            OVRIER::DISABLED => false,
            OVRIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVRIER {
        match value {
            false => OVRIER::DISABLED,
            true => OVRIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OVRIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OVRIER::ENABLED
    }
}
#[doc = "Possible values of the field `RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESR {
    #[doc = "12-bit (15 ADCCLK cycles)"]
    TWELVEBIT,
    #[doc = "10-bit (13 ADCCLK cycles)"]
    TENBIT,
    #[doc = "8-bit (11 ADCCLK cycles)"]
    EIGHTBIT,
    #[doc = "6-bit (9 ADCCLK cycles)"]
    SIXBIT,
}
impl RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESR::TWELVEBIT => 0,
            RESR::TENBIT => 0x01,
            RESR::EIGHTBIT => 0x02,
            RESR::SIXBIT => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESR {
        match value {
            0 => RESR::TWELVEBIT,
            1 => RESR::TENBIT,
            2 => RESR::EIGHTBIT,
            3 => RESR::SIXBIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TWELVEBIT`"]
    #[inline]
    pub fn is_twelve_bit(&self) -> bool {
        *self == RESR::TWELVEBIT
    }
    #[doc = "Checks if the value of the field is `TENBIT`"]
    #[inline]
    pub fn is_ten_bit(&self) -> bool {
        *self == RESR::TENBIT
    }
    #[doc = "Checks if the value of the field is `EIGHTBIT`"]
    #[inline]
    pub fn is_eight_bit(&self) -> bool {
        *self == RESR::EIGHTBIT
    }
    #[doc = "Checks if the value of the field is `SIXBIT`"]
    #[inline]
    pub fn is_six_bit(&self) -> bool {
        *self == RESR::SIXBIT
    }
}
#[doc = "Possible values of the field `AWDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDENR {
    #[doc = "Analog watchdog disabled on regular channels"]
    DISABLED,
    #[doc = "Analog watchdog enabled on regular channels"]
    ENABLED,
}
impl AWDENR {
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
            AWDENR::DISABLED => false,
            AWDENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AWDENR {
        match value {
            false => AWDENR::DISABLED,
            true => AWDENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == AWDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == AWDENR::ENABLED
    }
}
#[doc = "Possible values of the field `JAWDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAWDENR {
    #[doc = "Analog watchdog disabled on injected channels"]
    DISABLED,
    #[doc = "Analog watchdog enabled on injected channels"]
    ENABLED,
}
impl JAWDENR {
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
            JAWDENR::DISABLED => false,
            JAWDENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JAWDENR {
        match value {
            false => JAWDENR::DISABLED,
            true => JAWDENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == JAWDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == JAWDENR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct DISCNUMR {
    bits: u8,
}
impl DISCNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `JDISCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JDISCENR {
    #[doc = "Discontinuous mode on injected channels disabled"]
    DISABLED,
    #[doc = "Discontinuous mode on injected channels enabled"]
    ENABLED,
}
impl JDISCENR {
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
            JDISCENR::DISABLED => false,
            JDISCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JDISCENR {
        match value {
            false => JDISCENR::DISABLED,
            true => JDISCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCENR::ENABLED
    }
}
#[doc = "Possible values of the field `DISCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCENR {
    #[doc = "Discontinuous mode on regular channels disabled"]
    DISABLED,
    #[doc = "Discontinuous mode on regular channels enabled"]
    ENABLED,
}
impl DISCENR {
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
            DISCENR::DISABLED => false,
            DISCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISCENR {
        match value {
            false => DISCENR::DISABLED,
            true => DISCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DISCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DISCENR::ENABLED
    }
}
#[doc = "Possible values of the field `JAUTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAUTOR {
    #[doc = "Automatic injected group conversion disabled"]
    DISABLED,
    #[doc = "Automatic injected group conversion enabled"]
    ENABLED,
}
impl JAUTOR {
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
            JAUTOR::DISABLED => false,
            JAUTOR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JAUTOR {
        match value {
            false => JAUTOR::DISABLED,
            true => JAUTOR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTOR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTOR::ENABLED
    }
}
#[doc = "Possible values of the field `AWDSGL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDSGLR {
    #[doc = "Analog watchdog enabled on all channels"]
    ALLCHANNELS,
    #[doc = "Analog watchdog enabled on a single channel"]
    SINGLECHANNEL,
}
impl AWDSGLR {
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
            AWDSGLR::ALLCHANNELS => false,
            AWDSGLR::SINGLECHANNEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AWDSGLR {
        match value {
            false => AWDSGLR::ALLCHANNELS,
            true => AWDSGLR::SINGLECHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `ALLCHANNELS`"]
    #[inline]
    pub fn is_all_channels(&self) -> bool {
        *self == AWDSGLR::ALLCHANNELS
    }
    #[doc = "Checks if the value of the field is `SINGLECHANNEL`"]
    #[inline]
    pub fn is_single_channel(&self) -> bool {
        *self == AWDSGLR::SINGLECHANNEL
    }
}
#[doc = "Possible values of the field `SCAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCANR {
    #[doc = "Scan mode disabled"]
    DISABLED,
    #[doc = "Scan mode enabled"]
    ENABLED,
}
impl SCANR {
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
            SCANR::DISABLED => false,
            SCANR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCANR {
        match value {
            false => SCANR::DISABLED,
            true => SCANR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SCANR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SCANR::ENABLED
    }
}
#[doc = "Possible values of the field `JEOCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOCIER {
    #[doc = "JEOC interrupt disabled"]
    DISABLED,
    #[doc = "JEOC interrupt enabled"]
    ENABLED,
}
impl JEOCIER {
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
            JEOCIER::DISABLED => false,
            JEOCIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JEOCIER {
        match value {
            false => JEOCIER::DISABLED,
            true => JEOCIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == JEOCIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == JEOCIER::ENABLED
    }
}
#[doc = "Possible values of the field `AWDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDIER {
    #[doc = "Analogue watchdog interrupt disabled"]
    DISABLED,
    #[doc = "Analogue watchdog interrupt enabled"]
    ENABLED,
}
impl AWDIER {
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
            AWDIER::DISABLED => false,
            AWDIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AWDIER {
        match value {
            false => AWDIER::DISABLED,
            true => AWDIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == AWDIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == AWDIER::ENABLED
    }
}
#[doc = "Possible values of the field `EOCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCIER {
    #[doc = "EOC interrupt disabled"]
    DISABLED,
    #[doc = "EOC interrupt enabled"]
    ENABLED,
}
impl EOCIER {
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
            EOCIER::DISABLED => false,
            EOCIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOCIER {
        match value {
            false => EOCIER::DISABLED,
            true => EOCIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIER::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct AWDCHR {
    bits: u8,
}
impl AWDCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `OVRIE`"]
pub enum OVRIEW {
    #[doc = "Overrun interrupt disabled"]
    DISABLED,
    #[doc = "Overrun interrupt enabled"]
    ENABLED,
}
impl OVRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVRIEW::DISABLED => false,
            OVRIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _OVRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Overrun interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRIEW::DISABLED)
    }
    #[doc = "Overrun interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRIEW::ENABLED)
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
#[doc = "Values that can be written to the field `RES`"]
pub enum RESW {
    #[doc = "12-bit (15 ADCCLK cycles)"]
    TWELVEBIT,
    #[doc = "10-bit (13 ADCCLK cycles)"]
    TENBIT,
    #[doc = "8-bit (11 ADCCLK cycles)"]
    EIGHTBIT,
    #[doc = "6-bit (9 ADCCLK cycles)"]
    SIXBIT,
}
impl RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESW::TWELVEBIT => 0,
            RESW::TENBIT => 1,
            RESW::EIGHTBIT => 2,
            RESW::SIXBIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESW<'a> {
    w: &'a mut W,
}
impl<'a> _RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "12-bit (15 ADCCLK cycles)"]
    #[inline]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(RESW::TWELVEBIT)
    }
    #[doc = "10-bit (13 ADCCLK cycles)"]
    #[inline]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(RESW::TENBIT)
    }
    #[doc = "8-bit (11 ADCCLK cycles)"]
    #[inline]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(RESW::EIGHTBIT)
    }
    #[doc = "6-bit (9 ADCCLK cycles)"]
    #[inline]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(RESW::SIXBIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AWDEN`"]
pub enum AWDENW {
    #[doc = "Analog watchdog disabled on regular channels"]
    DISABLED,
    #[doc = "Analog watchdog enabled on regular channels"]
    ENABLED,
}
impl AWDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AWDENW::DISABLED => false,
            AWDENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AWDENW<'a> {
    w: &'a mut W,
}
impl<'a> _AWDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog watchdog disabled on regular channels"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDENW::DISABLED)
    }
    #[doc = "Analog watchdog enabled on regular channels"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDENW::ENABLED)
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
#[doc = "Values that can be written to the field `JAWDEN`"]
pub enum JAWDENW {
    #[doc = "Analog watchdog disabled on injected channels"]
    DISABLED,
    #[doc = "Analog watchdog enabled on injected channels"]
    ENABLED,
}
impl JAWDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JAWDENW::DISABLED => false,
            JAWDENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JAWDENW<'a> {
    w: &'a mut W,
}
impl<'a> _JAWDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JAWDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog watchdog disabled on injected channels"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAWDENW::DISABLED)
    }
    #[doc = "Analog watchdog enabled on injected channels"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAWDENW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _DISCNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `JDISCEN`"]
pub enum JDISCENW {
    #[doc = "Discontinuous mode on injected channels disabled"]
    DISABLED,
    #[doc = "Discontinuous mode on injected channels enabled"]
    ENABLED,
}
impl JDISCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JDISCENW::DISABLED => false,
            JDISCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JDISCENW<'a> {
    w: &'a mut W,
}
impl<'a> _JDISCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JDISCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Discontinuous mode on injected channels disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JDISCENW::DISABLED)
    }
    #[doc = "Discontinuous mode on injected channels enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JDISCENW::ENABLED)
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
#[doc = "Values that can be written to the field `DISCEN`"]
pub enum DISCENW {
    #[doc = "Discontinuous mode on regular channels disabled"]
    DISABLED,
    #[doc = "Discontinuous mode on regular channels enabled"]
    ENABLED,
}
impl DISCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISCENW::DISABLED => false,
            DISCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISCENW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCENW::DISABLED)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCENW::ENABLED)
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
#[doc = "Values that can be written to the field `JAUTO`"]
pub enum JAUTOW {
    #[doc = "Automatic injected group conversion disabled"]
    DISABLED,
    #[doc = "Automatic injected group conversion enabled"]
    ENABLED,
}
impl JAUTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JAUTOW::DISABLED => false,
            JAUTOW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JAUTOW<'a> {
    w: &'a mut W,
}
impl<'a> _JAUTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JAUTOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic injected group conversion disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAUTOW::DISABLED)
    }
    #[doc = "Automatic injected group conversion enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAUTOW::ENABLED)
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
#[doc = "Values that can be written to the field `AWDSGL`"]
pub enum AWDSGLW {
    #[doc = "Analog watchdog enabled on all channels"]
    ALLCHANNELS,
    #[doc = "Analog watchdog enabled on a single channel"]
    SINGLECHANNEL,
}
impl AWDSGLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AWDSGLW::ALLCHANNELS => false,
            AWDSGLW::SINGLECHANNEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AWDSGLW<'a> {
    w: &'a mut W,
}
impl<'a> _AWDSGLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWDSGLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline]
    pub fn all_channels(self) -> &'a mut W {
        self.variant(AWDSGLW::ALLCHANNELS)
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(AWDSGLW::SINGLECHANNEL)
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
#[doc = "Values that can be written to the field `SCAN`"]
pub enum SCANW {
    #[doc = "Scan mode disabled"]
    DISABLED,
    #[doc = "Scan mode enabled"]
    ENABLED,
}
impl SCANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCANW::DISABLED => false,
            SCANW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCANW<'a> {
    w: &'a mut W,
}
impl<'a> _SCANW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCANW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Scan mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCANW::DISABLED)
    }
    #[doc = "Scan mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCANW::ENABLED)
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
#[doc = "Values that can be written to the field `JEOCIE`"]
pub enum JEOCIEW {
    #[doc = "JEOC interrupt disabled"]
    DISABLED,
    #[doc = "JEOC interrupt enabled"]
    ENABLED,
}
impl JEOCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JEOCIEW::DISABLED => false,
            JEOCIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JEOCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _JEOCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JEOCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "JEOC interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEOCIEW::DISABLED)
    }
    #[doc = "JEOC interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JEOCIEW::ENABLED)
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
#[doc = "Values that can be written to the field `AWDIE`"]
pub enum AWDIEW {
    #[doc = "Analogue watchdog interrupt disabled"]
    DISABLED,
    #[doc = "Analogue watchdog interrupt enabled"]
    ENABLED,
}
impl AWDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AWDIEW::DISABLED => false,
            AWDIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AWDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AWDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analogue watchdog interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDIEW::DISABLED)
    }
    #[doc = "Analogue watchdog interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDIEW::ENABLED)
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
#[doc = "Values that can be written to the field `EOCIE`"]
pub enum EOCIEW {
    #[doc = "EOC interrupt disabled"]
    DISABLED,
    #[doc = "EOC interrupt enabled"]
    ENABLED,
}
impl EOCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOCIEW::DISABLED => false,
            EOCIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EOCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EOC interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCIEW::DISABLED)
    }
    #[doc = "EOC interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCIEW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _AWDCHW<'a> {
    w: &'a mut W,
}
impl<'a> _AWDCHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
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
    #[doc = "Bit 26 - Overrun interrupt enable"]
    #[inline]
    pub fn ovrie(&self) -> OVRIER {
        OVRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - Resolution"]
    #[inline]
    pub fn res(&self) -> RESR {
        RESR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline]
    pub fn awden(&self) -> AWDENR {
        AWDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline]
    pub fn jawden(&self) -> JAWDENR {
        JAWDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline]
    pub fn discnum(&self) -> DISCNUMR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DISCNUMR { bits }
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline]
    pub fn jdiscen(&self) -> JDISCENR {
        JDISCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline]
    pub fn discen(&self) -> DISCENR {
        DISCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline]
    pub fn jauto(&self) -> JAUTOR {
        JAUTOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline]
    pub fn awdsgl(&self) -> AWDSGLR {
        AWDSGLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline]
    pub fn scan(&self) -> SCANR {
        SCANR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline]
    pub fn jeocie(&self) -> JEOCIER {
        JEOCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline]
    pub fn awdie(&self) -> AWDIER {
        AWDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline]
    pub fn eocie(&self) -> EOCIER {
        EOCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline]
    pub fn awdch(&self) -> AWDCHR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AWDCHR { bits }
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
    #[doc = "Bit 26 - Overrun interrupt enable"]
    #[inline]
    pub fn ovrie(&mut self) -> _OVRIEW {
        _OVRIEW { w: self }
    }
    #[doc = "Bits 24:25 - Resolution"]
    #[inline]
    pub fn res(&mut self) -> _RESW {
        _RESW { w: self }
    }
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline]
    pub fn awden(&mut self) -> _AWDENW {
        _AWDENW { w: self }
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline]
    pub fn jawden(&mut self) -> _JAWDENW {
        _JAWDENW { w: self }
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline]
    pub fn discnum(&mut self) -> _DISCNUMW {
        _DISCNUMW { w: self }
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline]
    pub fn jdiscen(&mut self) -> _JDISCENW {
        _JDISCENW { w: self }
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline]
    pub fn discen(&mut self) -> _DISCENW {
        _DISCENW { w: self }
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline]
    pub fn jauto(&mut self) -> _JAUTOW {
        _JAUTOW { w: self }
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline]
    pub fn awdsgl(&mut self) -> _AWDSGLW {
        _AWDSGLW { w: self }
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline]
    pub fn scan(&mut self) -> _SCANW {
        _SCANW { w: self }
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline]
    pub fn jeocie(&mut self) -> _JEOCIEW {
        _JEOCIEW { w: self }
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline]
    pub fn awdie(&mut self) -> _AWDIEW {
        _AWDIEW { w: self }
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline]
    pub fn eocie(&mut self) -> _EOCIEW {
        _EOCIEW { w: self }
    }
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline]
    pub fn awdch(&mut self) -> _AWDCHW {
        _AWDCHW { w: self }
    }
}
