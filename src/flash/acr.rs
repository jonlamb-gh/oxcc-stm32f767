#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACR {
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
#[doc = "Possible values of the field `LATENCY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATENCYR {
    #[doc = "0 wait states"]
    WS0,
    #[doc = "1 wait states"]
    WS1,
    #[doc = "2 wait states"]
    WS2,
    #[doc = "3 wait states"]
    WS3,
    #[doc = "4 wait states"]
    WS4,
    #[doc = "5 wait states"]
    WS5,
    #[doc = "6 wait states"]
    WS6,
    #[doc = "7 wait states"]
    WS7,
    #[doc = "8 wait states"]
    WS8,
    #[doc = "9 wait states"]
    WS9,
    #[doc = "10 wait states"]
    WS10,
    #[doc = "11 wait states"]
    WS11,
    #[doc = "12 wait states"]
    WS12,
    #[doc = "13 wait states"]
    WS13,
    #[doc = "14 wait states"]
    WS14,
    #[doc = "15 wait states"]
    WS15,
}
impl LATENCYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LATENCYR::WS0 => 0,
            LATENCYR::WS1 => 0x01,
            LATENCYR::WS2 => 0x02,
            LATENCYR::WS3 => 0x03,
            LATENCYR::WS4 => 0x04,
            LATENCYR::WS5 => 0x05,
            LATENCYR::WS6 => 0x06,
            LATENCYR::WS7 => 0x07,
            LATENCYR::WS8 => 0x08,
            LATENCYR::WS9 => 0x09,
            LATENCYR::WS10 => 0x0a,
            LATENCYR::WS11 => 0x0b,
            LATENCYR::WS12 => 0x0c,
            LATENCYR::WS13 => 0x0d,
            LATENCYR::WS14 => 0x0e,
            LATENCYR::WS15 => 0x0f,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LATENCYR {
        match value {
            0 => LATENCYR::WS0,
            1 => LATENCYR::WS1,
            2 => LATENCYR::WS2,
            3 => LATENCYR::WS3,
            4 => LATENCYR::WS4,
            5 => LATENCYR::WS5,
            6 => LATENCYR::WS6,
            7 => LATENCYR::WS7,
            8 => LATENCYR::WS8,
            9 => LATENCYR::WS9,
            10 => LATENCYR::WS10,
            11 => LATENCYR::WS11,
            12 => LATENCYR::WS12,
            13 => LATENCYR::WS13,
            14 => LATENCYR::WS14,
            15 => LATENCYR::WS15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCYR::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCYR::WS1
    }
    #[doc = "Checks if the value of the field is `WS2`"]
    #[inline]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCYR::WS2
    }
    #[doc = "Checks if the value of the field is `WS3`"]
    #[inline]
    pub fn is_ws3(&self) -> bool {
        *self == LATENCYR::WS3
    }
    #[doc = "Checks if the value of the field is `WS4`"]
    #[inline]
    pub fn is_ws4(&self) -> bool {
        *self == LATENCYR::WS4
    }
    #[doc = "Checks if the value of the field is `WS5`"]
    #[inline]
    pub fn is_ws5(&self) -> bool {
        *self == LATENCYR::WS5
    }
    #[doc = "Checks if the value of the field is `WS6`"]
    #[inline]
    pub fn is_ws6(&self) -> bool {
        *self == LATENCYR::WS6
    }
    #[doc = "Checks if the value of the field is `WS7`"]
    #[inline]
    pub fn is_ws7(&self) -> bool {
        *self == LATENCYR::WS7
    }
    #[doc = "Checks if the value of the field is `WS8`"]
    #[inline]
    pub fn is_ws8(&self) -> bool {
        *self == LATENCYR::WS8
    }
    #[doc = "Checks if the value of the field is `WS9`"]
    #[inline]
    pub fn is_ws9(&self) -> bool {
        *self == LATENCYR::WS9
    }
    #[doc = "Checks if the value of the field is `WS10`"]
    #[inline]
    pub fn is_ws10(&self) -> bool {
        *self == LATENCYR::WS10
    }
    #[doc = "Checks if the value of the field is `WS11`"]
    #[inline]
    pub fn is_ws11(&self) -> bool {
        *self == LATENCYR::WS11
    }
    #[doc = "Checks if the value of the field is `WS12`"]
    #[inline]
    pub fn is_ws12(&self) -> bool {
        *self == LATENCYR::WS12
    }
    #[doc = "Checks if the value of the field is `WS13`"]
    #[inline]
    pub fn is_ws13(&self) -> bool {
        *self == LATENCYR::WS13
    }
    #[doc = "Checks if the value of the field is `WS14`"]
    #[inline]
    pub fn is_ws14(&self) -> bool {
        *self == LATENCYR::WS14
    }
    #[doc = "Checks if the value of the field is `WS15`"]
    #[inline]
    pub fn is_ws15(&self) -> bool {
        *self == LATENCYR::WS15
    }
}
#[doc = "Possible values of the field `PRFTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTENR {
    #[doc = "Prefetch is disabled"]
    DISABLED,
    #[doc = "Prefetch is enabled"]
    ENABLED,
}
impl PRFTENR {
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
            PRFTENR::DISABLED => false,
            PRFTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRFTENR {
        match value {
            false => PRFTENR::DISABLED,
            true => PRFTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTENR::ENABLED
    }
}
#[doc = "Possible values of the field `ARTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARTENR {
    #[doc = "ART Accelerator is disabled"]
    DISABLED,
    #[doc = "ART Accelerator is enabled"]
    ENABLED,
}
impl ARTENR {
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
            ARTENR::DISABLED => false,
            ARTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARTENR {
        match value {
            false => ARTENR::DISABLED,
            true => ARTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ARTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ARTENR::ENABLED
    }
}
#[doc = "Possible values of the field `ARTRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARTRSTR {
    #[doc = "Accelerator is not reset"]
    NOTRESET,
    #[doc = "Accelerator is reset"]
    RESET,
}
impl ARTRSTR {
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
            ARTRSTR::NOTRESET => false,
            ARTRSTR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARTRSTR {
        match value {
            false => ARTRSTR::NOTRESET,
            true => ARTRSTR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRESET`"]
    #[inline]
    pub fn is_not_reset(&self) -> bool {
        *self == ARTRSTR::NOTRESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == ARTRSTR::RESET
    }
}
#[doc = "Values that can be written to the field `LATENCY`"]
pub enum LATENCYW {
    #[doc = "0 wait states"]
    WS0,
    #[doc = "1 wait states"]
    WS1,
    #[doc = "2 wait states"]
    WS2,
    #[doc = "3 wait states"]
    WS3,
    #[doc = "4 wait states"]
    WS4,
    #[doc = "5 wait states"]
    WS5,
    #[doc = "6 wait states"]
    WS6,
    #[doc = "7 wait states"]
    WS7,
    #[doc = "8 wait states"]
    WS8,
    #[doc = "9 wait states"]
    WS9,
    #[doc = "10 wait states"]
    WS10,
    #[doc = "11 wait states"]
    WS11,
    #[doc = "12 wait states"]
    WS12,
    #[doc = "13 wait states"]
    WS13,
    #[doc = "14 wait states"]
    WS14,
    #[doc = "15 wait states"]
    WS15,
}
impl LATENCYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LATENCYW::WS0 => 0,
            LATENCYW::WS1 => 1,
            LATENCYW::WS2 => 2,
            LATENCYW::WS3 => 3,
            LATENCYW::WS4 => 4,
            LATENCYW::WS5 => 5,
            LATENCYW::WS6 => 6,
            LATENCYW::WS7 => 7,
            LATENCYW::WS8 => 8,
            LATENCYW::WS9 => 9,
            LATENCYW::WS10 => 10,
            LATENCYW::WS11 => 11,
            LATENCYW::WS12 => 12,
            LATENCYW::WS13 => 13,
            LATENCYW::WS14 => 14,
            LATENCYW::WS15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LATENCYW<'a> {
    w: &'a mut W,
}
impl<'a> _LATENCYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LATENCYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0 wait states"]
    #[inline]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCYW::WS0)
    }
    #[doc = "1 wait states"]
    #[inline]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCYW::WS1)
    }
    #[doc = "2 wait states"]
    #[inline]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCYW::WS2)
    }
    #[doc = "3 wait states"]
    #[inline]
    pub fn ws3(self) -> &'a mut W {
        self.variant(LATENCYW::WS3)
    }
    #[doc = "4 wait states"]
    #[inline]
    pub fn ws4(self) -> &'a mut W {
        self.variant(LATENCYW::WS4)
    }
    #[doc = "5 wait states"]
    #[inline]
    pub fn ws5(self) -> &'a mut W {
        self.variant(LATENCYW::WS5)
    }
    #[doc = "6 wait states"]
    #[inline]
    pub fn ws6(self) -> &'a mut W {
        self.variant(LATENCYW::WS6)
    }
    #[doc = "7 wait states"]
    #[inline]
    pub fn ws7(self) -> &'a mut W {
        self.variant(LATENCYW::WS7)
    }
    #[doc = "8 wait states"]
    #[inline]
    pub fn ws8(self) -> &'a mut W {
        self.variant(LATENCYW::WS8)
    }
    #[doc = "9 wait states"]
    #[inline]
    pub fn ws9(self) -> &'a mut W {
        self.variant(LATENCYW::WS9)
    }
    #[doc = "10 wait states"]
    #[inline]
    pub fn ws10(self) -> &'a mut W {
        self.variant(LATENCYW::WS10)
    }
    #[doc = "11 wait states"]
    #[inline]
    pub fn ws11(self) -> &'a mut W {
        self.variant(LATENCYW::WS11)
    }
    #[doc = "12 wait states"]
    #[inline]
    pub fn ws12(self) -> &'a mut W {
        self.variant(LATENCYW::WS12)
    }
    #[doc = "13 wait states"]
    #[inline]
    pub fn ws13(self) -> &'a mut W {
        self.variant(LATENCYW::WS13)
    }
    #[doc = "14 wait states"]
    #[inline]
    pub fn ws14(self) -> &'a mut W {
        self.variant(LATENCYW::WS14)
    }
    #[doc = "15 wait states"]
    #[inline]
    pub fn ws15(self) -> &'a mut W {
        self.variant(LATENCYW::WS15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRFTEN`"]
pub enum PRFTENW {
    #[doc = "Prefetch is disabled"]
    DISABLED,
    #[doc = "Prefetch is enabled"]
    ENABLED,
}
impl PRFTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRFTENW::DISABLED => false,
            PRFTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRFTENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRFTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRFTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetch is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTENW::DISABLED)
    }
    #[doc = "Prefetch is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTENW::ENABLED)
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
#[doc = "Values that can be written to the field `ARTEN`"]
pub enum ARTENW {
    #[doc = "ART Accelerator is disabled"]
    DISABLED,
    #[doc = "ART Accelerator is enabled"]
    ENABLED,
}
impl ARTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARTENW::DISABLED => false,
            ARTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ART Accelerator is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARTENW::DISABLED)
    }
    #[doc = "ART Accelerator is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARTENW::ENABLED)
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
#[doc = "Values that can be written to the field `ARTRST`"]
pub enum ARTRSTW {
    #[doc = "Accelerator is not reset"]
    NOTRESET,
    #[doc = "Accelerator is reset"]
    RESET,
}
impl ARTRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARTRSTW::NOTRESET => false,
            ARTRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ARTRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARTRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accelerator is not reset"]
    #[inline]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(ARTRSTW::NOTRESET)
    }
    #[doc = "Accelerator is reset"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ARTRSTW::RESET)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Latency"]
    #[inline]
    pub fn latency(&self) -> LATENCYR {
        LATENCYR::_from({
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline]
    pub fn prften(&self) -> PRFTENR {
        PRFTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ART Accelerator Enable"]
    #[inline]
    pub fn arten(&self) -> ARTENR {
        ARTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - ART Accelerator reset"]
    #[inline]
    pub fn artrst(&self) -> ARTRSTR {
        ARTRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bits 0:3 - Latency"]
    #[inline]
    pub fn latency(&mut self) -> _LATENCYW {
        _LATENCYW { w: self }
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline]
    pub fn prften(&mut self) -> _PRFTENW {
        _PRFTENW { w: self }
    }
    #[doc = "Bit 9 - ART Accelerator Enable"]
    #[inline]
    pub fn arten(&mut self) -> _ARTENW {
        _ARTENW { w: self }
    }
    #[doc = "Bit 11 - ART Accelerator reset"]
    #[inline]
    pub fn artrst(&mut self) -> _ARTRSTW {
        _ARTRSTW { w: self }
    }
}
