#[doc = "Register `HISR` reader"]
pub struct R(crate::R<HISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Stream x direct mode error interrupt flag (x=7..4)"]
pub use DMEIF4_A as DMEIF7_A;
#[doc = "Stream x direct mode error interrupt flag (x=7..4)"]
pub use DMEIF4_A as DMEIF6_A;
#[doc = "Stream x direct mode error interrupt flag (x=7..4)"]
pub use DMEIF4_A as DMEIF5_A;
#[doc = "Field `DMEIF7` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub use DMEIF4_R as DMEIF7_R;
#[doc = "Field `DMEIF6` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub use DMEIF4_R as DMEIF6_R;
#[doc = "Field `DMEIF5` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub use DMEIF4_R as DMEIF5_R;
#[doc = "Stream x FIFO error interrupt flag (x=7..4)"]
pub use FEIF4_A as FEIF7_A;
#[doc = "Stream x FIFO error interrupt flag (x=7..4)"]
pub use FEIF4_A as FEIF6_A;
#[doc = "Stream x FIFO error interrupt flag (x=7..4)"]
pub use FEIF4_A as FEIF5_A;
#[doc = "Field `FEIF7` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub use FEIF4_R as FEIF7_R;
#[doc = "Field `FEIF6` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub use FEIF4_R as FEIF6_R;
#[doc = "Field `FEIF5` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub use FEIF4_R as FEIF5_R;
#[doc = "Stream x half transfer interrupt flag (x=7..4)"]
pub use HTIF4_A as HTIF7_A;
#[doc = "Stream x half transfer interrupt flag (x=7..4)"]
pub use HTIF4_A as HTIF6_A;
#[doc = "Stream x half transfer interrupt flag (x=7..4)"]
pub use HTIF4_A as HTIF5_A;
#[doc = "Field `HTIF7` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub use HTIF4_R as HTIF7_R;
#[doc = "Field `HTIF6` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub use HTIF4_R as HTIF6_R;
#[doc = "Field `HTIF5` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub use HTIF4_R as HTIF5_R;
#[doc = "Stream x transfer complete interrupt flag (x=7..4)"]
pub use TCIF4_A as TCIF7_A;
#[doc = "Stream x transfer complete interrupt flag (x=7..4)"]
pub use TCIF4_A as TCIF6_A;
#[doc = "Stream x transfer complete interrupt flag (x=7..4)"]
pub use TCIF4_A as TCIF5_A;
#[doc = "Field `TCIF7` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub use TCIF4_R as TCIF7_R;
#[doc = "Field `TCIF6` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub use TCIF4_R as TCIF6_R;
#[doc = "Field `TCIF5` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub use TCIF4_R as TCIF5_R;
#[doc = "Stream x transfer error interrupt flag (x=7..4)"]
pub use TEIF4_A as TEIF7_A;
#[doc = "Stream x transfer error interrupt flag (x=7..4)"]
pub use TEIF4_A as TEIF6_A;
#[doc = "Stream x transfer error interrupt flag (x=7..4)"]
pub use TEIF4_A as TEIF5_A;
#[doc = "Field `TEIF7` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub use TEIF4_R as TEIF7_R;
#[doc = "Field `TEIF6` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub use TEIF4_R as TEIF6_R;
#[doc = "Field `TEIF5` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub use TEIF4_R as TEIF5_R;
#[doc = "Stream x transfer complete interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF4_A {
    #[doc = "0: No transfer complete event on stream x"]
    NotComplete = 0,
    #[doc = "1: A transfer complete event occurred on stream x"]
    Complete = 1,
}
impl From<TCIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF4` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type TCIF4_R = crate::BitReader<TCIF4_A>;
impl TCIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF4_A {
        match self.bits {
            false => TCIF4_A::NotComplete,
            true => TCIF4_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF4_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF4_A::Complete
    }
}
#[doc = "Stream x half transfer interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF4_A {
    #[doc = "0: No half transfer event on stream x"]
    NotHalf = 0,
    #[doc = "1: A half transfer event occurred on stream x"]
    Half = 1,
}
impl From<HTIF4_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF4` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type HTIF4_R = crate::BitReader<HTIF4_A>;
impl HTIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF4_A {
        match self.bits {
            false => HTIF4_A::NotHalf,
            true => HTIF4_A::Half,
        }
    }
    #[doc = "Checks if the value of the field is `NotHalf`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF4_A::NotHalf
    }
    #[doc = "Checks if the value of the field is `Half`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF4_A::Half
    }
}
#[doc = "Stream x transfer error interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF4_A {
    #[doc = "0: No transfer error on stream x"]
    NoError = 0,
    #[doc = "1: A transfer error occurred on stream x"]
    Error = 1,
}
impl From<TEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF4` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type TEIF4_R = crate::BitReader<TEIF4_A>;
impl TEIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF4_A {
        match self.bits {
            false => TEIF4_A::NoError,
            true => TEIF4_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF4_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF4_A::Error
    }
}
#[doc = "Stream x direct mode error interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIF4_A {
    #[doc = "0: No Direct Mode error on stream x"]
    NoError = 0,
    #[doc = "1: A Direct Mode error occurred on stream x"]
    Error = 1,
}
impl From<DMEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: DMEIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMEIF4` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type DMEIF4_R = crate::BitReader<DMEIF4_A>;
impl DMEIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMEIF4_A {
        match self.bits {
            false => DMEIF4_A::NoError,
            true => DMEIF4_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == DMEIF4_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == DMEIF4_A::Error
    }
}
#[doc = "Stream x FIFO error interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIF4_A {
    #[doc = "0: No FIFO error event on stream x"]
    NoError = 0,
    #[doc = "1: A FIFO error event occurred on stream x"]
    Error = 1,
}
impl From<FEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: FEIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIF4` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type FEIF4_R = crate::BitReader<FEIF4_A>;
impl FEIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIF4_A {
        match self.bits {
            false => FEIF4_A::NoError,
            true => FEIF4_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FEIF4_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FEIF4_A::Error
    }
}
impl R {
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif7(&self) -> DMEIF7_R {
        DMEIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif7(&self) -> FEIF7_R {
        FEIF7_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif6(&self) -> DMEIF6_R {
        DMEIF6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif6(&self) -> FEIF6_R {
        FEIF6_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif5(&self) -> DMEIF5_R {
        DMEIF5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif5(&self) -> FEIF5_R {
        FEIF5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeif4(&self) -> DMEIF4_R {
        DMEIF4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feif4(&self) -> FEIF4_R {
        FEIF4_R::new((self.bits & 1) != 0)
    }
}
#[doc = "high interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hisr](index.html) module"]
pub struct HISR_SPEC;
impl crate::RegisterSpec for HISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hisr::R](R) reader structure"]
impl crate::Readable for HISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HISR to value 0"]
impl crate::Resettable for HISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
