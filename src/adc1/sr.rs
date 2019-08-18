#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
#[doc = "Possible values of the field `OVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRR {
    #[doc = "No overrun occurred"]
    NOOVERRUN,
    #[doc = "Overrun occurred"]
    OVERRUN,
}
impl OVRR {
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
            OVRR::NOOVERRUN => false,
            OVRR::OVERRUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVRR {
        match value {
            false => OVRR::NOOVERRUN,
            true => OVRR::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR::OVERRUN
    }
}
#[doc = "Possible values of the field `STRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRTR {
    #[doc = "No regular channel conversion started"]
    NOTSTARTED,
    #[doc = "Regular channel conversion has started"]
    STARTED,
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
            STRTR::NOTSTARTED => false,
            STRTR::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STRTR {
        match value {
            false => STRTR::NOTSTARTED,
            true => STRTR::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == STRTR::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == STRTR::STARTED
    }
}
#[doc = "Possible values of the field `JSTRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSTRTR {
    #[doc = "No injected channel conversion started"]
    NOTSTARTED,
    #[doc = "Injected channel conversion has started"]
    STARTED,
}
impl JSTRTR {
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
            JSTRTR::NOTSTARTED => false,
            JSTRTR::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JSTRTR {
        match value {
            false => JSTRTR::NOTSTARTED,
            true => JSTRTR::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRTR::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == JSTRTR::STARTED
    }
}
#[doc = "Possible values of the field `JEOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOCR {
    #[doc = "Conversion is not complete"]
    NOTCOMPLETE,
    #[doc = "Conversion complete"]
    COMPLETE,
}
impl JEOCR {
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
            JEOCR::NOTCOMPLETE => false,
            JEOCR::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JEOCR {
        match value {
            false => JEOCR::NOTCOMPLETE,
            true => JEOCR::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOCR::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == JEOCR::COMPLETE
    }
}
#[doc = "Possible values of the field `EOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCR {
    #[doc = "Conversion is not complete"]
    NOTCOMPLETE,
    #[doc = "Conversion complete"]
    COMPLETE,
}
impl EOCR {
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
            EOCR::NOTCOMPLETE => false,
            EOCR::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOCR {
        match value {
            false => EOCR::NOTCOMPLETE,
            true => EOCR::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == EOCR::COMPLETE
    }
}
#[doc = "Possible values of the field `AWD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDR {
    #[doc = "No analog watchdog event occurred"]
    NOEVENT,
    #[doc = "Analog watchdog event occurred"]
    EVENT,
}
impl AWDR {
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
            AWDR::NOEVENT => false,
            AWDR::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AWDR {
        match value {
            false => AWDR::NOEVENT,
            true => AWDR::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline]
    pub fn is_no_event(&self) -> bool {
        *self == AWDR::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == AWDR::EVENT
    }
}
#[doc = "Values that can be written to the field `OVR`"]
pub enum OVRW {
    #[doc = "No overrun occurred"]
    NOOVERRUN,
    #[doc = "Overrun occurred"]
    OVERRUN,
}
impl OVRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVRW::NOOVERRUN => false,
            OVRW::OVERRUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _OVRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overrun occurred"]
    #[inline]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OVRW::NOOVERRUN)
    }
    #[doc = "Overrun occurred"]
    #[inline]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OVRW::OVERRUN)
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
#[doc = "Values that can be written to the field `STRT`"]
pub enum STRTW {
    #[doc = "No regular channel conversion started"]
    NOTSTARTED,
    #[doc = "Regular channel conversion has started"]
    STARTED,
}
impl STRTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STRTW::NOTSTARTED => false,
            STRTW::STARTED => true,
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
    #[doc = "No regular channel conversion started"]
    #[inline]
    pub fn not_started(self) -> &'a mut W {
        self.variant(STRTW::NOTSTARTED)
    }
    #[doc = "Regular channel conversion has started"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(STRTW::STARTED)
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
#[doc = "Values that can be written to the field `JSTRT`"]
pub enum JSTRTW {
    #[doc = "No injected channel conversion started"]
    NOTSTARTED,
    #[doc = "Injected channel conversion has started"]
    STARTED,
}
impl JSTRTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JSTRTW::NOTSTARTED => false,
            JSTRTW::STARTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JSTRTW<'a> {
    w: &'a mut W,
}
impl<'a> _JSTRTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JSTRTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No injected channel conversion started"]
    #[inline]
    pub fn not_started(self) -> &'a mut W {
        self.variant(JSTRTW::NOTSTARTED)
    }
    #[doc = "Injected channel conversion has started"]
    #[inline]
    pub fn started(self) -> &'a mut W {
        self.variant(JSTRTW::STARTED)
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
#[doc = "Values that can be written to the field `JEOC`"]
pub enum JEOCW {
    #[doc = "Conversion is not complete"]
    NOTCOMPLETE,
    #[doc = "Conversion complete"]
    COMPLETE,
}
impl JEOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JEOCW::NOTCOMPLETE => false,
            JEOCW::COMPLETE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JEOCW<'a> {
    w: &'a mut W,
}
impl<'a> _JEOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JEOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline]
    pub fn not_complete(self) -> &'a mut W {
        self.variant(JEOCW::NOTCOMPLETE)
    }
    #[doc = "Conversion complete"]
    #[inline]
    pub fn complete(self) -> &'a mut W {
        self.variant(JEOCW::COMPLETE)
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
#[doc = "Values that can be written to the field `EOC`"]
pub enum EOCW {
    #[doc = "Conversion is not complete"]
    NOTCOMPLETE,
    #[doc = "Conversion complete"]
    COMPLETE,
}
impl EOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOCW::NOTCOMPLETE => false,
            EOCW::COMPLETE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOCW<'a> {
    w: &'a mut W,
}
impl<'a> _EOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline]
    pub fn not_complete(self) -> &'a mut W {
        self.variant(EOCW::NOTCOMPLETE)
    }
    #[doc = "Conversion complete"]
    #[inline]
    pub fn complete(self) -> &'a mut W {
        self.variant(EOCW::COMPLETE)
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
#[doc = "Values that can be written to the field `AWD`"]
pub enum AWDW {
    #[doc = "No analog watchdog event occurred"]
    NOEVENT,
    #[doc = "Analog watchdog event occurred"]
    EVENT,
}
impl AWDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AWDW::NOEVENT => false,
            AWDW::EVENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AWDW<'a> {
    w: &'a mut W,
}
impl<'a> _AWDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No analog watchdog event occurred"]
    #[inline]
    pub fn no_event(self) -> &'a mut W {
        self.variant(AWDW::NOEVENT)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(AWDW::EVENT)
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
    #[doc = "Bit 5 - Overrun"]
    #[inline]
    pub fn ovr(&self) -> OVRR {
        OVRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline]
    pub fn strt(&self) -> STRTR {
        STRTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline]
    pub fn jstrt(&self) -> JSTRTR {
        JSTRTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline]
    pub fn jeoc(&self) -> JEOCR {
        JEOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline]
    pub fn eoc(&self) -> EOCR {
        EOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline]
    pub fn awd(&self) -> AWDR {
        AWDR::_from({
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
    #[doc = "Bit 5 - Overrun"]
    #[inline]
    pub fn ovr(&mut self) -> _OVRW {
        _OVRW { w: self }
    }
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline]
    pub fn strt(&mut self) -> _STRTW {
        _STRTW { w: self }
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline]
    pub fn jstrt(&mut self) -> _JSTRTW {
        _JSTRTW { w: self }
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline]
    pub fn jeoc(&mut self) -> _JEOCW {
        _JEOCW { w: self }
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline]
    pub fn eoc(&mut self) -> _EOCW {
        _EOCW { w: self }
    }
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline]
    pub fn awd(&mut self) -> _AWDW {
        _AWDW { w: self }
    }
}
