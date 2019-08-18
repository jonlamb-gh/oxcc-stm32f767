#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMAOMR {
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
#[doc = "Possible values of the field `SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRR {
    #[doc = "Reception is stopped after transfer of the current frame"]
    STOPPED,
    #[doc = "Reception is placed in the Running state"]
    STARTED,
}
impl SRR {
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
            SRR::STOPPED => false,
            SRR::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRR {
        match value {
            false => SRR::STOPPED,
            true => SRR::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline]
    pub fn is_stopped(&self) -> bool {
        *self == SRR::STOPPED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == SRR::STARTED
    }
}
#[doc = r" Value of the field"]
pub struct OSFR {
    bits: bool,
}
impl OSFR {
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
#[doc = "Possible values of the field `RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCR {
    #[doc = "64 bytes"]
    RTC64,
    #[doc = "32 bytes"]
    RTC32,
    #[doc = "96 bytes"]
    RTC96,
    #[doc = "128 bytes"]
    RTC128,
}
impl RTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCR::RTC64 => 0,
            RTCR::RTC32 => 0x01,
            RTCR::RTC96 => 0x02,
            RTCR::RTC128 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCR {
        match value {
            0 => RTCR::RTC64,
            1 => RTCR::RTC32,
            2 => RTCR::RTC96,
            3 => RTCR::RTC128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTC64`"]
    #[inline]
    pub fn is_rtc64(&self) -> bool {
        *self == RTCR::RTC64
    }
    #[doc = "Checks if the value of the field is `RTC32`"]
    #[inline]
    pub fn is_rtc32(&self) -> bool {
        *self == RTCR::RTC32
    }
    #[doc = "Checks if the value of the field is `RTC96`"]
    #[inline]
    pub fn is_rtc96(&self) -> bool {
        *self == RTCR::RTC96
    }
    #[doc = "Checks if the value of the field is `RTC128`"]
    #[inline]
    pub fn is_rtc128(&self) -> bool {
        *self == RTCR::RTC128
    }
}
#[doc = "Possible values of the field `FUGF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUGFR {
    #[doc = "Rx FIFO drops all frames of less than 64 bytes"]
    DROP,
    #[doc = "Rx FIFO forwards undersized frames"]
    FORWARD,
}
impl FUGFR {
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
            FUGFR::DROP => false,
            FUGFR::FORWARD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FUGFR {
        match value {
            false => FUGFR::DROP,
            true => FUGFR::FORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `DROP`"]
    #[inline]
    pub fn is_drop(&self) -> bool {
        *self == FUGFR::DROP
    }
    #[doc = "Checks if the value of the field is `FORWARD`"]
    #[inline]
    pub fn is_forward(&self) -> bool {
        *self == FUGFR::FORWARD
    }
}
#[doc = "Possible values of the field `FEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEFR {
    #[doc = "Rx FIFO drops frames with error status"]
    DROP,
    #[doc = "All frames except runt error frames are forwarded to the DMA"]
    FORWARD,
}
impl FEFR {
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
            FEFR::DROP => false,
            FEFR::FORWARD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEFR {
        match value {
            false => FEFR::DROP,
            true => FEFR::FORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `DROP`"]
    #[inline]
    pub fn is_drop(&self) -> bool {
        *self == FEFR::DROP
    }
    #[doc = "Checks if the value of the field is `FORWARD`"]
    #[inline]
    pub fn is_forward(&self) -> bool {
        *self == FEFR::FORWARD
    }
}
#[doc = "Possible values of the field `ST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STR {
    #[doc = "Transmission is placed in the Stopped state"]
    STOPPED,
    #[doc = "Transmission is placed in Running state"]
    STARTED,
}
impl STR {
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
            STR::STOPPED => false,
            STR::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STR {
        match value {
            false => STR::STOPPED,
            true => STR::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline]
    pub fn is_stopped(&self) -> bool {
        *self == STR::STOPPED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == STR::STARTED
    }
}
#[doc = "Possible values of the field `TTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTCR {
    #[doc = "64 bytes"]
    TTC64,
    #[doc = "128 bytes"]
    TTC128,
    #[doc = "192 bytes"]
    TTC192,
    #[doc = "256 bytes"]
    TTC256,
    #[doc = "40 bytes"]
    TTC40,
    #[doc = "32 bytes"]
    TTC32,
    #[doc = "24 bytes"]
    TTC24,
    #[doc = "16 bytes"]
    TTC16,
}
impl TTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TTCR::TTC64 => 0,
            TTCR::TTC128 => 0x01,
            TTCR::TTC192 => 0x02,
            TTCR::TTC256 => 0x03,
            TTCR::TTC40 => 0x04,
            TTCR::TTC32 => 0x05,
            TTCR::TTC24 => 0x06,
            TTCR::TTC16 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TTCR {
        match value {
            0 => TTCR::TTC64,
            1 => TTCR::TTC128,
            2 => TTCR::TTC192,
            3 => TTCR::TTC256,
            4 => TTCR::TTC40,
            5 => TTCR::TTC32,
            6 => TTCR::TTC24,
            7 => TTCR::TTC16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TTC64`"]
    #[inline]
    pub fn is_ttc64(&self) -> bool {
        *self == TTCR::TTC64
    }
    #[doc = "Checks if the value of the field is `TTC128`"]
    #[inline]
    pub fn is_ttc128(&self) -> bool {
        *self == TTCR::TTC128
    }
    #[doc = "Checks if the value of the field is `TTC192`"]
    #[inline]
    pub fn is_ttc192(&self) -> bool {
        *self == TTCR::TTC192
    }
    #[doc = "Checks if the value of the field is `TTC256`"]
    #[inline]
    pub fn is_ttc256(&self) -> bool {
        *self == TTCR::TTC256
    }
    #[doc = "Checks if the value of the field is `TTC40`"]
    #[inline]
    pub fn is_ttc40(&self) -> bool {
        *self == TTCR::TTC40
    }
    #[doc = "Checks if the value of the field is `TTC32`"]
    #[inline]
    pub fn is_ttc32(&self) -> bool {
        *self == TTCR::TTC32
    }
    #[doc = "Checks if the value of the field is `TTC24`"]
    #[inline]
    pub fn is_ttc24(&self) -> bool {
        *self == TTCR::TTC24
    }
    #[doc = "Checks if the value of the field is `TTC16`"]
    #[inline]
    pub fn is_ttc16(&self) -> bool {
        *self == TTCR::TTC16
    }
}
#[doc = "Possible values of the field `FTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTFR {
    #[doc = "Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    FLUSH,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FTFR {
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
            FTFR::FLUSH => true,
            FTFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTFR {
        match value {
            true => FTFR::FLUSH,
            i => FTFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline]
    pub fn is_flush(&self) -> bool {
        *self == FTFR::FLUSH
    }
}
#[doc = "Possible values of the field `TSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSFR {
    #[doc = "Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    CUTTHROUGH,
    #[doc = "Transmission starts when a full frame is in the Tx FIFO"]
    STOREFORWARD,
}
impl TSFR {
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
            TSFR::CUTTHROUGH => false,
            TSFR::STOREFORWARD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSFR {
        match value {
            false => TSFR::CUTTHROUGH,
            true => TSFR::STOREFORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `CUTTHROUGH`"]
    #[inline]
    pub fn is_cut_through(&self) -> bool {
        *self == TSFR::CUTTHROUGH
    }
    #[doc = "Checks if the value of the field is `STOREFORWARD`"]
    #[inline]
    pub fn is_store_forward(&self) -> bool {
        *self == TSFR::STOREFORWARD
    }
}
#[doc = r" Value of the field"]
pub struct DFRFR {
    bits: bool,
}
impl DFRFR {
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
#[doc = "Possible values of the field `RSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSFR {
    #[doc = "Rx FIFO operates in cut-through mode, subject to RTC bits"]
    CUTTHROUGH,
    #[doc = "Frames are read from Rx FIFO after complete frame has been written"]
    STOREFORWARD,
}
impl RSFR {
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
            RSFR::CUTTHROUGH => false,
            RSFR::STOREFORWARD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSFR {
        match value {
            false => RSFR::CUTTHROUGH,
            true => RSFR::STOREFORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `CUTTHROUGH`"]
    #[inline]
    pub fn is_cut_through(&self) -> bool {
        *self == RSFR::CUTTHROUGH
    }
    #[doc = "Checks if the value of the field is `STOREFORWARD`"]
    #[inline]
    pub fn is_store_forward(&self) -> bool {
        *self == RSFR::STOREFORWARD
    }
}
#[doc = "Possible values of the field `DTCEFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCEFDR {
    #[doc = "Drop frames with errors only in the receive checksum offload engine"]
    ENABLED,
    #[doc = "Do not drop frames that only have errors in the receive checksum offload engine"]
    DISABLED,
}
impl DTCEFDR {
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
            DTCEFDR::ENABLED => false,
            DTCEFDR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTCEFDR {
        match value {
            false => DTCEFDR::ENABLED,
            true => DTCEFDR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DTCEFDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DTCEFDR::DISABLED
    }
}
#[doc = "Values that can be written to the field `SR`"]
pub enum SRW {
    #[doc = "Reception is stopped after transfer of the current frame"]
    STOPPED,
    #[doc = "Reception is placed in the Running state"]
    STARTED,
}
impl SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRW::STOPPED => false,
            SRW::STARTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reception is stopped after transfer of the current frame"]
    #[inline]
    pub fn stopped(self) -> &'a mut W {
        self.variant(SRW::STOPPED)
    }
    #[doc = "Reception is placed in the Running state"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(SRW::STARTED)
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
#[doc = r" Proxy"]
pub struct _OSFW<'a> {
    w: &'a mut W,
}
impl<'a> _OSFW<'a> {
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
#[doc = "Values that can be written to the field `RTC`"]
pub enum RTCW {
    #[doc = "64 bytes"]
    RTC64,
    #[doc = "32 bytes"]
    RTC32,
    #[doc = "96 bytes"]
    RTC96,
    #[doc = "128 bytes"]
    RTC128,
}
impl RTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCW::RTC64 => 0,
            RTCW::RTC32 => 1,
            RTCW::RTC96 => 2,
            RTCW::RTC128 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64 bytes"]
    #[inline]
    pub fn rtc64(self) -> &'a mut W {
        self.variant(RTCW::RTC64)
    }
    #[doc = "32 bytes"]
    #[inline]
    pub fn rtc32(self) -> &'a mut W {
        self.variant(RTCW::RTC32)
    }
    #[doc = "96 bytes"]
    #[inline]
    pub fn rtc96(self) -> &'a mut W {
        self.variant(RTCW::RTC96)
    }
    #[doc = "128 bytes"]
    #[inline]
    pub fn rtc128(self) -> &'a mut W {
        self.variant(RTCW::RTC128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FUGF`"]
pub enum FUGFW {
    #[doc = "Rx FIFO drops all frames of less than 64 bytes"]
    DROP,
    #[doc = "Rx FIFO forwards undersized frames"]
    FORWARD,
}
impl FUGFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FUGFW::DROP => false,
            FUGFW::FORWARD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUGFW<'a> {
    w: &'a mut W,
}
impl<'a> _FUGFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUGFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx FIFO drops all frames of less than 64 bytes"]
    #[inline]
    pub fn drop(self) -> &'a mut W {
        self.variant(FUGFW::DROP)
    }
    #[doc = "Rx FIFO forwards undersized frames"]
    #[inline]
    pub fn forward(self) -> &'a mut W {
        self.variant(FUGFW::FORWARD)
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
#[doc = "Values that can be written to the field `FEF`"]
pub enum FEFW {
    #[doc = "Rx FIFO drops frames with error status"]
    DROP,
    #[doc = "All frames except runt error frames are forwarded to the DMA"]
    FORWARD,
}
impl FEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEFW::DROP => false,
            FEFW::FORWARD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEFW<'a> {
    w: &'a mut W,
}
impl<'a> _FEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx FIFO drops frames with error status"]
    #[inline]
    pub fn drop(self) -> &'a mut W {
        self.variant(FEFW::DROP)
    }
    #[doc = "All frames except runt error frames are forwarded to the DMA"]
    #[inline]
    pub fn forward(self) -> &'a mut W {
        self.variant(FEFW::FORWARD)
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
#[doc = "Values that can be written to the field `ST`"]
pub enum STW {
    #[doc = "Transmission is placed in the Stopped state"]
    STOPPED,
    #[doc = "Transmission is placed in Running state"]
    STARTED,
}
impl STW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STW::STOPPED => false,
            STW::STARTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STW<'a> {
    w: &'a mut W,
}
impl<'a> _STW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmission is placed in the Stopped state"]
    #[inline]
    pub fn stopped(self) -> &'a mut W {
        self.variant(STW::STOPPED)
    }
    #[doc = "Transmission is placed in Running state"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(STW::STARTED)
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
#[doc = "Values that can be written to the field `TTC`"]
pub enum TTCW {
    #[doc = "64 bytes"]
    TTC64,
    #[doc = "128 bytes"]
    TTC128,
    #[doc = "192 bytes"]
    TTC192,
    #[doc = "256 bytes"]
    TTC256,
    #[doc = "40 bytes"]
    TTC40,
    #[doc = "32 bytes"]
    TTC32,
    #[doc = "24 bytes"]
    TTC24,
    #[doc = "16 bytes"]
    TTC16,
}
impl TTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TTCW::TTC64 => 0,
            TTCW::TTC128 => 1,
            TTCW::TTC192 => 2,
            TTCW::TTC256 => 3,
            TTCW::TTC40 => 4,
            TTCW::TTC32 => 5,
            TTCW::TTC24 => 6,
            TTCW::TTC16 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TTCW<'a> {
    w: &'a mut W,
}
impl<'a> _TTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TTCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64 bytes"]
    #[inline]
    pub fn ttc64(self) -> &'a mut W {
        self.variant(TTCW::TTC64)
    }
    #[doc = "128 bytes"]
    #[inline]
    pub fn ttc128(self) -> &'a mut W {
        self.variant(TTCW::TTC128)
    }
    #[doc = "192 bytes"]
    #[inline]
    pub fn ttc192(self) -> &'a mut W {
        self.variant(TTCW::TTC192)
    }
    #[doc = "256 bytes"]
    #[inline]
    pub fn ttc256(self) -> &'a mut W {
        self.variant(TTCW::TTC256)
    }
    #[doc = "40 bytes"]
    #[inline]
    pub fn ttc40(self) -> &'a mut W {
        self.variant(TTCW::TTC40)
    }
    #[doc = "32 bytes"]
    #[inline]
    pub fn ttc32(self) -> &'a mut W {
        self.variant(TTCW::TTC32)
    }
    #[doc = "24 bytes"]
    #[inline]
    pub fn ttc24(self) -> &'a mut W {
        self.variant(TTCW::TTC24)
    }
    #[doc = "16 bytes"]
    #[inline]
    pub fn ttc16(self) -> &'a mut W {
        self.variant(TTCW::TTC16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTF`"]
pub enum FTFW {
    #[doc = "Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    FLUSH,
}
impl FTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTFW::FLUSH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTFW<'a> {
    w: &'a mut W,
}
impl<'a> _FTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    #[inline]
    pub fn flush(self) -> &'a mut W {
        self.variant(FTFW::FLUSH)
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
#[doc = "Values that can be written to the field `TSF`"]
pub enum TSFW {
    #[doc = "Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    CUTTHROUGH,
    #[doc = "Transmission starts when a full frame is in the Tx FIFO"]
    STOREFORWARD,
}
impl TSFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSFW::CUTTHROUGH => false,
            TSFW::STOREFORWARD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSFW<'a> {
    w: &'a mut W,
}
impl<'a> _TSFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    #[inline]
    pub fn cut_through(self) -> &'a mut W {
        self.variant(TSFW::CUTTHROUGH)
    }
    #[doc = "Transmission starts when a full frame is in the Tx FIFO"]
    #[inline]
    pub fn store_forward(self) -> &'a mut W {
        self.variant(TSFW::STOREFORWARD)
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
#[doc = r" Proxy"]
pub struct _DFRFW<'a> {
    w: &'a mut W,
}
impl<'a> _DFRFW<'a> {
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
#[doc = "Values that can be written to the field `RSF`"]
pub enum RSFW {
    #[doc = "Rx FIFO operates in cut-through mode, subject to RTC bits"]
    CUTTHROUGH,
    #[doc = "Frames are read from Rx FIFO after complete frame has been written"]
    STOREFORWARD,
}
impl RSFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSFW::CUTTHROUGH => false,
            RSFW::STOREFORWARD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSFW<'a> {
    w: &'a mut W,
}
impl<'a> _RSFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx FIFO operates in cut-through mode, subject to RTC bits"]
    #[inline]
    pub fn cut_through(self) -> &'a mut W {
        self.variant(RSFW::CUTTHROUGH)
    }
    #[doc = "Frames are read from Rx FIFO after complete frame has been written"]
    #[inline]
    pub fn store_forward(self) -> &'a mut W {
        self.variant(RSFW::STOREFORWARD)
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
#[doc = "Values that can be written to the field `DTCEFD`"]
pub enum DTCEFDW {
    #[doc = "Drop frames with errors only in the receive checksum offload engine"]
    ENABLED,
    #[doc = "Do not drop frames that only have errors in the receive checksum offload engine"]
    DISABLED,
}
impl DTCEFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTCEFDW::ENABLED => false,
            DTCEFDW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTCEFDW<'a> {
    w: &'a mut W,
}
impl<'a> _DTCEFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTCEFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Drop frames with errors only in the receive checksum offload engine"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTCEFDW::ENABLED)
    }
    #[doc = "Do not drop frames that only have errors in the receive checksum offload engine"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTCEFDW::DISABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - SR"]
    #[inline]
    pub fn sr(&self) -> SRR {
        SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - OSF"]
    #[inline]
    pub fn osf(&self) -> OSFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSFR { bits }
    }
    #[doc = "Bits 3:4 - RTC"]
    #[inline]
    pub fn rtc(&self) -> RTCR {
        RTCR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - FUGF"]
    #[inline]
    pub fn fugf(&self) -> FUGFR {
        FUGFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - FEF"]
    #[inline]
    pub fn fef(&self) -> FEFR {
        FEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - ST"]
    #[inline]
    pub fn st(&self) -> STR {
        STR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:16 - TTC"]
    #[inline]
    pub fn ttc(&self) -> TTCR {
        TTCR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - FTF"]
    #[inline]
    pub fn ftf(&self) -> FTFR {
        FTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - TSF"]
    #[inline]
    pub fn tsf(&self) -> TSFR {
        TSFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - DFRF"]
    #[inline]
    pub fn dfrf(&self) -> DFRFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DFRFR { bits }
    }
    #[doc = "Bit 25 - RSF"]
    #[inline]
    pub fn rsf(&self) -> RSFR {
        RSFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - DTCEFD"]
    #[inline]
    pub fn dtcefd(&self) -> DTCEFDR {
        DTCEFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
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
    #[doc = "Bit 1 - SR"]
    #[inline]
    pub fn sr(&mut self) -> _SRW {
        _SRW { w: self }
    }
    #[doc = "Bit 2 - OSF"]
    #[inline]
    pub fn osf(&mut self) -> _OSFW {
        _OSFW { w: self }
    }
    #[doc = "Bits 3:4 - RTC"]
    #[inline]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
    #[doc = "Bit 6 - FUGF"]
    #[inline]
    pub fn fugf(&mut self) -> _FUGFW {
        _FUGFW { w: self }
    }
    #[doc = "Bit 7 - FEF"]
    #[inline]
    pub fn fef(&mut self) -> _FEFW {
        _FEFW { w: self }
    }
    #[doc = "Bit 13 - ST"]
    #[inline]
    pub fn st(&mut self) -> _STW {
        _STW { w: self }
    }
    #[doc = "Bits 14:16 - TTC"]
    #[inline]
    pub fn ttc(&mut self) -> _TTCW {
        _TTCW { w: self }
    }
    #[doc = "Bit 20 - FTF"]
    #[inline]
    pub fn ftf(&mut self) -> _FTFW {
        _FTFW { w: self }
    }
    #[doc = "Bit 21 - TSF"]
    #[inline]
    pub fn tsf(&mut self) -> _TSFW {
        _TSFW { w: self }
    }
    #[doc = "Bit 24 - DFRF"]
    #[inline]
    pub fn dfrf(&mut self) -> _DFRFW {
        _DFRFW { w: self }
    }
    #[doc = "Bit 25 - RSF"]
    #[inline]
    pub fn rsf(&mut self) -> _RSFW {
        _RSFW { w: self }
    }
    #[doc = "Bit 26 - DTCEFD"]
    #[inline]
    pub fn dtcefd(&mut self) -> _DTCEFDW {
        _DTCEFDW { w: self }
    }
}
