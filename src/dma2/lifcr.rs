#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LIFCR {
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
}
#[doc = "Values that can be written to the field `CTCIF3`"]
pub enum CTCIF3W {
    #[doc = "Clear the corresponding TCIFx flag"]
    CLEAR,
}
impl CTCIF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTCIF3W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTCIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF3`"]
pub enum CHTIF3W {
    #[doc = "Clear the corresponding HTIFx flag"]
    CLEAR,
}
impl CHTIF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHTIF3W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHTIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF3`"]
pub enum CTEIF3W {
    #[doc = "Clear the corresponding TEIFx flag"]
    CLEAR,
}
impl CTEIF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTEIF3W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTEIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CDMEIF3`"]
pub enum CDMEIF3W {
    #[doc = "Clear the corresponding DMEIFx flag"]
    CLEAR,
}
impl CDMEIF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDMEIF3W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDMEIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CDMEIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDMEIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CFEIF3`"]
pub enum CFEIF3W {
    #[doc = "Clear the corresponding CFEIFx flag"]
    CLEAR,
}
impl CFEIF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFEIF3W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFEIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CFEIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFEIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF2`"]
pub type CTCIF2W = CTCIF3W;
#[doc = r" Proxy"]
pub struct _CTCIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF2`"]
pub type CHTIF2W = CHTIF3W;
#[doc = r" Proxy"]
pub struct _CHTIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF3W::CLEAR)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTEIF2`"]
pub type CTEIF2W = CTEIF3W;
#[doc = r" Proxy"]
pub struct _CTEIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CDMEIF2`"]
pub type CDMEIF2W = CDMEIF3W;
#[doc = r" Proxy"]
pub struct _CDMEIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CDMEIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDMEIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CFEIF2`"]
pub type CFEIF2W = CFEIF3W;
#[doc = r" Proxy"]
pub struct _CFEIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CFEIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFEIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF1`"]
pub type CTCIF1W = CTCIF3W;
#[doc = r" Proxy"]
pub struct _CTCIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF1`"]
pub type CHTIF1W = CHTIF3W;
#[doc = r" Proxy"]
pub struct _CHTIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF1`"]
pub type CTEIF1W = CTEIF3W;
#[doc = r" Proxy"]
pub struct _CTEIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CDMEIF1`"]
pub type CDMEIF1W = CDMEIF3W;
#[doc = r" Proxy"]
pub struct _CDMEIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CDMEIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDMEIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CFEIF1`"]
pub type CFEIF1W = CFEIF3W;
#[doc = r" Proxy"]
pub struct _CFEIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CFEIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFEIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF0`"]
pub type CTCIF0W = CTCIF3W;
#[doc = r" Proxy"]
pub struct _CTCIF0W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF0`"]
pub type CHTIF0W = CHTIF3W;
#[doc = r" Proxy"]
pub struct _CHTIF0W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF0`"]
pub type CTEIF0W = CTEIF3W;
#[doc = r" Proxy"]
pub struct _CTEIF0W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CDMEIF0`"]
pub type CDMEIF0W = CDMEIF3W;
#[doc = r" Proxy"]
pub struct _CDMEIF0W<'a> {
    w: &'a mut W,
}
impl<'a> _CDMEIF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDMEIF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF3W::CLEAR)
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
#[doc = "Values that can be written to the field `CFEIF0`"]
pub type CFEIF0W = CFEIF3W;
#[doc = r" Proxy"]
pub struct _CFEIF0W<'a> {
    w: &'a mut W,
}
impl<'a> _CFEIF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFEIF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF3W::CLEAR)
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
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline]
    pub fn ctcif3(&mut self) -> _CTCIF3W {
        _CTCIF3W { w: self }
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline]
    pub fn chtif3(&mut self) -> _CHTIF3W {
        _CHTIF3W { w: self }
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cteif3(&mut self) -> _CTEIF3W {
        _CTEIF3W { w: self }
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cdmeif3(&mut self) -> _CDMEIF3W {
        _CDMEIF3W { w: self }
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cfeif3(&mut self) -> _CFEIF3W {
        _CFEIF3W { w: self }
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline]
    pub fn ctcif2(&mut self) -> _CTCIF2W {
        _CTCIF2W { w: self }
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline]
    pub fn chtif2(&mut self) -> _CHTIF2W {
        _CHTIF2W { w: self }
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cteif2(&mut self) -> _CTEIF2W {
        _CTEIF2W { w: self }
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cdmeif2(&mut self) -> _CDMEIF2W {
        _CDMEIF2W { w: self }
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cfeif2(&mut self) -> _CFEIF2W {
        _CFEIF2W { w: self }
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline]
    pub fn ctcif1(&mut self) -> _CTCIF1W {
        _CTCIF1W { w: self }
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline]
    pub fn chtif1(&mut self) -> _CHTIF1W {
        _CHTIF1W { w: self }
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cteif1(&mut self) -> _CTEIF1W {
        _CTEIF1W { w: self }
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cdmeif1(&mut self) -> _CDMEIF1W {
        _CDMEIF1W { w: self }
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cfeif1(&mut self) -> _CFEIF1W {
        _CFEIF1W { w: self }
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline]
    pub fn ctcif0(&mut self) -> _CTCIF0W {
        _CTCIF0W { w: self }
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline]
    pub fn chtif0(&mut self) -> _CHTIF0W {
        _CHTIF0W { w: self }
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cteif0(&mut self) -> _CTEIF0W {
        _CTEIF0W { w: self }
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cdmeif0(&mut self) -> _CDMEIF0W {
        _CDMEIF0W { w: self }
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline]
    pub fn cfeif0(&mut self) -> _CFEIF0W {
        _CFEIF0W { w: self }
    }
}
