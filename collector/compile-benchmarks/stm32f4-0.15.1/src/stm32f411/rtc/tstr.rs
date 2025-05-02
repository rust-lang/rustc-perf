#[doc = "Register `TSTR` reader"]
pub struct R(crate::R<TSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `HT` reader - Hour tens in BCD format"]
pub type HT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HU` reader - Hour units in BCD format"]
pub type HU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MNU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST` reader - Second tens in BCD format"]
pub type ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SU` reader - Second units in BCD format"]
pub type SU_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "time stamp time register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstr](index.html) module"]
pub struct TSTR_SPEC;
impl crate::RegisterSpec for TSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tstr::R](R) reader structure"]
impl crate::Readable for TSTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSTR to value 0"]
impl crate::Resettable for TSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
