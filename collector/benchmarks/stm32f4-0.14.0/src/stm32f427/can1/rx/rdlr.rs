#[doc = "Register `RDLR` reader"]
pub struct R(crate::R<RDLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA3` reader - DATA3"]
pub struct DATA3_R(crate::FieldReader<u8, u8>);
impl DATA3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA2` reader - DATA2"]
pub struct DATA2_R(crate::FieldReader<u8, u8>);
impl DATA2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA1` reader - DATA1"]
pub struct DATA1_R(crate::FieldReader<u8, u8>);
impl DATA1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA0` reader - DATA0"]
pub struct DATA0_R(crate::FieldReader<u8, u8>);
impl DATA0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdlr](index.html) module"]
pub struct RDLR_SPEC;
impl crate::RegisterSpec for RDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdlr::R](R) reader structure"]
impl crate::Readable for RDLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDLR to value 0"]
impl crate::Resettable for RDLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
