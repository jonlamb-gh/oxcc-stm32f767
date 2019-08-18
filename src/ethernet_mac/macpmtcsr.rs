#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACPMTCSR {
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
#[doc = "Possible values of the field `PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDR {
    #[doc = "All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    ENABLED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PDR {
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
            PDR::ENABLED => true,
            PDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDR {
        match value {
            true => PDR::ENABLED,
            i => PDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PDR::ENABLED
    }
}
#[doc = "Possible values of the field `MPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPER {
    #[doc = "No power management event generated due to Magic Packet reception"]
    DISABLED,
    #[doc = "Enable generation of a power management event due to Magic Packet reception"]
    ENABLED,
}
impl MPER {
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
            MPER::DISABLED => false,
            MPER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPER {
        match value {
            false => MPER::DISABLED,
            true => MPER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MPER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MPER::ENABLED
    }
}
#[doc = "Possible values of the field `WFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WFER {
    #[doc = "No power management event generated due to wakeup frame reception"]
    DISABLED,
    #[doc = "Enable generation of a power management event due to wakeup frame reception"]
    ENABLED,
}
impl WFER {
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
            WFER::DISABLED => false,
            WFER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WFER {
        match value {
            false => WFER::DISABLED,
            true => WFER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WFER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WFER::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct MPRR {
    bits: bool,
}
impl MPRR {
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
#[doc = r" Value of the field"]
pub struct WFRR {
    bits: bool,
}
impl WFRR {
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
#[doc = "Possible values of the field `GU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GUR {
    #[doc = "Normal operation"]
    DISABLED,
    #[doc = "Any unicast packet filtered by the MAC address recognition may be a wakeup frame"]
    ENABLED,
}
impl GUR {
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
            GUR::DISABLED => false,
            GUR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GUR {
        match value {
            false => GUR::DISABLED,
            true => GUR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == GUR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == GUR::ENABLED
    }
}
#[doc = "Possible values of the field `WFFRPR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WFFRPRR {
    #[doc = "Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WFFRPRR {
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
            WFFRPRR::RESET => true,
            WFFRPRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WFFRPRR {
        match value {
            true => WFFRPRR::RESET,
            i => WFFRPRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == WFFRPRR::RESET
    }
}
#[doc = "Values that can be written to the field `PD`"]
pub enum PDW {
    #[doc = "All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    ENABLED,
}
impl PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDW<'a> {
    w: &'a mut W,
}
impl<'a> _PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PDW::ENABLED)
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
#[doc = "Values that can be written to the field `MPE`"]
pub enum MPEW {
    #[doc = "No power management event generated due to Magic Packet reception"]
    DISABLED,
    #[doc = "Enable generation of a power management event due to Magic Packet reception"]
    ENABLED,
}
impl MPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPEW::DISABLED => false,
            MPEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPEW<'a> {
    w: &'a mut W,
}
impl<'a> _MPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No power management event generated due to Magic Packet reception"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MPEW::DISABLED)
    }
    #[doc = "Enable generation of a power management event due to Magic Packet reception"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MPEW::ENABLED)
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
#[doc = "Values that can be written to the field `WFE`"]
pub enum WFEW {
    #[doc = "No power management event generated due to wakeup frame reception"]
    DISABLED,
    #[doc = "Enable generation of a power management event due to wakeup frame reception"]
    ENABLED,
}
impl WFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WFEW::DISABLED => false,
            WFEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WFEW<'a> {
    w: &'a mut W,
}
impl<'a> _WFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No power management event generated due to wakeup frame reception"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WFEW::DISABLED)
    }
    #[doc = "Enable generation of a power management event due to wakeup frame reception"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WFEW::ENABLED)
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
pub struct _MPRW<'a> {
    w: &'a mut W,
}
impl<'a> _MPRW<'a> {
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
pub struct _WFRW<'a> {
    w: &'a mut W,
}
impl<'a> _WFRW<'a> {
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
#[doc = "Values that can be written to the field `GU`"]
pub enum GUW {
    #[doc = "Normal operation"]
    DISABLED,
    #[doc = "Any unicast packet filtered by the MAC address recognition may be a wakeup frame"]
    ENABLED,
}
impl GUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GUW::DISABLED => false,
            GUW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GUW<'a> {
    w: &'a mut W,
}
impl<'a> _GUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GUW::DISABLED)
    }
    #[doc = "Any unicast packet filtered by the MAC address recognition may be a wakeup frame"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GUW::ENABLED)
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
#[doc = "Values that can be written to the field `WFFRPR`"]
pub enum WFFRPRW {
    #[doc = "Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    RESET,
}
impl WFFRPRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WFFRPRW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WFFRPRW<'a> {
    w: &'a mut W,
}
impl<'a> _WFFRPRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WFFRPRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(WFFRPRW::RESET)
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
    #[doc = "Bit 0 - PD"]
    #[inline]
    pub fn pd(&self) -> PDR {
        PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - MPE"]
    #[inline]
    pub fn mpe(&self) -> MPER {
        MPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - WFE"]
    #[inline]
    pub fn wfe(&self) -> WFER {
        WFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - MPR"]
    #[inline]
    pub fn mpr(&self) -> MPRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MPRR { bits }
    }
    #[doc = "Bit 6 - WFR"]
    #[inline]
    pub fn wfr(&self) -> WFRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WFRR { bits }
    }
    #[doc = "Bit 9 - GU"]
    #[inline]
    pub fn gu(&self) -> GUR {
        GUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - WFFRPR"]
    #[inline]
    pub fn wffrpr(&self) -> WFFRPRR {
        WFFRPRR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - PD"]
    #[inline]
    pub fn pd(&mut self) -> _PDW {
        _PDW { w: self }
    }
    #[doc = "Bit 1 - MPE"]
    #[inline]
    pub fn mpe(&mut self) -> _MPEW {
        _MPEW { w: self }
    }
    #[doc = "Bit 2 - WFE"]
    #[inline]
    pub fn wfe(&mut self) -> _WFEW {
        _WFEW { w: self }
    }
    #[doc = "Bit 5 - MPR"]
    #[inline]
    pub fn mpr(&mut self) -> _MPRW {
        _MPRW { w: self }
    }
    #[doc = "Bit 6 - WFR"]
    #[inline]
    pub fn wfr(&mut self) -> _WFRW {
        _WFRW { w: self }
    }
    #[doc = "Bit 9 - GU"]
    #[inline]
    pub fn gu(&mut self) -> _GUW {
        _GUW { w: self }
    }
    #[doc = "Bit 31 - WFFRPR"]
    #[inline]
    pub fn wffrpr(&mut self) -> _WFFRPRW {
        _WFFRPRW { w: self }
    }
}
