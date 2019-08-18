#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMABMR {
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
    #[doc = "Reset all MAC subsystem internal registers and logic. Cleared automatically"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
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
            SRR::RESET => true,
            SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRR {
        match value {
            true => SRR::RESET,
            i => SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SRR::RESET
    }
}
#[doc = "Possible values of the field `DA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAR {
    #[doc = "Round-robin with Rx:Tx priority given by PM"]
    ROUNDROBIN,
    #[doc = "Rx has priority over Tx"]
    RXPRIORITY,
}
impl DAR {
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
            DAR::ROUNDROBIN => false,
            DAR::RXPRIORITY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAR {
        match value {
            false => DAR::ROUNDROBIN,
            true => DAR::RXPRIORITY,
        }
    }
    #[doc = "Checks if the value of the field is `ROUNDROBIN`"]
    #[inline]
    pub fn is_round_robin(&self) -> bool {
        *self == DAR::ROUNDROBIN
    }
    #[doc = "Checks if the value of the field is `RXPRIORITY`"]
    #[inline]
    pub fn is_rx_priority(&self) -> bool {
        *self == DAR::RXPRIORITY
    }
}
#[doc = r" Value of the field"]
pub struct DSLR {
    bits: u8,
}
impl DSLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EDFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDFER {
    #[doc = "Normal descriptor format"]
    DISABLED,
    #[doc = "Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload"]
    ENABLED,
}
impl EDFER {
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
            EDFER::DISABLED => false,
            EDFER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDFER {
        match value {
            false => EDFER::DISABLED,
            true => EDFER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EDFER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EDFER::ENABLED
    }
}
#[doc = "Possible values of the field `PBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBLR {
    #[doc = "Maximum of 1 beat per DMA transaction"]
    PBL1,
    #[doc = "Maximum of 2 beats per DMA transaction"]
    PBL2,
    #[doc = "Maximum of 4 beats per DMA transaction"]
    PBL4,
    #[doc = "Maximum of 8 beats per DMA transaction"]
    PBL8,
    #[doc = "Maximum of 16 beats per DMA transaction"]
    PBL16,
    #[doc = "Maximum of 32 beats per DMA transaction"]
    PBL32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PBLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PBLR::PBL1 => 0x01,
            PBLR::PBL2 => 0x02,
            PBLR::PBL4 => 0x04,
            PBLR::PBL8 => 0x08,
            PBLR::PBL16 => 0x10,
            PBLR::PBL32 => 0x20,
            PBLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PBLR {
        match value {
            1 => PBLR::PBL1,
            2 => PBLR::PBL2,
            4 => PBLR::PBL4,
            8 => PBLR::PBL8,
            16 => PBLR::PBL16,
            32 => PBLR::PBL32,
            i => PBLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PBL1`"]
    #[inline]
    pub fn is_pbl1(&self) -> bool {
        *self == PBLR::PBL1
    }
    #[doc = "Checks if the value of the field is `PBL2`"]
    #[inline]
    pub fn is_pbl2(&self) -> bool {
        *self == PBLR::PBL2
    }
    #[doc = "Checks if the value of the field is `PBL4`"]
    #[inline]
    pub fn is_pbl4(&self) -> bool {
        *self == PBLR::PBL4
    }
    #[doc = "Checks if the value of the field is `PBL8`"]
    #[inline]
    pub fn is_pbl8(&self) -> bool {
        *self == PBLR::PBL8
    }
    #[doc = "Checks if the value of the field is `PBL16`"]
    #[inline]
    pub fn is_pbl16(&self) -> bool {
        *self == PBLR::PBL16
    }
    #[doc = "Checks if the value of the field is `PBL32`"]
    #[inline]
    pub fn is_pbl32(&self) -> bool {
        *self == PBLR::PBL32
    }
}
#[doc = "Possible values of the field `PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMR {
    #[doc = "RxDMA priority over TxDMA is 1:1"]
    ONETOONE,
    #[doc = "RxDMA priority over TxDMA is 2:1"]
    TWOTOONE,
    #[doc = "RxDMA priority over TxDMA is 3:1"]
    THREETOONE,
    #[doc = "RxDMA priority over TxDMA is 4:1"]
    FOURTOONE,
}
impl PMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMR::ONETOONE => 0,
            PMR::TWOTOONE => 0x01,
            PMR::THREETOONE => 0x02,
            PMR::FOURTOONE => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMR {
        match value {
            0 => PMR::ONETOONE,
            1 => PMR::TWOTOONE,
            2 => PMR::THREETOONE,
            3 => PMR::FOURTOONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONETOONE`"]
    #[inline]
    pub fn is_one_to_one(&self) -> bool {
        *self == PMR::ONETOONE
    }
    #[doc = "Checks if the value of the field is `TWOTOONE`"]
    #[inline]
    pub fn is_two_to_one(&self) -> bool {
        *self == PMR::TWOTOONE
    }
    #[doc = "Checks if the value of the field is `THREETOONE`"]
    #[inline]
    pub fn is_three_to_one(&self) -> bool {
        *self == PMR::THREETOONE
    }
    #[doc = "Checks if the value of the field is `FOURTOONE`"]
    #[inline]
    pub fn is_four_to_one(&self) -> bool {
        *self == PMR::FOURTOONE
    }
}
#[doc = "Possible values of the field `FB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBR {
    #[doc = "AHB uses SINGLE and INCR burst transfers"]
    VARIABLE,
    #[doc = "AHB uses only fixed burst transfers"]
    FIXED,
}
impl FBR {
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
            FBR::VARIABLE => false,
            FBR::FIXED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FBR {
        match value {
            false => FBR::VARIABLE,
            true => FBR::FIXED,
        }
    }
    #[doc = "Checks if the value of the field is `VARIABLE`"]
    #[inline]
    pub fn is_variable(&self) -> bool {
        *self == FBR::VARIABLE
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline]
    pub fn is_fixed(&self) -> bool {
        *self == FBR::FIXED
    }
}
#[doc = "Possible values of the field `RDP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDPR {
    #[doc = "1 beat per RxDMA transaction"]
    RDP1,
    #[doc = "2 beats per RxDMA transaction"]
    RDP2,
    #[doc = "4 beats per RxDMA transaction"]
    RDP4,
    #[doc = "8 beats per RxDMA transaction"]
    RDP8,
    #[doc = "16 beats per RxDMA transaction"]
    RDP16,
    #[doc = "32 beats per RxDMA transaction"]
    RDP32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RDPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RDPR::RDP1 => 0x01,
            RDPR::RDP2 => 0x02,
            RDPR::RDP4 => 0x04,
            RDPR::RDP8 => 0x08,
            RDPR::RDP16 => 0x10,
            RDPR::RDP32 => 0x20,
            RDPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RDPR {
        match value {
            1 => RDPR::RDP1,
            2 => RDPR::RDP2,
            4 => RDPR::RDP4,
            8 => RDPR::RDP8,
            16 => RDPR::RDP16,
            32 => RDPR::RDP32,
            i => RDPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RDP1`"]
    #[inline]
    pub fn is_rdp1(&self) -> bool {
        *self == RDPR::RDP1
    }
    #[doc = "Checks if the value of the field is `RDP2`"]
    #[inline]
    pub fn is_rdp2(&self) -> bool {
        *self == RDPR::RDP2
    }
    #[doc = "Checks if the value of the field is `RDP4`"]
    #[inline]
    pub fn is_rdp4(&self) -> bool {
        *self == RDPR::RDP4
    }
    #[doc = "Checks if the value of the field is `RDP8`"]
    #[inline]
    pub fn is_rdp8(&self) -> bool {
        *self == RDPR::RDP8
    }
    #[doc = "Checks if the value of the field is `RDP16`"]
    #[inline]
    pub fn is_rdp16(&self) -> bool {
        *self == RDPR::RDP16
    }
    #[doc = "Checks if the value of the field is `RDP32`"]
    #[inline]
    pub fn is_rdp32(&self) -> bool {
        *self == RDPR::RDP32
    }
}
#[doc = "Possible values of the field `USP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USPR {
    #[doc = "PBL value used for both Rx and Tx DMA"]
    COMBINED,
    #[doc = "RxDMA uses RDP value, TxDMA uses PBL value"]
    SEPARATE,
}
impl USPR {
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
            USPR::COMBINED => false,
            USPR::SEPARATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USPR {
        match value {
            false => USPR::COMBINED,
            true => USPR::SEPARATE,
        }
    }
    #[doc = "Checks if the value of the field is `COMBINED`"]
    #[inline]
    pub fn is_combined(&self) -> bool {
        *self == USPR::COMBINED
    }
    #[doc = "Checks if the value of the field is `SEPARATE`"]
    #[inline]
    pub fn is_separate(&self) -> bool {
        *self == USPR::SEPARATE
    }
}
#[doc = "Possible values of the field `FPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPMR {
    #[doc = "PBL values used as-is"]
    X1,
    #[doc = "PBL values multiplied by 4"]
    X4,
}
impl FPMR {
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
            FPMR::X1 => false,
            FPMR::X4 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPMR {
        match value {
            false => FPMR::X1,
            true => FPMR::X4,
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline]
    pub fn is_x1(&self) -> bool {
        *self == FPMR::X1
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline]
    pub fn is_x4(&self) -> bool {
        *self == FPMR::X4
    }
}
#[doc = "Possible values of the field `AAB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AABR {
    #[doc = "Bursts are not aligned"]
    UNALIGNED,
    #[doc = "Align bursts to start address LS bits. First burst alignment depends on FB bit"]
    ALIGNED,
}
impl AABR {
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
            AABR::UNALIGNED => false,
            AABR::ALIGNED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AABR {
        match value {
            false => AABR::UNALIGNED,
            true => AABR::ALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `UNALIGNED`"]
    #[inline]
    pub fn is_unaligned(&self) -> bool {
        *self == AABR::UNALIGNED
    }
    #[doc = "Checks if the value of the field is `ALIGNED`"]
    #[inline]
    pub fn is_aligned(&self) -> bool {
        *self == AABR::ALIGNED
    }
}
#[doc = "Possible values of the field `MB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBR {
    #[doc = "Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below"]
    NORMAL,
    #[doc = "If FB is low, start all bursts greater than 16 with INCR (undefined burst)"]
    MIXED,
}
impl MBR {
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
            MBR::NORMAL => false,
            MBR::MIXED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MBR {
        match value {
            false => MBR::NORMAL,
            true => MBR::MIXED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == MBR::NORMAL
    }
    #[doc = "Checks if the value of the field is `MIXED`"]
    #[inline]
    pub fn is_mixed(&self) -> bool {
        *self == MBR::MIXED
    }
}
#[doc = "Values that can be written to the field `SR`"]
pub enum SRW {
    #[doc = "Reset all MAC subsystem internal registers and logic. Cleared automatically"]
    RESET,
}
impl SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRW::RESET => true,
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
    #[doc = "Reset all MAC subsystem internal registers and logic. Cleared automatically"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SRW::RESET)
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
#[doc = "Values that can be written to the field `DA`"]
pub enum DAW {
    #[doc = "Round-robin with Rx:Tx priority given by PM"]
    ROUNDROBIN,
    #[doc = "Rx has priority over Tx"]
    RXPRIORITY,
}
impl DAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAW::ROUNDROBIN => false,
            DAW::RXPRIORITY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAW<'a> {
    w: &'a mut W,
}
impl<'a> _DAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Round-robin with Rx:Tx priority given by PM"]
    #[inline]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(DAW::ROUNDROBIN)
    }
    #[doc = "Rx has priority over Tx"]
    #[inline]
    pub fn rx_priority(self) -> &'a mut W {
        self.variant(DAW::RXPRIORITY)
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
pub struct _DSLW<'a> {
    w: &'a mut W,
}
impl<'a> _DSLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDFE`"]
pub enum EDFEW {
    #[doc = "Normal descriptor format"]
    DISABLED,
    #[doc = "Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload"]
    ENABLED,
}
impl EDFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDFEW::DISABLED => false,
            EDFEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDFEW<'a> {
    w: &'a mut W,
}
impl<'a> _EDFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal descriptor format"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDFEW::DISABLED)
    }
    #[doc = "Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EDFEW::ENABLED)
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
#[doc = "Values that can be written to the field `PBL`"]
pub enum PBLW {
    #[doc = "Maximum of 1 beat per DMA transaction"]
    PBL1,
    #[doc = "Maximum of 2 beats per DMA transaction"]
    PBL2,
    #[doc = "Maximum of 4 beats per DMA transaction"]
    PBL4,
    #[doc = "Maximum of 8 beats per DMA transaction"]
    PBL8,
    #[doc = "Maximum of 16 beats per DMA transaction"]
    PBL16,
    #[doc = "Maximum of 32 beats per DMA transaction"]
    PBL32,
}
impl PBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PBLW::PBL1 => 1,
            PBLW::PBL2 => 2,
            PBLW::PBL4 => 4,
            PBLW::PBL8 => 8,
            PBLW::PBL16 => 16,
            PBLW::PBL32 => 32,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBLW<'a> {
    w: &'a mut W,
}
impl<'a> _PBLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Maximum of 1 beat per DMA transaction"]
    #[inline]
    pub fn pbl1(self) -> &'a mut W {
        self.variant(PBLW::PBL1)
    }
    #[doc = "Maximum of 2 beats per DMA transaction"]
    #[inline]
    pub fn pbl2(self) -> &'a mut W {
        self.variant(PBLW::PBL2)
    }
    #[doc = "Maximum of 4 beats per DMA transaction"]
    #[inline]
    pub fn pbl4(self) -> &'a mut W {
        self.variant(PBLW::PBL4)
    }
    #[doc = "Maximum of 8 beats per DMA transaction"]
    #[inline]
    pub fn pbl8(self) -> &'a mut W {
        self.variant(PBLW::PBL8)
    }
    #[doc = "Maximum of 16 beats per DMA transaction"]
    #[inline]
    pub fn pbl16(self) -> &'a mut W {
        self.variant(PBLW::PBL16)
    }
    #[doc = "Maximum of 32 beats per DMA transaction"]
    #[inline]
    pub fn pbl32(self) -> &'a mut W {
        self.variant(PBLW::PBL32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x3f;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PM`"]
pub enum PMW {
    #[doc = "RxDMA priority over TxDMA is 1:1"]
    ONETOONE,
    #[doc = "RxDMA priority over TxDMA is 2:1"]
    TWOTOONE,
    #[doc = "RxDMA priority over TxDMA is 3:1"]
    THREETOONE,
    #[doc = "RxDMA priority over TxDMA is 4:1"]
    FOURTOONE,
}
impl PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PMW::ONETOONE => 0,
            PMW::TWOTOONE => 1,
            PMW::THREETOONE => 2,
            PMW::FOURTOONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "RxDMA priority over TxDMA is 1:1"]
    #[inline]
    pub fn one_to_one(self) -> &'a mut W {
        self.variant(PMW::ONETOONE)
    }
    #[doc = "RxDMA priority over TxDMA is 2:1"]
    #[inline]
    pub fn two_to_one(self) -> &'a mut W {
        self.variant(PMW::TWOTOONE)
    }
    #[doc = "RxDMA priority over TxDMA is 3:1"]
    #[inline]
    pub fn three_to_one(self) -> &'a mut W {
        self.variant(PMW::THREETOONE)
    }
    #[doc = "RxDMA priority over TxDMA is 4:1"]
    #[inline]
    pub fn four_to_one(self) -> &'a mut W {
        self.variant(PMW::FOURTOONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FB`"]
pub enum FBW {
    #[doc = "AHB uses SINGLE and INCR burst transfers"]
    VARIABLE,
    #[doc = "AHB uses only fixed burst transfers"]
    FIXED,
}
impl FBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FBW::VARIABLE => false,
            FBW::FIXED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FBW<'a> {
    w: &'a mut W,
}
impl<'a> _FBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AHB uses SINGLE and INCR burst transfers"]
    #[inline]
    pub fn variable(self) -> &'a mut W {
        self.variant(FBW::VARIABLE)
    }
    #[doc = "AHB uses only fixed burst transfers"]
    #[inline]
    pub fn fixed(self) -> &'a mut W {
        self.variant(FBW::FIXED)
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
#[doc = "Values that can be written to the field `RDP`"]
pub enum RDPW {
    #[doc = "1 beat per RxDMA transaction"]
    RDP1,
    #[doc = "2 beats per RxDMA transaction"]
    RDP2,
    #[doc = "4 beats per RxDMA transaction"]
    RDP4,
    #[doc = "8 beats per RxDMA transaction"]
    RDP8,
    #[doc = "16 beats per RxDMA transaction"]
    RDP16,
    #[doc = "32 beats per RxDMA transaction"]
    RDP32,
}
impl RDPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RDPW::RDP1 => 1,
            RDPW::RDP2 => 2,
            RDPW::RDP4 => 4,
            RDPW::RDP8 => 8,
            RDPW::RDP16 => 16,
            RDPW::RDP32 => 32,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDPW<'a> {
    w: &'a mut W,
}
impl<'a> _RDPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 beat per RxDMA transaction"]
    #[inline]
    pub fn rdp1(self) -> &'a mut W {
        self.variant(RDPW::RDP1)
    }
    #[doc = "2 beats per RxDMA transaction"]
    #[inline]
    pub fn rdp2(self) -> &'a mut W {
        self.variant(RDPW::RDP2)
    }
    #[doc = "4 beats per RxDMA transaction"]
    #[inline]
    pub fn rdp4(self) -> &'a mut W {
        self.variant(RDPW::RDP4)
    }
    #[doc = "8 beats per RxDMA transaction"]
    #[inline]
    pub fn rdp8(self) -> &'a mut W {
        self.variant(RDPW::RDP8)
    }
    #[doc = "16 beats per RxDMA transaction"]
    #[inline]
    pub fn rdp16(self) -> &'a mut W {
        self.variant(RDPW::RDP16)
    }
    #[doc = "32 beats per RxDMA transaction"]
    #[inline]
    pub fn rdp32(self) -> &'a mut W {
        self.variant(RDPW::RDP32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x3f;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USP`"]
pub enum USPW {
    #[doc = "PBL value used for both Rx and Tx DMA"]
    COMBINED,
    #[doc = "RxDMA uses RDP value, TxDMA uses PBL value"]
    SEPARATE,
}
impl USPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USPW::COMBINED => false,
            USPW::SEPARATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USPW<'a> {
    w: &'a mut W,
}
impl<'a> _USPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PBL value used for both Rx and Tx DMA"]
    #[inline]
    pub fn combined(self) -> &'a mut W {
        self.variant(USPW::COMBINED)
    }
    #[doc = "RxDMA uses RDP value, TxDMA uses PBL value"]
    #[inline]
    pub fn separate(self) -> &'a mut W {
        self.variant(USPW::SEPARATE)
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
#[doc = "Values that can be written to the field `FPM`"]
pub enum FPMW {
    #[doc = "PBL values used as-is"]
    X1,
    #[doc = "PBL values multiplied by 4"]
    X4,
}
impl FPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FPMW::X1 => false,
            FPMW::X4 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPMW<'a> {
    w: &'a mut W,
}
impl<'a> _FPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PBL values used as-is"]
    #[inline]
    pub fn x1(self) -> &'a mut W {
        self.variant(FPMW::X1)
    }
    #[doc = "PBL values multiplied by 4"]
    #[inline]
    pub fn x4(self) -> &'a mut W {
        self.variant(FPMW::X4)
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
#[doc = "Values that can be written to the field `AAB`"]
pub enum AABW {
    #[doc = "Bursts are not aligned"]
    UNALIGNED,
    #[doc = "Align bursts to start address LS bits. First burst alignment depends on FB bit"]
    ALIGNED,
}
impl AABW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AABW::UNALIGNED => false,
            AABW::ALIGNED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AABW<'a> {
    w: &'a mut W,
}
impl<'a> _AABW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AABW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bursts are not aligned"]
    #[inline]
    pub fn unaligned(self) -> &'a mut W {
        self.variant(AABW::UNALIGNED)
    }
    #[doc = "Align bursts to start address LS bits. First burst alignment depends on FB bit"]
    #[inline]
    pub fn aligned(self) -> &'a mut W {
        self.variant(AABW::ALIGNED)
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
#[doc = "Values that can be written to the field `MB`"]
pub enum MBW {
    #[doc = "Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below"]
    NORMAL,
    #[doc = "If FB is low, start all bursts greater than 16 with INCR (undefined burst)"]
    MIXED,
}
impl MBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MBW::NORMAL => false,
            MBW::MIXED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MBW<'a> {
    w: &'a mut W,
}
impl<'a> _MBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(MBW::NORMAL)
    }
    #[doc = "If FB is low, start all bursts greater than 16 with INCR (undefined burst)"]
    #[inline]
    pub fn mixed(self) -> &'a mut W {
        self.variant(MBW::MIXED)
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
    #[doc = "Bit 0 - SR"]
    #[inline]
    pub fn sr(&self) -> SRR {
        SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DA"]
    #[inline]
    pub fn da(&self) -> DAR {
        DAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:6 - DSL"]
    #[inline]
    pub fn dsl(&self) -> DSLR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DSLR { bits }
    }
    #[doc = "Bit 7 - EDFE"]
    #[inline]
    pub fn edfe(&self) -> EDFER {
        EDFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:13 - PBL"]
    #[inline]
    pub fn pbl(&self) -> PBLR {
        PBLR::_from({
            const MASK: u8 = 0x3f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - RTPR"]
    #[inline]
    pub fn pm(&self) -> PMR {
        PMR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - FB"]
    #[inline]
    pub fn fb(&self) -> FBR {
        FBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:22 - RDP"]
    #[inline]
    pub fn rdp(&self) -> RDPR {
        RDPR::_from({
            const MASK: u8 = 0x3f;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - USP"]
    #[inline]
    pub fn usp(&self) -> USPR {
        USPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - FPM"]
    #[inline]
    pub fn fpm(&self) -> FPMR {
        FPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - AAB"]
    #[inline]
    pub fn aab(&self) -> AABR {
        AABR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - MB"]
    #[inline]
    pub fn mb(&self) -> MBR {
        MBR::_from({
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
        W { bits: 0x2101 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SR"]
    #[inline]
    pub fn sr(&mut self) -> _SRW {
        _SRW { w: self }
    }
    #[doc = "Bit 1 - DA"]
    #[inline]
    pub fn da(&mut self) -> _DAW {
        _DAW { w: self }
    }
    #[doc = "Bits 2:6 - DSL"]
    #[inline]
    pub fn dsl(&mut self) -> _DSLW {
        _DSLW { w: self }
    }
    #[doc = "Bit 7 - EDFE"]
    #[inline]
    pub fn edfe(&mut self) -> _EDFEW {
        _EDFEW { w: self }
    }
    #[doc = "Bits 8:13 - PBL"]
    #[inline]
    pub fn pbl(&mut self) -> _PBLW {
        _PBLW { w: self }
    }
    #[doc = "Bits 14:15 - RTPR"]
    #[inline]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 16 - FB"]
    #[inline]
    pub fn fb(&mut self) -> _FBW {
        _FBW { w: self }
    }
    #[doc = "Bits 17:22 - RDP"]
    #[inline]
    pub fn rdp(&mut self) -> _RDPW {
        _RDPW { w: self }
    }
    #[doc = "Bit 23 - USP"]
    #[inline]
    pub fn usp(&mut self) -> _USPW {
        _USPW { w: self }
    }
    #[doc = "Bit 24 - FPM"]
    #[inline]
    pub fn fpm(&mut self) -> _FPMW {
        _FPMW { w: self }
    }
    #[doc = "Bit 25 - AAB"]
    #[inline]
    pub fn aab(&mut self) -> _AABW {
        _AABW { w: self }
    }
    #[doc = "Bit 26 - MB"]
    #[inline]
    pub fn mb(&mut self) -> _MBW {
        _MBW { w: self }
    }
}
