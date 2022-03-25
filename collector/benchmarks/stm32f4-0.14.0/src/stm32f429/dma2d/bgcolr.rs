#[doc = "Register `BGCOLR` reader"]
pub struct R(crate::R<BGCOLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGCOLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGCOLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGCOLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGCOLR` writer"]
pub struct W(crate::W<BGCOLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGCOLR_SPEC>;
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
impl From<crate::W<BGCOLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGCOLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RED` reader - Red Value"]
pub struct RED_R(crate::FieldReader<u8, u8>);
impl RED_R {
    pub(crate) fn new(bits: u8) -> Self {
        RED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RED` writer - Red Value"]
pub struct RED_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `GREEN` reader - Green Value"]
pub struct GREEN_R(crate::FieldReader<u8, u8>);
impl GREEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        GREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GREEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GREEN` writer - Green Value"]
pub struct GREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BLUE` reader - Blue Value"]
pub struct BLUE_R(crate::FieldReader<u8, u8>);
impl BLUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLUE` writer - Blue Value"]
pub struct BLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
    pub fn red(&mut self) -> RED_W {
        RED_W { w: self }
    }
    #[doc = "Bits 8:15 - Green Value"]
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W {
        GREEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Blue Value"]
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W {
        BLUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "background color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgcolr](index.html) module"]
pub struct BGCOLR_SPEC;
impl crate::RegisterSpec for BGCOLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgcolr::R](R) reader structure"]
impl crate::Readable for BGCOLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgcolr::W](W) writer structure"]
impl crate::Writable for BGCOLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGCOLR to value 0"]
impl crate::Resettable for BGCOLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
