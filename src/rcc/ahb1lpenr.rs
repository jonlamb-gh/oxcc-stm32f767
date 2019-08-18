#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHB1LPENR {
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
#[doc = "Possible values of the field `GPIOALPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOALPENR {
    #[doc = "Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl GPIOALPENR {
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
            GPIOALPENR::DISABLEDINSLEEP => false,
            GPIOALPENR::ENABLEDINSLEEP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIOALPENR {
        match value {
            false => GPIOALPENR::DISABLEDINSLEEP,
            true => GPIOALPENR::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == GPIOALPENR::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == GPIOALPENR::ENABLEDINSLEEP
    }
}
#[doc = "Possible values of the field `GPIOBLPEN`"]
pub type GPIOBLPENR = GPIOALPENR;
#[doc = "Possible values of the field `GPIOCLPEN`"]
pub type GPIOCLPENR = GPIOALPENR;
#[doc = "Possible values of the field `GPIODLPEN`"]
pub type GPIODLPENR = GPIOALPENR;
#[doc = "Possible values of the field `GPIOELPEN`"]
pub type GPIOELPENR = GPIOALPENR;
#[doc = "Possible values of the field `GPIOFLPEN`"]
pub type GPIOFLPENR = GPIOALPENR;
#[doc = "Possible values of the field `GPIOGLPEN`"]
pub type GPIOGLPENR = GPIOALPENR;
#[doc = "Possible values of the field `GPIOHLPEN`"]
pub type GPIOHLPENR = GPIOALPENR;
#[doc = "Possible values of the field `GPIOILPEN`"]
pub type GPIOILPENR = GPIOALPENR;
#[doc = "Possible values of the field `GPIOJLPEN`"]
pub type GPIOJLPENR = GPIOALPENR;
#[doc = "Possible values of the field `GPIOKLPEN`"]
pub type GPIOKLPENR = GPIOALPENR;
#[doc = "Possible values of the field `CRCLPEN`"]
pub type CRCLPENR = GPIOALPENR;
#[doc = "Possible values of the field `FLITFLPEN`"]
pub type FLITFLPENR = GPIOALPENR;
#[doc = "Possible values of the field `SRAM1LPEN`"]
pub type SRAM1LPENR = GPIOALPENR;
#[doc = "Possible values of the field `SRAM2LPEN`"]
pub type SRAM2LPENR = GPIOALPENR;
#[doc = "Possible values of the field `BKPSRAMLPEN`"]
pub type BKPSRAMLPENR = GPIOALPENR;
#[doc = "Possible values of the field `SRAM3LPEN`"]
pub type SRAM3LPENR = GPIOALPENR;
#[doc = "Possible values of the field `DMA1LPEN`"]
pub type DMA1LPENR = GPIOALPENR;
#[doc = "Possible values of the field `DMA2LPEN`"]
pub type DMA2LPENR = GPIOALPENR;
#[doc = "Possible values of the field `DMA2DLPEN`"]
pub type DMA2DLPENR = GPIOALPENR;
#[doc = "Possible values of the field `ETHMACLPEN`"]
pub type ETHMACLPENR = GPIOALPENR;
#[doc = "Possible values of the field `ETHMACTXLPEN`"]
pub type ETHMACTXLPENR = GPIOALPENR;
#[doc = "Possible values of the field `ETHMACRXLPEN`"]
pub type ETHMACRXLPENR = GPIOALPENR;
#[doc = "Possible values of the field `ETHMACPTPLPEN`"]
pub type ETHMACPTPLPENR = GPIOALPENR;
#[doc = "Possible values of the field `OTGHSLPEN`"]
pub type OTGHSLPENR = GPIOALPENR;
#[doc = "Possible values of the field `OTGHSULPILPEN`"]
pub type OTGHSULPILPENR = GPIOALPENR;
#[doc = "Values that can be written to the field `GPIOALPEN`"]
pub enum GPIOALPENW {
    #[doc = "Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP,
    #[doc = "Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP,
}
impl GPIOALPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIOALPENW::DISABLEDINSLEEP => false,
            GPIOALPENW::ENABLEDINSLEEP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIOALPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOALPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOALPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `GPIOBLPEN`"]
pub type GPIOBLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _GPIOBLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOBLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOBLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `GPIOCLPEN`"]
pub type GPIOCLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _GPIOCLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOCLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOCLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `GPIODLPEN`"]
pub type GPIODLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _GPIODLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIODLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIODLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `GPIOELPEN`"]
pub type GPIOELPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _GPIOELPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOELPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOELPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `GPIOFLPEN`"]
pub type GPIOFLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _GPIOFLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOFLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOFLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `GPIOGLPEN`"]
pub type GPIOGLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _GPIOGLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOGLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOGLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOHLPEN`"]
pub type GPIOHLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _GPIOHLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOHLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOHLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `GPIOILPEN`"]
pub type GPIOILPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _GPIOILPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOILPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOILPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `GPIOJLPEN`"]
pub type GPIOJLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _GPIOJLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOJLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOJLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `GPIOKLPEN`"]
pub type GPIOKLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _GPIOKLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOKLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOKLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `CRCLPEN`"]
pub type CRCLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _CRCLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `FLITFLPEN`"]
pub type FLITFLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _FLITFLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLITFLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLITFLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SRAM1LPEN`"]
pub type SRAM1LPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _SRAM1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SRAM2LPEN`"]
pub type SRAM2LPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _SRAM2LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM2LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM2LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BKPSRAMLPEN`"]
pub type BKPSRAMLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _BKPSRAMLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _BKPSRAMLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BKPSRAMLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `SRAM3LPEN`"]
pub type SRAM3LPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _SRAM3LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM3LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM3LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `DMA1LPEN`"]
pub type DMA1LPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _DMA1LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA1LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA2LPEN`"]
pub type DMA2LPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _DMA2LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA2LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `DMA2DLPEN`"]
pub type DMA2DLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _DMA2DLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2DLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA2DLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `ETHMACLPEN`"]
pub type ETHMACLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _ETHMACLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHMACLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHMACLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `ETHMACTXLPEN`"]
pub type ETHMACTXLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _ETHMACTXLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHMACTXLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHMACTXLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
#[doc = "Values that can be written to the field `ETHMACRXLPEN`"]
pub type ETHMACRXLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _ETHMACRXLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHMACRXLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHMACRXLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETHMACPTPLPEN`"]
pub type ETHMACPTPLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _ETHMACPTPLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHMACPTPLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHMACPTPLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OTGHSLPEN`"]
pub type OTGHSLPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _OTGHSLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGHSLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGHSLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OTGHSULPILPEN`"]
pub type OTGHSULPILPENW = GPIOALPENW;
#[doc = r" Proxy"]
pub struct _OTGHSULPILPENW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGHSULPILPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGHSULPILPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPENW::ENABLEDINSLEEP)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline]
    pub fn gpioalpen(&self) -> GPIOALPENR {
        GPIOALPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline]
    pub fn gpioblpen(&self) -> GPIOBLPENR {
        GPIOBLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline]
    pub fn gpioclpen(&self) -> GPIOCLPENR {
        GPIOCLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline]
    pub fn gpiodlpen(&self) -> GPIODLPENR {
        GPIODLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline]
    pub fn gpioelpen(&self) -> GPIOELPENR {
        GPIOELPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - IO port F clock enable during Sleep mode"]
    #[inline]
    pub fn gpioflpen(&self) -> GPIOFLPENR {
        GPIOFLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - IO port G clock enable during Sleep mode"]
    #[inline]
    pub fn gpioglpen(&self) -> GPIOGLPENR {
        GPIOGLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline]
    pub fn gpiohlpen(&self) -> GPIOHLPENR {
        GPIOHLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - IO port I clock enable during Sleep mode"]
    #[inline]
    pub fn gpioilpen(&self) -> GPIOILPENR {
        GPIOILPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - IO port J clock enable during Sleep mode"]
    #[inline]
    pub fn gpiojlpen(&self) -> GPIOJLPENR {
        GPIOJLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - IO port K clock enable during Sleep mode"]
    #[inline]
    pub fn gpioklpen(&self) -> GPIOKLPENR {
        GPIOKLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline]
    pub fn crclpen(&self) -> CRCLPENR {
        CRCLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline]
    pub fn flitflpen(&self) -> FLITFLPENR {
        FLITFLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline]
    pub fn sram1lpen(&self) -> SRAM1LPENR {
        SRAM1LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - SRAM 2 interface clock enable during Sleep mode"]
    #[inline]
    pub fn sram2lpen(&self) -> SRAM2LPENR {
        SRAM2LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable during Sleep mode"]
    #[inline]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPENR {
        BKPSRAMLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - SRAM 3 interface clock enable during Sleep mode"]
    #[inline]
    pub fn sram3lpen(&self) -> SRAM3LPENR {
        SRAM3LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline]
    pub fn dma1lpen(&self) -> DMA1LPENR {
        DMA1LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline]
    pub fn dma2lpen(&self) -> DMA2LPENR {
        DMA2LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - DMA2D clock enable during Sleep mode"]
    #[inline]
    pub fn dma2dlpen(&self) -> DMA2DLPENR {
        DMA2DLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable during Sleep mode"]
    #[inline]
    pub fn ethmaclpen(&self) -> ETHMACLPENR {
        ETHMACLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Ethernet transmission clock enable during Sleep mode"]
    #[inline]
    pub fn ethmactxlpen(&self) -> ETHMACTXLPENR {
        ETHMACTXLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Ethernet reception clock enable during Sleep mode"]
    #[inline]
    pub fn ethmacrxlpen(&self) -> ETHMACRXLPENR {
        ETHMACRXLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable during Sleep mode"]
    #[inline]
    pub fn ethmacptplpen(&self) -> ETHMACPTPLPENR {
        ETHMACPTPLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - USB OTG HS clock enable during Sleep mode"]
    #[inline]
    pub fn otghslpen(&self) -> OTGHSLPENR {
        OTGHSLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - USB OTG HS ULPI clock enable during Sleep mode"]
    #[inline]
    pub fn otghsulpilpen(&self) -> OTGHSULPILPENR {
        OTGHSULPILPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x7e67_91ff }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline]
    pub fn gpioalpen(&mut self) -> _GPIOALPENW {
        _GPIOALPENW { w: self }
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline]
    pub fn gpioblpen(&mut self) -> _GPIOBLPENW {
        _GPIOBLPENW { w: self }
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline]
    pub fn gpioclpen(&mut self) -> _GPIOCLPENW {
        _GPIOCLPENW { w: self }
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline]
    pub fn gpiodlpen(&mut self) -> _GPIODLPENW {
        _GPIODLPENW { w: self }
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline]
    pub fn gpioelpen(&mut self) -> _GPIOELPENW {
        _GPIOELPENW { w: self }
    }
    #[doc = "Bit 5 - IO port F clock enable during Sleep mode"]
    #[inline]
    pub fn gpioflpen(&mut self) -> _GPIOFLPENW {
        _GPIOFLPENW { w: self }
    }
    #[doc = "Bit 6 - IO port G clock enable during Sleep mode"]
    #[inline]
    pub fn gpioglpen(&mut self) -> _GPIOGLPENW {
        _GPIOGLPENW { w: self }
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline]
    pub fn gpiohlpen(&mut self) -> _GPIOHLPENW {
        _GPIOHLPENW { w: self }
    }
    #[doc = "Bit 8 - IO port I clock enable during Sleep mode"]
    #[inline]
    pub fn gpioilpen(&mut self) -> _GPIOILPENW {
        _GPIOILPENW { w: self }
    }
    #[doc = "Bit 9 - IO port J clock enable during Sleep mode"]
    #[inline]
    pub fn gpiojlpen(&mut self) -> _GPIOJLPENW {
        _GPIOJLPENW { w: self }
    }
    #[doc = "Bit 10 - IO port K clock enable during Sleep mode"]
    #[inline]
    pub fn gpioklpen(&mut self) -> _GPIOKLPENW {
        _GPIOKLPENW { w: self }
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline]
    pub fn crclpen(&mut self) -> _CRCLPENW {
        _CRCLPENW { w: self }
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline]
    pub fn flitflpen(&mut self) -> _FLITFLPENW {
        _FLITFLPENW { w: self }
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline]
    pub fn sram1lpen(&mut self) -> _SRAM1LPENW {
        _SRAM1LPENW { w: self }
    }
    #[doc = "Bit 17 - SRAM 2 interface clock enable during Sleep mode"]
    #[inline]
    pub fn sram2lpen(&mut self) -> _SRAM2LPENW {
        _SRAM2LPENW { w: self }
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable during Sleep mode"]
    #[inline]
    pub fn bkpsramlpen(&mut self) -> _BKPSRAMLPENW {
        _BKPSRAMLPENW { w: self }
    }
    #[doc = "Bit 19 - SRAM 3 interface clock enable during Sleep mode"]
    #[inline]
    pub fn sram3lpen(&mut self) -> _SRAM3LPENW {
        _SRAM3LPENW { w: self }
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline]
    pub fn dma1lpen(&mut self) -> _DMA1LPENW {
        _DMA1LPENW { w: self }
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline]
    pub fn dma2lpen(&mut self) -> _DMA2LPENW {
        _DMA2LPENW { w: self }
    }
    #[doc = "Bit 23 - DMA2D clock enable during Sleep mode"]
    #[inline]
    pub fn dma2dlpen(&mut self) -> _DMA2DLPENW {
        _DMA2DLPENW { w: self }
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable during Sleep mode"]
    #[inline]
    pub fn ethmaclpen(&mut self) -> _ETHMACLPENW {
        _ETHMACLPENW { w: self }
    }
    #[doc = "Bit 26 - Ethernet transmission clock enable during Sleep mode"]
    #[inline]
    pub fn ethmactxlpen(&mut self) -> _ETHMACTXLPENW {
        _ETHMACTXLPENW { w: self }
    }
    #[doc = "Bit 27 - Ethernet reception clock enable during Sleep mode"]
    #[inline]
    pub fn ethmacrxlpen(&mut self) -> _ETHMACRXLPENW {
        _ETHMACRXLPENW { w: self }
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable during Sleep mode"]
    #[inline]
    pub fn ethmacptplpen(&mut self) -> _ETHMACPTPLPENW {
        _ETHMACPTPLPENW { w: self }
    }
    #[doc = "Bit 29 - USB OTG HS clock enable during Sleep mode"]
    #[inline]
    pub fn otghslpen(&mut self) -> _OTGHSLPENW {
        _OTGHSLPENW { w: self }
    }
    #[doc = "Bit 30 - USB OTG HS ULPI clock enable during Sleep mode"]
    #[inline]
    pub fn otghsulpilpen(&mut self) -> _OTGHSULPILPENW {
        _OTGHSULPILPENW { w: self }
    }
}
