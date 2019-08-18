#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HISR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TCIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF7R {
    #[doc = "No transfer complete event on stream x"]
    NOTCOMPLETE,
    #[doc = "A transfer complete event occurred on stream x"]
    COMPLETE,
}
impl TCIF7R {
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
            TCIF7R::NOTCOMPLETE => false,
            TCIF7R::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIF7R {
        match value {
            false => TCIF7R::NOTCOMPLETE,
            true => TCIF7R::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF7R::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == TCIF7R::COMPLETE
    }
}
#[doc = "Possible values of the field `HTIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF7R {
    #[doc = "No half transfer event on stream x"]
    NOTHALF,
    #[doc = "A half transfer event occurred on stream x"]
    HALF,
}
impl HTIF7R {
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
            HTIF7R::NOTHALF => false,
            HTIF7R::HALF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HTIF7R {
        match value {
            false => HTIF7R::NOTHALF,
            true => HTIF7R::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF7R::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == HTIF7R::HALF
    }
}
#[doc = "Possible values of the field `TEIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF7R {
    #[doc = "No transfer error on stream x"]
    NOERROR,
    #[doc = "A transfer error occurred on stream x"]
    ERROR,
}
impl TEIF7R {
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
            TEIF7R::NOERROR => false,
            TEIF7R::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEIF7R {
        match value {
            false => TEIF7R::NOERROR,
            true => TEIF7R::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF7R::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == TEIF7R::ERROR
    }
}
#[doc = "Possible values of the field `DMEIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIF7R {
    #[doc = "No Direct Mode error on stream x"]
    NOERROR,
    #[doc = "A Direct Mode error occurred on stream x"]
    ERROR,
}
impl DMEIF7R {
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
            DMEIF7R::NOERROR => false,
            DMEIF7R::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMEIF7R {
        match value {
            false => DMEIF7R::NOERROR,
            true => DMEIF7R::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == DMEIF7R::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == DMEIF7R::ERROR
    }
}
#[doc = "Possible values of the field `FEIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIF7R {
    #[doc = "No FIFO error event on stream x"]
    NOERROR,
    #[doc = "A FIFO error event occurred on stream x"]
    ERROR,
}
impl FEIF7R {
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
            FEIF7R::NOERROR => false,
            FEIF7R::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEIF7R {
        match value {
            false => FEIF7R::NOERROR,
            true => FEIF7R::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == FEIF7R::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == FEIF7R::ERROR
    }
}
#[doc = "Possible values of the field `TCIF6`"]
pub type TCIF6R = TCIF7R;
#[doc = "Possible values of the field `HTIF6`"]
pub type HTIF6R = HTIF7R;
#[doc = "Possible values of the field `TEIF6`"]
pub type TEIF6R = TEIF7R;
#[doc = "Possible values of the field `DMEIF6`"]
pub type DMEIF6R = DMEIF7R;
#[doc = "Possible values of the field `FEIF6`"]
pub type FEIF6R = FEIF7R;
#[doc = "Possible values of the field `TCIF5`"]
pub type TCIF5R = TCIF7R;
#[doc = "Possible values of the field `HTIF5`"]
pub type HTIF5R = HTIF7R;
#[doc = "Possible values of the field `TEIF5`"]
pub type TEIF5R = TEIF7R;
#[doc = "Possible values of the field `DMEIF5`"]
pub type DMEIF5R = DMEIF7R;
#[doc = "Possible values of the field `FEIF5`"]
pub type FEIF5R = FEIF7R;
#[doc = "Possible values of the field `TCIF4`"]
pub type TCIF4R = TCIF7R;
#[doc = "Possible values of the field `HTIF4`"]
pub type HTIF4R = HTIF7R;
#[doc = "Possible values of the field `TEIF4`"]
pub type TEIF4R = TEIF7R;
#[doc = "Possible values of the field `DMEIF4`"]
pub type DMEIF4R = DMEIF7R;
#[doc = "Possible values of the field `FEIF4`"]
pub type FEIF4R = FEIF7R;
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline]
    pub fn tcif7(&self) -> TCIF7R {
        TCIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline]
    pub fn htif7(&self) -> HTIF7R {
        HTIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline]
    pub fn teif7(&self) -> TEIF7R {
        TEIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline]
    pub fn dmeif7(&self) -> DMEIF7R {
        DMEIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline]
    pub fn feif7(&self) -> FEIF7R {
        FEIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline]
    pub fn tcif6(&self) -> TCIF6R {
        TCIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline]
    pub fn htif6(&self) -> HTIF6R {
        HTIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline]
    pub fn teif6(&self) -> TEIF6R {
        TEIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline]
    pub fn dmeif6(&self) -> DMEIF6R {
        DMEIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline]
    pub fn feif6(&self) -> FEIF6R {
        FEIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline]
    pub fn tcif5(&self) -> TCIF5R {
        TCIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline]
    pub fn htif5(&self) -> HTIF5R {
        HTIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline]
    pub fn teif5(&self) -> TEIF5R {
        TEIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline]
    pub fn dmeif5(&self) -> DMEIF5R {
        DMEIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline]
    pub fn feif5(&self) -> FEIF5R {
        FEIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline]
    pub fn tcif4(&self) -> TCIF4R {
        TCIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline]
    pub fn htif4(&self) -> HTIF4R {
        HTIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline]
    pub fn teif4(&self) -> TEIF4R {
        TEIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline]
    pub fn dmeif4(&self) -> DMEIF4R {
        DMEIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline]
    pub fn feif4(&self) -> FEIF4R {
        FEIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
