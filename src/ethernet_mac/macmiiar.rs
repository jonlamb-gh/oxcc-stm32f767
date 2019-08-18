#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACMIIAR {
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
#[doc = "Possible values of the field `MB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBR {
    #[doc = "This bit is set to 1 by the application to indicate that a read or write access is in progress"]
    BUSY,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MBR {
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
            MBR::BUSY => true,
            MBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MBR {
        match value {
            true => MBR::BUSY,
            i => MBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == MBR::BUSY
    }
}
#[doc = "Possible values of the field `MW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWR {
    #[doc = "Read operation"]
    READ,
    #[doc = "Write operation"]
    WRITE,
}
impl MWR {
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
            MWR::READ => false,
            MWR::WRITE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MWR {
        match value {
            false => MWR::READ,
            true => MWR::WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == MWR::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == MWR::WRITE
    }
}
#[doc = "Possible values of the field `CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRR {
    #[doc = "60-100MHz HCLK/42"]
    CR_60_100,
    #[doc = "100-150 MHz HCLK/62"]
    CR_100_150,
    #[doc = "20-35MHz HCLK/16"]
    CR_20_35,
    #[doc = "35-60MHz HCLK/16"]
    CR_35_60,
    #[doc = "150-168MHz HCLK/102"]
    CR_150_168,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRR::CR_60_100 => 0,
            CRR::CR_100_150 => 0x01,
            CRR::CR_20_35 => 0x02,
            CRR::CR_35_60 => 0x03,
            CRR::CR_150_168 => 0x04,
            CRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRR {
        match value {
            0 => CRR::CR_60_100,
            1 => CRR::CR_100_150,
            2 => CRR::CR_20_35,
            3 => CRR::CR_35_60,
            4 => CRR::CR_150_168,
            i => CRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CR_60_100`"]
    #[inline]
    pub fn is_cr_60_100(&self) -> bool {
        *self == CRR::CR_60_100
    }
    #[doc = "Checks if the value of the field is `CR_100_150`"]
    #[inline]
    pub fn is_cr_100_150(&self) -> bool {
        *self == CRR::CR_100_150
    }
    #[doc = "Checks if the value of the field is `CR_20_35`"]
    #[inline]
    pub fn is_cr_20_35(&self) -> bool {
        *self == CRR::CR_20_35
    }
    #[doc = "Checks if the value of the field is `CR_35_60`"]
    #[inline]
    pub fn is_cr_35_60(&self) -> bool {
        *self == CRR::CR_35_60
    }
    #[doc = "Checks if the value of the field is `CR_150_168`"]
    #[inline]
    pub fn is_cr_150_168(&self) -> bool {
        *self == CRR::CR_150_168
    }
}
#[doc = r" Value of the field"]
pub struct MRR {
    bits: u8,
}
impl MRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PAR {
    bits: u8,
}
impl PAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MB`"]
pub enum MBW {
    #[doc = "This bit is set to 1 by the application to indicate that a read or write access is in progress"]
    BUSY,
}
impl MBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MBW::BUSY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MBW<'a> {
    w: &'a mut W,
}
impl<'a> _MBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is set to 1 by the application to indicate that a read or write access is in progress"]
    #[inline]
    pub fn busy(self) -> &'a mut W {
        self.variant(MBW::BUSY)
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
#[doc = "Values that can be written to the field `MW`"]
pub enum MWW {
    #[doc = "Read operation"]
    READ,
    #[doc = "Write operation"]
    WRITE,
}
impl MWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MWW::READ => false,
            MWW::WRITE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MWW<'a> {
    w: &'a mut W,
}
impl<'a> _MWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read operation"]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(MWW::READ)
    }
    #[doc = "Write operation"]
    #[inline]
    pub fn write(self) -> &'a mut W {
        self.variant(MWW::WRITE)
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
#[doc = "Values that can be written to the field `CR`"]
pub enum CRW {
    #[doc = "60-100MHz HCLK/42"]
    CR_60_100,
    #[doc = "100-150 MHz HCLK/62"]
    CR_100_150,
    #[doc = "20-35MHz HCLK/16"]
    CR_20_35,
    #[doc = "35-60MHz HCLK/16"]
    CR_35_60,
    #[doc = "150-168MHz HCLK/102"]
    CR_150_168,
}
impl CRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRW::CR_60_100 => 0,
            CRW::CR_100_150 => 1,
            CRW::CR_20_35 => 2,
            CRW::CR_35_60 => 3,
            CRW::CR_150_168 => 4,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "60-100MHz HCLK/42"]
    #[inline]
    pub fn cr_60_100(self) -> &'a mut W {
        self.variant(CRW::CR_60_100)
    }
    #[doc = "100-150 MHz HCLK/62"]
    #[inline]
    pub fn cr_100_150(self) -> &'a mut W {
        self.variant(CRW::CR_100_150)
    }
    #[doc = "20-35MHz HCLK/16"]
    #[inline]
    pub fn cr_20_35(self) -> &'a mut W {
        self.variant(CRW::CR_20_35)
    }
    #[doc = "35-60MHz HCLK/16"]
    #[inline]
    pub fn cr_35_60(self) -> &'a mut W {
        self.variant(CRW::CR_35_60)
    }
    #[doc = "150-168MHz HCLK/102"]
    #[inline]
    pub fn cr_150_168(self) -> &'a mut W {
        self.variant(CRW::CR_150_168)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MRW<'a> {
    w: &'a mut W,
}
impl<'a> _MRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAW<'a> {
    w: &'a mut W,
}
impl<'a> _PAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
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
    #[doc = "Bit 0 - MB"]
    #[inline]
    pub fn mb(&self) -> MBR {
        MBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - MW"]
    #[inline]
    pub fn mw(&self) -> MWR {
        MWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:4 - CR"]
    #[inline]
    pub fn cr(&self) -> CRR {
        CRR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:10 - MR"]
    #[inline]
    pub fn mr(&self) -> MRR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MRR { bits }
    }
    #[doc = "Bits 11:15 - PA"]
    #[inline]
    pub fn pa(&self) -> PAR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAR { bits }
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
    #[doc = "Bit 0 - MB"]
    #[inline]
    pub fn mb(&mut self) -> _MBW {
        _MBW { w: self }
    }
    #[doc = "Bit 1 - MW"]
    #[inline]
    pub fn mw(&mut self) -> _MWW {
        _MWW { w: self }
    }
    #[doc = "Bits 2:4 - CR"]
    #[inline]
    pub fn cr(&mut self) -> _CRW {
        _CRW { w: self }
    }
    #[doc = "Bits 6:10 - MR"]
    #[inline]
    pub fn mr(&mut self) -> _MRW {
        _MRW { w: self }
    }
    #[doc = "Bits 11:15 - PA"]
    #[inline]
    pub fn pa(&mut self) -> _PAW {
        _PAW { w: self }
    }
}
