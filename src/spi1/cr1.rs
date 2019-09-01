#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bidirectional data mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIMODE_A {
    #[doc = "0: 2-line unidirectional data mode selected"]
    UNIDIRECTIONAL,
    #[doc = "1: 1-line bidirectional data mode selected"]
    BIDIRECTIONAL,
}
impl From<BIDIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIMODE_A) -> Self {
        match variant {
            BIDIMODE_A::UNIDIRECTIONAL => false,
            BIDIMODE_A::BIDIRECTIONAL => true,
        }
    }
}
#[doc = "Reader of field `BIDIMODE`"]
pub type BIDIMODE_R = crate::R<bool, BIDIMODE_A>;
impl BIDIMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIMODE_A {
        match self.bits {
            false => BIDIMODE_A::UNIDIRECTIONAL,
            true => BIDIMODE_A::BIDIRECTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `UNIDIRECTIONAL`"]
    #[inline(always)]
    pub fn is_unidirectional(&self) -> bool {
        *self == BIDIMODE_A::UNIDIRECTIONAL
    }
    #[doc = "Checks if the value of the field is `BIDIRECTIONAL`"]
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BIDIMODE_A::BIDIRECTIONAL
    }
}
#[doc = "Write proxy for field `BIDIMODE`"]
pub struct BIDIMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIDIMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIDIMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "2-line unidirectional data mode selected"]
    #[inline(always)]
    pub fn unidirectional(self) -> &'a mut W {
        self.variant(BIDIMODE_A::UNIDIRECTIONAL)
    }
    #[doc = "1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut W {
        self.variant(BIDIMODE_A::BIDIRECTIONAL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Output enable in bidirectional mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIOE_A {
    #[doc = "0: Output disabled (receive-only mode)"]
    OUTPUTDISABLED,
    #[doc = "1: Output enabled (transmit-only mode)"]
    OUTPUTENABLED,
}
impl From<BIDIOE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIOE_A) -> Self {
        match variant {
            BIDIOE_A::OUTPUTDISABLED => false,
            BIDIOE_A::OUTPUTENABLED => true,
        }
    }
}
#[doc = "Reader of field `BIDIOE`"]
pub type BIDIOE_R = crate::R<bool, BIDIOE_A>;
impl BIDIOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIOE_A {
        match self.bits {
            false => BIDIOE_A::OUTPUTDISABLED,
            true => BIDIOE_A::OUTPUTENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUTDISABLED`"]
    #[inline(always)]
    pub fn is_output_disabled(&self) -> bool {
        *self == BIDIOE_A::OUTPUTDISABLED
    }
    #[doc = "Checks if the value of the field is `OUTPUTENABLED`"]
    #[inline(always)]
    pub fn is_output_enabled(&self) -> bool {
        *self == BIDIOE_A::OUTPUTENABLED
    }
}
#[doc = "Write proxy for field `BIDIOE`"]
pub struct BIDIOE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIDIOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIDIOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output disabled (receive-only mode)"]
    #[inline(always)]
    pub fn output_disabled(self) -> &'a mut W {
        self.variant(BIDIOE_A::OUTPUTDISABLED)
    }
    #[doc = "Output enabled (transmit-only mode)"]
    #[inline(always)]
    pub fn output_enabled(self) -> &'a mut W {
        self.variant(BIDIOE_A::OUTPUTENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Hardware CRC calculation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCEN_A {
    #[doc = "0: CRC calculation disabled"]
    DISABLED,
    #[doc = "1: CRC calculation enabled"]
    ENABLED,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        match variant {
            CRCEN_A::DISABLED => false,
            CRCEN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `CRCEN`"]
pub type CRCEN_R = crate::R<bool, CRCEN_A>;
impl CRCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::DISABLED,
            true => CRCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CRCEN`"]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::DISABLED)
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "CRC transfer next\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCNEXT_A {
    #[doc = "0: Next transmit value is from Tx buffer"]
    TXBUFFER,
    #[doc = "1: Next transmit value is from Tx CRC register"]
    CRC,
}
impl From<CRCNEXT_A> for bool {
    #[inline(always)]
    fn from(variant: CRCNEXT_A) -> Self {
        match variant {
            CRCNEXT_A::TXBUFFER => false,
            CRCNEXT_A::CRC => true,
        }
    }
}
#[doc = "Reader of field `CRCNEXT`"]
pub type CRCNEXT_R = crate::R<bool, CRCNEXT_A>;
impl CRCNEXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCNEXT_A {
        match self.bits {
            false => CRCNEXT_A::TXBUFFER,
            true => CRCNEXT_A::CRC,
        }
    }
    #[doc = "Checks if the value of the field is `TXBUFFER`"]
    #[inline(always)]
    pub fn is_tx_buffer(&self) -> bool {
        *self == CRCNEXT_A::TXBUFFER
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == CRCNEXT_A::CRC
    }
}
#[doc = "Write proxy for field `CRCNEXT`"]
pub struct CRCNEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCNEXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCNEXT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Next transmit value is from Tx buffer"]
    #[inline(always)]
    pub fn tx_buffer(self) -> &'a mut W {
        self.variant(CRCNEXT_A::TXBUFFER)
    }
    #[doc = "Next transmit value is from Tx CRC register"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(CRCNEXT_A::CRC)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "CRC length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCL_A {
    #[doc = "0: 8-bit CRC length"]
    EIGHTBIT,
    #[doc = "1: 16-bit CRC length"]
    SIXTEENBIT,
}
impl From<CRCL_A> for bool {
    #[inline(always)]
    fn from(variant: CRCL_A) -> Self {
        match variant {
            CRCL_A::EIGHTBIT => false,
            CRCL_A::SIXTEENBIT => true,
        }
    }
}
#[doc = "Reader of field `CRCL`"]
pub type CRCL_R = crate::R<bool, CRCL_A>;
impl CRCL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCL_A {
        match self.bits {
            false => CRCL_A::EIGHTBIT,
            true => CRCL_A::SIXTEENBIT,
        }
    }
    #[doc = "Checks if the value of the field is `EIGHTBIT`"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == CRCL_A::EIGHTBIT
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == CRCL_A::SIXTEENBIT
    }
}
#[doc = "Write proxy for field `CRCL`"]
pub struct CRCL_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "8-bit CRC length"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(CRCL_A::EIGHTBIT)
    }
    #[doc = "16-bit CRC length"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(CRCL_A::SIXTEENBIT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Receive only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXONLY_A {
    #[doc = "0: Full duplex (Transmit and receive)"]
    FULLDUPLEX,
    #[doc = "1: Output disabled (Receive-only mode)"]
    OUTPUTDISABLED,
}
impl From<RXONLY_A> for bool {
    #[inline(always)]
    fn from(variant: RXONLY_A) -> Self {
        match variant {
            RXONLY_A::FULLDUPLEX => false,
            RXONLY_A::OUTPUTDISABLED => true,
        }
    }
}
#[doc = "Reader of field `RXONLY`"]
pub type RXONLY_R = crate::R<bool, RXONLY_A>;
impl RXONLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXONLY_A {
        match self.bits {
            false => RXONLY_A::FULLDUPLEX,
            true => RXONLY_A::OUTPUTDISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `FULLDUPLEX`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == RXONLY_A::FULLDUPLEX
    }
    #[doc = "Checks if the value of the field is `OUTPUTDISABLED`"]
    #[inline(always)]
    pub fn is_output_disabled(&self) -> bool {
        *self == RXONLY_A::OUTPUTDISABLED
    }
}
#[doc = "Write proxy for field `RXONLY`"]
pub struct RXONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXONLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXONLY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Full duplex (Transmit and receive)"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(RXONLY_A::FULLDUPLEX)
    }
    #[doc = "Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn output_disabled(self) -> &'a mut W {
        self.variant(RXONLY_A::OUTPUTDISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Software slave management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSM_A {
    #[doc = "0: Software slave management disabled"]
    DISABLED,
    #[doc = "1: Software slave management enabled"]
    ENABLED,
}
impl From<SSM_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_A) -> Self {
        match variant {
            SSM_A::DISABLED => false,
            SSM_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `SSM`"]
pub type SSM_R = crate::R<bool, SSM_A>;
impl SSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSM_A {
        match self.bits {
            false => SSM_A::DISABLED,
            true => SSM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSM_A::ENABLED
    }
}
#[doc = "Write proxy for field `SSM`"]
pub struct SSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSM_A::DISABLED)
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSM_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SSI`"]
pub type SSI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI`"]
pub struct SSI_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFIRST_A {
    #[doc = "0: Data is transmitted/received with the MSB first"]
    MSBFIRST,
    #[doc = "1: Data is transmitted/received with the LSB first"]
    LSBFIRST,
}
impl From<LSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST_A) -> Self {
        match variant {
            LSBFIRST_A::MSBFIRST => false,
            LSBFIRST_A::LSBFIRST => true,
        }
    }
}
#[doc = "Reader of field `LSBFIRST`"]
pub type LSBFIRST_R = crate::R<bool, LSBFIRST_A>;
impl LSBFIRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBFIRST_A {
        match self.bits {
            false => LSBFIRST_A::MSBFIRST,
            true => LSBFIRST_A::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LSBFIRST_A::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LSBFIRST_A::LSBFIRST
    }
}
#[doc = "Write proxy for field `LSBFIRST`"]
pub struct LSBFIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBFIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is transmitted/received with the MSB first"]
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(LSBFIRST_A::MSBFIRST)
    }
    #[doc = "Data is transmitted/received with the LSB first"]
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(LSBFIRST_A::LSBFIRST)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "SPI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPE_A {
    #[doc = "0: Peripheral disabled"]
    DISABLED,
    #[doc = "1: Peripheral enabled"]
    ENABLED,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        match variant {
            SPE_A::DISABLED => false,
            SPE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `SPE`"]
pub type SPE_R = crate::R<bool, SPE_A>;
impl SPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::DISABLED,
            true => SPE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SPE`"]
pub struct SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPE_A::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Baud rate control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BR_A {
    #[doc = "0: f_PCLK / 2"]
    DIV2,
    #[doc = "1: f_PCLK / 4"]
    DIV4,
    #[doc = "2: f_PCLK / 8"]
    DIV8,
    #[doc = "3: f_PCLK / 16"]
    DIV16,
    #[doc = "4: f_PCLK / 32"]
    DIV32,
    #[doc = "5: f_PCLK / 64"]
    DIV64,
    #[doc = "6: f_PCLK / 128"]
    DIV128,
    #[doc = "7: f_PCLK / 256"]
    DIV256,
}
impl From<BR_A> for u8 {
    #[inline(always)]
    fn from(variant: BR_A) -> Self {
        match variant {
            BR_A::DIV2 => 0,
            BR_A::DIV4 => 1,
            BR_A::DIV8 => 2,
            BR_A::DIV16 => 3,
            BR_A::DIV32 => 4,
            BR_A::DIV64 => 5,
            BR_A::DIV128 => 6,
            BR_A::DIV256 => 7,
        }
    }
}
#[doc = "Reader of field `BR`"]
pub type BR_R = crate::R<u8, BR_A>;
impl BR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BR_A {
        match self.bits {
            0 => BR_A::DIV2,
            1 => BR_A::DIV4,
            2 => BR_A::DIV8,
            3 => BR_A::DIV16,
            4 => BR_A::DIV32,
            5 => BR_A::DIV64,
            6 => BR_A::DIV128,
            7 => BR_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == BR_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BR_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BR_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BR_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BR_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BR_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BR_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == BR_A::DIV256
    }
}
#[doc = "Write proxy for field `BR`"]
pub struct BR_W<'a> {
    w: &'a mut W,
}
impl<'a> BR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "f_PCLK / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(BR_A::DIV2)
    }
    #[doc = "f_PCLK / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(BR_A::DIV4)
    }
    #[doc = "f_PCLK / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(BR_A::DIV8)
    }
    #[doc = "f_PCLK / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(BR_A::DIV16)
    }
    #[doc = "f_PCLK / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(BR_A::DIV32)
    }
    #[doc = "f_PCLK / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(BR_A::DIV64)
    }
    #[doc = "f_PCLK / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(BR_A::DIV128)
    }
    #[doc = "f_PCLK / 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(BR_A::DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Master selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR_A {
    #[doc = "0: Slave configuration"]
    SLAVE,
    #[doc = "1: Master configuration"]
    MASTER,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        match variant {
            MSTR_A::SLAVE => false,
            MSTR_A::MASTER => true,
        }
    }
}
#[doc = "Reader of field `MSTR`"]
pub type MSTR_R = crate::R<bool, MSTR_A>;
impl MSTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::SLAVE,
            true => MSTR_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTR_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTR_A::MASTER
    }
}
#[doc = "Write proxy for field `MSTR`"]
pub struct MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTR_A::SLAVE)
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTR_A::MASTER)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: CK to 0 when idle"]
    IDLELOW,
    #[doc = "1: CK to 1 when idle"]
    IDLEHIGH,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        match variant {
            CPOL_A::IDLELOW => false,
            CPOL_A::IDLEHIGH => true,
        }
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::IDLELOW,
            true => CPOL_A::IDLEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLELOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL_A::IDLELOW
    }
    #[doc = "Checks if the value of the field is `IDLEHIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL_A::IDLEHIGH
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CK to 0 when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOL_A::IDLELOW)
    }
    #[doc = "CK to 1 when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOL_A::IDLEHIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: The first clock transition is the first data capture edge"]
    FIRSTEDGE,
    #[doc = "1: The second clock transition is the first data capture edge"]
    SECONDEDGE,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        match variant {
            CPHA_A::FIRSTEDGE => false,
            CPHA_A::SECONDEDGE => true,
        }
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::FIRSTEDGE,
            true => CPHA_A::SECONDEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FIRSTEDGE`"]
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHA_A::FIRSTEDGE
    }
    #[doc = "Checks if the value of the field is `SECONDEDGE`"]
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHA_A::SECONDEDGE
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut W {
        self.variant(CPHA_A::FIRSTEDGE)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut W {
        self.variant(CPHA_A::SECONDEDGE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CRC length"]
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    pub fn bidimode(&mut self) -> BIDIMODE_W {
        BIDIMODE_W { w: self }
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn bidioe(&mut self) -> BIDIOE_W {
        BIDIOE_W { w: self }
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnext(&mut self) -> CRCNEXT_W {
        CRCNEXT_W { w: self }
    }
    #[doc = "Bit 11 - CRC length"]
    #[inline(always)]
    pub fn crcl(&mut self) -> CRCL_W {
        CRCL_W { w: self }
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn rxonly(&mut self) -> RXONLY_W {
        RXONLY_W { w: self }
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W {
        SSM_W { w: self }
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W {
        SSI_W { w: self }
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W {
        LSBFIRST_W { w: self }
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W { w: self }
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    pub fn br(&mut self) -> BR_W {
        BR_W { w: self }
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W { w: self }
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
}
