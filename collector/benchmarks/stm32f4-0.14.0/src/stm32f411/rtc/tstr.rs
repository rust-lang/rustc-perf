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
pub struct PM_R(crate::FieldReader<bool, bool>);
impl PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HT` reader - Hour tens in BCD format"]
pub struct HT_R(crate::FieldReader<u8, u8>);
impl HT_R {
    pub(crate) fn new(bits: u8) -> Self {
        HT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HU` reader - Hour units in BCD format"]
pub struct HU_R(crate::FieldReader<u8, u8>);
impl HU_R {
    pub(crate) fn new(bits: u8) -> Self {
        HU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub struct MNT_R(crate::FieldReader<u8, u8>);
impl MNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub struct MNU_R(crate::FieldReader<u8, u8>);
impl MNU_R {
    pub(crate) fn new(bits: u8) -> Self {
        MNU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MNU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST` reader - Second tens in BCD format"]
pub struct ST_R(crate::FieldReader<u8, u8>);
impl ST_R {
    pub(crate) fn new(bits: u8) -> Self {
        ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SU` reader - Second units in BCD format"]
pub struct SU_R(crate::FieldReader<u8, u8>);
impl SU_R {
    pub(crate) fn new(bits: u8) -> Self {
        SU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 0x07) as u8)
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
