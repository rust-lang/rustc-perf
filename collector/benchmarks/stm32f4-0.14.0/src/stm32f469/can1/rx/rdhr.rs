#[doc = "Register `RDHR` reader"]
pub struct R(crate::R<RDHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA7` reader - DATA7"]
pub struct DATA7_R(crate::FieldReader<u8, u8>);
impl DATA7_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA6` reader - DATA6"]
pub struct DATA6_R(crate::FieldReader<u8, u8>);
impl DATA6_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA5` reader - DATA5"]
pub struct DATA5_R(crate::FieldReader<u8, u8>);
impl DATA5_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA4` reader - DATA4"]
pub struct DATA4_R(crate::FieldReader<u8, u8>);
impl DATA4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "receive FIFO mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdhr](index.html) module"]
pub struct RDHR_SPEC;
impl crate::RegisterSpec for RDHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdhr::R](R) reader structure"]
impl crate::Readable for RDHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDHR to value 0"]
impl crate::Resettable for RDHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
