#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACCR {
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
#[doc = "Possible values of the field `RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RER {
    #[doc = "MAC receive state machine is disabled after the completion of the reception of the current frame"]
    DISABLED,
    #[doc = "MAC receive state machine is enabled"]
    ENABLED,
}
impl RER {
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
            RER::DISABLED => false,
            RER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RER {
        match value {
            false => RER::DISABLED,
            true => RER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RER::ENABLED
    }
}
#[doc = "Possible values of the field `TE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TER {
    #[doc = "MAC transmit state machine is disabled after completion of the transmission of the current frame"]
    DISABLED,
    #[doc = "MAC transmit state machine is enabled"]
    ENABLED,
}
impl TER {
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
            TER::DISABLED => false,
            TER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TER {
        match value {
            false => TER::DISABLED,
            true => TER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TER::ENABLED
    }
}
#[doc = "Possible values of the field `DC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCR {
    #[doc = "MAC defers until CRS signal goes inactive"]
    DISABLED,
    #[doc = "Deferral check function enabled"]
    ENABLED,
}
impl DCR {
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
            DCR::DISABLED => false,
            DCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCR {
        match value {
            false => DCR::DISABLED,
            true => DCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DCR::ENABLED
    }
}
#[doc = "Possible values of the field `BL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLR {
    #[doc = "For retransmission n, wait up to 2^min(n, 10) time slots"]
    BL10,
    #[doc = "For retransmission n, wait up to 2^min(n, 8) time slots"]
    BL8,
    #[doc = "For retransmission n, wait up to 2^min(n, 4) time slots"]
    BL4,
    #[doc = "For retransmission n, wait up to 2^min(n, 1) time slots"]
    BL1,
}
impl BLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BLR::BL10 => 0,
            BLR::BL8 => 0x01,
            BLR::BL4 => 0x02,
            BLR::BL1 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BLR {
        match value {
            0 => BLR::BL10,
            1 => BLR::BL8,
            2 => BLR::BL4,
            3 => BLR::BL1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BL10`"]
    #[inline]
    pub fn is_bl10(&self) -> bool {
        *self == BLR::BL10
    }
    #[doc = "Checks if the value of the field is `BL8`"]
    #[inline]
    pub fn is_bl8(&self) -> bool {
        *self == BLR::BL8
    }
    #[doc = "Checks if the value of the field is `BL4`"]
    #[inline]
    pub fn is_bl4(&self) -> bool {
        *self == BLR::BL4
    }
    #[doc = "Checks if the value of the field is `BL1`"]
    #[inline]
    pub fn is_bl1(&self) -> bool {
        *self == BLR::BL1
    }
}
#[doc = "Possible values of the field `APCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APCSR {
    #[doc = "MAC passes all incoming frames unmodified"]
    DISABLED,
    #[doc = "MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes"]
    STRIP,
}
impl APCSR {
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
            APCSR::DISABLED => false,
            APCSR::STRIP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APCSR {
        match value {
            false => APCSR::DISABLED,
            true => APCSR::STRIP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == APCSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STRIP`"]
    #[inline]
    pub fn is_strip(&self) -> bool {
        *self == APCSR::STRIP
    }
}
#[doc = "Possible values of the field `RD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDR {
    #[doc = "MAC attempts retries based on the settings of BL"]
    ENABLED,
    #[doc = "MAC attempts only 1 transmission"]
    DISABLED,
}
impl RDR {
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
            RDR::ENABLED => false,
            RDR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDR {
        match value {
            false => RDR::ENABLED,
            true => RDR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RDR::DISABLED
    }
}
#[doc = "Possible values of the field `IPCO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPCOR {
    #[doc = "IPv4 checksum offload disabled"]
    DISABLED,
    #[doc = "IPv4 checksums are checked in received frames"]
    OFFLOAD,
}
impl IPCOR {
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
            IPCOR::DISABLED => false,
            IPCOR::OFFLOAD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPCOR {
        match value {
            false => IPCOR::DISABLED,
            true => IPCOR::OFFLOAD,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IPCOR::DISABLED
    }
    #[doc = "Checks if the value of the field is `OFFLOAD`"]
    #[inline]
    pub fn is_offload(&self) -> bool {
        *self == IPCOR::OFFLOAD
    }
}
#[doc = "Possible values of the field `DM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMR {
    #[doc = "MAC operates in half-duplex mode"]
    HALFDUPLEX,
    #[doc = "MAC operates in full-duplex mode"]
    FULLDUPLEX,
}
impl DMR {
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
            DMR::HALFDUPLEX => false,
            DMR::FULLDUPLEX => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMR {
        match value {
            false => DMR::HALFDUPLEX,
            true => DMR::FULLDUPLEX,
        }
    }
    #[doc = "Checks if the value of the field is `HALFDUPLEX`"]
    #[inline]
    pub fn is_half_duplex(&self) -> bool {
        *self == DMR::HALFDUPLEX
    }
    #[doc = "Checks if the value of the field is `FULLDUPLEX`"]
    #[inline]
    pub fn is_full_duplex(&self) -> bool {
        *self == DMR::FULLDUPLEX
    }
}
#[doc = "Possible values of the field `LM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMR {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "MAC operates in loopback mode at the MII"]
    LOOPBACK,
}
impl LMR {
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
            LMR::NORMAL => false,
            LMR::LOOPBACK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LMR {
        match value {
            false => LMR::NORMAL,
            true => LMR::LOOPBACK,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == LMR::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOOPBACK`"]
    #[inline]
    pub fn is_loopback(&self) -> bool {
        *self == LMR::LOOPBACK
    }
}
#[doc = "Possible values of the field `ROD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RODR {
    #[doc = "MAC receives all packets from PHY while transmitting"]
    ENABLED,
    #[doc = "MAC disables reception of frames in half-duplex mode"]
    DISABLED,
}
impl RODR {
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
            RODR::ENABLED => false,
            RODR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RODR {
        match value {
            false => RODR::ENABLED,
            true => RODR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RODR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RODR::DISABLED
    }
}
#[doc = "Possible values of the field `FES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FESR {
    #[doc = "10 Mbit/s"]
    FES10,
    #[doc = "100 Mbit/s"]
    FES100,
}
impl FESR {
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
            FESR::FES10 => false,
            FESR::FES100 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FESR {
        match value {
            false => FESR::FES10,
            true => FESR::FES100,
        }
    }
    #[doc = "Checks if the value of the field is `FES10`"]
    #[inline]
    pub fn is_fes10(&self) -> bool {
        *self == FESR::FES10
    }
    #[doc = "Checks if the value of the field is `FES100`"]
    #[inline]
    pub fn is_fes100(&self) -> bool {
        *self == FESR::FES100
    }
}
#[doc = "Possible values of the field `CSD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSDR {
    #[doc = "Errors generated due to loss of carrier"]
    ENABLED,
    #[doc = "No error generated due to loss of carrier"]
    DISABLED,
}
impl CSDR {
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
            CSDR::ENABLED => false,
            CSDR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSDR {
        match value {
            false => CSDR::ENABLED,
            true => CSDR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CSDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CSDR::DISABLED
    }
}
#[doc = "Possible values of the field `IFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFGR {
    #[doc = "96 bit times"]
    IFG96,
    #[doc = "88 bit times"]
    IFG88,
    #[doc = "48 bit times"]
    IFG80,
    #[doc = "40 bit times"]
    IFG40,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IFGR::IFG96 => 0,
            IFGR::IFG88 => 0x01,
            IFGR::IFG80 => 0x06,
            IFGR::IFG40 => 0x07,
            IFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IFGR {
        match value {
            0 => IFGR::IFG96,
            1 => IFGR::IFG88,
            6 => IFGR::IFG80,
            7 => IFGR::IFG40,
            i => IFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IFG96`"]
    #[inline]
    pub fn is_ifg96(&self) -> bool {
        *self == IFGR::IFG96
    }
    #[doc = "Checks if the value of the field is `IFG88`"]
    #[inline]
    pub fn is_ifg88(&self) -> bool {
        *self == IFGR::IFG88
    }
    #[doc = "Checks if the value of the field is `IFG80`"]
    #[inline]
    pub fn is_ifg80(&self) -> bool {
        *self == IFGR::IFG80
    }
    #[doc = "Checks if the value of the field is `IFG40`"]
    #[inline]
    pub fn is_ifg40(&self) -> bool {
        *self == IFGR::IFG40
    }
}
#[doc = "Possible values of the field `JD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JDR {
    #[doc = "Jabber enabled, transmit frames up to 2048 bytes"]
    ENABLED,
    #[doc = "Jabber disabled, transmit frames up to 16384 bytes"]
    DISABLED,
}
impl JDR {
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
            JDR::ENABLED => false,
            JDR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JDR {
        match value {
            false => JDR::ENABLED,
            true => JDR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == JDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == JDR::DISABLED
    }
}
#[doc = "Possible values of the field `WD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDR {
    #[doc = "Watchdog enabled, receive frames limited to 2048 bytes"]
    ENABLED,
    #[doc = "Watchdog disabled, receive frames may be up to to 16384 bytes"]
    DISABLED,
}
impl WDR {
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
            WDR::ENABLED => false,
            WDR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDR {
        match value {
            false => WDR::ENABLED,
            true => WDR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WDR::DISABLED
    }
}
#[doc = "Possible values of the field `CSTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSTFR {
    #[doc = "CRC not stripped"]
    DISABLED,
    #[doc = "CRC stripped"]
    ENABLED,
}
impl CSTFR {
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
            CSTFR::DISABLED => false,
            CSTFR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSTFR {
        match value {
            false => CSTFR::DISABLED,
            true => CSTFR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CSTFR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CSTFR::ENABLED
    }
}
#[doc = "Values that can be written to the field `RE`"]
pub enum REW {
    #[doc = "MAC receive state machine is disabled after the completion of the reception of the current frame"]
    DISABLED,
    #[doc = "MAC receive state machine is enabled"]
    ENABLED,
}
impl REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REW::DISABLED => false,
            REW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC receive state machine is disabled after the completion of the reception of the current frame"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REW::DISABLED)
    }
    #[doc = "MAC receive state machine is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REW::ENABLED)
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
#[doc = "Values that can be written to the field `TE`"]
pub enum TEW {
    #[doc = "MAC transmit state machine is disabled after completion of the transmission of the current frame"]
    DISABLED,
    #[doc = "MAC transmit state machine is enabled"]
    ENABLED,
}
impl TEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEW::DISABLED => false,
            TEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC transmit state machine is disabled after completion of the transmission of the current frame"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEW::DISABLED)
    }
    #[doc = "MAC transmit state machine is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEW::ENABLED)
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
#[doc = "Values that can be written to the field `DC`"]
pub enum DCW {
    #[doc = "MAC defers until CRS signal goes inactive"]
    DISABLED,
    #[doc = "Deferral check function enabled"]
    ENABLED,
}
impl DCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCW::DISABLED => false,
            DCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCW<'a> {
    w: &'a mut W,
}
impl<'a> _DCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC defers until CRS signal goes inactive"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCW::DISABLED)
    }
    #[doc = "Deferral check function enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCW::ENABLED)
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
#[doc = "Values that can be written to the field `BL`"]
pub enum BLW {
    #[doc = "For retransmission n, wait up to 2^min(n, 10) time slots"]
    BL10,
    #[doc = "For retransmission n, wait up to 2^min(n, 8) time slots"]
    BL8,
    #[doc = "For retransmission n, wait up to 2^min(n, 4) time slots"]
    BL4,
    #[doc = "For retransmission n, wait up to 2^min(n, 1) time slots"]
    BL1,
}
impl BLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BLW::BL10 => 0,
            BLW::BL8 => 1,
            BLW::BL4 => 2,
            BLW::BL1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLW<'a> {
    w: &'a mut W,
}
impl<'a> _BLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 10) time slots"]
    #[inline]
    pub fn bl10(self) -> &'a mut W {
        self.variant(BLW::BL10)
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 8) time slots"]
    #[inline]
    pub fn bl8(self) -> &'a mut W {
        self.variant(BLW::BL8)
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 4) time slots"]
    #[inline]
    pub fn bl4(self) -> &'a mut W {
        self.variant(BLW::BL4)
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 1) time slots"]
    #[inline]
    pub fn bl1(self) -> &'a mut W {
        self.variant(BLW::BL1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `APCS`"]
pub enum APCSW {
    #[doc = "MAC passes all incoming frames unmodified"]
    DISABLED,
    #[doc = "MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes"]
    STRIP,
}
impl APCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APCSW::DISABLED => false,
            APCSW::STRIP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APCSW<'a> {
    w: &'a mut W,
}
impl<'a> _APCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC passes all incoming frames unmodified"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(APCSW::DISABLED)
    }
    #[doc = "MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes"]
    #[inline]
    pub fn strip(self) -> &'a mut W {
        self.variant(APCSW::STRIP)
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
#[doc = "Values that can be written to the field `RD`"]
pub enum RDW {
    #[doc = "MAC attempts retries based on the settings of BL"]
    ENABLED,
    #[doc = "MAC attempts only 1 transmission"]
    DISABLED,
}
impl RDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDW::ENABLED => false,
            RDW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDW<'a> {
    w: &'a mut W,
}
impl<'a> _RDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC attempts retries based on the settings of BL"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDW::ENABLED)
    }
    #[doc = "MAC attempts only 1 transmission"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDW::DISABLED)
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
#[doc = "Values that can be written to the field `IPCO`"]
pub enum IPCOW {
    #[doc = "IPv4 checksum offload disabled"]
    DISABLED,
    #[doc = "IPv4 checksums are checked in received frames"]
    OFFLOAD,
}
impl IPCOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPCOW::DISABLED => false,
            IPCOW::OFFLOAD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPCOW<'a> {
    w: &'a mut W,
}
impl<'a> _IPCOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPCOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IPv4 checksum offload disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IPCOW::DISABLED)
    }
    #[doc = "IPv4 checksums are checked in received frames"]
    #[inline]
    pub fn offload(self) -> &'a mut W {
        self.variant(IPCOW::OFFLOAD)
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
#[doc = "Values that can be written to the field `DM`"]
pub enum DMW {
    #[doc = "MAC operates in half-duplex mode"]
    HALFDUPLEX,
    #[doc = "MAC operates in full-duplex mode"]
    FULLDUPLEX,
}
impl DMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMW::HALFDUPLEX => false,
            DMW::FULLDUPLEX => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC operates in half-duplex mode"]
    #[inline]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(DMW::HALFDUPLEX)
    }
    #[doc = "MAC operates in full-duplex mode"]
    #[inline]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(DMW::FULLDUPLEX)
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
#[doc = "Values that can be written to the field `LM`"]
pub enum LMW {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "MAC operates in loopback mode at the MII"]
    LOOPBACK,
}
impl LMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LMW::NORMAL => false,
            LMW::LOOPBACK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LMW<'a> {
    w: &'a mut W,
}
impl<'a> _LMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(LMW::NORMAL)
    }
    #[doc = "MAC operates in loopback mode at the MII"]
    #[inline]
    pub fn loopback(self) -> &'a mut W {
        self.variant(LMW::LOOPBACK)
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
#[doc = "Values that can be written to the field `ROD`"]
pub enum RODW {
    #[doc = "MAC receives all packets from PHY while transmitting"]
    ENABLED,
    #[doc = "MAC disables reception of frames in half-duplex mode"]
    DISABLED,
}
impl RODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RODW::ENABLED => false,
            RODW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RODW<'a> {
    w: &'a mut W,
}
impl<'a> _RODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC receives all packets from PHY while transmitting"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RODW::ENABLED)
    }
    #[doc = "MAC disables reception of frames in half-duplex mode"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RODW::DISABLED)
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
#[doc = "Values that can be written to the field `FES`"]
pub enum FESW {
    #[doc = "10 Mbit/s"]
    FES10,
    #[doc = "100 Mbit/s"]
    FES100,
}
impl FESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FESW::FES10 => false,
            FESW::FES100 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FESW<'a> {
    w: &'a mut W,
}
impl<'a> _FESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "10 Mbit/s"]
    #[inline]
    pub fn fes10(self) -> &'a mut W {
        self.variant(FESW::FES10)
    }
    #[doc = "100 Mbit/s"]
    #[inline]
    pub fn fes100(self) -> &'a mut W {
        self.variant(FESW::FES100)
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
#[doc = "Values that can be written to the field `CSD`"]
pub enum CSDW {
    #[doc = "Errors generated due to loss of carrier"]
    ENABLED,
    #[doc = "No error generated due to loss of carrier"]
    DISABLED,
}
impl CSDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSDW::ENABLED => false,
            CSDW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSDW<'a> {
    w: &'a mut W,
}
impl<'a> _CSDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Errors generated due to loss of carrier"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSDW::ENABLED)
    }
    #[doc = "No error generated due to loss of carrier"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSDW::DISABLED)
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
#[doc = "Values that can be written to the field `IFG`"]
pub enum IFGW {
    #[doc = "96 bit times"]
    IFG96,
    #[doc = "88 bit times"]
    IFG88,
    #[doc = "48 bit times"]
    IFG80,
    #[doc = "40 bit times"]
    IFG40,
}
impl IFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IFGW::IFG96 => 0,
            IFGW::IFG88 => 1,
            IFGW::IFG80 => 6,
            IFGW::IFG40 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IFGW<'a> {
    w: &'a mut W,
}
impl<'a> _IFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "96 bit times"]
    #[inline]
    pub fn ifg96(self) -> &'a mut W {
        self.variant(IFGW::IFG96)
    }
    #[doc = "88 bit times"]
    #[inline]
    pub fn ifg88(self) -> &'a mut W {
        self.variant(IFGW::IFG88)
    }
    #[doc = "48 bit times"]
    #[inline]
    pub fn ifg80(self) -> &'a mut W {
        self.variant(IFGW::IFG80)
    }
    #[doc = "40 bit times"]
    #[inline]
    pub fn ifg40(self) -> &'a mut W {
        self.variant(IFGW::IFG40)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `JD`"]
pub enum JDW {
    #[doc = "Jabber enabled, transmit frames up to 2048 bytes"]
    ENABLED,
    #[doc = "Jabber disabled, transmit frames up to 16384 bytes"]
    DISABLED,
}
impl JDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JDW::ENABLED => false,
            JDW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JDW<'a> {
    w: &'a mut W,
}
impl<'a> _JDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Jabber enabled, transmit frames up to 2048 bytes"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JDW::ENABLED)
    }
    #[doc = "Jabber disabled, transmit frames up to 16384 bytes"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JDW::DISABLED)
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
#[doc = "Values that can be written to the field `WD`"]
pub enum WDW {
    #[doc = "Watchdog enabled, receive frames limited to 2048 bytes"]
    ENABLED,
    #[doc = "Watchdog disabled, receive frames may be up to to 16384 bytes"]
    DISABLED,
}
impl WDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDW::ENABLED => false,
            WDW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDW<'a> {
    w: &'a mut W,
}
impl<'a> _WDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog enabled, receive frames limited to 2048 bytes"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDW::ENABLED)
    }
    #[doc = "Watchdog disabled, receive frames may be up to to 16384 bytes"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDW::DISABLED)
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
#[doc = "Values that can be written to the field `CSTF`"]
pub enum CSTFW {
    #[doc = "CRC not stripped"]
    DISABLED,
    #[doc = "CRC stripped"]
    ENABLED,
}
impl CSTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSTFW::DISABLED => false,
            CSTFW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSTFW<'a> {
    w: &'a mut W,
}
impl<'a> _CSTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CRC not stripped"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSTFW::DISABLED)
    }
    #[doc = "CRC stripped"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSTFW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - RE"]
    #[inline]
    pub fn re(&self) -> RER {
        RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - TE"]
    #[inline]
    pub fn te(&self) -> TER {
        TER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - DC"]
    #[inline]
    pub fn dc(&self) -> DCR {
        DCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - BL"]
    #[inline]
    pub fn bl(&self) -> BLR {
        BLR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - APCS"]
    #[inline]
    pub fn apcs(&self) -> APCSR {
        APCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - RD"]
    #[inline]
    pub fn rd(&self) -> RDR {
        RDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - IPCO"]
    #[inline]
    pub fn ipco(&self) -> IPCOR {
        IPCOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - DM"]
    #[inline]
    pub fn dm(&self) -> DMR {
        DMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - LM"]
    #[inline]
    pub fn lm(&self) -> LMR {
        LMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - ROD"]
    #[inline]
    pub fn rod(&self) -> RODR {
        RODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - FES"]
    #[inline]
    pub fn fes(&self) -> FESR {
        FESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - CSD"]
    #[inline]
    pub fn csd(&self) -> CSDR {
        CSDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:19 - IFG"]
    #[inline]
    pub fn ifg(&self) -> IFGR {
        IFGR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - JD"]
    #[inline]
    pub fn jd(&self) -> JDR {
        JDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - WD"]
    #[inline]
    pub fn wd(&self) -> WDR {
        WDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CSTF"]
    #[inline]
    pub fn cstf(&self) -> CSTFR {
        CSTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x8000 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - RE"]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 3 - TE"]
    #[inline]
    pub fn te(&mut self) -> _TEW {
        _TEW { w: self }
    }
    #[doc = "Bit 4 - DC"]
    #[inline]
    pub fn dc(&mut self) -> _DCW {
        _DCW { w: self }
    }
    #[doc = "Bits 5:6 - BL"]
    #[inline]
    pub fn bl(&mut self) -> _BLW {
        _BLW { w: self }
    }
    #[doc = "Bit 7 - APCS"]
    #[inline]
    pub fn apcs(&mut self) -> _APCSW {
        _APCSW { w: self }
    }
    #[doc = "Bit 9 - RD"]
    #[inline]
    pub fn rd(&mut self) -> _RDW {
        _RDW { w: self }
    }
    #[doc = "Bit 10 - IPCO"]
    #[inline]
    pub fn ipco(&mut self) -> _IPCOW {
        _IPCOW { w: self }
    }
    #[doc = "Bit 11 - DM"]
    #[inline]
    pub fn dm(&mut self) -> _DMW {
        _DMW { w: self }
    }
    #[doc = "Bit 12 - LM"]
    #[inline]
    pub fn lm(&mut self) -> _LMW {
        _LMW { w: self }
    }
    #[doc = "Bit 13 - ROD"]
    #[inline]
    pub fn rod(&mut self) -> _RODW {
        _RODW { w: self }
    }
    #[doc = "Bit 14 - FES"]
    #[inline]
    pub fn fes(&mut self) -> _FESW {
        _FESW { w: self }
    }
    #[doc = "Bit 16 - CSD"]
    #[inline]
    pub fn csd(&mut self) -> _CSDW {
        _CSDW { w: self }
    }
    #[doc = "Bits 17:19 - IFG"]
    #[inline]
    pub fn ifg(&mut self) -> _IFGW {
        _IFGW { w: self }
    }
    #[doc = "Bit 22 - JD"]
    #[inline]
    pub fn jd(&mut self) -> _JDW {
        _JDW { w: self }
    }
    #[doc = "Bit 23 - WD"]
    #[inline]
    pub fn wd(&mut self) -> _WDW {
        _WDW { w: self }
    }
    #[doc = "Bit 25 - CSTF"]
    #[inline]
    pub fn cstf(&mut self) -> _CSTFW {
        _CSTFW { w: self }
    }
}
