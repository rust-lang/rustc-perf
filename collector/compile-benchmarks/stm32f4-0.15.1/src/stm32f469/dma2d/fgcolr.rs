#[doc = "Register `FGCOLR` reader"]
pub struct R(crate::R<FGCOLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGCOLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGCOLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGCOLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGCOLR` writer"]
pub struct W(crate::W<FGCOLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGCOLR_SPEC>;
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
impl From<crate::W<FGCOLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGCOLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RED` reader - Red Value"]
pub type RED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RED` writer - Red Value"]
pub type RED_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FGCOLR_SPEC, u8, u8, 8, O>;
#[doc = "Field `GREEN` reader - Green Value"]
pub type GREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GREEN` writer - Green Value"]
pub type GREEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FGCOLR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BLUE` reader - Blue Value"]
pub type BLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLUE` writer - Blue Value"]
pub type BLUE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FGCOLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 16:23 - Red Value"]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green Value"]
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Blue Value"]
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Red Value"]
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<16> {
        RED_W::new(self)
    }
    #[doc = "Bits 8:15 - Green Value"]
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<8> {
        GREEN_W::new(self)
    }
    #[doc = "Bits 0:7 - Blue Value"]
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<0> {
        BLUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "foreground color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgcolr](index.html) module"]
pub struct FGCOLR_SPEC;
impl crate::RegisterSpec for FGCOLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgcolr::R](R) reader structure"]
impl crate::Readable for FGCOLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgcolr::W](W) writer structure"]
impl crate::Writable for FGCOLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FGCOLR to value 0"]
impl crate::Resettable for FGCOLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
