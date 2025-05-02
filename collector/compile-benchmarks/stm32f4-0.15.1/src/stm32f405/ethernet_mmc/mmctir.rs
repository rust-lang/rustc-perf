#[doc = "Register `MMCTIR` reader"]
pub struct R(crate::R<MMCTIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TGFSCS` reader - TGFSCS"]
pub type TGFSCS_R = crate::BitReader<bool>;
#[doc = "Field `TGFMSCS` reader - TGFMSCS"]
pub type TGFMSCS_R = crate::BitReader<bool>;
#[doc = "Field `TGFS` reader - TGFS"]
pub type TGFS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 14 - TGFSCS"]
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TGFMSCS"]
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - TGFS"]
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Ethernet MMC transmit interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctir](index.html) module"]
pub struct MMCTIR_SPEC;
impl crate::RegisterSpec for MMCTIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctir::R](R) reader structure"]
impl crate::Readable for MMCTIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMCTIR to value 0"]
impl crate::Resettable for MMCTIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
