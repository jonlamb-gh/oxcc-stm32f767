#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `TSVREFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSVREFER {
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    DISABLED,
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    ENABLED,
}
impl TSVREFER {
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
            TSVREFER::DISABLED => false,
            TSVREFER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSVREFER {
        match value {
            false => TSVREFER::DISABLED,
            true => TSVREFER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREFER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREFER::ENABLED
    }
}
#[doc = "Possible values of the field `VBATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATER {
    #[doc = "V_BAT channel disabled"]
    DISABLED,
    #[doc = "V_BAT channel enabled"]
    ENABLED,
}
impl VBATER {
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
            VBATER::DISABLED => false,
            VBATER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBATER {
        match value {
            false => VBATER::DISABLED,
            true => VBATER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == VBATER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == VBATER::ENABLED
    }
}
#[doc = "Possible values of the field `ADCPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPRER {
    #[doc = "PCLK2 divided by 2"]
    DIV2,
    #[doc = "PCLK2 divided by 4"]
    DIV4,
    #[doc = "PCLK2 divided by 6"]
    DIV6,
    #[doc = "PCLK2 divided by 8"]
    DIV8,
}
impl ADCPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCPRER::DIV2 => 0,
            ADCPRER::DIV4 => 0x01,
            ADCPRER::DIV6 => 0x02,
            ADCPRER::DIV8 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCPRER {
        match value {
            0 => ADCPRER::DIV2,
            1 => ADCPRER::DIV4,
            2 => ADCPRER::DIV6,
            3 => ADCPRER::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == ADCPRER::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == ADCPRER::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline]
    pub fn is_div6(&self) -> bool {
        *self == ADCPRER::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == ADCPRER::DIV8
    }
}
#[doc = "Possible values of the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR {
    #[doc = "DMA mode disabled"]
    DISABLED,
    #[doc = "DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    MODE1,
    #[doc = "DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    MODE2,
    #[doc = "DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    MODE3,
}
impl DMAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAR::DISABLED => 0,
            DMAR::MODE1 => 0x01,
            DMAR::MODE2 => 0x02,
            DMAR::MODE3 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAR {
        match value {
            0 => DMAR::DISABLED,
            1 => DMAR::MODE1,
            2 => DMAR::MODE2,
            3 => DMAR::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == DMAR::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline]
    pub fn is_mode2(&self) -> bool {
        *self == DMAR::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline]
    pub fn is_mode3(&self) -> bool {
        *self == DMAR::MODE3
    }
}
#[doc = "Possible values of the field `DDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDSR {
    #[doc = "No new DMA request is issued after the last transfer"]
    SINGLE,
    #[doc = "DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    CONTINUOUS,
}
impl DDSR {
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
            DDSR::SINGLE => false,
            DDSR::CONTINUOUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DDSR {
        match value {
            false => DDSR::SINGLE,
            true => DDSR::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == DDSR::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == DDSR::CONTINUOUS
    }
}
#[doc = r" Value of the field"]
pub struct DELAYR {
    bits: u8,
}
impl DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MULTI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTIR {
    #[doc = "All the ADCs independent: independent mode"]
    INDEPENDENT,
    #[doc = "Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    DUALRJ,
    #[doc = "Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    DUALRA,
    #[doc = "Dual ADC1 and ADC2, injected simultaneous mode only"]
    DUALJ,
    #[doc = "Dual ADC1 and ADC2, regular simultaneous mode only"]
    DUALR,
    #[doc = "Dual ADC1 and ADC2, interleaved mode only"]
    DUALI,
    #[doc = "Dual ADC1 and ADC2, alternate trigger mode only"]
    DUALA,
    #[doc = "Triple ADC, regular and injected simultaneous mode"]
    TRIPLERJ,
    #[doc = "Triple ADC, regular and alternate trigger mode"]
    TRIPLERA,
    #[doc = "Triple ADC, injected simultaneous mode only"]
    TRIPLEJ,
    #[doc = "Triple ADC, regular simultaneous mode only"]
    TRIPLER,
    #[doc = "Triple ADC, interleaved mode only"]
    TRIPLEI,
    #[doc = "Triple ADC, alternate trigger mode only"]
    TRIPLEA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MULTIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MULTIR::INDEPENDENT => 0,
            MULTIR::DUALRJ => 0x01,
            MULTIR::DUALRA => 0x02,
            MULTIR::DUALJ => 0x05,
            MULTIR::DUALR => 0x06,
            MULTIR::DUALI => 0x07,
            MULTIR::DUALA => 0x09,
            MULTIR::TRIPLERJ => 0x11,
            MULTIR::TRIPLERA => 0x12,
            MULTIR::TRIPLEJ => 0x15,
            MULTIR::TRIPLER => 0x16,
            MULTIR::TRIPLEI => 0x17,
            MULTIR::TRIPLEA => 0x18,
            MULTIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MULTIR {
        match value {
            0 => MULTIR::INDEPENDENT,
            1 => MULTIR::DUALRJ,
            2 => MULTIR::DUALRA,
            5 => MULTIR::DUALJ,
            6 => MULTIR::DUALR,
            7 => MULTIR::DUALI,
            9 => MULTIR::DUALA,
            17 => MULTIR::TRIPLERJ,
            18 => MULTIR::TRIPLERA,
            21 => MULTIR::TRIPLEJ,
            22 => MULTIR::TRIPLER,
            23 => MULTIR::TRIPLEI,
            24 => MULTIR::TRIPLEA,
            i => MULTIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == MULTIR::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `DUALRJ`"]
    #[inline]
    pub fn is_dual_rj(&self) -> bool {
        *self == MULTIR::DUALRJ
    }
    #[doc = "Checks if the value of the field is `DUALRA`"]
    #[inline]
    pub fn is_dual_ra(&self) -> bool {
        *self == MULTIR::DUALRA
    }
    #[doc = "Checks if the value of the field is `DUALJ`"]
    #[inline]
    pub fn is_dual_j(&self) -> bool {
        *self == MULTIR::DUALJ
    }
    #[doc = "Checks if the value of the field is `DUALR`"]
    #[inline]
    pub fn is_dual_r(&self) -> bool {
        *self == MULTIR::DUALR
    }
    #[doc = "Checks if the value of the field is `DUALI`"]
    #[inline]
    pub fn is_dual_i(&self) -> bool {
        *self == MULTIR::DUALI
    }
    #[doc = "Checks if the value of the field is `DUALA`"]
    #[inline]
    pub fn is_dual_a(&self) -> bool {
        *self == MULTIR::DUALA
    }
    #[doc = "Checks if the value of the field is `TRIPLERJ`"]
    #[inline]
    pub fn is_triple_rj(&self) -> bool {
        *self == MULTIR::TRIPLERJ
    }
    #[doc = "Checks if the value of the field is `TRIPLERA`"]
    #[inline]
    pub fn is_triple_ra(&self) -> bool {
        *self == MULTIR::TRIPLERA
    }
    #[doc = "Checks if the value of the field is `TRIPLEJ`"]
    #[inline]
    pub fn is_triple_j(&self) -> bool {
        *self == MULTIR::TRIPLEJ
    }
    #[doc = "Checks if the value of the field is `TRIPLER`"]
    #[inline]
    pub fn is_triple_r(&self) -> bool {
        *self == MULTIR::TRIPLER
    }
    #[doc = "Checks if the value of the field is `TRIPLEI`"]
    #[inline]
    pub fn is_triple_i(&self) -> bool {
        *self == MULTIR::TRIPLEI
    }
    #[doc = "Checks if the value of the field is `TRIPLEA`"]
    #[inline]
    pub fn is_triple_a(&self) -> bool {
        *self == MULTIR::TRIPLEA
    }
}
#[doc = "Values that can be written to the field `TSVREFE`"]
pub enum TSVREFEW {
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    DISABLED,
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    ENABLED,
}
impl TSVREFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSVREFEW::DISABLED => false,
            TSVREFEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSVREFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSVREFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSVREFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSVREFEW::DISABLED)
    }
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSVREFEW::ENABLED)
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
#[doc = "Values that can be written to the field `VBATE`"]
pub enum VBATEW {
    #[doc = "V_BAT channel disabled"]
    DISABLED,
    #[doc = "V_BAT channel enabled"]
    ENABLED,
}
impl VBATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBATEW::DISABLED => false,
            VBATEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBATEW<'a> {
    w: &'a mut W,
}
impl<'a> _VBATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "V_BAT channel disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATEW::DISABLED)
    }
    #[doc = "V_BAT channel enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATEW::ENABLED)
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
#[doc = "Values that can be written to the field `ADCPRE`"]
pub enum ADCPREW {
    #[doc = "PCLK2 divided by 2"]
    DIV2,
    #[doc = "PCLK2 divided by 4"]
    DIV4,
    #[doc = "PCLK2 divided by 6"]
    DIV6,
    #[doc = "PCLK2 divided by 8"]
    DIV8,
}
impl ADCPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCPREW::DIV2 => 0,
            ADCPREW::DIV4 => 1,
            ADCPREW::DIV6 => 2,
            ADCPREW::DIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCPREW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCPREW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PCLK2 divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(ADCPREW::DIV2)
    }
    #[doc = "PCLK2 divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(ADCPREW::DIV4)
    }
    #[doc = "PCLK2 divided by 6"]
    #[inline]
    pub fn div6(self) -> &'a mut W {
        self.variant(ADCPREW::DIV6)
    }
    #[doc = "PCLK2 divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(ADCPREW::DIV8)
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
#[doc = "Values that can be written to the field `DMA`"]
pub enum DMAW {
    #[doc = "DMA mode disabled"]
    DISABLED,
    #[doc = "DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    MODE1,
    #[doc = "DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    MODE2,
    #[doc = "DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    MODE3,
}
impl DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAW::DISABLED => 0,
            DMAW::MODE1 => 1,
            DMAW::MODE2 => 2,
            DMAW::MODE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DMA mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAW::DISABLED)
    }
    #[doc = "DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    #[inline]
    pub fn mode1(self) -> &'a mut W {
        self.variant(DMAW::MODE1)
    }
    #[doc = "DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    #[inline]
    pub fn mode2(self) -> &'a mut W {
        self.variant(DMAW::MODE2)
    }
    #[doc = "DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    #[inline]
    pub fn mode3(self) -> &'a mut W {
        self.variant(DMAW::MODE3)
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
#[doc = "Values that can be written to the field `DDS`"]
pub enum DDSW {
    #[doc = "No new DMA request is issued after the last transfer"]
    SINGLE,
    #[doc = "DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    CONTINUOUS,
}
impl DDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DDSW::SINGLE => false,
            DDSW::CONTINUOUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DDSW<'a> {
    w: &'a mut W,
}
impl<'a> _DDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DDSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No new DMA request is issued after the last transfer"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(DDSW::SINGLE)
    }
    #[doc = "DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DDSW::CONTINUOUS)
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
#[doc = r" Proxy"]
pub struct _DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MULTI`"]
pub enum MULTIW {
    #[doc = "All the ADCs independent: independent mode"]
    INDEPENDENT,
    #[doc = "Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    DUALRJ,
    #[doc = "Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    DUALRA,
    #[doc = "Dual ADC1 and ADC2, injected simultaneous mode only"]
    DUALJ,
    #[doc = "Dual ADC1 and ADC2, regular simultaneous mode only"]
    DUALR,
    #[doc = "Dual ADC1 and ADC2, interleaved mode only"]
    DUALI,
    #[doc = "Dual ADC1 and ADC2, alternate trigger mode only"]
    DUALA,
    #[doc = "Triple ADC, regular and injected simultaneous mode"]
    TRIPLERJ,
    #[doc = "Triple ADC, regular and alternate trigger mode"]
    TRIPLERA,
    #[doc = "Triple ADC, injected simultaneous mode only"]
    TRIPLEJ,
    #[doc = "Triple ADC, regular simultaneous mode only"]
    TRIPLER,
    #[doc = "Triple ADC, interleaved mode only"]
    TRIPLEI,
    #[doc = "Triple ADC, alternate trigger mode only"]
    TRIPLEA,
}
impl MULTIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MULTIW::INDEPENDENT => 0,
            MULTIW::DUALRJ => 1,
            MULTIW::DUALRA => 2,
            MULTIW::DUALJ => 5,
            MULTIW::DUALR => 6,
            MULTIW::DUALI => 7,
            MULTIW::DUALA => 9,
            MULTIW::TRIPLERJ => 17,
            MULTIW::TRIPLERA => 18,
            MULTIW::TRIPLEJ => 21,
            MULTIW::TRIPLER => 22,
            MULTIW::TRIPLEI => 23,
            MULTIW::TRIPLEA => 24,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MULTIW<'a> {
    w: &'a mut W,
}
impl<'a> _MULTIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MULTIW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "All the ADCs independent: independent mode"]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(MULTIW::INDEPENDENT)
    }
    #[doc = "Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    #[inline]
    pub fn dual_rj(self) -> &'a mut W {
        self.variant(MULTIW::DUALRJ)
    }
    #[doc = "Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    #[inline]
    pub fn dual_ra(self) -> &'a mut W {
        self.variant(MULTIW::DUALRA)
    }
    #[doc = "Dual ADC1 and ADC2, injected simultaneous mode only"]
    #[inline]
    pub fn dual_j(self) -> &'a mut W {
        self.variant(MULTIW::DUALJ)
    }
    #[doc = "Dual ADC1 and ADC2, regular simultaneous mode only"]
    #[inline]
    pub fn dual_r(self) -> &'a mut W {
        self.variant(MULTIW::DUALR)
    }
    #[doc = "Dual ADC1 and ADC2, interleaved mode only"]
    #[inline]
    pub fn dual_i(self) -> &'a mut W {
        self.variant(MULTIW::DUALI)
    }
    #[doc = "Dual ADC1 and ADC2, alternate trigger mode only"]
    #[inline]
    pub fn dual_a(self) -> &'a mut W {
        self.variant(MULTIW::DUALA)
    }
    #[doc = "Triple ADC, regular and injected simultaneous mode"]
    #[inline]
    pub fn triple_rj(self) -> &'a mut W {
        self.variant(MULTIW::TRIPLERJ)
    }
    #[doc = "Triple ADC, regular and alternate trigger mode"]
    #[inline]
    pub fn triple_ra(self) -> &'a mut W {
        self.variant(MULTIW::TRIPLERA)
    }
    #[doc = "Triple ADC, injected simultaneous mode only"]
    #[inline]
    pub fn triple_j(self) -> &'a mut W {
        self.variant(MULTIW::TRIPLEJ)
    }
    #[doc = "Triple ADC, regular simultaneous mode only"]
    #[inline]
    pub fn triple_r(self) -> &'a mut W {
        self.variant(MULTIW::TRIPLER)
    }
    #[doc = "Triple ADC, interleaved mode only"]
    #[inline]
    pub fn triple_i(self) -> &'a mut W {
        self.variant(MULTIW::TRIPLEI)
    }
    #[doc = "Triple ADC, alternate trigger mode only"]
    #[inline]
    pub fn triple_a(self) -> &'a mut W {
        self.variant(MULTIW::TRIPLEA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
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
    #[doc = "Bit 23 - Temperature sensor and V_REFINT enable"]
    #[inline]
    pub fn tsvrefe(&self) -> TSVREFER {
        TSVREFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - V_BAT enable"]
    #[inline]
    pub fn vbate(&self) -> VBATER {
        VBATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - ADC prescaler"]
    #[inline]
    pub fn adcpre(&self) -> ADCPRER {
        ADCPRER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline]
    pub fn dma(&self) -> DMAR {
        DMAR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - DMA disable selection (for multi-ADC mode)"]
    #[inline]
    pub fn dds(&self) -> DDSR {
        DDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline]
    pub fn delay(&self) -> DELAYR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DELAYR { bits }
    }
    #[doc = "Bits 0:3 - Multi ADC mode selection"]
    #[inline]
    pub fn multi(&self) -> MULTIR {
        MULTIR::_from({
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 23 - Temperature sensor and V_REFINT enable"]
    #[inline]
    pub fn tsvrefe(&mut self) -> _TSVREFEW {
        _TSVREFEW { w: self }
    }
    #[doc = "Bit 22 - V_BAT enable"]
    #[inline]
    pub fn vbate(&mut self) -> _VBATEW {
        _VBATEW { w: self }
    }
    #[doc = "Bits 16:17 - ADC prescaler"]
    #[inline]
    pub fn adcpre(&mut self) -> _ADCPREW {
        _ADCPREW { w: self }
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline]
    pub fn dma(&mut self) -> _DMAW {
        _DMAW { w: self }
    }
    #[doc = "Bit 13 - DMA disable selection (for multi-ADC mode)"]
    #[inline]
    pub fn dds(&mut self) -> _DDSW {
        _DDSW { w: self }
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline]
    pub fn delay(&mut self) -> _DELAYW {
        _DELAYW { w: self }
    }
    #[doc = "Bits 0:3 - Multi ADC mode selection"]
    #[inline]
    pub fn multi(&mut self) -> _MULTIW {
        _MULTIW { w: self }
    }
}
