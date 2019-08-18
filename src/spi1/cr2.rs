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
#[doc = "Possible values of the field `RXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAENR {
    #[doc = "Rx buffer DMA disabled"]
    DISABLED,
    #[doc = "Rx buffer DMA enabled"]
    ENABLED,
}
impl RXDMAENR {
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
            RXDMAENR::DISABLED => false,
            RXDMAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDMAENR {
        match value {
            false => RXDMAENR::DISABLED,
            true => RXDMAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAENR::ENABLED
    }
}
#[doc = "Possible values of the field `TXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAENR {
    #[doc = "Tx buffer DMA disabled"]
    DISABLED,
    #[doc = "Tx buffer DMA enabled"]
    ENABLED,
}
impl TXDMAENR {
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
            TXDMAENR::DISABLED => false,
            TXDMAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDMAENR {
        match value {
            false => TXDMAENR::DISABLED,
            true => TXDMAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAENR::ENABLED
    }
}
#[doc = "Possible values of the field `SSOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOER {
    #[doc = "SS output is disabled in master mode"]
    DISABLED,
    #[doc = "SS output is enabled in master mode"]
    ENABLED,
}
impl SSOER {
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
            SSOER::DISABLED => false,
            SSOER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSOER {
        match value {
            false => SSOER::DISABLED,
            true => SSOER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SSOER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SSOER::ENABLED
    }
}
#[doc = "Possible values of the field `NSSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSSPR {
    #[doc = "No NSS pulse"]
    NOPULSE,
    #[doc = "NSS pulse generated"]
    PULSEGENERATED,
}
impl NSSPR {
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
            NSSPR::NOPULSE => false,
            NSSPR::PULSEGENERATED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSSPR {
        match value {
            false => NSSPR::NOPULSE,
            true => NSSPR::PULSEGENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOPULSE`"]
    #[inline]
    pub fn is_no_pulse(&self) -> bool {
        *self == NSSPR::NOPULSE
    }
    #[doc = "Checks if the value of the field is `PULSEGENERATED`"]
    #[inline]
    pub fn is_pulse_generated(&self) -> bool {
        *self == NSSPR::PULSEGENERATED
    }
}
#[doc = "Possible values of the field `FRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRFR {
    #[doc = "SPI Motorola mode"]
    MOTOROLA,
    #[doc = "SPI TI mode"]
    TI,
}
impl FRFR {
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
            FRFR::MOTOROLA => false,
            FRFR::TI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRFR {
        match value {
            false => FRFR::MOTOROLA,
            true => FRFR::TI,
        }
    }
    #[doc = "Checks if the value of the field is `MOTOROLA`"]
    #[inline]
    pub fn is_motorola(&self) -> bool {
        *self == FRFR::MOTOROLA
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline]
    pub fn is_ti(&self) -> bool {
        *self == FRFR::TI
    }
}
#[doc = "Possible values of the field `ERRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIER {
    #[doc = "Error interrupt masked"]
    MASKED,
    #[doc = "Error interrupt not masked"]
    NOTMASKED,
}
impl ERRIER {
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
            ERRIER::MASKED => false,
            ERRIER::NOTMASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRIER {
        match value {
            false => ERRIER::MASKED,
            true => ERRIER::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == ERRIER::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIER::NOTMASKED
    }
}
#[doc = "Possible values of the field `RXNEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIER {
    #[doc = "RXE interrupt masked"]
    MASKED,
    #[doc = "RXE interrupt not masked"]
    NOTMASKED,
}
impl RXNEIER {
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
            RXNEIER::MASKED => false,
            RXNEIER::NOTMASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXNEIER {
        match value {
            false => RXNEIER::MASKED,
            true => RXNEIER::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIER::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIER::NOTMASKED
    }
}
#[doc = "Possible values of the field `TXEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIER {
    #[doc = "TXE interrupt masked"]
    MASKED,
    #[doc = "TXE interrupt not masked"]
    NOTMASKED,
}
impl TXEIER {
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
            TXEIER::MASKED => false,
            TXEIER::NOTMASKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEIER {
        match value {
            false => TXEIER::MASKED,
            true => TXEIER::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == TXEIER::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIER::NOTMASKED
    }
}
#[doc = "Possible values of the field `DS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSR {
    #[doc = "4-bit"]
    FOURBIT,
    #[doc = "5-bit"]
    FIVEBIT,
    #[doc = "6-bit"]
    SIXBIT,
    #[doc = "7-bit"]
    SEVENBIT,
    #[doc = "8-bit"]
    EIGHTBIT,
    #[doc = "9-bit"]
    NINEBIT,
    #[doc = "10-bit"]
    TENBIT,
    #[doc = "11-bit"]
    ELEVENBIT,
    #[doc = "12-bit"]
    TWELVEBIT,
    #[doc = "13-bit"]
    THIRTEENBIT,
    #[doc = "14-bit"]
    FOURTEENBIT,
    #[doc = "15-bit"]
    FIFTEENBIT,
    #[doc = "16-bit"]
    SIXTEENBIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSR::FOURBIT => 0x03,
            DSR::FIVEBIT => 0x04,
            DSR::SIXBIT => 0x05,
            DSR::SEVENBIT => 0x06,
            DSR::EIGHTBIT => 0x07,
            DSR::NINEBIT => 0x08,
            DSR::TENBIT => 0x09,
            DSR::ELEVENBIT => 0x0a,
            DSR::TWELVEBIT => 0x0b,
            DSR::THIRTEENBIT => 0x0c,
            DSR::FOURTEENBIT => 0x0d,
            DSR::FIFTEENBIT => 0x0e,
            DSR::SIXTEENBIT => 0x0f,
            DSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSR {
        match value {
            3 => DSR::FOURBIT,
            4 => DSR::FIVEBIT,
            5 => DSR::SIXBIT,
            6 => DSR::SEVENBIT,
            7 => DSR::EIGHTBIT,
            8 => DSR::NINEBIT,
            9 => DSR::TENBIT,
            10 => DSR::ELEVENBIT,
            11 => DSR::TWELVEBIT,
            12 => DSR::THIRTEENBIT,
            13 => DSR::FOURTEENBIT,
            14 => DSR::FIFTEENBIT,
            15 => DSR::SIXTEENBIT,
            i => DSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FOURBIT`"]
    #[inline]
    pub fn is_four_bit(&self) -> bool {
        *self == DSR::FOURBIT
    }
    #[doc = "Checks if the value of the field is `FIVEBIT`"]
    #[inline]
    pub fn is_five_bit(&self) -> bool {
        *self == DSR::FIVEBIT
    }
    #[doc = "Checks if the value of the field is `SIXBIT`"]
    #[inline]
    pub fn is_six_bit(&self) -> bool {
        *self == DSR::SIXBIT
    }
    #[doc = "Checks if the value of the field is `SEVENBIT`"]
    #[inline]
    pub fn is_seven_bit(&self) -> bool {
        *self == DSR::SEVENBIT
    }
    #[doc = "Checks if the value of the field is `EIGHTBIT`"]
    #[inline]
    pub fn is_eight_bit(&self) -> bool {
        *self == DSR::EIGHTBIT
    }
    #[doc = "Checks if the value of the field is `NINEBIT`"]
    #[inline]
    pub fn is_nine_bit(&self) -> bool {
        *self == DSR::NINEBIT
    }
    #[doc = "Checks if the value of the field is `TENBIT`"]
    #[inline]
    pub fn is_ten_bit(&self) -> bool {
        *self == DSR::TENBIT
    }
    #[doc = "Checks if the value of the field is `ELEVENBIT`"]
    #[inline]
    pub fn is_eleven_bit(&self) -> bool {
        *self == DSR::ELEVENBIT
    }
    #[doc = "Checks if the value of the field is `TWELVEBIT`"]
    #[inline]
    pub fn is_twelve_bit(&self) -> bool {
        *self == DSR::TWELVEBIT
    }
    #[doc = "Checks if the value of the field is `THIRTEENBIT`"]
    #[inline]
    pub fn is_thirteen_bit(&self) -> bool {
        *self == DSR::THIRTEENBIT
    }
    #[doc = "Checks if the value of the field is `FOURTEENBIT`"]
    #[inline]
    pub fn is_fourteen_bit(&self) -> bool {
        *self == DSR::FOURTEENBIT
    }
    #[doc = "Checks if the value of the field is `FIFTEENBIT`"]
    #[inline]
    pub fn is_fifteen_bit(&self) -> bool {
        *self == DSR::FIFTEENBIT
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DSR::SIXTEENBIT
    }
}
#[doc = "Possible values of the field `FRXTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRXTHR {
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    HALF,
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    QUARTER,
}
impl FRXTHR {
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
            FRXTHR::HALF => false,
            FRXTHR::QUARTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRXTHR {
        match value {
            false => FRXTHR::HALF,
            true => FRXTHR::QUARTER,
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == FRXTHR::HALF
    }
    #[doc = "Checks if the value of the field is `QUARTER`"]
    #[inline]
    pub fn is_quarter(&self) -> bool {
        *self == FRXTHR::QUARTER
    }
}
#[doc = "Possible values of the field `LDMA_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMA_RXR {
    #[doc = "Number of data to transfer for receive is even"]
    EVEN,
    #[doc = "Number of data to transfer for receive is odd"]
    ODD,
}
impl LDMA_RXR {
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
            LDMA_RXR::EVEN => false,
            LDMA_RXR::ODD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDMA_RXR {
        match value {
            false => LDMA_RXR::EVEN,
            true => LDMA_RXR::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == LDMA_RXR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_RXR::ODD
    }
}
#[doc = "Possible values of the field `LDMA_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMA_TXR {
    #[doc = "Number of data to transfer for transmit is even"]
    EVEN,
    #[doc = "Number of data to transfer for transmit is odd"]
    ODD,
}
impl LDMA_TXR {
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
            LDMA_TXR::EVEN => false,
            LDMA_TXR::ODD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDMA_TXR {
        match value {
            false => LDMA_TXR::EVEN,
            true => LDMA_TXR::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == LDMA_TXR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_TXR::ODD
    }
}
#[doc = "Values that can be written to the field `RXDMAEN`"]
pub enum RXDMAENW {
    #[doc = "Rx buffer DMA disabled"]
    DISABLED,
    #[doc = "Rx buffer DMA enabled"]
    ENABLED,
}
impl RXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDMAENW::DISABLED => false,
            RXDMAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx buffer DMA disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAENW::DISABLED)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAENW::ENABLED)
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
#[doc = "Values that can be written to the field `TXDMAEN`"]
pub enum TXDMAENW {
    #[doc = "Tx buffer DMA disabled"]
    DISABLED,
    #[doc = "Tx buffer DMA enabled"]
    ENABLED,
}
impl TXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDMAENW::DISABLED => false,
            TXDMAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tx buffer DMA disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAENW::DISABLED)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAENW::ENABLED)
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
#[doc = "Values that can be written to the field `SSOE`"]
pub enum SSOEW {
    #[doc = "SS output is disabled in master mode"]
    DISABLED,
    #[doc = "SS output is enabled in master mode"]
    ENABLED,
}
impl SSOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSOEW::DISABLED => false,
            SSOEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSOEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SS output is disabled in master mode"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOEW::DISABLED)
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOEW::ENABLED)
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
#[doc = "Values that can be written to the field `NSSP`"]
pub enum NSSPW {
    #[doc = "No NSS pulse"]
    NOPULSE,
    #[doc = "NSS pulse generated"]
    PULSEGENERATED,
}
impl NSSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSSPW::NOPULSE => false,
            NSSPW::PULSEGENERATED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSSPW<'a> {
    w: &'a mut W,
}
impl<'a> _NSSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No NSS pulse"]
    #[inline]
    pub fn no_pulse(self) -> &'a mut W {
        self.variant(NSSPW::NOPULSE)
    }
    #[doc = "NSS pulse generated"]
    #[inline]
    pub fn pulse_generated(self) -> &'a mut W {
        self.variant(NSSPW::PULSEGENERATED)
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
#[doc = "Values that can be written to the field `FRF`"]
pub enum FRFW {
    #[doc = "SPI Motorola mode"]
    MOTOROLA,
    #[doc = "SPI TI mode"]
    TI,
}
impl FRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRFW::MOTOROLA => false,
            FRFW::TI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRFW<'a> {
    w: &'a mut W,
}
impl<'a> _FRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI Motorola mode"]
    #[inline]
    pub fn motorola(self) -> &'a mut W {
        self.variant(FRFW::MOTOROLA)
    }
    #[doc = "SPI TI mode"]
    #[inline]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRFW::TI)
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
#[doc = "Values that can be written to the field `ERRIE`"]
pub enum ERRIEW {
    #[doc = "Error interrupt masked"]
    MASKED,
    #[doc = "Error interrupt not masked"]
    NOTMASKED,
}
impl ERRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRIEW::MASKED => false,
            ERRIEW::NOTMASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error interrupt masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(ERRIEW::MASKED)
    }
    #[doc = "Error interrupt not masked"]
    #[inline]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(ERRIEW::NOTMASKED)
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
#[doc = "Values that can be written to the field `RXNEIE`"]
pub enum RXNEIEW {
    #[doc = "RXE interrupt masked"]
    MASKED,
    #[doc = "RXE interrupt not masked"]
    NOTMASKED,
}
impl RXNEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXNEIEW::MASKED => false,
            RXNEIEW::NOTMASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXNEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXNEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXNEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXE interrupt masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXNEIEW::MASKED)
    }
    #[doc = "RXE interrupt not masked"]
    #[inline]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXNEIEW::NOTMASKED)
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
#[doc = "Values that can be written to the field `TXEIE`"]
pub enum TXEIEW {
    #[doc = "TXE interrupt masked"]
    MASKED,
    #[doc = "TXE interrupt not masked"]
    NOTMASKED,
}
impl TXEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEIEW::MASKED => false,
            TXEIEW::NOTMASKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXE interrupt masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXEIEW::MASKED)
    }
    #[doc = "TXE interrupt not masked"]
    #[inline]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXEIEW::NOTMASKED)
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
#[doc = "Values that can be written to the field `DS`"]
pub enum DSW {
    #[doc = "4-bit"]
    FOURBIT,
    #[doc = "5-bit"]
    FIVEBIT,
    #[doc = "6-bit"]
    SIXBIT,
    #[doc = "7-bit"]
    SEVENBIT,
    #[doc = "8-bit"]
    EIGHTBIT,
    #[doc = "9-bit"]
    NINEBIT,
    #[doc = "10-bit"]
    TENBIT,
    #[doc = "11-bit"]
    ELEVENBIT,
    #[doc = "12-bit"]
    TWELVEBIT,
    #[doc = "13-bit"]
    THIRTEENBIT,
    #[doc = "14-bit"]
    FOURTEENBIT,
    #[doc = "15-bit"]
    FIFTEENBIT,
    #[doc = "16-bit"]
    SIXTEENBIT,
}
impl DSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSW::FOURBIT => 3,
            DSW::FIVEBIT => 4,
            DSW::SIXBIT => 5,
            DSW::SEVENBIT => 6,
            DSW::EIGHTBIT => 7,
            DSW::NINEBIT => 8,
            DSW::TENBIT => 9,
            DSW::ELEVENBIT => 10,
            DSW::TWELVEBIT => 11,
            DSW::THIRTEENBIT => 12,
            DSW::FOURTEENBIT => 13,
            DSW::FIFTEENBIT => 14,
            DSW::SIXTEENBIT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "4-bit"]
    #[inline]
    pub fn four_bit(self) -> &'a mut W {
        self.variant(DSW::FOURBIT)
    }
    #[doc = "5-bit"]
    #[inline]
    pub fn five_bit(self) -> &'a mut W {
        self.variant(DSW::FIVEBIT)
    }
    #[doc = "6-bit"]
    #[inline]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(DSW::SIXBIT)
    }
    #[doc = "7-bit"]
    #[inline]
    pub fn seven_bit(self) -> &'a mut W {
        self.variant(DSW::SEVENBIT)
    }
    #[doc = "8-bit"]
    #[inline]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(DSW::EIGHTBIT)
    }
    #[doc = "9-bit"]
    #[inline]
    pub fn nine_bit(self) -> &'a mut W {
        self.variant(DSW::NINEBIT)
    }
    #[doc = "10-bit"]
    #[inline]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(DSW::TENBIT)
    }
    #[doc = "11-bit"]
    #[inline]
    pub fn eleven_bit(self) -> &'a mut W {
        self.variant(DSW::ELEVENBIT)
    }
    #[doc = "12-bit"]
    #[inline]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(DSW::TWELVEBIT)
    }
    #[doc = "13-bit"]
    #[inline]
    pub fn thirteen_bit(self) -> &'a mut W {
        self.variant(DSW::THIRTEENBIT)
    }
    #[doc = "14-bit"]
    #[inline]
    pub fn fourteen_bit(self) -> &'a mut W {
        self.variant(DSW::FOURTEENBIT)
    }
    #[doc = "15-bit"]
    #[inline]
    pub fn fifteen_bit(self) -> &'a mut W {
        self.variant(DSW::FIFTEENBIT)
    }
    #[doc = "16-bit"]
    #[inline]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DSW::SIXTEENBIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRXTH`"]
pub enum FRXTHW {
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    HALF,
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    QUARTER,
}
impl FRXTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRXTHW::HALF => false,
            FRXTHW::QUARTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRXTHW<'a> {
    w: &'a mut W,
}
impl<'a> _FRXTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRXTHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline]
    pub fn half(self) -> &'a mut W {
        self.variant(FRXTHW::HALF)
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline]
    pub fn quarter(self) -> &'a mut W {
        self.variant(FRXTHW::QUARTER)
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
#[doc = "Values that can be written to the field `LDMA_RX`"]
pub enum LDMA_RXW {
    #[doc = "Number of data to transfer for receive is even"]
    EVEN,
    #[doc = "Number of data to transfer for receive is odd"]
    ODD,
}
impl LDMA_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDMA_RXW::EVEN => false,
            LDMA_RXW::ODD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDMA_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _LDMA_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDMA_RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Number of data to transfer for receive is even"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_RXW::EVEN)
    }
    #[doc = "Number of data to transfer for receive is odd"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_RXW::ODD)
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
#[doc = "Values that can be written to the field `LDMA_TX`"]
pub enum LDMA_TXW {
    #[doc = "Number of data to transfer for transmit is even"]
    EVEN,
    #[doc = "Number of data to transfer for transmit is odd"]
    ODD,
}
impl LDMA_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDMA_TXW::EVEN => false,
            LDMA_TXW::ODD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDMA_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _LDMA_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDMA_TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Number of data to transfer for transmit is even"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_TXW::EVEN)
    }
    #[doc = "Number of data to transfer for transmit is odd"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_TXW::ODD)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline]
    pub fn rxdmaen(&self) -> RXDMAENR {
        RXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline]
    pub fn txdmaen(&self) -> TXDMAENR {
        TXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline]
    pub fn ssoe(&self) -> SSOER {
        SSOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline]
    pub fn nssp(&self) -> NSSPR {
        NSSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline]
    pub fn frf(&self) -> FRFR {
        FRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline]
    pub fn errie(&self) -> ERRIER {
        ERRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline]
    pub fn rxneie(&self) -> RXNEIER {
        RXNEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline]
    pub fn txeie(&self) -> TXEIER {
        TXEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline]
    pub fn ds(&self) -> DSR {
        DSR::_from({
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline]
    pub fn frxth(&self) -> FRXTHR {
        FRXTHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline]
    pub fn ldma_rx(&self) -> LDMA_RXR {
        LDMA_RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline]
    pub fn ldma_tx(&self) -> LDMA_TXR {
        LDMA_TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x0700 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline]
    pub fn rxdmaen(&mut self) -> _RXDMAENW {
        _RXDMAENW { w: self }
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline]
    pub fn txdmaen(&mut self) -> _TXDMAENW {
        _TXDMAENW { w: self }
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline]
    pub fn ssoe(&mut self) -> _SSOEW {
        _SSOEW { w: self }
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline]
    pub fn nssp(&mut self) -> _NSSPW {
        _NSSPW { w: self }
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline]
    pub fn frf(&mut self) -> _FRFW {
        _FRFW { w: self }
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline]
    pub fn errie(&mut self) -> _ERRIEW {
        _ERRIEW { w: self }
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline]
    pub fn rxneie(&mut self) -> _RXNEIEW {
        _RXNEIEW { w: self }
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline]
    pub fn txeie(&mut self) -> _TXEIEW {
        _TXEIEW { w: self }
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline]
    pub fn ds(&mut self) -> _DSW {
        _DSW { w: self }
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline]
    pub fn frxth(&mut self) -> _FRXTHW {
        _FRXTHW { w: self }
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline]
    pub fn ldma_rx(&mut self) -> _LDMA_RXW {
        _LDMA_RXW { w: self }
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline]
    pub fn ldma_tx(&mut self) -> _LDMA_TXW {
        _LDMA_TXW { w: self }
    }
}
