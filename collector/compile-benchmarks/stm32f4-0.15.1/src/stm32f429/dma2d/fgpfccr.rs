#[doc = "Register `FGPFCCR` reader"]
pub struct R(crate::R<FGPFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGPFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGPFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGPFCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGPFCCR` writer"]
pub struct W(crate::W<FGPFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGPFCCR_SPEC>;
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
impl From<crate::W<FGPFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGPFCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALPHA` reader - Alpha value"]
pub type ALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALPHA` writer - Alpha value"]
pub type ALPHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `AM` reader - Alpha mode"]
pub type AM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AM` writer - Alpha mode"]
pub type AM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CS` reader - CLUT size"]
pub type CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS` writer - CLUT size"]
pub type CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGPFCCR_SPEC, bool, O>;
#[doc = "Field `CCM` reader - CLUT color mode"]
pub type CCM_R = crate::BitReader<bool>;
#[doc = "Field `CCM` writer - CLUT color mode"]
pub type CCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGPFCCR_SPEC, bool, O>;
#[doc = "Field `CM` reader - Color mode"]
pub type CM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CM` writer - Color mode"]
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 24:31 - Alpha value"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Alpha mode"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 8:15 - CLUT size"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 5 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - CLUT color mode"]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:3 - Color mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Alpha value"]
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<24> {
        ALPHA_W::new(self)
    }
    #[doc = "Bits 16:17 - Alpha mode"]
    #[inline(always)]
    pub fn am(&mut self) -> AM_W<16> {
        AM_W::new(self)
    }
    #[doc = "Bits 8:15 - CLUT size"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W<8> {
        CS_W::new(self)
    }
    #[doc = "Bit 5 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<5> {
        START_W::new(self)
    }
    #[doc = "Bit 4 - CLUT color mode"]
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W<4> {
        CCM_W::new(self)
    }
    #[doc = "Bits 0:3 - Color mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<0> {
        CM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "foreground PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgpfccr](index.html) module"]
pub struct FGPFCCR_SPEC;
impl crate::RegisterSpec for FGPFCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgpfccr::R](R) reader structure"]
impl crate::Readable for FGPFCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgpfccr::W](W) writer structure"]
impl crate::Writable for FGPFCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FGPFCCR to value 0"]
impl crate::Resettable for FGPFCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
