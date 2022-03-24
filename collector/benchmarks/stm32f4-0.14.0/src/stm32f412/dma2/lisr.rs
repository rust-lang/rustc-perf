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
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF3_A = TCIF0_A;
#[doc = "Field `TCIF3` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF3_R = TCIF0_R;
#[doc = "Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF3_A = HTIF0_A;
#[doc = "Field `HTIF3` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF3_R = HTIF0_R;
#[doc = "Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF3_A = TEIF0_A;
#[doc = "Field `TEIF3` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF3_R = TEIF0_R;
#[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF3_A = DMEIF0_A;
#[doc = "Field `DMEIF3` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF3_R = DMEIF0_R;
#[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF3_A = FEIF0_A;
#[doc = "Field `FEIF3` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF3_R = FEIF0_R;
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF2_A = TCIF0_A;
#[doc = "Field `TCIF2` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF2_R = TCIF0_R;
#[doc = "Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF2_A = HTIF0_A;
#[doc = "Field `HTIF2` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF2_R = HTIF0_R;
#[doc = "Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF2_A = TEIF0_A;
#[doc = "Field `TEIF2` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF2_R = TEIF0_R;
#[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF2_A = DMEIF0_A;
#[doc = "Field `DMEIF2` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF2_R = DMEIF0_R;
#[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF2_A = FEIF0_A;
#[doc = "Field `FEIF2` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF2_R = FEIF0_R;
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF1_A = TCIF0_A;
#[doc = "Field `TCIF1` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF1_R = TCIF0_R;
#[doc = "Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF1_A = HTIF0_A;
#[doc = "Field `HTIF1` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF1_R = HTIF0_R;
#[doc = "Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF1_A = TEIF0_A;
#[doc = "Field `TEIF1` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF1_R = TEIF0_R;
#[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF1_A = DMEIF0_A;
#[doc = "Field `DMEIF1` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF1_R = DMEIF0_R;
#[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF1_A = FEIF0_A;
#[doc = "Field `FEIF1` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF1_R = FEIF0_R;
#[doc = "Stream x transfer complete interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF0_A {
    #[doc = "0: No transfer complete event on stream x"]
    NOTCOMPLETE = 0,
    #[doc = "1: A transfer complete event occurred on stream x"]
    COMPLETE = 1,
}
impl From<TCIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF0` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub struct TCIF0_R(crate::FieldReader<bool, TCIF0_A>);
impl TCIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF0_A {
        match self.bits {
            false => TCIF0_A::NOTCOMPLETE,
            true => TCIF0_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == TCIF0_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == TCIF0_A::COMPLETE
    }
}
impl core::ops::Deref for TCIF0_R {
    type Target = crate::FieldReader<bool, TCIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stream x half transfer interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF0_A {
    #[doc = "0: No half transfer event on stream x"]
    NOTHALF = 0,
    #[doc = "1: A half transfer event occurred on stream x"]
    HALF = 1,
}
impl From<HTIF0_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF0` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub struct HTIF0_R(crate::FieldReader<bool, HTIF0_A>);
impl HTIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF0_A {
        match self.bits {
            false => HTIF0_A::NOTHALF,
            true => HTIF0_A::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        **self == HTIF0_A::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == HTIF0_A::HALF
    }
}
impl core::ops::Deref for HTIF0_R {
    type Target = crate::FieldReader<bool, HTIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stream x transfer error interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF0_A {
    #[doc = "0: No transfer error on stream x"]
    NOERROR = 0,
    #[doc = "1: A transfer error occurred on stream x"]
    ERROR = 1,
}
impl From<TEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF0` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub struct TEIF0_R(crate::FieldReader<bool, TEIF0_A>);
impl TEIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF0_A {
        match self.bits {
            false => TEIF0_A::NOERROR,
            true => TEIF0_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == TEIF0_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == TEIF0_A::ERROR
    }
}
impl core::ops::Deref for TEIF0_R {
    type Target = crate::FieldReader<bool, TEIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stream x direct mode error interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIF0_A {
    #[doc = "0: No Direct Mode error on stream x"]
    NOERROR = 0,
    #[doc = "1: A Direct Mode error occurred on stream x"]
    ERROR = 1,
}
impl From<DMEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: DMEIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMEIF0` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub struct DMEIF0_R(crate::FieldReader<bool, DMEIF0_A>);
impl DMEIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMEIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMEIF0_A {
        match self.bits {
            false => DMEIF0_A::NOERROR,
            true => DMEIF0_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == DMEIF0_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == DMEIF0_A::ERROR
    }
}
impl core::ops::Deref for DMEIF0_R {
    type Target = crate::FieldReader<bool, DMEIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stream x FIFO error interrupt flag (x=3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIF0_A {
    #[doc = "0: No FIFO error event on stream x"]
    NOERROR = 0,
    #[doc = "1: A FIFO error event occurred on stream x"]
    ERROR = 1,
}
impl From<FEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: FEIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIF0` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub struct FEIF0_R(crate::FieldReader<bool, FEIF0_A>);
impl FEIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIF0_A {
        match self.bits {
            false => FEIF0_A::NOERROR,
            true => FEIF0_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == FEIF0_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == FEIF0_A::ERROR
    }
}
impl core::ops::Deref for FEIF0_R {
    type Target = crate::FieldReader<bool, FEIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
