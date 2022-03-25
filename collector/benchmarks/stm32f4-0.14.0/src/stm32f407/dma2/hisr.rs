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
#[doc = "Stream x transfer complete interrupt flag (x=7..4)"]
pub type TCIF7_A = TCIF4_A;
#[doc = "Field `TCIF7` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type TCIF7_R = TCIF4_R;
#[doc = "Stream x half transfer interrupt flag (x=7..4)"]
pub type HTIF7_A = HTIF4_A;
#[doc = "Field `HTIF7` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type HTIF7_R = HTIF4_R;
#[doc = "Stream x transfer error interrupt flag (x=7..4)"]
pub type TEIF7_A = TEIF4_A;
#[doc = "Field `TEIF7` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type TEIF7_R = TEIF4_R;
#[doc = "Stream x direct mode error interrupt flag (x=7..4)"]
pub type DMEIF7_A = DMEIF4_A;
#[doc = "Field `DMEIF7` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type DMEIF7_R = DMEIF4_R;
#[doc = "Stream x FIFO error interrupt flag (x=7..4)"]
pub type FEIF7_A = FEIF4_A;
#[doc = "Field `FEIF7` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type FEIF7_R = FEIF4_R;
#[doc = "Stream x transfer complete interrupt flag (x=7..4)"]
pub type TCIF6_A = TCIF4_A;
#[doc = "Field `TCIF6` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type TCIF6_R = TCIF4_R;
#[doc = "Stream x half transfer interrupt flag (x=7..4)"]
pub type HTIF6_A = HTIF4_A;
#[doc = "Field `HTIF6` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type HTIF6_R = HTIF4_R;
#[doc = "Stream x transfer error interrupt flag (x=7..4)"]
pub type TEIF6_A = TEIF4_A;
#[doc = "Field `TEIF6` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type TEIF6_R = TEIF4_R;
#[doc = "Stream x direct mode error interrupt flag (x=7..4)"]
pub type DMEIF6_A = DMEIF4_A;
#[doc = "Field `DMEIF6` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type DMEIF6_R = DMEIF4_R;
#[doc = "Stream x FIFO error interrupt flag (x=7..4)"]
pub type FEIF6_A = FEIF4_A;
#[doc = "Field `FEIF6` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type FEIF6_R = FEIF4_R;
#[doc = "Stream x transfer complete interrupt flag (x=7..4)"]
pub type TCIF5_A = TCIF4_A;
#[doc = "Field `TCIF5` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type TCIF5_R = TCIF4_R;
#[doc = "Stream x half transfer interrupt flag (x=7..4)"]
pub type HTIF5_A = HTIF4_A;
#[doc = "Field `HTIF5` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type HTIF5_R = HTIF4_R;
#[doc = "Stream x transfer error interrupt flag (x=7..4)"]
pub type TEIF5_A = TEIF4_A;
#[doc = "Field `TEIF5` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type TEIF5_R = TEIF4_R;
#[doc = "Stream x direct mode error interrupt flag (x=7..4)"]
pub type DMEIF5_A = DMEIF4_A;
#[doc = "Field `DMEIF5` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type DMEIF5_R = DMEIF4_R;
#[doc = "Stream x FIFO error interrupt flag (x=7..4)"]
pub type FEIF5_A = FEIF4_A;
#[doc = "Field `FEIF5` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type FEIF5_R = FEIF4_R;
#[doc = "Stream x transfer complete interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF4_A {
    #[doc = "0: No transfer complete event on stream x"]
    NOTCOMPLETE = 0,
    #[doc = "1: A transfer complete event occurred on stream x"]
    COMPLETE = 1,
}
impl From<TCIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF4` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub struct TCIF4_R(crate::FieldReader<bool, TCIF4_A>);
impl TCIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF4_A {
        match self.bits {
            false => TCIF4_A::NOTCOMPLETE,
            true => TCIF4_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == TCIF4_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == TCIF4_A::COMPLETE
    }
}
impl core::ops::Deref for TCIF4_R {
    type Target = crate::FieldReader<bool, TCIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stream x half transfer interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF4_A {
    #[doc = "0: No half transfer event on stream x"]
    NOTHALF = 0,
    #[doc = "1: A half transfer event occurred on stream x"]
    HALF = 1,
}
impl From<HTIF4_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF4` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub struct HTIF4_R(crate::FieldReader<bool, HTIF4_A>);
impl HTIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF4_A {
        match self.bits {
            false => HTIF4_A::NOTHALF,
            true => HTIF4_A::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        **self == HTIF4_A::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == HTIF4_A::HALF
    }
}
impl core::ops::Deref for HTIF4_R {
    type Target = crate::FieldReader<bool, HTIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stream x transfer error interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF4_A {
    #[doc = "0: No transfer error on stream x"]
    NOERROR = 0,
    #[doc = "1: A transfer error occurred on stream x"]
    ERROR = 1,
}
impl From<TEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF4` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub struct TEIF4_R(crate::FieldReader<bool, TEIF4_A>);
impl TEIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF4_A {
        match self.bits {
            false => TEIF4_A::NOERROR,
            true => TEIF4_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == TEIF4_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == TEIF4_A::ERROR
    }
}
impl core::ops::Deref for TEIF4_R {
    type Target = crate::FieldReader<bool, TEIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stream x direct mode error interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIF4_A {
    #[doc = "0: No Direct Mode error on stream x"]
    NOERROR = 0,
    #[doc = "1: A Direct Mode error occurred on stream x"]
    ERROR = 1,
}
impl From<DMEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: DMEIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMEIF4` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub struct DMEIF4_R(crate::FieldReader<bool, DMEIF4_A>);
impl DMEIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMEIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMEIF4_A {
        match self.bits {
            false => DMEIF4_A::NOERROR,
            true => DMEIF4_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == DMEIF4_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == DMEIF4_A::ERROR
    }
}
impl core::ops::Deref for DMEIF4_R {
    type Target = crate::FieldReader<bool, DMEIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stream x FIFO error interrupt flag (x=7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIF4_A {
    #[doc = "0: No FIFO error event on stream x"]
    NOERROR = 0,
    #[doc = "1: A FIFO error event occurred on stream x"]
    ERROR = 1,
}
impl From<FEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: FEIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIF4` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub struct FEIF4_R(crate::FieldReader<bool, FEIF4_A>);
impl FEIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIF4_A {
        match self.bits {
            false => FEIF4_A::NOERROR,
            true => FEIF4_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == FEIF4_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == FEIF4_A::ERROR
    }
}
impl core::ops::Deref for FEIF4_R {
    type Target = crate::FieldReader<bool, FEIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
