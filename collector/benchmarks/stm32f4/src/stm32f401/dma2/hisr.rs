#[doc = "Reader of register HISR"]
pub type R = crate::R<u32, super::HISR>;
#[doc = "Stream x transfer complete interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF7_A {
    #[doc = "0: No transfer complete event on stream x"]
    NOTCOMPLETE = 0,
    #[doc = "1: A transfer complete event occurred on stream x"]
    COMPLETE = 1,
}
impl From<TCIF7_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF7`"]
pub type TCIF7_R = crate::R<bool, TCIF7_A>;
impl TCIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF7_A {
        match self.bits {
            false => TCIF7_A::NOTCOMPLETE,
            true => TCIF7_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF7_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF7_A::COMPLETE
    }
}
#[doc = "Stream x half transfer interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF7_A {
    #[doc = "0: No half transfer event on stream x"]
    NOTHALF = 0,
    #[doc = "1: A half transfer event occurred on stream x"]
    HALF = 1,
}
impl From<HTIF7_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HTIF7`"]
pub type HTIF7_R = crate::R<bool, HTIF7_A>;
impl HTIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF7_A {
        match self.bits {
            false => HTIF7_A::NOTHALF,
            true => HTIF7_A::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF7_A::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF7_A::HALF
    }
}
#[doc = "Stream x transfer error interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF7_A {
    #[doc = "0: No transfer error on stream x"]
    NOERROR = 0,
    #[doc = "1: A transfer error occurred on stream x"]
    ERROR = 1,
}
impl From<TEIF7_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF7`"]
pub type TEIF7_R = crate::R<bool, TEIF7_A>;
impl TEIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF7_A {
        match self.bits {
            false => TEIF7_A::NOERROR,
            true => TEIF7_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF7_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF7_A::ERROR
    }
}
#[doc = "Stream x direct mode error interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIF7_A {
    #[doc = "0: No Direct Mode error on stream x"]
    NOERROR = 0,
    #[doc = "1: A Direct Mode error occurred on stream x"]
    ERROR = 1,
}
impl From<DMEIF7_A> for bool {
    #[inline(always)]
    fn from(variant: DMEIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMEIF7`"]
pub type DMEIF7_R = crate::R<bool, DMEIF7_A>;
impl DMEIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMEIF7_A {
        match self.bits {
            false => DMEIF7_A::NOERROR,
            true => DMEIF7_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == DMEIF7_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == DMEIF7_A::ERROR
    }
}
#[doc = "Stream x FIFO error interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIF7_A {
    #[doc = "0: No FIFO error event on stream x"]
    NOERROR = 0,
    #[doc = "1: A FIFO error event occurred on stream x"]
    ERROR = 1,
}
impl From<FEIF7_A> for bool {
    #[inline(always)]
    fn from(variant: FEIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FEIF7`"]
pub type FEIF7_R = crate::R<bool, FEIF7_A>;
impl FEIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIF7_A {
        match self.bits {
            false => FEIF7_A::NOERROR,
            true => FEIF7_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FEIF7_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FEIF7_A::ERROR
    }
}
#[doc = "Stream x transfer complete interrupt flag (x=7..4)"]
pub type TCIF6_A = TCIF7_A;
#[doc = "Reader of field `TCIF6`"]
pub type TCIF6_R = crate::R<bool, TCIF7_A>;
#[doc = "Stream x half transfer interrupt flag (x=7..4)"]
pub type HTIF6_A = HTIF7_A;
#[doc = "Reader of field `HTIF6`"]
pub type HTIF6_R = crate::R<bool, HTIF7_A>;
#[doc = "Stream x transfer error interrupt flag (x=7..4)"]
pub type TEIF6_A = TEIF7_A;
#[doc = "Reader of field `TEIF6`"]
pub type TEIF6_R = crate::R<bool, TEIF7_A>;
#[doc = "Stream x direct mode error interrupt flag (x=7..4)"]
pub type DMEIF6_A = DMEIF7_A;
#[doc = "Reader of field `DMEIF6`"]
pub type DMEIF6_R = crate::R<bool, DMEIF7_A>;
#[doc = "Stream x FIFO error interrupt flag (x=7..4)"]
pub type FEIF6_A = FEIF7_A;
#[doc = "Reader of field `FEIF6`"]
pub type FEIF6_R = crate::R<bool, FEIF7_A>;
#[doc = "Stream x transfer complete interrupt flag (x=7..4)"]
pub type TCIF5_A = TCIF7_A;
#[doc = "Reader of field `TCIF5`"]
pub type TCIF5_R = crate::R<bool, TCIF7_A>;
#[doc = "Stream x half transfer interrupt flag (x=7..4)"]
pub type HTIF5_A = HTIF7_A;
#[doc = "Reader of field `HTIF5`"]
pub type HTIF5_R = crate::R<bool, HTIF7_A>;
#[doc = "Stream x transfer error interrupt flag (x=7..4)"]
pub type TEIF5_A = TEIF7_A;
#[doc = "Reader of field `TEIF5`"]
pub type TEIF5_R = crate::R<bool, TEIF7_A>;
#[doc = "Stream x direct mode error interrupt flag (x=7..4)"]
pub type DMEIF5_A = DMEIF7_A;
#[doc = "Reader of field `DMEIF5`"]
pub type DMEIF5_R = crate::R<bool, DMEIF7_A>;
#[doc = "Stream x FIFO error interrupt flag (x=7..4)"]
pub type FEIF5_A = FEIF7_A;
#[doc = "Reader of field `FEIF5`"]
pub type FEIF5_R = crate::R<bool, FEIF7_A>;
#[doc = "Stream x transfer complete interrupt flag (x=7..4)"]
pub type TCIF4_A = TCIF7_A;
#[doc = "Reader of field `TCIF4`"]
pub type TCIF4_R = crate::R<bool, TCIF7_A>;
#[doc = "Stream x half transfer interrupt flag (x=7..4)"]
pub type HTIF4_A = HTIF7_A;
#[doc = "Reader of field `HTIF4`"]
pub type HTIF4_R = crate::R<bool, HTIF7_A>;
#[doc = "Stream x transfer error interrupt flag (x=7..4)"]
pub type TEIF4_A = TEIF7_A;
#[doc = "Reader of field `TEIF4`"]
pub type TEIF4_R = crate::R<bool, TEIF7_A>;
#[doc = "Stream x direct mode error interrupt flag (x=7..4)"]
pub type DMEIF4_A = DMEIF7_A;
#[doc = "Reader of field `DMEIF4`"]
pub type DMEIF4_R = crate::R<bool, DMEIF7_A>;
#[doc = "Stream x FIFO error interrupt flag (x=7..4)"]
pub type FEIF4_A = FEIF7_A;
#[doc = "Reader of field `FEIF4`"]
pub type FEIF4_R = crate::R<bool, FEIF7_A>;
impl R {
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif7(&self) -> DMEIF7_R {
        DMEIF7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif7(&self) -> FEIF7_R {
        FEIF7_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif6(&self) -> DMEIF6_R {
        DMEIF6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif6(&self) -> FEIF6_R {
        FEIF6_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif5(&self) -> DMEIF5_R {
        DMEIF5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif5(&self) -> FEIF5_R {
        FEIF5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif4(&self) -> DMEIF4_R {
        DMEIF4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif4(&self) -> FEIF4_R {
        FEIF4_R::new((self.bits & 0x01) != 0)
    }
}
