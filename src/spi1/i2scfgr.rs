#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::I2SCFGR {
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
#[doc = "Possible values of the field `I2SMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SMODR {
    #[doc = "SPI mode is selected"]
    SPIMODE,
    #[doc = "I2S mode is selected"]
    I2SMODE,
}
impl I2SMODR {
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
            I2SMODR::SPIMODE => false,
            I2SMODR::I2SMODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2SMODR {
        match value {
            false => I2SMODR::SPIMODE,
            true => I2SMODR::I2SMODE,
        }
    }
    #[doc = "Checks if the value of the field is `SPIMODE`"]
    #[inline]
    pub fn is_spimode(&self) -> bool {
        *self == I2SMODR::SPIMODE
    }
    #[doc = "Checks if the value of the field is `I2SMODE`"]
    #[inline]
    pub fn is_i2smode(&self) -> bool {
        *self == I2SMODR::I2SMODE
    }
}
#[doc = "Possible values of the field `I2SE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SER {
    #[doc = "I2S peripheral is disabled"]
    DISABLED,
    #[doc = "I2S peripheral is enabled"]
    ENABLED,
}
impl I2SER {
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
            I2SER::DISABLED => false,
            I2SER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2SER {
        match value {
            false => I2SER::DISABLED,
            true => I2SER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == I2SER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == I2SER::ENABLED
    }
}
#[doc = "Possible values of the field `I2SCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SCFGR {
    #[doc = "Slave - transmit"]
    SLAVETX,
    #[doc = "Slave - receive"]
    SLAVERX,
    #[doc = "Master - transmit"]
    MASTERTX,
    #[doc = "Master - receive"]
    MASTERRX,
}
impl I2SCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2SCFGR::SLAVETX => 0,
            I2SCFGR::SLAVERX => 0x01,
            I2SCFGR::MASTERTX => 0x02,
            I2SCFGR::MASTERRX => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2SCFGR {
        match value {
            0 => I2SCFGR::SLAVETX,
            1 => I2SCFGR::SLAVERX,
            2 => I2SCFGR::MASTERTX,
            3 => I2SCFGR::MASTERRX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVETX`"]
    #[inline]
    pub fn is_slave_tx(&self) -> bool {
        *self == I2SCFGR::SLAVETX
    }
    #[doc = "Checks if the value of the field is `SLAVERX`"]
    #[inline]
    pub fn is_slave_rx(&self) -> bool {
        *self == I2SCFGR::SLAVERX
    }
    #[doc = "Checks if the value of the field is `MASTERTX`"]
    #[inline]
    pub fn is_master_tx(&self) -> bool {
        *self == I2SCFGR::MASTERTX
    }
    #[doc = "Checks if the value of the field is `MASTERRX`"]
    #[inline]
    pub fn is_master_rx(&self) -> bool {
        *self == I2SCFGR::MASTERRX
    }
}
#[doc = "Possible values of the field `PCMSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCMSYNCR {
    #[doc = "Short frame synchronisation"]
    SHORT,
    #[doc = "Long frame synchronisation"]
    LONG,
}
impl PCMSYNCR {
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
            PCMSYNCR::SHORT => false,
            PCMSYNCR::LONG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCMSYNCR {
        match value {
            false => PCMSYNCR::SHORT,
            true => PCMSYNCR::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline]
    pub fn is_short(&self) -> bool {
        *self == PCMSYNCR::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline]
    pub fn is_long(&self) -> bool {
        *self == PCMSYNCR::LONG
    }
}
#[doc = "Possible values of the field `I2SSTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SSTDR {
    #[doc = "I2S Philips standard"]
    PHILIPS,
    #[doc = "MSB justified standard"]
    MSB,
    #[doc = "LSB justified standard"]
    LSB,
    #[doc = "PCM standard"]
    PCM,
}
impl I2SSTDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2SSTDR::PHILIPS => 0,
            I2SSTDR::MSB => 0x01,
            I2SSTDR::LSB => 0x02,
            I2SSTDR::PCM => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2SSTDR {
        match value {
            0 => I2SSTDR::PHILIPS,
            1 => I2SSTDR::MSB,
            2 => I2SSTDR::LSB,
            3 => I2SSTDR::PCM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PHILIPS`"]
    #[inline]
    pub fn is_philips(&self) -> bool {
        *self == I2SSTDR::PHILIPS
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline]
    pub fn is_msb(&self) -> bool {
        *self == I2SSTDR::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline]
    pub fn is_lsb(&self) -> bool {
        *self == I2SSTDR::LSB
    }
    #[doc = "Checks if the value of the field is `PCM`"]
    #[inline]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTDR::PCM
    }
}
#[doc = "Possible values of the field `CKPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKPOLR {
    #[doc = "I2S clock inactive state is low level"]
    IDLELOW,
    #[doc = "I2S clock inactive state is high level"]
    IDLEHIGH,
}
impl CKPOLR {
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
            CKPOLR::IDLELOW => false,
            CKPOLR::IDLEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CKPOLR {
        match value {
            false => CKPOLR::IDLELOW,
            true => CKPOLR::IDLEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLELOW`"]
    #[inline]
    pub fn is_idle_low(&self) -> bool {
        *self == CKPOLR::IDLELOW
    }
    #[doc = "Checks if the value of the field is `IDLEHIGH`"]
    #[inline]
    pub fn is_idle_high(&self) -> bool {
        *self == CKPOLR::IDLEHIGH
    }
}
#[doc = "Possible values of the field `DATLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATLENR {
    #[doc = "16-bit data length"]
    SIXTEENBIT,
    #[doc = "24-bit data length"]
    TWENTYFOURBIT,
    #[doc = "32-bit data length"]
    THIRTYTWOBIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATLENR::SIXTEENBIT => 0,
            DATLENR::TWENTYFOURBIT => 0x01,
            DATLENR::THIRTYTWOBIT => 0x02,
            DATLENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATLENR {
        match value {
            0 => DATLENR::SIXTEENBIT,
            1 => DATLENR::TWENTYFOURBIT,
            2 => DATLENR::THIRTYTWOBIT,
            i => DATLENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DATLENR::SIXTEENBIT
    }
    #[doc = "Checks if the value of the field is `TWENTYFOURBIT`"]
    #[inline]
    pub fn is_twenty_four_bit(&self) -> bool {
        *self == DATLENR::TWENTYFOURBIT
    }
    #[doc = "Checks if the value of the field is `THIRTYTWOBIT`"]
    #[inline]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == DATLENR::THIRTYTWOBIT
    }
}
#[doc = "Possible values of the field `CHLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHLENR {
    #[doc = "16-bit wide"]
    SIXTEENBIT,
    #[doc = "32-bit wide"]
    THIRTYTWOBIT,
}
impl CHLENR {
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
            CHLENR::SIXTEENBIT => false,
            CHLENR::THIRTYTWOBIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHLENR {
        match value {
            false => CHLENR::SIXTEENBIT,
            true => CHLENR::THIRTYTWOBIT,
        }
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == CHLENR::SIXTEENBIT
    }
    #[doc = "Checks if the value of the field is `THIRTYTWOBIT`"]
    #[inline]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == CHLENR::THIRTYTWOBIT
    }
}
#[doc = r" Value of the field"]
pub struct ASTRTENR {
    bits: bool,
}
impl ASTRTENR {
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
#[doc = "Values that can be written to the field `I2SMOD`"]
pub enum I2SMODW {
    #[doc = "SPI mode is selected"]
    SPIMODE,
    #[doc = "I2S mode is selected"]
    I2SMODE,
}
impl I2SMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2SMODW::SPIMODE => false,
            I2SMODW::I2SMODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2SMODW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2SMODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI mode is selected"]
    #[inline]
    pub fn spimode(self) -> &'a mut W {
        self.variant(I2SMODW::SPIMODE)
    }
    #[doc = "I2S mode is selected"]
    #[inline]
    pub fn i2smode(self) -> &'a mut W {
        self.variant(I2SMODW::I2SMODE)
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
#[doc = "Values that can be written to the field `I2SE`"]
pub enum I2SEW {
    #[doc = "I2S peripheral is disabled"]
    DISABLED,
    #[doc = "I2S peripheral is enabled"]
    ENABLED,
}
impl I2SEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2SEW::DISABLED => false,
            I2SEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2SEW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2SEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2S peripheral is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2SEW::DISABLED)
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2SEW::ENABLED)
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
#[doc = "Values that can be written to the field `I2SCFG`"]
pub enum I2SCFGW {
    #[doc = "Slave - transmit"]
    SLAVETX,
    #[doc = "Slave - receive"]
    SLAVERX,
    #[doc = "Master - transmit"]
    MASTERTX,
    #[doc = "Master - receive"]
    MASTERRX,
}
impl I2SCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2SCFGW::SLAVETX => 0,
            I2SCFGW::SLAVERX => 1,
            I2SCFGW::MASTERTX => 2,
            I2SCFGW::MASTERRX => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2SCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2SCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Slave - transmit"]
    #[inline]
    pub fn slave_tx(self) -> &'a mut W {
        self.variant(I2SCFGW::SLAVETX)
    }
    #[doc = "Slave - receive"]
    #[inline]
    pub fn slave_rx(self) -> &'a mut W {
        self.variant(I2SCFGW::SLAVERX)
    }
    #[doc = "Master - transmit"]
    #[inline]
    pub fn master_tx(self) -> &'a mut W {
        self.variant(I2SCFGW::MASTERTX)
    }
    #[doc = "Master - receive"]
    #[inline]
    pub fn master_rx(self) -> &'a mut W {
        self.variant(I2SCFGW::MASTERRX)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCMSYNC`"]
pub enum PCMSYNCW {
    #[doc = "Short frame synchronisation"]
    SHORT,
    #[doc = "Long frame synchronisation"]
    LONG,
}
impl PCMSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCMSYNCW::SHORT => false,
            PCMSYNCW::LONG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCMSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCMSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCMSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Short frame synchronisation"]
    #[inline]
    pub fn short(self) -> &'a mut W {
        self.variant(PCMSYNCW::SHORT)
    }
    #[doc = "Long frame synchronisation"]
    #[inline]
    pub fn long(self) -> &'a mut W {
        self.variant(PCMSYNCW::LONG)
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
#[doc = "Values that can be written to the field `I2SSTD`"]
pub enum I2SSTDW {
    #[doc = "I2S Philips standard"]
    PHILIPS,
    #[doc = "MSB justified standard"]
    MSB,
    #[doc = "LSB justified standard"]
    LSB,
    #[doc = "PCM standard"]
    PCM,
}
impl I2SSTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2SSTDW::PHILIPS => 0,
            I2SSTDW::MSB => 1,
            I2SSTDW::LSB => 2,
            I2SSTDW::PCM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2SSTDW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SSTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2SSTDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "I2S Philips standard"]
    #[inline]
    pub fn philips(self) -> &'a mut W {
        self.variant(I2SSTDW::PHILIPS)
    }
    #[doc = "MSB justified standard"]
    #[inline]
    pub fn msb(self) -> &'a mut W {
        self.variant(I2SSTDW::MSB)
    }
    #[doc = "LSB justified standard"]
    #[inline]
    pub fn lsb(self) -> &'a mut W {
        self.variant(I2SSTDW::LSB)
    }
    #[doc = "PCM standard"]
    #[inline]
    pub fn pcm(self) -> &'a mut W {
        self.variant(I2SSTDW::PCM)
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
#[doc = "Values that can be written to the field `CKPOL`"]
pub enum CKPOLW {
    #[doc = "I2S clock inactive state is low level"]
    IDLELOW,
    #[doc = "I2S clock inactive state is high level"]
    IDLEHIGH,
}
impl CKPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CKPOLW::IDLELOW => false,
            CKPOLW::IDLEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CKPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2S clock inactive state is low level"]
    #[inline]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CKPOLW::IDLELOW)
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CKPOLW::IDLEHIGH)
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
#[doc = "Values that can be written to the field `DATLEN`"]
pub enum DATLENW {
    #[doc = "16-bit data length"]
    SIXTEENBIT,
    #[doc = "24-bit data length"]
    TWENTYFOURBIT,
    #[doc = "32-bit data length"]
    THIRTYTWOBIT,
}
impl DATLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATLENW::SIXTEENBIT => 0,
            DATLENW::TWENTYFOURBIT => 1,
            DATLENW::THIRTYTWOBIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATLENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "16-bit data length"]
    #[inline]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DATLENW::SIXTEENBIT)
    }
    #[doc = "24-bit data length"]
    #[inline]
    pub fn twenty_four_bit(self) -> &'a mut W {
        self.variant(DATLENW::TWENTYFOURBIT)
    }
    #[doc = "32-bit data length"]
    #[inline]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(DATLENW::THIRTYTWOBIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHLEN`"]
pub enum CHLENW {
    #[doc = "16-bit wide"]
    SIXTEENBIT,
    #[doc = "32-bit wide"]
    THIRTYTWOBIT,
}
impl CHLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHLENW::SIXTEENBIT => false,
            CHLENW::THIRTYTWOBIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHLENW<'a> {
    w: &'a mut W,
}
impl<'a> _CHLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "16-bit wide"]
    #[inline]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(CHLENW::SIXTEENBIT)
    }
    #[doc = "32-bit wide"]
    #[inline]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(CHLENW::THIRTYTWOBIT)
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
#[doc = r" Proxy"]
pub struct _ASTRTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ASTRTENW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline]
    pub fn i2smod(&self) -> I2SMODR {
        I2SMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline]
    pub fn i2se(&self) -> I2SER {
        I2SER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline]
    pub fn i2scfg(&self) -> I2SCFGR {
        I2SCFGR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline]
    pub fn pcmsync(&self) -> PCMSYNCR {
        PCMSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline]
    pub fn i2sstd(&self) -> I2SSTDR {
        I2SSTDR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Steady state clock polarity"]
    #[inline]
    pub fn ckpol(&self) -> CKPOLR {
        CKPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline]
    pub fn datlen(&self) -> DATLENR {
        DATLENR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline]
    pub fn chlen(&self) -> CHLENR {
        CHLENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Asynchronous start enable"]
    #[inline]
    pub fn astrten(&self) -> ASTRTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASTRTENR { bits }
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
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline]
    pub fn i2smod(&mut self) -> _I2SMODW {
        _I2SMODW { w: self }
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline]
    pub fn i2se(&mut self) -> _I2SEW {
        _I2SEW { w: self }
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline]
    pub fn i2scfg(&mut self) -> _I2SCFGW {
        _I2SCFGW { w: self }
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline]
    pub fn pcmsync(&mut self) -> _PCMSYNCW {
        _PCMSYNCW { w: self }
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline]
    pub fn i2sstd(&mut self) -> _I2SSTDW {
        _I2SSTDW { w: self }
    }
    #[doc = "Bit 3 - Steady state clock polarity"]
    #[inline]
    pub fn ckpol(&mut self) -> _CKPOLW {
        _CKPOLW { w: self }
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline]
    pub fn datlen(&mut self) -> _DATLENW {
        _DATLENW { w: self }
    }
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline]
    pub fn chlen(&mut self) -> _CHLENW {
        _CHLENW { w: self }
    }
    #[doc = "Bit 12 - Asynchronous start enable"]
    #[inline]
    pub fn astrten(&mut self) -> _ASTRTENW {
        _ASTRTENW { w: self }
    }
}
