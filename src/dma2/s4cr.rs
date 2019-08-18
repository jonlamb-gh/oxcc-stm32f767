#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::S4CR {
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
#[doc = r" Value of the field"]
pub struct CHSELR {
    bits: u8,
}
impl CHSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MBURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBURSTR {
    #[doc = "Single transfer"]
    SINGLE,
    #[doc = "Incremental burst of 4 beats"]
    INCR4,
    #[doc = "Incremental burst of 8 beats"]
    INCR8,
    #[doc = "Incremental burst of 16 beats"]
    INCR16,
}
impl MBURSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MBURSTR::SINGLE => 0,
            MBURSTR::INCR4 => 0x01,
            MBURSTR::INCR8 => 0x02,
            MBURSTR::INCR16 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MBURSTR {
        match value {
            0 => MBURSTR::SINGLE,
            1 => MBURSTR::INCR4,
            2 => MBURSTR::INCR8,
            3 => MBURSTR::INCR16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == MBURSTR::SINGLE
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline]
    pub fn is_incr4(&self) -> bool {
        *self == MBURSTR::INCR4
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline]
    pub fn is_incr8(&self) -> bool {
        *self == MBURSTR::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline]
    pub fn is_incr16(&self) -> bool {
        *self == MBURSTR::INCR16
    }
}
#[doc = "Possible values of the field `PBURST`"]
pub type PBURSTR = MBURSTR;
#[doc = r" Value of the field"]
pub struct ACKR {
    bits: bool,
}
impl ACKR {
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
#[doc = "Possible values of the field `CT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTR {
    #[doc = "The current target memory is Memory 0"]
    MEMORY0,
    #[doc = "The current target memory is Memory 1"]
    MEMORY1,
}
impl CTR {
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
            CTR::MEMORY0 => false,
            CTR::MEMORY1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTR {
        match value {
            false => CTR::MEMORY0,
            true => CTR::MEMORY1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMORY0`"]
    #[inline]
    pub fn is_memory0(&self) -> bool {
        *self == CTR::MEMORY0
    }
    #[doc = "Checks if the value of the field is `MEMORY1`"]
    #[inline]
    pub fn is_memory1(&self) -> bool {
        *self == CTR::MEMORY1
    }
}
#[doc = "Possible values of the field `DBM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBMR {
    #[doc = "No buffer switching at the end of transfer"]
    DISABLED,
    #[doc = "Memory target switched at the end of the DMA transfer"]
    ENABLED,
}
impl DBMR {
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
            DBMR::DISABLED => false,
            DBMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBMR {
        match value {
            false => DBMR::DISABLED,
            true => DBMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DBMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DBMR::ENABLED
    }
}
#[doc = "Possible values of the field `PL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLR {
    #[doc = "Low"]
    LOW,
    #[doc = "Medium"]
    MEDIUM,
    #[doc = "High"]
    HIGH,
    #[doc = "Very high"]
    VERYHIGH,
}
impl PLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLR::LOW => 0,
            PLR::MEDIUM => 0x01,
            PLR::HIGH => 0x02,
            PLR::VERYHIGH => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLR {
        match value {
            0 => PLR::LOW,
            1 => PLR::MEDIUM,
            2 => PLR::HIGH,
            3 => PLR::VERYHIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PLR::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline]
    pub fn is_medium(&self) -> bool {
        *self == PLR::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PLR::HIGH
    }
    #[doc = "Checks if the value of the field is `VERYHIGH`"]
    #[inline]
    pub fn is_very_high(&self) -> bool {
        *self == PLR::VERYHIGH
    }
}
#[doc = "Possible values of the field `PINCOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCOSR {
    #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
    PSIZE,
    #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    FIXED4,
}
impl PINCOSR {
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
            PINCOSR::PSIZE => false,
            PINCOSR::FIXED4 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINCOSR {
        match value {
            false => PINCOSR::PSIZE,
            true => PINCOSR::FIXED4,
        }
    }
    #[doc = "Checks if the value of the field is `PSIZE`"]
    #[inline]
    pub fn is_psize(&self) -> bool {
        *self == PINCOSR::PSIZE
    }
    #[doc = "Checks if the value of the field is `FIXED4`"]
    #[inline]
    pub fn is_fixed4(&self) -> bool {
        *self == PINCOSR::FIXED4
    }
}
#[doc = "Possible values of the field `MSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIZER {
    #[doc = "Byte (8-bit)"]
    BYTE,
    #[doc = "Half-word (16-bit)"]
    HALFWORD,
    #[doc = "Word (32-bit)"]
    WORD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSIZER::BYTE => 0,
            MSIZER::HALFWORD => 0x01,
            MSIZER::WORD => 0x02,
            MSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSIZER {
        match value {
            0 => MSIZER::BYTE,
            1 => MSIZER::HALFWORD,
            2 => MSIZER::WORD,
            i => MSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline]
    pub fn is_byte(&self) -> bool {
        *self == MSIZER::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline]
    pub fn is_half_word(&self) -> bool {
        *self == MSIZER::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline]
    pub fn is_word(&self) -> bool {
        *self == MSIZER::WORD
    }
}
#[doc = "Possible values of the field `PSIZE`"]
pub type PSIZER = MSIZER;
#[doc = "Possible values of the field `MINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MINCR {
    #[doc = "Address pointer is fixed"]
    FIXED,
    #[doc = "Address pointer is incremented after each data transfer"]
    INCREMENTED,
}
impl MINCR {
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
            MINCR::FIXED => false,
            MINCR::INCREMENTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MINCR {
        match value {
            false => MINCR::FIXED,
            true => MINCR::INCREMENTED,
        }
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline]
    pub fn is_fixed(&self) -> bool {
        *self == MINCR::FIXED
    }
    #[doc = "Checks if the value of the field is `INCREMENTED`"]
    #[inline]
    pub fn is_incremented(&self) -> bool {
        *self == MINCR::INCREMENTED
    }
}
#[doc = "Possible values of the field `PINC`"]
pub type PINCR = MINCR;
#[doc = "Possible values of the field `CIRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIRCR {
    #[doc = "Circular mode disabled"]
    DISABLED,
    #[doc = "Circular mode enabled"]
    ENABLED,
}
impl CIRCR {
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
            CIRCR::DISABLED => false,
            CIRCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIRCR {
        match value {
            false => CIRCR::DISABLED,
            true => CIRCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CIRCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CIRCR::ENABLED
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Peripheral-to-memory"]
    PERIPHERALTOMEMORY,
    #[doc = "Memory-to-peripheral"]
    MEMORYTOPERIPHERAL,
    #[doc = "Memory-to-memory"]
    MEMORYTOMEMORY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIRR::PERIPHERALTOMEMORY => 0,
            DIRR::MEMORYTOPERIPHERAL => 0x01,
            DIRR::MEMORYTOMEMORY => 0x02,
            DIRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIRR {
        match value {
            0 => DIRR::PERIPHERALTOMEMORY,
            1 => DIRR::MEMORYTOPERIPHERAL,
            2 => DIRR::MEMORYTOMEMORY,
            i => DIRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PERIPHERALTOMEMORY`"]
    #[inline]
    pub fn is_peripheral_to_memory(&self) -> bool {
        *self == DIRR::PERIPHERALTOMEMORY
    }
    #[doc = "Checks if the value of the field is `MEMORYTOPERIPHERAL`"]
    #[inline]
    pub fn is_memory_to_peripheral(&self) -> bool {
        *self == DIRR::MEMORYTOPERIPHERAL
    }
    #[doc = "Checks if the value of the field is `MEMORYTOMEMORY`"]
    #[inline]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == DIRR::MEMORYTOMEMORY
    }
}
#[doc = "Possible values of the field `PFCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFCTRLR {
    #[doc = "The DMA is the flow controller"]
    DMA,
    #[doc = "The peripheral is the flow controller"]
    PERIPHERAL,
}
impl PFCTRLR {
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
            PFCTRLR::DMA => false,
            PFCTRLR::PERIPHERAL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFCTRLR {
        match value {
            false => PFCTRLR::DMA,
            true => PFCTRLR::PERIPHERAL,
        }
    }
    #[doc = "Checks if the value of the field is `DMA`"]
    #[inline]
    pub fn is_dma(&self) -> bool {
        *self == PFCTRLR::DMA
    }
    #[doc = "Checks if the value of the field is `PERIPHERAL`"]
    #[inline]
    pub fn is_peripheral(&self) -> bool {
        *self == PFCTRLR::PERIPHERAL
    }
}
#[doc = "Possible values of the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIER {
    #[doc = "TC interrupt disabled"]
    DISABLED,
    #[doc = "TC interrupt enabled"]
    ENABLED,
}
impl TCIER {
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
            TCIER::DISABLED => false,
            TCIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIER {
        match value {
            false => TCIER::DISABLED,
            true => TCIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TCIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TCIER::ENABLED
    }
}
#[doc = "Possible values of the field `HTIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIER {
    #[doc = "HT interrupt disabled"]
    DISABLED,
    #[doc = "HT interrupt enabled"]
    ENABLED,
}
impl HTIER {
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
            HTIER::DISABLED => false,
            HTIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HTIER {
        match value {
            false => HTIER::DISABLED,
            true => HTIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HTIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HTIER::ENABLED
    }
}
#[doc = "Possible values of the field `TEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIER {
    #[doc = "TE interrupt disabled"]
    DISABLED,
    #[doc = "TE interrupt enabled"]
    ENABLED,
}
impl TEIER {
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
            TEIER::DISABLED => false,
            TEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEIER {
        match value {
            false => TEIER::DISABLED,
            true => TEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TEIER::ENABLED
    }
}
#[doc = "Possible values of the field `DMEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIER {
    #[doc = "DME interrupt disabled"]
    DISABLED,
    #[doc = "DME interrupt enabled"]
    ENABLED,
}
impl DMEIER {
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
            DMEIER::DISABLED => false,
            DMEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMEIER {
        match value {
            false => DMEIER::DISABLED,
            true => DMEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMEIER::ENABLED
    }
}
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "Stream disabled"]
    DISABLED,
    #[doc = "Stream enabled"]
    ENABLED,
}
impl ENR {
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
            ENR::DISABLED => false,
            ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::DISABLED,
            true => ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENR::ENABLED
    }
}
#[doc = r" Proxy"]
pub struct _CHSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MBURST`"]
pub enum MBURSTW {
    #[doc = "Single transfer"]
    SINGLE,
    #[doc = "Incremental burst of 4 beats"]
    INCR4,
    #[doc = "Incremental burst of 8 beats"]
    INCR8,
    #[doc = "Incremental burst of 16 beats"]
    INCR16,
}
impl MBURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MBURSTW::SINGLE => 0,
            MBURSTW::INCR4 => 1,
            MBURSTW::INCR8 => 2,
            MBURSTW::INCR16 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MBURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MBURSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MBURSTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single transfer"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(MBURSTW::SINGLE)
    }
    #[doc = "Incremental burst of 4 beats"]
    #[inline]
    pub fn incr4(self) -> &'a mut W {
        self.variant(MBURSTW::INCR4)
    }
    #[doc = "Incremental burst of 8 beats"]
    #[inline]
    pub fn incr8(self) -> &'a mut W {
        self.variant(MBURSTW::INCR8)
    }
    #[doc = "Incremental burst of 16 beats"]
    #[inline]
    pub fn incr16(self) -> &'a mut W {
        self.variant(MBURSTW::INCR16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PBURST`"]
pub type PBURSTW = MBURSTW;
#[doc = r" Proxy"]
pub struct _PBURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PBURSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBURSTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single transfer"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(MBURSTW::SINGLE)
    }
    #[doc = "Incremental burst of 4 beats"]
    #[inline]
    pub fn incr4(self) -> &'a mut W {
        self.variant(MBURSTW::INCR4)
    }
    #[doc = "Incremental burst of 8 beats"]
    #[inline]
    pub fn incr8(self) -> &'a mut W {
        self.variant(MBURSTW::INCR8)
    }
    #[doc = "Incremental burst of 16 beats"]
    #[inline]
    pub fn incr16(self) -> &'a mut W {
        self.variant(MBURSTW::INCR16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _ACKW<'a> {
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
#[doc = "Values that can be written to the field `CT`"]
pub enum CTW {
    #[doc = "The current target memory is Memory 0"]
    MEMORY0,
    #[doc = "The current target memory is Memory 1"]
    MEMORY1,
}
impl CTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTW::MEMORY0 => false,
            CTW::MEMORY1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The current target memory is Memory 0"]
    #[inline]
    pub fn memory0(self) -> &'a mut W {
        self.variant(CTW::MEMORY0)
    }
    #[doc = "The current target memory is Memory 1"]
    #[inline]
    pub fn memory1(self) -> &'a mut W {
        self.variant(CTW::MEMORY1)
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
#[doc = "Values that can be written to the field `DBM`"]
pub enum DBMW {
    #[doc = "No buffer switching at the end of transfer"]
    DISABLED,
    #[doc = "Memory target switched at the end of the DMA transfer"]
    ENABLED,
}
impl DBMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBMW::DISABLED => false,
            DBMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBMW<'a> {
    w: &'a mut W,
}
impl<'a> _DBMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No buffer switching at the end of transfer"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBMW::DISABLED)
    }
    #[doc = "Memory target switched at the end of the DMA transfer"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBMW::ENABLED)
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
#[doc = "Values that can be written to the field `PL`"]
pub enum PLW {
    #[doc = "Low"]
    LOW,
    #[doc = "Medium"]
    MEDIUM,
    #[doc = "High"]
    HIGH,
    #[doc = "Very high"]
    VERYHIGH,
}
impl PLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLW::LOW => 0,
            PLW::MEDIUM => 1,
            PLW::HIGH => 2,
            PLW::VERYHIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLW<'a> {
    w: &'a mut W,
}
impl<'a> _PLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PLW::LOW)
    }
    #[doc = "Medium"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(PLW::MEDIUM)
    }
    #[doc = "High"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PLW::HIGH)
    }
    #[doc = "Very high"]
    #[inline]
    pub fn very_high(self) -> &'a mut W {
        self.variant(PLW::VERYHIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINCOS`"]
pub enum PINCOSW {
    #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
    PSIZE,
    #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    FIXED4,
}
impl PINCOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINCOSW::PSIZE => false,
            PINCOSW::FIXED4 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINCOSW<'a> {
    w: &'a mut W,
}
impl<'a> _PINCOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINCOSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
    #[inline]
    pub fn psize(self) -> &'a mut W {
        self.variant(PINCOSW::PSIZE)
    }
    #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    #[inline]
    pub fn fixed4(self) -> &'a mut W {
        self.variant(PINCOSW::FIXED4)
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
#[doc = "Values that can be written to the field `MSIZE`"]
pub enum MSIZEW {
    #[doc = "Byte (8-bit)"]
    BYTE,
    #[doc = "Half-word (16-bit)"]
    HALFWORD,
    #[doc = "Word (32-bit)"]
    WORD,
}
impl MSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSIZEW::BYTE => 0,
            MSIZEW::HALFWORD => 1,
            MSIZEW::WORD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Byte (8-bit)"]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(MSIZEW::BYTE)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline]
    pub fn half_word(self) -> &'a mut W {
        self.variant(MSIZEW::HALFWORD)
    }
    #[doc = "Word (32-bit)"]
    #[inline]
    pub fn word(self) -> &'a mut W {
        self.variant(MSIZEW::WORD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSIZE`"]
pub type PSIZEW = MSIZEW;
#[doc = r" Proxy"]
pub struct _PSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Byte (8-bit)"]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(MSIZEW::BYTE)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline]
    pub fn half_word(self) -> &'a mut W {
        self.variant(MSIZEW::HALFWORD)
    }
    #[doc = "Word (32-bit)"]
    #[inline]
    pub fn word(self) -> &'a mut W {
        self.variant(MSIZEW::WORD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MINC`"]
pub enum MINCW {
    #[doc = "Address pointer is fixed"]
    FIXED,
    #[doc = "Address pointer is incremented after each data transfer"]
    INCREMENTED,
}
impl MINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MINCW::FIXED => false,
            MINCW::INCREMENTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MINCW<'a> {
    w: &'a mut W,
}
impl<'a> _MINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Address pointer is fixed"]
    #[inline]
    pub fn fixed(self) -> &'a mut W {
        self.variant(MINCW::FIXED)
    }
    #[doc = "Address pointer is incremented after each data transfer"]
    #[inline]
    pub fn incremented(self) -> &'a mut W {
        self.variant(MINCW::INCREMENTED)
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
#[doc = "Values that can be written to the field `PINC`"]
pub type PINCW = MINCW;
#[doc = r" Proxy"]
pub struct _PINCW<'a> {
    w: &'a mut W,
}
impl<'a> _PINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Address pointer is fixed"]
    #[inline]
    pub fn fixed(self) -> &'a mut W {
        self.variant(MINCW::FIXED)
    }
    #[doc = "Address pointer is incremented after each data transfer"]
    #[inline]
    pub fn incremented(self) -> &'a mut W {
        self.variant(MINCW::INCREMENTED)
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
#[doc = "Values that can be written to the field `CIRC`"]
pub enum CIRCW {
    #[doc = "Circular mode disabled"]
    DISABLED,
    #[doc = "Circular mode enabled"]
    ENABLED,
}
impl CIRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIRCW::DISABLED => false,
            CIRCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CIRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Circular mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CIRCW::DISABLED)
    }
    #[doc = "Circular mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIRCW::ENABLED)
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
#[doc = "Values that can be written to the field `DIR`"]
pub enum DIRW {
    #[doc = "Peripheral-to-memory"]
    PERIPHERALTOMEMORY,
    #[doc = "Memory-to-peripheral"]
    MEMORYTOPERIPHERAL,
    #[doc = "Memory-to-memory"]
    MEMORYTOMEMORY,
}
impl DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIRW::PERIPHERALTOMEMORY => 0,
            DIRW::MEMORYTOPERIPHERAL => 1,
            DIRW::MEMORYTOMEMORY => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral-to-memory"]
    #[inline]
    pub fn peripheral_to_memory(self) -> &'a mut W {
        self.variant(DIRW::PERIPHERALTOMEMORY)
    }
    #[doc = "Memory-to-peripheral"]
    #[inline]
    pub fn memory_to_peripheral(self) -> &'a mut W {
        self.variant(DIRW::MEMORYTOPERIPHERAL)
    }
    #[doc = "Memory-to-memory"]
    #[inline]
    pub fn memory_to_memory(self) -> &'a mut W {
        self.variant(DIRW::MEMORYTOMEMORY)
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
#[doc = "Values that can be written to the field `PFCTRL`"]
pub enum PFCTRLW {
    #[doc = "The DMA is the flow controller"]
    DMA,
    #[doc = "The peripheral is the flow controller"]
    PERIPHERAL,
}
impl PFCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PFCTRLW::DMA => false,
            PFCTRLW::PERIPHERAL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _PFCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA is the flow controller"]
    #[inline]
    pub fn dma(self) -> &'a mut W {
        self.variant(PFCTRLW::DMA)
    }
    #[doc = "The peripheral is the flow controller"]
    #[inline]
    pub fn peripheral(self) -> &'a mut W {
        self.variant(PFCTRLW::PERIPHERAL)
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
#[doc = "Values that can be written to the field `TCIE`"]
pub enum TCIEW {
    #[doc = "TC interrupt disabled"]
    DISABLED,
    #[doc = "TC interrupt enabled"]
    ENABLED,
}
impl TCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIEW::DISABLED => false,
            TCIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TC interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIEW::DISABLED)
    }
    #[doc = "TC interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIEW::ENABLED)
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
#[doc = "Values that can be written to the field `HTIE`"]
pub enum HTIEW {
    #[doc = "HT interrupt disabled"]
    DISABLED,
    #[doc = "HT interrupt enabled"]
    ENABLED,
}
impl HTIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HTIEW::DISABLED => false,
            HTIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HTIEW<'a> {
    w: &'a mut W,
}
impl<'a> _HTIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HTIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HT interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HTIEW::DISABLED)
    }
    #[doc = "HT interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HTIEW::ENABLED)
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
#[doc = "Values that can be written to the field `TEIE`"]
pub enum TEIEW {
    #[doc = "TE interrupt disabled"]
    DISABLED,
    #[doc = "TE interrupt enabled"]
    ENABLED,
}
impl TEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEIEW::DISABLED => false,
            TEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TE interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIEW::DISABLED)
    }
    #[doc = "TE interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `DMEIE`"]
pub enum DMEIEW {
    #[doc = "DME interrupt disabled"]
    DISABLED,
    #[doc = "DME interrupt enabled"]
    ENABLED,
}
impl DMEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMEIEW::DISABLED => false,
            DMEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DME interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMEIEW::DISABLED)
    }
    #[doc = "DME interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "Stream disabled"]
    DISABLED,
    #[doc = "Stream enabled"]
    ENABLED,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::DISABLED => false,
            ENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stream disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENW::DISABLED)
    }
    #[doc = "Stream enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENW::ENABLED)
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
    #[doc = "Bits 25:27 - Channel selection"]
    #[inline]
    pub fn chsel(&self) -> CHSELR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHSELR { bits }
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline]
    pub fn mburst(&self) -> MBURSTR {
        MBURSTR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline]
    pub fn pburst(&self) -> PBURSTR {
        PBURSTR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - ACK"]
    #[inline]
    pub fn ack(&self) -> ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACKR { bits }
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline]
    pub fn ct(&self) -> CTR {
        CTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline]
    pub fn dbm(&self) -> DBMR {
        DBMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline]
    pub fn pl(&self) -> PLR {
        PLR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline]
    pub fn pincos(&self) -> PINCOSR {
        PINCOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline]
    pub fn msize(&self) -> MSIZER {
        MSIZER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline]
    pub fn psize(&self) -> PSIZER {
        PSIZER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline]
    pub fn minc(&self) -> MINCR {
        MINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline]
    pub fn pinc(&self) -> PINCR {
        PINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline]
    pub fn circ(&self) -> CIRCR {
        CIRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline]
    pub fn pfctrl(&self) -> PFCTRLR {
        PFCTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline]
    pub fn tcie(&self) -> TCIER {
        TCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline]
    pub fn htie(&self) -> HTIER {
        HTIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline]
    pub fn teie(&self) -> TEIER {
        TEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline]
    pub fn dmeie(&self) -> DMEIER {
        DMEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
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
    #[doc = "Bits 25:27 - Channel selection"]
    #[inline]
    pub fn chsel(&mut self) -> _CHSELW {
        _CHSELW { w: self }
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline]
    pub fn mburst(&mut self) -> _MBURSTW {
        _MBURSTW { w: self }
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline]
    pub fn pburst(&mut self) -> _PBURSTW {
        _PBURSTW { w: self }
    }
    #[doc = "Bit 20 - ACK"]
    #[inline]
    pub fn ack(&mut self) -> _ACKW {
        _ACKW { w: self }
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline]
    pub fn ct(&mut self) -> _CTW {
        _CTW { w: self }
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline]
    pub fn dbm(&mut self) -> _DBMW {
        _DBMW { w: self }
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline]
    pub fn pl(&mut self) -> _PLW {
        _PLW { w: self }
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline]
    pub fn pincos(&mut self) -> _PINCOSW {
        _PINCOSW { w: self }
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline]
    pub fn msize(&mut self) -> _MSIZEW {
        _MSIZEW { w: self }
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline]
    pub fn psize(&mut self) -> _PSIZEW {
        _PSIZEW { w: self }
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline]
    pub fn minc(&mut self) -> _MINCW {
        _MINCW { w: self }
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline]
    pub fn pinc(&mut self) -> _PINCW {
        _PINCW { w: self }
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline]
    pub fn circ(&mut self) -> _CIRCW {
        _CIRCW { w: self }
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline]
    pub fn pfctrl(&mut self) -> _PFCTRLW {
        _PFCTRLW { w: self }
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline]
    pub fn tcie(&mut self) -> _TCIEW {
        _TCIEW { w: self }
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline]
    pub fn htie(&mut self) -> _HTIEW {
        _HTIEW { w: self }
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline]
    pub fn teie(&mut self) -> _TEIEW {
        _TEIEW { w: self }
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline]
    pub fn dmeie(&mut self) -> _DMEIEW {
        _DMEIEW { w: self }
    }
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
