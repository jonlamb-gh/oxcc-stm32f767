#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMCCR {
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
#[doc = "Possible values of the field `CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRR {
    #[doc = "Reset all counters. Cleared automatically"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CRR {
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
            CRR::RESET => true,
            CRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRR {
        match value {
            true => CRR::RESET,
            i => CRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == CRR::RESET
    }
}
#[doc = "Possible values of the field `CSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRR {
    #[doc = "Counters roll over to zero after reaching the maximum value"]
    DISABLED,
    #[doc = "Counters do not roll over to zero after reaching the maximum value"]
    ENABLED,
}
impl CSRR {
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
            CSRR::DISABLED => false,
            CSRR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSRR {
        match value {
            false => CSRR::DISABLED,
            true => CSRR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CSRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CSRR::ENABLED
    }
}
#[doc = "Possible values of the field `ROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RORR {
    #[doc = "MMC counters do not reset on read"]
    DISABLED,
    #[doc = "MMC counters reset to zero after read"]
    ENABLED,
}
impl RORR {
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
            RORR::DISABLED => false,
            RORR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RORR {
        match value {
            false => RORR::DISABLED,
            true => RORR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RORR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RORR::ENABLED
    }
}
#[doc = "Possible values of the field `MCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCFR {
    #[doc = "All MMC counters update normally"]
    UNFROZEN,
    #[doc = "All MMC counters frozen to their current value"]
    FROZEN,
}
impl MCFR {
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
            MCFR::UNFROZEN => false,
            MCFR::FROZEN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCFR {
        match value {
            false => MCFR::UNFROZEN,
            true => MCFR::FROZEN,
        }
    }
    #[doc = "Checks if the value of the field is `UNFROZEN`"]
    #[inline]
    pub fn is_unfrozen(&self) -> bool {
        *self == MCFR::UNFROZEN
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline]
    pub fn is_frozen(&self) -> bool {
        *self == MCFR::FROZEN
    }
}
#[doc = "Possible values of the field `MCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCPR {
    #[doc = "MMC counters will be preset to almost full or almost half. Cleared automatically"]
    PRESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MCPR {
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
            MCPR::PRESET => true,
            MCPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCPR {
        match value {
            true => MCPR::PRESET,
            i => MCPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRESET`"]
    #[inline]
    pub fn is_preset(&self) -> bool {
        *self == MCPR::PRESET
    }
}
#[doc = "Possible values of the field `MCFHP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCFHPR {
    #[doc = "When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0"]
    ALMOSTHALF,
    #[doc = "When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0"]
    ALMOSTFULL,
}
impl MCFHPR {
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
            MCFHPR::ALMOSTHALF => false,
            MCFHPR::ALMOSTFULL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCFHPR {
        match value {
            false => MCFHPR::ALMOSTHALF,
            true => MCFHPR::ALMOSTFULL,
        }
    }
    #[doc = "Checks if the value of the field is `ALMOSTHALF`"]
    #[inline]
    pub fn is_almost_half(&self) -> bool {
        *self == MCFHPR::ALMOSTHALF
    }
    #[doc = "Checks if the value of the field is `ALMOSTFULL`"]
    #[inline]
    pub fn is_almost_full(&self) -> bool {
        *self == MCFHPR::ALMOSTFULL
    }
}
#[doc = "Values that can be written to the field `CR`"]
pub enum CRW {
    #[doc = "Reset all counters. Cleared automatically"]
    RESET,
}
impl CRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset all counters. Cleared automatically"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRW::RESET)
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
#[doc = "Values that can be written to the field `CSR`"]
pub enum CSRW {
    #[doc = "Counters roll over to zero after reaching the maximum value"]
    DISABLED,
    #[doc = "Counters do not roll over to zero after reaching the maximum value"]
    ENABLED,
}
impl CSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSRW::DISABLED => false,
            CSRW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSRW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counters roll over to zero after reaching the maximum value"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSRW::DISABLED)
    }
    #[doc = "Counters do not roll over to zero after reaching the maximum value"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSRW::ENABLED)
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
#[doc = "Values that can be written to the field `ROR`"]
pub enum RORW {
    #[doc = "MMC counters do not reset on read"]
    DISABLED,
    #[doc = "MMC counters reset to zero after read"]
    ENABLED,
}
impl RORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RORW::DISABLED => false,
            RORW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RORW<'a> {
    w: &'a mut W,
}
impl<'a> _RORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MMC counters do not reset on read"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RORW::DISABLED)
    }
    #[doc = "MMC counters reset to zero after read"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RORW::ENABLED)
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
#[doc = "Values that can be written to the field `MCF`"]
pub enum MCFW {
    #[doc = "All MMC counters update normally"]
    UNFROZEN,
    #[doc = "All MMC counters frozen to their current value"]
    FROZEN,
}
impl MCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCFW::UNFROZEN => false,
            MCFW::FROZEN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCFW<'a> {
    w: &'a mut W,
}
impl<'a> _MCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All MMC counters update normally"]
    #[inline]
    pub fn unfrozen(self) -> &'a mut W {
        self.variant(MCFW::UNFROZEN)
    }
    #[doc = "All MMC counters frozen to their current value"]
    #[inline]
    pub fn frozen(self) -> &'a mut W {
        self.variant(MCFW::FROZEN)
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
#[doc = "Values that can be written to the field `MCP`"]
pub enum MCPW {
    #[doc = "MMC counters will be preset to almost full or almost half. Cleared automatically"]
    PRESET,
}
impl MCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCPW::PRESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCPW<'a> {
    w: &'a mut W,
}
impl<'a> _MCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MMC counters will be preset to almost full or almost half. Cleared automatically"]
    #[inline]
    pub fn preset(self) -> &'a mut W {
        self.variant(MCPW::PRESET)
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
#[doc = "Values that can be written to the field `MCFHP`"]
pub enum MCFHPW {
    #[doc = "When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0"]
    ALMOSTHALF,
    #[doc = "When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0"]
    ALMOSTFULL,
}
impl MCFHPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCFHPW::ALMOSTHALF => false,
            MCFHPW::ALMOSTFULL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCFHPW<'a> {
    w: &'a mut W,
}
impl<'a> _MCFHPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCFHPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0"]
    #[inline]
    pub fn almost_half(self) -> &'a mut W {
        self.variant(MCFHPW::ALMOSTHALF)
    }
    #[doc = "When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0"]
    #[inline]
    pub fn almost_full(self) -> &'a mut W {
        self.variant(MCFHPW::ALMOSTFULL)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - CR"]
    #[inline]
    pub fn cr(&self) -> CRR {
        CRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CSR"]
    #[inline]
    pub fn csr(&self) -> CSRR {
        CSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - ROR"]
    #[inline]
    pub fn ror(&self) -> RORR {
        RORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - MCF"]
    #[inline]
    pub fn mcf(&self) -> MCFR {
        MCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - MCP"]
    #[inline]
    pub fn mcp(&self) -> MCPR {
        MCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - MCFHP"]
    #[inline]
    pub fn mcfhp(&self) -> MCFHPR {
        MCFHPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - CR"]
    #[inline]
    pub fn cr(&mut self) -> _CRW {
        _CRW { w: self }
    }
    #[doc = "Bit 1 - CSR"]
    #[inline]
    pub fn csr(&mut self) -> _CSRW {
        _CSRW { w: self }
    }
    #[doc = "Bit 2 - ROR"]
    #[inline]
    pub fn ror(&mut self) -> _RORW {
        _RORW { w: self }
    }
    #[doc = "Bit 3 - MCF"]
    #[inline]
    pub fn mcf(&mut self) -> _MCFW {
        _MCFW { w: self }
    }
    #[doc = "Bit 4 - MCP"]
    #[inline]
    pub fn mcp(&mut self) -> _MCPW {
        _MCPW { w: self }
    }
    #[doc = "Bit 5 - MCFHP"]
    #[inline]
    pub fn mcfhp(&mut self) -> _MCFHPW {
        _MCFHPW { w: self }
    }
}
