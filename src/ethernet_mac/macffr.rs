#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACFFR {
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
#[doc = "Possible values of the field `PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMR {
    #[doc = "Normal address filtering"]
    DISABLED,
    #[doc = "Address filters pass all incoming frames regardless of their destination or source address"]
    ENABLED,
}
impl PMR {
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
            PMR::DISABLED => false,
            PMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMR {
        match value {
            false => PMR::DISABLED,
            true => PMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PMR::ENABLED
    }
}
#[doc = "Possible values of the field `HU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HUR {
    #[doc = "MAC performs a perfect destination address filtering for unicast frames"]
    PERFECT,
    #[doc = "MAC performs destination address filtering of received unicast frames according to the hash table"]
    HASH,
}
impl HUR {
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
            HUR::PERFECT => false,
            HUR::HASH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HUR {
        match value {
            false => HUR::PERFECT,
            true => HUR::HASH,
        }
    }
    #[doc = "Checks if the value of the field is `PERFECT`"]
    #[inline]
    pub fn is_perfect(&self) -> bool {
        *self == HUR::PERFECT
    }
    #[doc = "Checks if the value of the field is `HASH`"]
    #[inline]
    pub fn is_hash(&self) -> bool {
        *self == HUR::HASH
    }
}
#[doc = "Possible values of the field `HM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HMR {
    #[doc = "MAC performs a perfect destination address filtering for multicast frames"]
    PERFECT,
    #[doc = "MAC performs destination address filtering of received multicast frames according to the hash table"]
    HASH,
}
impl HMR {
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
            HMR::PERFECT => false,
            HMR::HASH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HMR {
        match value {
            false => HMR::PERFECT,
            true => HMR::HASH,
        }
    }
    #[doc = "Checks if the value of the field is `PERFECT`"]
    #[inline]
    pub fn is_perfect(&self) -> bool {
        *self == HMR::PERFECT
    }
    #[doc = "Checks if the value of the field is `HASH`"]
    #[inline]
    pub fn is_hash(&self) -> bool {
        *self == HMR::HASH
    }
}
#[doc = "Possible values of the field `DAIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAIFR {
    #[doc = "Normal filtering of frames"]
    NORMAL,
    #[doc = "Address check block operates in inverse filtering mode for the DA address comparison"]
    INVERT,
}
impl DAIFR {
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
            DAIFR::NORMAL => false,
            DAIFR::INVERT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAIFR {
        match value {
            false => DAIFR::NORMAL,
            true => DAIFR::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == DAIFR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline]
    pub fn is_invert(&self) -> bool {
        *self == DAIFR::INVERT
    }
}
#[doc = "Possible values of the field `RAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMR {
    #[doc = "Filtering of multicast frames depends on HM"]
    DISABLED,
    #[doc = "All received frames with a multicast destination address are passed"]
    ENABLED,
}
impl RAMR {
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
            RAMR::DISABLED => false,
            RAMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAMR {
        match value {
            false => RAMR::DISABLED,
            true => RAMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RAMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RAMR::ENABLED
    }
}
#[doc = "Possible values of the field `BFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFDR {
    #[doc = "Address filters pass all received broadcast frames"]
    ENABLED,
    #[doc = "Address filters filter all incoming broadcast frames"]
    DISABLED,
}
impl BFDR {
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
            BFDR::ENABLED => false,
            BFDR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFDR {
        match value {
            false => BFDR::ENABLED,
            true => BFDR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BFDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BFDR::DISABLED
    }
}
#[doc = "Possible values of the field `PCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCFR {
    #[doc = "MAC prevents all control frames from reaching the application"]
    PREVENTALL,
    #[doc = "MAC forwards all control frames to application except Pause"]
    FORWARDALLEXCEPTPAUSE,
    #[doc = "MAC forwards all control frames to application even if they fail the address filter"]
    FORWARDALL,
    #[doc = "MAC forwards control frames that pass the address filter"]
    FORWARDALLFILTERED,
}
impl PCFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCFR::PREVENTALL => 0,
            PCFR::FORWARDALLEXCEPTPAUSE => 0x01,
            PCFR::FORWARDALL => 0x02,
            PCFR::FORWARDALLFILTERED => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCFR {
        match value {
            0 => PCFR::PREVENTALL,
            1 => PCFR::FORWARDALLEXCEPTPAUSE,
            2 => PCFR::FORWARDALL,
            3 => PCFR::FORWARDALLFILTERED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PREVENTALL`"]
    #[inline]
    pub fn is_prevent_all(&self) -> bool {
        *self == PCFR::PREVENTALL
    }
    #[doc = "Checks if the value of the field is `FORWARDALLEXCEPTPAUSE`"]
    #[inline]
    pub fn is_forward_all_except_pause(&self) -> bool {
        *self == PCFR::FORWARDALLEXCEPTPAUSE
    }
    #[doc = "Checks if the value of the field is `FORWARDALL`"]
    #[inline]
    pub fn is_forward_all(&self) -> bool {
        *self == PCFR::FORWARDALL
    }
    #[doc = "Checks if the value of the field is `FORWARDALLFILTERED`"]
    #[inline]
    pub fn is_forward_all_filtered(&self) -> bool {
        *self == PCFR::FORWARDALLFILTERED
    }
}
#[doc = "Possible values of the field `SAIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAIFR {
    #[doc = "Source address filter operates normally"]
    NORMAL,
    #[doc = "Source address filter operation inverted"]
    INVERT,
}
impl SAIFR {
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
            SAIFR::NORMAL => false,
            SAIFR::INVERT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAIFR {
        match value {
            false => SAIFR::NORMAL,
            true => SAIFR::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == SAIFR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline]
    pub fn is_invert(&self) -> bool {
        *self == SAIFR::INVERT
    }
}
#[doc = "Possible values of the field `SAF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAFR {
    #[doc = "Source address ignored"]
    DISABLED,
    #[doc = "MAC drops frames that fail the source address filter"]
    ENABLED,
}
impl SAFR {
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
            SAFR::DISABLED => false,
            SAFR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAFR {
        match value {
            false => SAFR::DISABLED,
            true => SAFR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SAFR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SAFR::ENABLED
    }
}
#[doc = "Possible values of the field `HPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPFR {
    #[doc = "If HM or HU is set, only frames that match the Hash filter are passed"]
    HASHONLY,
    #[doc = "If HM or HU is set, frames that match either the perfect filter or the hash filter are passed"]
    HASHORPERFECT,
}
impl HPFR {
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
            HPFR::HASHONLY => false,
            HPFR::HASHORPERFECT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPFR {
        match value {
            false => HPFR::HASHONLY,
            true => HPFR::HASHORPERFECT,
        }
    }
    #[doc = "Checks if the value of the field is `HASHONLY`"]
    #[inline]
    pub fn is_hash_only(&self) -> bool {
        *self == HPFR::HASHONLY
    }
    #[doc = "Checks if the value of the field is `HASHORPERFECT`"]
    #[inline]
    pub fn is_hash_or_perfect(&self) -> bool {
        *self == HPFR::HASHORPERFECT
    }
}
#[doc = "Possible values of the field `RA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAR {
    #[doc = "MAC receiver passes on to the application only those frames that have passed the SA/DA address file"]
    DISABLED,
    #[doc = "MAC receiver passes oll received frames on to the application"]
    ENABLED,
}
impl RAR {
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
            RAR::DISABLED => false,
            RAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAR {
        match value {
            false => RAR::DISABLED,
            true => RAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `PM`"]
pub enum PMW {
    #[doc = "Normal address filtering"]
    DISABLED,
    #[doc = "Address filters pass all incoming frames regardless of their destination or source address"]
    ENABLED,
}
impl PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMW::DISABLED => false,
            PMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal address filtering"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PMW::DISABLED)
    }
    #[doc = "Address filters pass all incoming frames regardless of their destination or source address"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PMW::ENABLED)
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
#[doc = "Values that can be written to the field `HU`"]
pub enum HUW {
    #[doc = "MAC performs a perfect destination address filtering for unicast frames"]
    PERFECT,
    #[doc = "MAC performs destination address filtering of received unicast frames according to the hash table"]
    HASH,
}
impl HUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HUW::PERFECT => false,
            HUW::HASH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HUW<'a> {
    w: &'a mut W,
}
impl<'a> _HUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC performs a perfect destination address filtering for unicast frames"]
    #[inline]
    pub fn perfect(self) -> &'a mut W {
        self.variant(HUW::PERFECT)
    }
    #[doc = "MAC performs destination address filtering of received unicast frames according to the hash table"]
    #[inline]
    pub fn hash(self) -> &'a mut W {
        self.variant(HUW::HASH)
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
#[doc = "Values that can be written to the field `HM`"]
pub enum HMW {
    #[doc = "MAC performs a perfect destination address filtering for multicast frames"]
    PERFECT,
    #[doc = "MAC performs destination address filtering of received multicast frames according to the hash table"]
    HASH,
}
impl HMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HMW::PERFECT => false,
            HMW::HASH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HMW<'a> {
    w: &'a mut W,
}
impl<'a> _HMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC performs a perfect destination address filtering for multicast frames"]
    #[inline]
    pub fn perfect(self) -> &'a mut W {
        self.variant(HMW::PERFECT)
    }
    #[doc = "MAC performs destination address filtering of received multicast frames according to the hash table"]
    #[inline]
    pub fn hash(self) -> &'a mut W {
        self.variant(HMW::HASH)
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
#[doc = "Values that can be written to the field `DAIF`"]
pub enum DAIFW {
    #[doc = "Normal filtering of frames"]
    NORMAL,
    #[doc = "Address check block operates in inverse filtering mode for the DA address comparison"]
    INVERT,
}
impl DAIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAIFW::NORMAL => false,
            DAIFW::INVERT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DAIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal filtering of frames"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(DAIFW::NORMAL)
    }
    #[doc = "Address check block operates in inverse filtering mode for the DA address comparison"]
    #[inline]
    pub fn invert(self) -> &'a mut W {
        self.variant(DAIFW::INVERT)
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
#[doc = "Values that can be written to the field `RAM`"]
pub enum RAMW {
    #[doc = "Filtering of multicast frames depends on HM"]
    DISABLED,
    #[doc = "All received frames with a multicast destination address are passed"]
    ENABLED,
}
impl RAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAMW::DISABLED => false,
            RAMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Filtering of multicast frames depends on HM"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAMW::DISABLED)
    }
    #[doc = "All received frames with a multicast destination address are passed"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAMW::ENABLED)
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
#[doc = "Values that can be written to the field `BFD`"]
pub enum BFDW {
    #[doc = "Address filters pass all received broadcast frames"]
    ENABLED,
    #[doc = "Address filters filter all incoming broadcast frames"]
    DISABLED,
}
impl BFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFDW::ENABLED => false,
            BFDW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFDW<'a> {
    w: &'a mut W,
}
impl<'a> _BFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Address filters pass all received broadcast frames"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BFDW::ENABLED)
    }
    #[doc = "Address filters filter all incoming broadcast frames"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BFDW::DISABLED)
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
#[doc = "Values that can be written to the field `PCF`"]
pub enum PCFW {
    #[doc = "MAC prevents all control frames from reaching the application"]
    PREVENTALL,
    #[doc = "MAC forwards all control frames to application except Pause"]
    FORWARDALLEXCEPTPAUSE,
    #[doc = "MAC forwards all control frames to application even if they fail the address filter"]
    FORWARDALL,
    #[doc = "MAC forwards control frames that pass the address filter"]
    FORWARDALLFILTERED,
}
impl PCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCFW::PREVENTALL => 0,
            PCFW::FORWARDALLEXCEPTPAUSE => 1,
            PCFW::FORWARDALL => 2,
            PCFW::FORWARDALLFILTERED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCFW<'a> {
    w: &'a mut W,
}
impl<'a> _PCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "MAC prevents all control frames from reaching the application"]
    #[inline]
    pub fn prevent_all(self) -> &'a mut W {
        self.variant(PCFW::PREVENTALL)
    }
    #[doc = "MAC forwards all control frames to application except Pause"]
    #[inline]
    pub fn forward_all_except_pause(self) -> &'a mut W {
        self.variant(PCFW::FORWARDALLEXCEPTPAUSE)
    }
    #[doc = "MAC forwards all control frames to application even if they fail the address filter"]
    #[inline]
    pub fn forward_all(self) -> &'a mut W {
        self.variant(PCFW::FORWARDALL)
    }
    #[doc = "MAC forwards control frames that pass the address filter"]
    #[inline]
    pub fn forward_all_filtered(self) -> &'a mut W {
        self.variant(PCFW::FORWARDALLFILTERED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAIF`"]
pub enum SAIFW {
    #[doc = "Source address filter operates normally"]
    NORMAL,
    #[doc = "Source address filter operation inverted"]
    INVERT,
}
impl SAIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAIFW::NORMAL => false,
            SAIFW::INVERT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAIFW<'a> {
    w: &'a mut W,
}
impl<'a> _SAIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Source address filter operates normally"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(SAIFW::NORMAL)
    }
    #[doc = "Source address filter operation inverted"]
    #[inline]
    pub fn invert(self) -> &'a mut W {
        self.variant(SAIFW::INVERT)
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
#[doc = "Values that can be written to the field `SAF`"]
pub enum SAFW {
    #[doc = "Source address ignored"]
    DISABLED,
    #[doc = "MAC drops frames that fail the source address filter"]
    ENABLED,
}
impl SAFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAFW::DISABLED => false,
            SAFW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAFW<'a> {
    w: &'a mut W,
}
impl<'a> _SAFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Source address ignored"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAFW::DISABLED)
    }
    #[doc = "MAC drops frames that fail the source address filter"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAFW::ENABLED)
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
#[doc = "Values that can be written to the field `HPF`"]
pub enum HPFW {
    #[doc = "If HM or HU is set, only frames that match the Hash filter are passed"]
    HASHONLY,
    #[doc = "If HM or HU is set, frames that match either the perfect filter or the hash filter are passed"]
    HASHORPERFECT,
}
impl HPFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPFW::HASHONLY => false,
            HPFW::HASHORPERFECT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPFW<'a> {
    w: &'a mut W,
}
impl<'a> _HPFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If HM or HU is set, only frames that match the Hash filter are passed"]
    #[inline]
    pub fn hash_only(self) -> &'a mut W {
        self.variant(HPFW::HASHONLY)
    }
    #[doc = "If HM or HU is set, frames that match either the perfect filter or the hash filter are passed"]
    #[inline]
    pub fn hash_or_perfect(self) -> &'a mut W {
        self.variant(HPFW::HASHORPERFECT)
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
#[doc = "Values that can be written to the field `RA`"]
pub enum RAW {
    #[doc = "MAC receiver passes on to the application only those frames that have passed the SA/DA address file"]
    DISABLED,
    #[doc = "MAC receiver passes oll received frames on to the application"]
    ENABLED,
}
impl RAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAW::DISABLED => false,
            RAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAW<'a> {
    w: &'a mut W,
}
impl<'a> _RAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC receiver passes on to the application only those frames that have passed the SA/DA address file"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAW::DISABLED)
    }
    #[doc = "MAC receiver passes oll received frames on to the application"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAW::ENABLED)
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
    #[doc = "Bit 0 - PM"]
    #[inline]
    pub fn pm(&self) -> PMR {
        PMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - HU"]
    #[inline]
    pub fn hu(&self) -> HUR {
        HUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - HM"]
    #[inline]
    pub fn hm(&self) -> HMR {
        HMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline]
    pub fn daif(&self) -> DAIFR {
        DAIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RAM"]
    #[inline]
    pub fn ram(&self) -> RAMR {
        RAMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - BFD"]
    #[inline]
    pub fn bfd(&self) -> BFDR {
        BFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - PCF"]
    #[inline]
    pub fn pcf(&self) -> PCFR {
        PCFR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - SAIF"]
    #[inline]
    pub fn saif(&self) -> SAIFR {
        SAIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - SAF"]
    #[inline]
    pub fn saf(&self) -> SAFR {
        SAFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - HPF"]
    #[inline]
    pub fn hpf(&self) -> HPFR {
        HPFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - RA"]
    #[inline]
    pub fn ra(&self) -> RAR {
        RAR::_from({
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
    #[doc = "Bit 0 - PM"]
    #[inline]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 1 - HU"]
    #[inline]
    pub fn hu(&mut self) -> _HUW {
        _HUW { w: self }
    }
    #[doc = "Bit 2 - HM"]
    #[inline]
    pub fn hm(&mut self) -> _HMW {
        _HMW { w: self }
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline]
    pub fn daif(&mut self) -> _DAIFW {
        _DAIFW { w: self }
    }
    #[doc = "Bit 4 - RAM"]
    #[inline]
    pub fn ram(&mut self) -> _RAMW {
        _RAMW { w: self }
    }
    #[doc = "Bit 5 - BFD"]
    #[inline]
    pub fn bfd(&mut self) -> _BFDW {
        _BFDW { w: self }
    }
    #[doc = "Bits 6:7 - PCF"]
    #[inline]
    pub fn pcf(&mut self) -> _PCFW {
        _PCFW { w: self }
    }
    #[doc = "Bit 7 - SAIF"]
    #[inline]
    pub fn saif(&mut self) -> _SAIFW {
        _SAIFW { w: self }
    }
    #[doc = "Bit 8 - SAF"]
    #[inline]
    pub fn saf(&mut self) -> _SAFW {
        _SAFW { w: self }
    }
    #[doc = "Bit 9 - HPF"]
    #[inline]
    pub fn hpf(&mut self) -> _HPFW {
        _HPFW { w: self }
    }
    #[doc = "Bit 31 - RA"]
    #[inline]
    pub fn ra(&mut self) -> _RAW {
        _RAW { w: self }
    }
}
