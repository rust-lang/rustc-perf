#[doc = "Register `MACFFR` reader"]
pub struct R(crate::R<MACFFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACFFR` writer"]
pub struct W(crate::W<MACFFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MACFFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM` reader - PM"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - PM"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
#[doc = "Field `HU` reader - HU"]
pub type HU_R = crate::BitReader<bool>;
#[doc = "Field `HU` writer - HU"]
pub type HU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
#[doc = "Field `HM` reader - HM"]
pub type HM_R = crate::BitReader<bool>;
#[doc = "Field `HM` writer - HM"]
pub type HM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
#[doc = "Field `DAIF` reader - DAIF"]
pub type DAIF_R = crate::BitReader<bool>;
#[doc = "Field `DAIF` writer - DAIF"]
pub type DAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
#[doc = "Field `RAM` reader - RAM"]
pub type RAM_R = crate::BitReader<bool>;
#[doc = "Field `RAM` writer - RAM"]
pub type RAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
#[doc = "Field `BFD` reader - BFD"]
pub type BFD_R = crate::BitReader<bool>;
#[doc = "Field `BFD` writer - BFD"]
pub type BFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
#[doc = "Field `PCF` reader - PCF"]
pub type PCF_R = crate::BitReader<bool>;
#[doc = "Field `PCF` writer - PCF"]
pub type PCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
#[doc = "Field `SAIF` reader - SAIF"]
pub type SAIF_R = crate::BitReader<bool>;
#[doc = "Field `SAIF` writer - SAIF"]
pub type SAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
#[doc = "Field `SAF` reader - SAF"]
pub type SAF_R = crate::BitReader<bool>;
#[doc = "Field `SAF` writer - SAF"]
pub type SAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
#[doc = "Field `HPF` reader - HPF"]
pub type HPF_R = crate::BitReader<bool>;
#[doc = "Field `HPF` writer - HPF"]
pub type HPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
#[doc = "Field `RA` reader - RA"]
pub type RA_R = crate::BitReader<bool>;
#[doc = "Field `RA` writer - RA"]
pub type RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HU"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HM"]
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RAM"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BFD"]
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCF"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SAIF"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SAF"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HPF"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PM"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<0> {
        PM_W::new(self)
    }
    #[doc = "Bit 1 - HU"]
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<1> {
        HU_W::new(self)
    }
    #[doc = "Bit 2 - HM"]
    #[inline(always)]
    pub fn hm(&mut self) -> HM_W<2> {
        HM_W::new(self)
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W<3> {
        DAIF_W::new(self)
    }
    #[doc = "Bit 4 - RAM"]
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W<4> {
        RAM_W::new(self)
    }
    #[doc = "Bit 5 - BFD"]
    #[inline(always)]
    pub fn bfd(&mut self) -> BFD_W<5> {
        BFD_W::new(self)
    }
    #[doc = "Bit 6 - PCF"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W<6> {
        PCF_W::new(self)
    }
    #[doc = "Bit 7 - SAIF"]
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W<7> {
        SAIF_W::new(self)
    }
    #[doc = "Bit 8 - SAF"]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W<8> {
        SAF_W::new(self)
    }
    #[doc = "Bit 9 - HPF"]
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W<9> {
        HPF_W::new(self)
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<31> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC frame filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macffr](index.html) module"]
pub struct MACFFR_SPEC;
impl crate::RegisterSpec for MACFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macffr::R](R) reader structure"]
impl crate::Readable for MACFFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macffr::W](W) writer structure"]
impl crate::Writable for MACFFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACFFR to value 0"]
impl crate::Resettable for MACFFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
