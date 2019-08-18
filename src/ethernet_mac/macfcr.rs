#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACFCR {
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
#[doc = "Possible values of the field `FCB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCBR {
    #[doc = "In full duplex, initiate a Pause control frame. In half duplex, assert back pressure"]
    PAUSEORBACKPRESSURE,
    #[doc = "In half duplex only, deasserts back pressure"]
    DISABLEBACKPRESSURE,
}
impl FCBR {
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
            FCBR::PAUSEORBACKPRESSURE => true,
            FCBR::DISABLEBACKPRESSURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCBR {
        match value {
            true => FCBR::PAUSEORBACKPRESSURE,
            false => FCBR::DISABLEBACKPRESSURE,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSEORBACKPRESSURE`"]
    #[inline]
    pub fn is_pause_or_back_pressure(&self) -> bool {
        *self == FCBR::PAUSEORBACKPRESSURE
    }
    #[doc = "Checks if the value of the field is `DISABLEBACKPRESSURE`"]
    #[inline]
    pub fn is_disable_back_pressure(&self) -> bool {
        *self == FCBR::DISABLEBACKPRESSURE
    }
}
#[doc = "Possible values of the field `TFCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFCER {
    #[doc = "In full duplex, flow control is disabled. In half duplex, back pressure is disabled"]
    DISABLED,
    #[doc = "In full duplex, flow control is enabled. In half duplex, back pressure is enabled"]
    ENABLED,
}
impl TFCER {
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
            TFCER::DISABLED => false,
            TFCER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFCER {
        match value {
            false => TFCER::DISABLED,
            true => TFCER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TFCER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TFCER::ENABLED
    }
}
#[doc = "Possible values of the field `RFCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCER {
    #[doc = "Pause frames are not decoded"]
    DISABLED,
    #[doc = "MAC decodes received Pause frames and disables its transmitted for a specified time"]
    ENABLED,
}
impl RFCER {
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
            RFCER::DISABLED => false,
            RFCER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFCER {
        match value {
            false => RFCER::DISABLED,
            true => RFCER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RFCER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RFCER::ENABLED
    }
}
#[doc = "Possible values of the field `UPFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPFDR {
    #[doc = "MAC detects only a Pause frame with the multicast address specified in the 802.3x standard"]
    DISABLED,
    #[doc = "MAC additionally detects Pause frames with the station's unicast address"]
    ENABLED,
}
impl UPFDR {
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
            UPFDR::DISABLED => false,
            UPFDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UPFDR {
        match value {
            false => UPFDR::DISABLED,
            true => UPFDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == UPFDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == UPFDR::ENABLED
    }
}
#[doc = "Possible values of the field `PLT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLTR {
    #[doc = "Pause time minus 4 slot times"]
    PLT4,
    #[doc = "Pause time minus 28 slot times"]
    PLT28,
    #[doc = "Pause time minus 144 slot times"]
    PLT144,
    #[doc = "Pause time minus 256 slot times"]
    PLT256,
}
impl PLTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLTR::PLT4 => 0,
            PLTR::PLT28 => 0x01,
            PLTR::PLT144 => 0x02,
            PLTR::PLT256 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLTR {
        match value {
            0 => PLTR::PLT4,
            1 => PLTR::PLT28,
            2 => PLTR::PLT144,
            3 => PLTR::PLT256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PLT4`"]
    #[inline]
    pub fn is_plt4(&self) -> bool {
        *self == PLTR::PLT4
    }
    #[doc = "Checks if the value of the field is `PLT28`"]
    #[inline]
    pub fn is_plt28(&self) -> bool {
        *self == PLTR::PLT28
    }
    #[doc = "Checks if the value of the field is `PLT144`"]
    #[inline]
    pub fn is_plt144(&self) -> bool {
        *self == PLTR::PLT144
    }
    #[doc = "Checks if the value of the field is `PLT256`"]
    #[inline]
    pub fn is_plt256(&self) -> bool {
        *self == PLTR::PLT256
    }
}
#[doc = "Possible values of the field `ZQPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZQPDR {
    #[doc = "Normal operation with automatic zero-quanta pause control frame generation"]
    ENABLED,
    #[doc = "Automatic generation of zero-quanta pause control frames is disabled"]
    DISABLED,
}
impl ZQPDR {
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
            ZQPDR::ENABLED => false,
            ZQPDR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZQPDR {
        match value {
            false => ZQPDR::ENABLED,
            true => ZQPDR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ZQPDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ZQPDR::DISABLED
    }
}
#[doc = r" Value of the field"]
pub struct PTR {
    bits: u16,
}
impl PTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `FCB`"]
pub enum FCBW {
    #[doc = "In full duplex, initiate a Pause control frame. In half duplex, assert back pressure"]
    PAUSEORBACKPRESSURE,
    #[doc = "In half duplex only, deasserts back pressure"]
    DISABLEBACKPRESSURE,
}
impl FCBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCBW::PAUSEORBACKPRESSURE => true,
            FCBW::DISABLEBACKPRESSURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCBW<'a> {
    w: &'a mut W,
}
impl<'a> _FCBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "In full duplex, initiate a Pause control frame. In half duplex, assert back pressure"]
    #[inline]
    pub fn pause_or_back_pressure(self) -> &'a mut W {
        self.variant(FCBW::PAUSEORBACKPRESSURE)
    }
    #[doc = "In half duplex only, deasserts back pressure"]
    #[inline]
    pub fn disable_back_pressure(self) -> &'a mut W {
        self.variant(FCBW::DISABLEBACKPRESSURE)
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
#[doc = "Values that can be written to the field `TFCE`"]
pub enum TFCEW {
    #[doc = "In full duplex, flow control is disabled. In half duplex, back pressure is disabled"]
    DISABLED,
    #[doc = "In full duplex, flow control is enabled. In half duplex, back pressure is enabled"]
    ENABLED,
}
impl TFCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFCEW::DISABLED => false,
            TFCEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFCEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "In full duplex, flow control is disabled. In half duplex, back pressure is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TFCEW::DISABLED)
    }
    #[doc = "In full duplex, flow control is enabled. In half duplex, back pressure is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TFCEW::ENABLED)
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
#[doc = "Values that can be written to the field `RFCE`"]
pub enum RFCEW {
    #[doc = "Pause frames are not decoded"]
    DISABLED,
    #[doc = "MAC decodes received Pause frames and disables its transmitted for a specified time"]
    ENABLED,
}
impl RFCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFCEW::DISABLED => false,
            RFCEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFCEW<'a> {
    w: &'a mut W,
}
impl<'a> _RFCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pause frames are not decoded"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFCEW::DISABLED)
    }
    #[doc = "MAC decodes received Pause frames and disables its transmitted for a specified time"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFCEW::ENABLED)
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
#[doc = "Values that can be written to the field `UPFD`"]
pub enum UPFDW {
    #[doc = "MAC detects only a Pause frame with the multicast address specified in the 802.3x standard"]
    DISABLED,
    #[doc = "MAC additionally detects Pause frames with the station's unicast address"]
    ENABLED,
}
impl UPFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UPFDW::DISABLED => false,
            UPFDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPFDW<'a> {
    w: &'a mut W,
}
impl<'a> _UPFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC detects only a Pause frame with the multicast address specified in the 802.3x standard"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPFDW::DISABLED)
    }
    #[doc = "MAC additionally detects Pause frames with the station's unicast address"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPFDW::ENABLED)
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
#[doc = "Values that can be written to the field `PLT`"]
pub enum PLTW {
    #[doc = "Pause time minus 4 slot times"]
    PLT4,
    #[doc = "Pause time minus 28 slot times"]
    PLT28,
    #[doc = "Pause time minus 144 slot times"]
    PLT144,
    #[doc = "Pause time minus 256 slot times"]
    PLT256,
}
impl PLTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLTW::PLT4 => 0,
            PLTW::PLT28 => 1,
            PLTW::PLT144 => 2,
            PLTW::PLT256 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLTW<'a> {
    w: &'a mut W,
}
impl<'a> _PLTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pause time minus 4 slot times"]
    #[inline]
    pub fn plt4(self) -> &'a mut W {
        self.variant(PLTW::PLT4)
    }
    #[doc = "Pause time minus 28 slot times"]
    #[inline]
    pub fn plt28(self) -> &'a mut W {
        self.variant(PLTW::PLT28)
    }
    #[doc = "Pause time minus 144 slot times"]
    #[inline]
    pub fn plt144(self) -> &'a mut W {
        self.variant(PLTW::PLT144)
    }
    #[doc = "Pause time minus 256 slot times"]
    #[inline]
    pub fn plt256(self) -> &'a mut W {
        self.variant(PLTW::PLT256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ZQPD`"]
pub enum ZQPDW {
    #[doc = "Normal operation with automatic zero-quanta pause control frame generation"]
    ENABLED,
    #[doc = "Automatic generation of zero-quanta pause control frames is disabled"]
    DISABLED,
}
impl ZQPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZQPDW::ENABLED => false,
            ZQPDW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZQPDW<'a> {
    w: &'a mut W,
}
impl<'a> _ZQPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZQPDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation with automatic zero-quanta pause control frame generation"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ZQPDW::ENABLED)
    }
    #[doc = "Automatic generation of zero-quanta pause control frames is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ZQPDW::DISABLED)
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
#[doc = r" Proxy"]
pub struct _PTW<'a> {
    w: &'a mut W,
}
impl<'a> _PTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0xffff;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - FCB"]
    #[inline]
    pub fn fcb(&self) -> FCBR {
        FCBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TFCE"]
    #[inline]
    pub fn tfce(&self) -> TFCER {
        TFCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RFCE"]
    #[inline]
    pub fn rfce(&self) -> RFCER {
        RFCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - UPFD"]
    #[inline]
    pub fn upfd(&self) -> UPFDR {
        UPFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - PLT"]
    #[inline]
    pub fn plt(&self) -> PLTR {
        PLTR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - ZQPD"]
    #[inline]
    pub fn zqpd(&self) -> ZQPDR {
        ZQPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline]
    pub fn pt(&self) -> PTR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PTR { bits }
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
    #[doc = "Bit 0 - FCB"]
    #[inline]
    pub fn fcb(&mut self) -> _FCBW {
        _FCBW { w: self }
    }
    #[doc = "Bit 1 - TFCE"]
    #[inline]
    pub fn tfce(&mut self) -> _TFCEW {
        _TFCEW { w: self }
    }
    #[doc = "Bit 2 - RFCE"]
    #[inline]
    pub fn rfce(&mut self) -> _RFCEW {
        _RFCEW { w: self }
    }
    #[doc = "Bit 3 - UPFD"]
    #[inline]
    pub fn upfd(&mut self) -> _UPFDW {
        _UPFDW { w: self }
    }
    #[doc = "Bits 4:5 - PLT"]
    #[inline]
    pub fn plt(&mut self) -> _PLTW {
        _PLTW { w: self }
    }
    #[doc = "Bit 7 - ZQPD"]
    #[inline]
    pub fn zqpd(&mut self) -> _ZQPDW {
        _ZQPDW { w: self }
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline]
    pub fn pt(&mut self) -> _PTW {
        _PTW { w: self }
    }
}
