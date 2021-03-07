#[doc = "Reader of register LISR"]
pub type R = crate::R<u32, super::LISR>;
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF3_A {
    #[doc = "0: No transfer complete event on stream x"]
    NOTCOMPLETE = 0,
    #[doc = "1: A transfer complete event occurred on stream x"]
    COMPLETE = 1,
}
impl From<TCIF3_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF3`"]
pub type TCIF3_R = crate::R<bool, TCIF3_A>;
impl TCIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF3_A {
        match self.bits {
            false => TCIF3_A::NOTCOMPLETE,
            true => TCIF3_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF3_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF3_A::COMPLETE
    }
}
#[doc = "Stream x half transfer interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF3_A {
    #[doc = "0: No half transfer event on stream x"]
    NOTHALF = 0,
    #[doc = "1: A half transfer event occurred on stream x"]
    HALF = 1,
}
impl From<HTIF3_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HTIF3`"]
pub type HTIF3_R = crate::R<bool, HTIF3_A>;
impl HTIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF3_A {
        match self.bits {
            false => HTIF3_A::NOTHALF,
            true => HTIF3_A::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF3_A::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF3_A::HALF
    }
}
#[doc = "Stream x transfer error interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF3_A {
    #[doc = "0: No transfer error on stream x"]
    NOERROR = 0,
    #[doc = "1: A transfer error occurred on stream x"]
    ERROR = 1,
}
impl From<TEIF3_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF3`"]
pub type TEIF3_R = crate::R<bool, TEIF3_A>;
impl TEIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF3_A {
        match self.bits {
            false => TEIF3_A::NOERROR,
            true => TEIF3_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF3_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF3_A::ERROR
    }
}
#[doc = "Stream x direct mode error interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIF3_A {
    #[doc = "0: No Direct Mode error on stream x"]
    NOERROR = 0,
    #[doc = "1: A Direct Mode error occurred on stream x"]
    ERROR = 1,
}
impl From<DMEIF3_A> for bool {
    #[inline(always)]
    fn from(variant: DMEIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMEIF3`"]
pub type DMEIF3_R = crate::R<bool, DMEIF3_A>;
impl DMEIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMEIF3_A {
        match self.bits {
            false => DMEIF3_A::NOERROR,
            true => DMEIF3_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == DMEIF3_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == DMEIF3_A::ERROR
    }
}
#[doc = "Stream x FIFO error interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIF3_A {
    #[doc = "0: No FIFO error event on stream x"]
    NOERROR = 0,
    #[doc = "1: A FIFO error event occurred on stream x"]
    ERROR = 1,
}
impl From<FEIF3_A> for bool {
    #[inline(always)]
    fn from(variant: FEIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FEIF3`"]
pub type FEIF3_R = crate::R<bool, FEIF3_A>;
impl FEIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIF3_A {
        match self.bits {
            false => FEIF3_A::NOERROR,
            true => FEIF3_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FEIF3_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FEIF3_A::ERROR
    }
}
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF2_A = TCIF3_A;
#[doc = "Reader of field `TCIF2`"]
pub type TCIF2_R = crate::R<bool, TCIF3_A>;
#[doc = "Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF2_A = HTIF3_A;
#[doc = "Reader of field `HTIF2`"]
pub type HTIF2_R = crate::R<bool, HTIF3_A>;
#[doc = "Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF2_A = TEIF3_A;
#[doc = "Reader of field `TEIF2`"]
pub type TEIF2_R = crate::R<bool, TEIF3_A>;
#[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF2_A = DMEIF3_A;
#[doc = "Reader of field `DMEIF2`"]
pub type DMEIF2_R = crate::R<bool, DMEIF3_A>;
#[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF2_A = FEIF3_A;
#[doc = "Reader of field `FEIF2`"]
pub type FEIF2_R = crate::R<bool, FEIF3_A>;
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF1_A = TCIF3_A;
#[doc = "Reader of field `TCIF1`"]
pub type TCIF1_R = crate::R<bool, TCIF3_A>;
#[doc = "Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF1_A = HTIF3_A;
#[doc = "Reader of field `HTIF1`"]
pub type HTIF1_R = crate::R<bool, HTIF3_A>;
#[doc = "Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF1_A = TEIF3_A;
#[doc = "Reader of field `TEIF1`"]
pub type TEIF1_R = crate::R<bool, TEIF3_A>;
#[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF1_A = DMEIF3_A;
#[doc = "Reader of field `DMEIF1`"]
pub type DMEIF1_R = crate::R<bool, DMEIF3_A>;
#[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF1_A = FEIF3_A;
#[doc = "Reader of field `FEIF1`"]
pub type FEIF1_R = crate::R<bool, FEIF3_A>;
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF0_A = TCIF3_A;
#[doc = "Reader of field `TCIF0`"]
pub type TCIF0_R = crate::R<bool, TCIF3_A>;
#[doc = "Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF0_A = HTIF3_A;
#[doc = "Reader of field `HTIF0`"]
pub type HTIF0_R = crate::R<bool, HTIF3_A>;
#[doc = "Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF0_A = TEIF3_A;
#[doc = "Reader of field `TEIF0`"]
pub type TEIF0_R = crate::R<bool, TEIF3_A>;
#[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF0_A = DMEIF3_A;
#[doc = "Reader of field `DMEIF0`"]
pub type DMEIF0_R = crate::R<bool, DMEIF3_A>;
#[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF0_A = FEIF3_A;
#[doc = "Reader of field `FEIF0`"]
pub type FEIF0_R = crate::R<bool, FEIF3_A>;
impl R {
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif3(&self) -> DMEIF3_R {
        DMEIF3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif3(&self) -> FEIF3_R {
        FEIF3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif2(&self) -> DMEIF2_R {
        DMEIF2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif2(&self) -> FEIF2_R {
        FEIF2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif1(&self) -> DMEIF1_R {
        DMEIF1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif1(&self) -> FEIF1_R {
        FEIF1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif0(&self) -> DMEIF0_R {
        DMEIF0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif0(&self) -> FEIF0_R {
        FEIF0_R::new((self.bits & 0x01) != 0)
    }
}
