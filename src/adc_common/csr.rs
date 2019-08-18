#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `OVR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR3R {
    #[doc = "No overrun occurred"]
    NOOVERRUN,
    #[doc = "Overrun occurred"]
    OVERRUN,
}
impl OVR3R {
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
            OVR3R::NOOVERRUN => false,
            OVR3R::OVERRUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVR3R {
        match value {
            false => OVR3R::NOOVERRUN,
            true => OVR3R::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR3R::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline]
    pub fn is_overrun(&self) -> bool {
        *self == OVR3R::OVERRUN
    }
}
#[doc = "Possible values of the field `STRT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT3R {
    #[doc = "No regular channel conversion started"]
    NOTSTARTED,
    #[doc = "Regular channel conversion has started"]
    STARTED,
}
impl STRT3R {
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
            STRT3R::NOTSTARTED => false,
            STRT3R::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STRT3R {
        match value {
            false => STRT3R::NOTSTARTED,
            true => STRT3R::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == STRT3R::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == STRT3R::STARTED
    }
}
#[doc = "Possible values of the field `JSTRT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSTRT3R {
    #[doc = "No injected channel conversion started"]
    NOTSTARTED,
    #[doc = "Injected channel conversion has started"]
    STARTED,
}
impl JSTRT3R {
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
            JSTRT3R::NOTSTARTED => false,
            JSTRT3R::STARTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JSTRT3R {
        match value {
            false => JSTRT3R::NOTSTARTED,
            true => JSTRT3R::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRT3R::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline]
    pub fn is_started(&self) -> bool {
        *self == JSTRT3R::STARTED
    }
}
#[doc = "Possible values of the field `JEOC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC3R {
    #[doc = "Conversion is not complete"]
    NOTCOMPLETE,
    #[doc = "Conversion complete"]
    COMPLETE,
}
impl JEOC3R {
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
            JEOC3R::NOTCOMPLETE => false,
            JEOC3R::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JEOC3R {
        match value {
            false => JEOC3R::NOTCOMPLETE,
            true => JEOC3R::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC3R::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == JEOC3R::COMPLETE
    }
}
#[doc = "Possible values of the field `EOC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC3R {
    #[doc = "Conversion is not complete"]
    NOTCOMPLETE,
    #[doc = "Conversion complete"]
    COMPLETE,
}
impl EOC3R {
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
            EOC3R::NOTCOMPLETE => false,
            EOC3R::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOC3R {
        match value {
            false => EOC3R::NOTCOMPLETE,
            true => EOC3R::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC3R::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == EOC3R::COMPLETE
    }
}
#[doc = "Possible values of the field `AWD3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD3R {
    #[doc = "No analog watchdog event occurred"]
    NOEVENT,
    #[doc = "Analog watchdog event occurred"]
    EVENT,
}
impl AWD3R {
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
            AWD3R::NOEVENT => false,
            AWD3R::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AWD3R {
        match value {
            false => AWD3R::NOEVENT,
            true => AWD3R::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline]
    pub fn is_no_event(&self) -> bool {
        *self == AWD3R::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == AWD3R::EVENT
    }
}
#[doc = "Possible values of the field `OVR2`"]
pub type OVR2R = OVR3R;
#[doc = "Possible values of the field `STRT2`"]
pub type STRT2R = STRT3R;
#[doc = "Possible values of the field `JSTRT2`"]
pub type JSTRT2R = JSTRT3R;
#[doc = "Possible values of the field `JEOC2`"]
pub type JEOC2R = JEOC3R;
#[doc = "Possible values of the field `EOC2`"]
pub type EOC2R = EOC3R;
#[doc = "Possible values of the field `AWD2`"]
pub type AWD2R = AWD3R;
#[doc = "Possible values of the field `OVR1`"]
pub type OVR1R = OVR3R;
#[doc = "Possible values of the field `STRT1`"]
pub type STRT1R = STRT3R;
#[doc = "Possible values of the field `JSTRT1`"]
pub type JSTRT1R = JSTRT3R;
#[doc = "Possible values of the field `JEOC1`"]
pub type JEOC1R = JEOC3R;
#[doc = "Possible values of the field `EOC1`"]
pub type EOC1R = EOC3R;
#[doc = "Possible values of the field `AWD1`"]
pub type AWD1R = AWD3R;
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 21 - Overrun flag of ADC3"]
    #[inline]
    pub fn ovr3(&self) -> OVR3R {
        OVR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Regular channel Start flag of ADC3"]
    #[inline]
    pub fn strt3(&self) -> STRT3R {
        STRT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Injected channel Start flag of ADC3"]
    #[inline]
    pub fn jstrt3(&self) -> JSTRT3R {
        JSTRT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Injected channel end of conversion of ADC3"]
    #[inline]
    pub fn jeoc3(&self) -> JEOC3R {
        JEOC3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - End of conversion of ADC3"]
    #[inline]
    pub fn eoc3(&self) -> EOC3R {
        EOC3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Analog watchdog flag of ADC3"]
    #[inline]
    pub fn awd3(&self) -> AWD3R {
        AWD3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Overrun flag of ADC2"]
    #[inline]
    pub fn ovr2(&self) -> OVR2R {
        OVR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Regular channel Start flag of ADC2"]
    #[inline]
    pub fn strt2(&self) -> STRT2R {
        STRT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Injected channel Start flag of ADC2"]
    #[inline]
    pub fn jstrt2(&self) -> JSTRT2R {
        JSTRT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Injected channel end of conversion of ADC2"]
    #[inline]
    pub fn jeoc2(&self) -> JEOC2R {
        JEOC2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - End of conversion of ADC2"]
    #[inline]
    pub fn eoc2(&self) -> EOC2R {
        EOC2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Analog watchdog flag of ADC2"]
    #[inline]
    pub fn awd2(&self) -> AWD2R {
        AWD2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Overrun flag of ADC1"]
    #[inline]
    pub fn ovr1(&self) -> OVR1R {
        OVR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Regular channel Start flag of ADC1"]
    #[inline]
    pub fn strt1(&self) -> STRT1R {
        STRT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Injected channel Start flag of ADC1"]
    #[inline]
    pub fn jstrt1(&self) -> JSTRT1R {
        JSTRT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Injected channel end of conversion of ADC1"]
    #[inline]
    pub fn jeoc1(&self) -> JEOC1R {
        JEOC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - End of conversion of ADC1"]
    #[inline]
    pub fn eoc1(&self) -> EOC1R {
        EOC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Analog watchdog flag of ADC1"]
    #[inline]
    pub fn awd1(&self) -> AWD1R {
        AWD1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
