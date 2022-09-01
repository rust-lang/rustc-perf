#[doc = "Register `RESPCMD` reader"]
pub struct R(crate::R<RESPCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESPCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESPCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESPCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPCMD` reader - Response command index Read-only bit field. Contains the command index of the last command response received."]
pub struct RESPCMD_R(crate::FieldReader<u8, u8>);
impl RESPCMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESPCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPCMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Response command index Read-only bit field. Contains the command index of the last command response received."]
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [respcmd](index.html) module"]
pub struct RESPCMD_SPEC;
impl crate::RegisterSpec for RESPCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [respcmd::R](R) reader structure"]
impl crate::Readable for RESPCMD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESPCMD to value 0"]
impl crate::Resettable for RESPCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
