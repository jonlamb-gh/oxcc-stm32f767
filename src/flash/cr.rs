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
#[doc = "Possible values of the field `PG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGR {
    #[doc = "Flash programming activated"]
    PROGRAM,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PGR {
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
            PGR::PROGRAM => true,
            PGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PGR {
        match value {
            true => PGR::PROGRAM,
            i => PGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROGRAM`"]
    #[inline]
    pub fn is_program(&self) -> bool {
        *self == PGR::PROGRAM
    }
}
#[doc = "Possible values of the field `SER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SERR {
    #[doc = "Erase activated for selected sector"]
    SECTORERASE,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SERR {
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
            SERR::SECTORERASE => true,
            SERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SERR {
        match value {
            true => SERR::SECTORERASE,
            i => SERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SECTORERASE`"]
    #[inline]
    pub fn is_sector_erase(&self) -> bool {
        *self == SERR::SECTORERASE
    }
}
#[doc = "Possible values of the field `MER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MERR {
    #[doc = "Erase activated for all user sectors"]
    MASSERASE,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MERR {
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
            MERR::MASSERASE => true,
            MERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MERR {
        match value {
            true => MERR::MASSERASE,
            i => MERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASSERASE`"]
    #[inline]
    pub fn is_mass_erase(&self) -> bool {
        *self == MERR::MASSERASE
    }
}
#[doc = r" Value of the field"]
pub struct SNBR {
    bits: u8,
}
impl SNBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSIZER {
    #[doc = "Program x8"]
    PSIZE8,
    #[doc = "Program x16"]
    PSIZE16,
    #[doc = "Program x32"]
    PSIZE32,
    #[doc = "Program x64"]
    PSIZE64,
}
impl PSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSIZER::PSIZE8 => 0,
            PSIZER::PSIZE16 => 0x01,
            PSIZER::PSIZE32 => 0x02,
            PSIZER::PSIZE64 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSIZER {
        match value {
            0 => PSIZER::PSIZE8,
            1 => PSIZER::PSIZE16,
            2 => PSIZER::PSIZE32,
            3 => PSIZER::PSIZE64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSIZE8`"]
    #[inline]
    pub fn is_psize8(&self) -> bool {
        *self == PSIZER::PSIZE8
    }
    #[doc = "Checks if the value of the field is `PSIZE16`"]
    #[inline]
    pub fn is_psize16(&self) -> bool {
        *self == PSIZER::PSIZE16
    }
    #[doc = "Checks if the value of the field is `PSIZE32`"]
    #[inline]
    pub fn is_psize32(&self) -> bool {
        *self == PSIZER::PSIZE32
    }
    #[doc = "Checks if the value of the field is `PSIZE64`"]
    #[inline]
    pub fn is_psize64(&self) -> bool {
        *self == PSIZER::PSIZE64
    }
}
#[doc = r" Value of the field"]
pub struct MER1R {
    bits: bool,
}
impl MER1R {
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
#[doc = "Possible values of the field `STRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRTR {
    #[doc = "Trigger an erase operation"]
    START,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl STRTR {
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
            STRTR::START => true,
            STRTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STRTR {
        match value {
            true => STRTR::START,
            i => STRTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == STRTR::START
    }
}
#[doc = "Possible values of the field `EOPIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOPIER {
    #[doc = "End of operation interrupt disabled"]
    DISABLED,
    #[doc = "End of operation interrupt enabled"]
    ENABLED,
}
impl EOPIER {
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
            EOPIER::DISABLED => false,
            EOPIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOPIER {
        match value {
            false => EOPIER::DISABLED,
            true => EOPIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIER::ENABLED
    }
}
#[doc = "Possible values of the field `ERRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIER {
    #[doc = "Error interrupt generation disabled"]
    DISABLED,
    #[doc = "Error interrupt generation enabled"]
    ENABLED,
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
            ERRIER::DISABLED => false,
            ERRIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRIER {
        match value {
            false => ERRIER::DISABLED,
            true => ERRIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIER::ENABLED
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "FLASH_CR register is unlocked"]
    UNLOCKED,
    #[doc = "FLASH_CR register is locked"]
    LOCKED,
}
impl LOCKR {
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
            LOCKR::UNLOCKED => false,
            LOCKR::LOCKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::UNLOCKED,
            true => LOCKR::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKR::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR::LOCKED
    }
}
#[doc = "Values that can be written to the field `PG`"]
pub enum PGW {
    #[doc = "Flash programming activated"]
    PROGRAM,
}
impl PGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PGW::PROGRAM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PGW<'a> {
    w: &'a mut W,
}
impl<'a> _PGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash programming activated"]
    #[inline]
    pub fn program(self) -> &'a mut W {
        self.variant(PGW::PROGRAM)
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
#[doc = "Values that can be written to the field `SER`"]
pub enum SERW {
    #[doc = "Erase activated for selected sector"]
    SECTORERASE,
}
impl SERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SERW::SECTORERASE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SERW<'a> {
    w: &'a mut W,
}
impl<'a> _SERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Erase activated for selected sector"]
    #[inline]
    pub fn sector_erase(self) -> &'a mut W {
        self.variant(SERW::SECTORERASE)
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
#[doc = "Values that can be written to the field `MER`"]
pub enum MERW {
    #[doc = "Erase activated for all user sectors"]
    MASSERASE,
}
impl MERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MERW::MASSERASE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MERW<'a> {
    w: &'a mut W,
}
impl<'a> _MERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Erase activated for all user sectors"]
    #[inline]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MERW::MASSERASE)
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
#[doc = r" Proxy"]
pub struct _SNBW<'a> {
    w: &'a mut W,
}
impl<'a> _SNBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSIZE`"]
pub enum PSIZEW {
    #[doc = "Program x8"]
    PSIZE8,
    #[doc = "Program x16"]
    PSIZE16,
    #[doc = "Program x32"]
    PSIZE32,
    #[doc = "Program x64"]
    PSIZE64,
}
impl PSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSIZEW::PSIZE8 => 0,
            PSIZEW::PSIZE16 => 1,
            PSIZEW::PSIZE32 => 2,
            PSIZEW::PSIZE64 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Program x8"]
    #[inline]
    pub fn psize8(self) -> &'a mut W {
        self.variant(PSIZEW::PSIZE8)
    }
    #[doc = "Program x16"]
    #[inline]
    pub fn psize16(self) -> &'a mut W {
        self.variant(PSIZEW::PSIZE16)
    }
    #[doc = "Program x32"]
    #[inline]
    pub fn psize32(self) -> &'a mut W {
        self.variant(PSIZEW::PSIZE32)
    }
    #[doc = "Program x64"]
    #[inline]
    pub fn psize64(self) -> &'a mut W {
        self.variant(PSIZEW::PSIZE64)
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
#[doc = r" Proxy"]
pub struct _MER1W<'a> {
    w: &'a mut W,
}
impl<'a> _MER1W<'a> {
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
#[doc = "Values that can be written to the field `STRT`"]
pub enum STRTW {
    #[doc = "Trigger an erase operation"]
    START,
}
impl STRTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STRTW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STRTW<'a> {
    w: &'a mut W,
}
impl<'a> _STRTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STRTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger an erase operation"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(STRTW::START)
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
#[doc = "Values that can be written to the field `EOPIE`"]
pub enum EOPIEW {
    #[doc = "End of operation interrupt disabled"]
    DISABLED,
    #[doc = "End of operation interrupt enabled"]
    ENABLED,
}
impl EOPIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOPIEW::DISABLED => false,
            EOPIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOPIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EOPIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOPIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "End of operation interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOPIEW::DISABLED)
    }
    #[doc = "End of operation interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOPIEW::ENABLED)
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
#[doc = "Values that can be written to the field `ERRIE`"]
pub enum ERRIEW {
    #[doc = "Error interrupt generation disabled"]
    DISABLED,
    #[doc = "Error interrupt generation enabled"]
    ENABLED,
}
impl ERRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRIEW::DISABLED => false,
            ERRIEW::ENABLED => true,
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
    #[doc = "Error interrupt generation disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIEW::DISABLED)
    }
    #[doc = "Error interrupt generation enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIEW::ENABLED)
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
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "FLASH_CR register is unlocked"]
    UNLOCKED,
    #[doc = "FLASH_CR register is locked"]
    LOCKED,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKW::UNLOCKED => false,
            LOCKW::LOCKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FLASH_CR register is unlocked"]
    #[inline]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKW::UNLOCKED)
    }
    #[doc = "FLASH_CR register is locked"]
    #[inline]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKW::LOCKED)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Programming"]
    #[inline]
    pub fn pg(&self) -> PGR {
        PGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Sector Erase"]
    #[inline]
    pub fn ser(&self) -> SERR {
        SERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Mass Erase of sectors 0 to 11"]
    #[inline]
    pub fn mer(&self) -> MERR {
        MERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:7 - Sector number"]
    #[inline]
    pub fn snb(&self) -> SNBR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SNBR { bits }
    }
    #[doc = "Bits 8:9 - Program size"]
    #[inline]
    pub fn psize(&self) -> PSIZER {
        PSIZER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Mass Erase of sectors 12 to 23"]
    #[inline]
    pub fn mer1(&self) -> MER1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MER1R { bits }
    }
    #[doc = "Bit 16 - Start"]
    #[inline]
    pub fn strt(&self) -> STRTR {
        STRTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline]
    pub fn eopie(&self) -> EOPIER {
        EOPIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline]
    pub fn errie(&self) -> ERRIER {
        ERRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Lock"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x8000_0000 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Programming"]
    #[inline]
    pub fn pg(&mut self) -> _PGW {
        _PGW { w: self }
    }
    #[doc = "Bit 1 - Sector Erase"]
    #[inline]
    pub fn ser(&mut self) -> _SERW {
        _SERW { w: self }
    }
    #[doc = "Bit 2 - Mass Erase of sectors 0 to 11"]
    #[inline]
    pub fn mer(&mut self) -> _MERW {
        _MERW { w: self }
    }
    #[doc = "Bits 3:7 - Sector number"]
    #[inline]
    pub fn snb(&mut self) -> _SNBW {
        _SNBW { w: self }
    }
    #[doc = "Bits 8:9 - Program size"]
    #[inline]
    pub fn psize(&mut self) -> _PSIZEW {
        _PSIZEW { w: self }
    }
    #[doc = "Bit 15 - Mass Erase of sectors 12 to 23"]
    #[inline]
    pub fn mer1(&mut self) -> _MER1W {
        _MER1W { w: self }
    }
    #[doc = "Bit 16 - Start"]
    #[inline]
    pub fn strt(&mut self) -> _STRTW {
        _STRTW { w: self }
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline]
    pub fn eopie(&mut self) -> _EOPIEW {
        _EOPIEW { w: self }
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline]
    pub fn errie(&mut self) -> _ERRIEW {
        _ERRIEW { w: self }
    }
    #[doc = "Bit 31 - Lock"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}
