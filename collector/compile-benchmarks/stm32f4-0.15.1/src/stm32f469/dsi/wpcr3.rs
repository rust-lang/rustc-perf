#[doc = "Register `WPCR3` reader"]
pub struct R(crate::R<WPCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR3` writer"]
pub struct W(crate::W<WPCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR3_SPEC>;
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
impl From<crate::W<WPCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THSTRAIL` reader - tHSTRAIL"]
pub type THSTRAIL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THSTRAIL` writer - tHSTRAIL"]
pub type THSTRAIL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `THSPREP` reader - tHS-PREPARE"]
pub type THSPREP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THSPREP` writer - tHS-PREPARE"]
pub type THSPREP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `TCLKZEO` reader - tCLK-ZERO"]
pub type TCLKZEO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCLKZEO` writer - tCLK-ZERO"]
pub type TCLKZEO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `TCLKPREP` reader - tCLK-PREPARE"]
pub type TCLKPREP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCLKPREP` writer - tCLK-PREPARE"]
pub type TCLKPREP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - tHSTRAIL"]
    #[inline(always)]
    pub fn thstrail(&self) -> THSTRAIL_R {
        THSTRAIL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - tHS-PREPARE"]
    #[inline(always)]
    pub fn thsprep(&self) -> THSPREP_R {
        THSPREP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - tCLK-ZERO"]
    #[inline(always)]
    pub fn tclkzeo(&self) -> TCLKZEO_R {
        TCLKZEO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - tCLK-PREPARE"]
    #[inline(always)]
    pub fn tclkprep(&self) -> TCLKPREP_R {
        TCLKPREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - tHSTRAIL"]
    #[inline(always)]
    pub fn thstrail(&mut self) -> THSTRAIL_W<24> {
        THSTRAIL_W::new(self)
    }
    #[doc = "Bits 16:23 - tHS-PREPARE"]
    #[inline(always)]
    pub fn thsprep(&mut self) -> THSPREP_W<16> {
        THSPREP_W::new(self)
    }
    #[doc = "Bits 8:15 - tCLK-ZERO"]
    #[inline(always)]
    pub fn tclkzeo(&mut self) -> TCLKZEO_W<8> {
        TCLKZEO_W::new(self)
    }
    #[doc = "Bits 0:7 - tCLK-PREPARE"]
    #[inline(always)]
    pub fn tclkprep(&mut self) -> TCLKPREP_W<0> {
        TCLKPREP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper PHY Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr3](index.html) module"]
pub struct WPCR3_SPEC;
impl crate::RegisterSpec for WPCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr3::R](R) reader structure"]
impl crate::Readable for WPCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr3::W](W) writer structure"]
impl crate::Writable for WPCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR3 to value 0"]
impl crate::Resettable for WPCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
