#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LISR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TCIF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF3R {
    #[doc = "No transfer complete event on stream x"]
    NOTCOMPLETE,
    #[doc = "A transfer complete event occurred on stream x"]
    COMPLETE,
}
impl TCIF3R {
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
            TCIF3R::NOTCOMPLETE => false,
            TCIF3R::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIF3R {
        match value {
            false => TCIF3R::NOTCOMPLETE,
            true => TCIF3R::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF3R::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == TCIF3R::COMPLETE
    }
}
#[doc = "Possible values of the field `HTIF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF3R {
    #[doc = "No half transfer event on stream x"]
    NOTHALF,
    #[doc = "A half transfer event occurred on stream x"]
    HALF,
}
impl HTIF3R {
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
            HTIF3R::NOTHALF => false,
            HTIF3R::HALF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HTIF3R {
        match value {
            false => HTIF3R::NOTHALF,
            true => HTIF3R::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF3R::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == HTIF3R::HALF
    }
}
#[doc = "Possible values of the field `TEIF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF3R {
    #[doc = "No transfer error on stream x"]
    NOERROR,
    #[doc = "A transfer error occurred on stream x"]
    ERROR,
}
impl TEIF3R {
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
            TEIF3R::NOERROR => false,
            TEIF3R::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEIF3R {
        match value {
            false => TEIF3R::NOERROR,
            true => TEIF3R::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF3R::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == TEIF3R::ERROR
    }
}
#[doc = "Possible values of the field `DMEIF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIF3R {
    #[doc = "No Direct Mode error on stream x"]
    NOERROR,
    #[doc = "A Direct Mode error occurred on stream x"]
    ERROR,
}
impl DMEIF3R {
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
            DMEIF3R::NOERROR => false,
            DMEIF3R::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMEIF3R {
        match value {
            false => DMEIF3R::NOERROR,
            true => DMEIF3R::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == DMEIF3R::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == DMEIF3R::ERROR
    }
}
#[doc = "Possible values of the field `FEIF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIF3R {
    #[doc = "No FIFO error event on stream x"]
    NOERROR,
    #[doc = "A FIFO error event occurred on stream x"]
    ERROR,
}
impl FEIF3R {
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
            FEIF3R::NOERROR => false,
            FEIF3R::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEIF3R {
        match value {
            false => FEIF3R::NOERROR,
            true => FEIF3R::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == FEIF3R::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == FEIF3R::ERROR
    }
}
#[doc = "Possible values of the field `TCIF2`"]
pub type TCIF2R = TCIF3R;
#[doc = "Possible values of the field `HTIF2`"]
pub type HTIF2R = HTIF3R;
#[doc = "Possible values of the field `TEIF2`"]
pub type TEIF2R = TEIF3R;
#[doc = "Possible values of the field `DMEIF2`"]
pub type DMEIF2R = DMEIF3R;
#[doc = "Possible values of the field `FEIF2`"]
pub type FEIF2R = FEIF3R;
#[doc = "Possible values of the field `TCIF1`"]
pub type TCIF1R = TCIF3R;
#[doc = "Possible values of the field `HTIF1`"]
pub type HTIF1R = HTIF3R;
#[doc = "Possible values of the field `TEIF1`"]
pub type TEIF1R = TEIF3R;
#[doc = "Possible values of the field `DMEIF1`"]
pub type DMEIF1R = DMEIF3R;
#[doc = "Possible values of the field `FEIF1`"]
pub type FEIF1R = FEIF3R;
#[doc = "Possible values of the field `TCIF0`"]
pub type TCIF0R = TCIF3R;
#[doc = "Possible values of the field `HTIF0`"]
pub type HTIF0R = HTIF3R;
#[doc = "Possible values of the field `TEIF0`"]
pub type TEIF0R = TEIF3R;
#[doc = "Possible values of the field `DMEIF0`"]
pub type DMEIF0R = DMEIF3R;
#[doc = "Possible values of the field `FEIF0`"]
pub type FEIF0R = FEIF3R;
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline]
    pub fn tcif3(&self) -> TCIF3R {
        TCIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline]
    pub fn htif3(&self) -> HTIF3R {
        HTIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline]
    pub fn teif3(&self) -> TEIF3R {
        TEIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline]
    pub fn dmeif3(&self) -> DMEIF3R {
        DMEIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline]
    pub fn feif3(&self) -> FEIF3R {
        FEIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline]
    pub fn tcif2(&self) -> TCIF2R {
        TCIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline]
    pub fn htif2(&self) -> HTIF2R {
        HTIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline]
    pub fn teif2(&self) -> TEIF2R {
        TEIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline]
    pub fn dmeif2(&self) -> DMEIF2R {
        DMEIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline]
    pub fn feif2(&self) -> FEIF2R {
        FEIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline]
    pub fn tcif1(&self) -> TCIF1R {
        TCIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline]
    pub fn htif1(&self) -> HTIF1R {
        HTIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline]
    pub fn teif1(&self) -> TEIF1R {
        TEIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline]
    pub fn dmeif1(&self) -> DMEIF1R {
        DMEIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline]
    pub fn feif1(&self) -> FEIF1R {
        FEIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline]
    pub fn tcif0(&self) -> TCIF0R {
        TCIF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline]
    pub fn htif0(&self) -> HTIF0R {
        HTIF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline]
    pub fn teif0(&self) -> TEIF0R {
        TEIF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline]
    pub fn dmeif0(&self) -> DMEIF0R {
        DMEIF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline]
    pub fn feif0(&self) -> FEIF0R {
        FEIF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
