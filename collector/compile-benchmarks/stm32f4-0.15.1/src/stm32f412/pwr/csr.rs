#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WUF_R = crate::BitReader<bool>;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader<bool>;
#[doc = "Field `PVDO` reader - PVD output"]
pub type PVDO_R = crate::BitReader<bool>;
#[doc = "Field `BRR` reader - Backup regulator ready"]
pub type BRR_R = crate::BitReader<bool>;
#[doc = "Field `EWUP` reader - Enable WKUP pin"]
pub type EWUP_R = crate::BitReader<bool>;
#[doc = "Field `EWUP` writer - Enable WKUP pin"]
pub type EWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `BRE` reader - Backup regulator enable"]
pub type BRE_R = crate::BitReader<bool>;
#[doc = "Field `BRE` writer - Backup regulator enable"]
pub type BRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `VOSRDY` reader - Regulator voltage scaling output selection ready bit"]
pub type VOSRDY_R = crate::BitReader<bool>;
#[doc = "Field `VOSRDY` writer - Regulator voltage scaling output selection ready bit"]
pub type VOSRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Backup regulator ready"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn ewup(&self) -> EWUP_R {
        EWUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Backup regulator enable"]
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Regulator voltage scaling output selection ready bit"]
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn ewup(&mut self) -> EWUP_W<8> {
        EWUP_W::new(self)
    }
    #[doc = "Bit 9 - Backup regulator enable"]
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W<9> {
        BRE_W::new(self)
    }
    #[doc = "Bit 14 - Regulator voltage scaling output selection ready bit"]
    #[inline(always)]
    pub fn vosrdy(&mut self) -> VOSRDY_W<14> {
        VOSRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
