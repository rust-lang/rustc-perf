#[doc = "Register `LISR` reader"]
pub struct R(crate::R<LISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
pub use DMEIF0_A as DMEIF3_A;
#[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
pub use DMEIF0_A as DMEIF2_A;
#[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
pub use DMEIF0_A as DMEIF1_A;
#[doc = "Field `DMEIF3` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub use DMEIF0_R as DMEIF3_R;
#[doc = "Field `DMEIF2` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub use DMEIF0_R as DMEIF2_R;
#[doc = "Field `DMEIF1` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub use DMEIF0_R as DMEIF1_R;
#[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
pub use FEIF0_A as FEIF3_A;
#[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
pub use FEIF0_A as FEIF2_A;
#[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
pub use FEIF0_A as FEIF1_A;
#[doc = "Field `FEIF3` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub use FEIF0_R as FEIF3_R;
#[doc = "Field `FEIF2` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub use FEIF0_R as FEIF2_R;
#[doc = "Field `FEIF1` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub use FEIF0_R as FEIF1_R;
#[doc = "Stream x half transfer interrupt flag (x=3..0)"]
pub use HTIF0_A as HTIF3_A;
#[doc = "Stream x half transfer interrupt flag (x=3..0)"]
pub use HTIF0_A as HTIF2_A;
#[doc = "Stream x half transfer interrupt flag (x=3..0)"]
pub use HTIF0_A as HTIF1_A;
#[doc = "Field `HTIF3` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub use HTIF0_R as HTIF3_R;
#[doc = "Field `HTIF2` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub use HTIF0_R as HTIF2_R;
#[doc = "Field `HTIF1` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub use HTIF0_R as HTIF1_R;
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
pub use TCIF0_A as TCIF3_A;
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
pub use TCIF0_A as TCIF2_A;
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
pub use TCIF0_A as TCIF1_A;
#[doc = "Field `TCIF3` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub use TCIF0_R as TCIF3_R;
#[doc = "Field `TCIF2` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub use TCIF0_R as TCIF2_R;
#[doc = "Field `TCIF1` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub use TCIF0_R as TCIF1_R;
#[doc = "Stream x transfer error interrupt flag (x=3..0)"]
pub use TEIF0_A as TEIF3_A;
#[doc = "Stream x transfer error interrupt flag (x=3..0)"]
pub use TEIF0_A as TEIF2_A;
#[doc = "Stream x transfer error interrupt flag (x=3..0)"]
pub use TEIF0_A as TEIF1_A;
#[doc = "Field `TEIF3` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub use TEIF0_R as TEIF3_R;
#[doc = "Field `TEIF2` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub use TEIF0_R as TEIF2_R;
#[doc = "Field `TEIF1` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub use TEIF0_R as TEIF1_R;
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF0_A {
    #[doc = "0: No transfer complete event on stream x"]
    NotComplete = 0,
    #[doc = "1: A transfer complete event occurred on stream x"]
    Complete = 1,
}
impl From<TCIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF0` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF0_R = crate::BitReader<TCIF0_A>;
impl TCIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF0_A {
        match self.bits {
            false => TCIF0_A::NotComplete,
            true => TCIF0_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF0_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF0_A::Complete
    }
}
#[doc = "Stream x half transfer interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF0_A {
    #[doc = "0: No half transfer event on stream x"]
    NotHalf = 0,
    #[doc = "1: A half transfer event occurred on stream x"]
    Half = 1,
}
impl From<HTIF0_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF0` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF0_R = crate::BitReader<HTIF0_A>;
impl HTIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF0_A {
        match self.bits {
            false => HTIF0_A::NotHalf,
            true => HTIF0_A::Half,
        }
    }
    #[doc = "Checks if the value of the field is `NotHalf`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF0_A::NotHalf
    }
    #[doc = "Checks if the value of the field is `Half`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF0_A::Half
    }
}
#[doc = "Stream x transfer error interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF0_A {
    #[doc = "0: No transfer error on stream x"]
    NoError = 0,
    #[doc = "1: A transfer error occurred on stream x"]
    Error = 1,
}
impl From<TEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF0` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF0_R = crate::BitReader<TEIF0_A>;
impl TEIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF0_A {
        match self.bits {
            false => TEIF0_A::NoError,
            true => TEIF0_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF0_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF0_A::Error
    }
}
#[doc = "Stream x direct mode error interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIF0_A {
    #[doc = "0: No Direct Mode error on stream x"]
    NoError = 0,
    #[doc = "1: A Direct Mode error occurred on stream x"]
    Error = 1,
}
impl From<DMEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: DMEIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMEIF0` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF0_R = crate::BitReader<DMEIF0_A>;
impl DMEIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMEIF0_A {
        match self.bits {
            false => DMEIF0_A::NoError,
            true => DMEIF0_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == DMEIF0_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == DMEIF0_A::Error
    }
}
#[doc = "Stream x FIFO error interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIF0_A {
    #[doc = "0: No FIFO error event on stream x"]
    NoError = 0,
    #[doc = "1: A FIFO error event occurred on stream x"]
    Error = 1,
}
impl From<FEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: FEIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIF0` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF0_R = crate::BitReader<FEIF0_A>;
impl FEIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIF0_A {
        match self.bits {
            false => FEIF0_A::NoError,
            true => FEIF0_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FEIF0_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FEIF0_A::Error
    }
}
impl R {
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif3(&self) -> DMEIF3_R {
        DMEIF3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif3(&self) -> FEIF3_R {
        FEIF3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif2(&self) -> DMEIF2_R {
        DMEIF2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif2(&self) -> FEIF2_R {
        FEIF2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif1(&self) -> DMEIF1_R {
        DMEIF1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif1(&self) -> FEIF1_R {
        FEIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif0(&self) -> DMEIF0_R {
        DMEIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif0(&self) -> FEIF0_R {
        FEIF0_R::new((self.bits & 1) != 0)
    }
}
#[doc = "low interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lisr](index.html) module"]
pub struct LISR_SPEC;
impl crate::RegisterSpec for LISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lisr::R](R) reader structure"]
impl crate::Readable for LISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LISR to value 0"]
impl crate::Resettable for LISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
