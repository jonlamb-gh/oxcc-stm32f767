#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMPR2 {
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
#[doc = "Possible values of the field `SMP9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMP9R {
    #[doc = "3 cycles"]
    CYCLES3,
    #[doc = "15 cycles"]
    CYCLES15,
    #[doc = "28 cycles"]
    CYCLES28,
    #[doc = "56 cycles"]
    CYCLES56,
    #[doc = "84 cycles"]
    CYCLES84,
    #[doc = "112 cycles"]
    CYCLES112,
    #[doc = "144 cycles"]
    CYCLES144,
    #[doc = "480 cycles"]
    CYCLES480,
}
impl SMP9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMP9R::CYCLES3 => 0,
            SMP9R::CYCLES15 => 0x01,
            SMP9R::CYCLES28 => 0x02,
            SMP9R::CYCLES56 => 0x03,
            SMP9R::CYCLES84 => 0x04,
            SMP9R::CYCLES112 => 0x05,
            SMP9R::CYCLES144 => 0x06,
            SMP9R::CYCLES480 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMP9R {
        match value {
            0 => SMP9R::CYCLES3,
            1 => SMP9R::CYCLES15,
            2 => SMP9R::CYCLES28,
            3 => SMP9R::CYCLES56,
            4 => SMP9R::CYCLES84,
            5 => SMP9R::CYCLES112,
            6 => SMP9R::CYCLES144,
            7 => SMP9R::CYCLES480,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES3`"]
    #[inline]
    pub fn is_cycles3(&self) -> bool {
        *self == SMP9R::CYCLES3
    }
    #[doc = "Checks if the value of the field is `CYCLES15`"]
    #[inline]
    pub fn is_cycles15(&self) -> bool {
        *self == SMP9R::CYCLES15
    }
    #[doc = "Checks if the value of the field is `CYCLES28`"]
    #[inline]
    pub fn is_cycles28(&self) -> bool {
        *self == SMP9R::CYCLES28
    }
    #[doc = "Checks if the value of the field is `CYCLES56`"]
    #[inline]
    pub fn is_cycles56(&self) -> bool {
        *self == SMP9R::CYCLES56
    }
    #[doc = "Checks if the value of the field is `CYCLES84`"]
    #[inline]
    pub fn is_cycles84(&self) -> bool {
        *self == SMP9R::CYCLES84
    }
    #[doc = "Checks if the value of the field is `CYCLES112`"]
    #[inline]
    pub fn is_cycles112(&self) -> bool {
        *self == SMP9R::CYCLES112
    }
    #[doc = "Checks if the value of the field is `CYCLES144`"]
    #[inline]
    pub fn is_cycles144(&self) -> bool {
        *self == SMP9R::CYCLES144
    }
    #[doc = "Checks if the value of the field is `CYCLES480`"]
    #[inline]
    pub fn is_cycles480(&self) -> bool {
        *self == SMP9R::CYCLES480
    }
}
#[doc = r" Value of the field"]
pub struct SMP8R {
    bits: u8,
}
impl SMP8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP7R {
    bits: u8,
}
impl SMP7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP6R {
    bits: u8,
}
impl SMP6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP5R {
    bits: u8,
}
impl SMP5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP4R {
    bits: u8,
}
impl SMP4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP3R {
    bits: u8,
}
impl SMP3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP2R {
    bits: u8,
}
impl SMP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP1R {
    bits: u8,
}
impl SMP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMP0R {
    bits: u8,
}
impl SMP0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SMP9`"]
pub enum SMP9W {
    #[doc = "3 cycles"]
    CYCLES3,
    #[doc = "15 cycles"]
    CYCLES15,
    #[doc = "28 cycles"]
    CYCLES28,
    #[doc = "56 cycles"]
    CYCLES56,
    #[doc = "84 cycles"]
    CYCLES84,
    #[doc = "112 cycles"]
    CYCLES112,
    #[doc = "144 cycles"]
    CYCLES144,
    #[doc = "480 cycles"]
    CYCLES480,
}
impl SMP9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMP9W::CYCLES3 => 0,
            SMP9W::CYCLES15 => 1,
            SMP9W::CYCLES28 => 2,
            SMP9W::CYCLES56 => 3,
            SMP9W::CYCLES84 => 4,
            SMP9W::CYCLES112 => 5,
            SMP9W::CYCLES144 => 6,
            SMP9W::CYCLES480 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMP9W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMP9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "3 cycles"]
    #[inline]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP9W::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP9W::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP9W::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP9W::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP9W::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP9W::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP9W::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP9W::CYCLES480)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP8W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP7W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP6W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP5W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP4W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP3W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP2W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMP0W<'a> {
    w: &'a mut W,
}
impl<'a> _SMP0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
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
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline]
    pub fn smp9(&self) -> SMP9R {
        SMP9R::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline]
    pub fn smp8(&self) -> SMP8R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP8R { bits }
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline]
    pub fn smp7(&self) -> SMP7R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP7R { bits }
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline]
    pub fn smp6(&self) -> SMP6R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP6R { bits }
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline]
    pub fn smp5(&self) -> SMP5R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP5R { bits }
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline]
    pub fn smp4(&self) -> SMP4R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP4R { bits }
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline]
    pub fn smp3(&self) -> SMP3R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP3R { bits }
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline]
    pub fn smp2(&self) -> SMP2R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP2R { bits }
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline]
    pub fn smp1(&self) -> SMP1R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP1R { bits }
    }
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline]
    pub fn smp0(&self) -> SMP0R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMP0R { bits }
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
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline]
    pub fn smp9(&mut self) -> _SMP9W {
        _SMP9W { w: self }
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline]
    pub fn smp8(&mut self) -> _SMP8W {
        _SMP8W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline]
    pub fn smp7(&mut self) -> _SMP7W {
        _SMP7W { w: self }
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline]
    pub fn smp6(&mut self) -> _SMP6W {
        _SMP6W { w: self }
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline]
    pub fn smp5(&mut self) -> _SMP5W {
        _SMP5W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline]
    pub fn smp4(&mut self) -> _SMP4W {
        _SMP4W { w: self }
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline]
    pub fn smp3(&mut self) -> _SMP3W {
        _SMP3W { w: self }
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline]
    pub fn smp2(&mut self) -> _SMP2W {
        _SMP2W { w: self }
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline]
    pub fn smp1(&mut self) -> _SMP1W {
        _SMP1W { w: self }
    }
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline]
    pub fn smp0(&mut self) -> _SMP0W {
        _SMP0W { w: self }
    }
}
