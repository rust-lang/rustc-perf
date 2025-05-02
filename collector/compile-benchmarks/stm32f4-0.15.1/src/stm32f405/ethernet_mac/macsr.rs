#[doc = "Register `MACSR` reader"]
pub struct R(crate::R<MACSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACSR` writer"]
pub struct W(crate::W<MACSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACSR_SPEC>;
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
impl From<crate::W<MACSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMTS` reader - PMTS"]
pub type PMTS_R = crate::BitReader<bool>;
#[doc = "Field `MMCS` reader - MMCS"]
pub type MMCS_R = crate::BitReader<bool>;
#[doc = "Field `MMCRS` reader - MMCRS"]
pub type MMCRS_R = crate::BitReader<bool>;
#[doc = "Field `MMCTS` reader - MMCTS"]
pub type MMCTS_R = crate::BitReader<bool>;
#[doc = "Field `TSTS` reader - TSTS"]
pub type TSTS_R = crate::BitReader<bool>;
#[doc = "Field `TSTS` writer - TSTS"]
pub type TSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - PMTS"]
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMCS"]
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMCRS"]
    #[inline(always)]
    pub fn mmcrs(&self) -> MMCRS_R {
        MMCRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMCTS"]
    #[inline(always)]
    pub fn mmcts(&self) -> MMCTS_R {
        MMCTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - TSTS"]
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - TSTS"]
    #[inline(always)]
    pub fn tsts(&mut self) -> TSTS_W<9> {
        TSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macsr](index.html) module"]
pub struct MACSR_SPEC;
impl crate::RegisterSpec for MACSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macsr::R](R) reader structure"]
impl crate::Readable for MACSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macsr::W](W) writer structure"]
impl crate::Writable for MACSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACSR to value 0"]
impl crate::Resettable for MACSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
