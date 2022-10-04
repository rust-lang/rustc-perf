#[doc = "Register `ESUR` reader"]
pub struct R(crate::R<ESUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESUR` writer"]
pub struct W(crate::W<ESUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESUR_SPEC>;
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
impl From<crate::W<ESUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEU` reader - Frame end delimiter unmask"]
pub struct FEU_R(crate::FieldReader<u8, u8>);
impl FEU_R {
    pub(crate) fn new(bits: u8) -> Self {
        FEU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEU` writer - Frame end delimiter unmask"]
pub struct FEU_W<'a> {
    w: &'a mut W,
}
impl<'a> FEU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `LEU` reader - Line end delimiter unmask"]
pub struct LEU_R(crate::FieldReader<u8, u8>);
impl LEU_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEU` writer - Line end delimiter unmask"]
pub struct LEU_W<'a> {
    w: &'a mut W,
}
impl<'a> LEU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `LSU` reader - Line start delimiter unmask"]
pub struct LSU_R(crate::FieldReader<u8, u8>);
impl LSU_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSU` writer - Line start delimiter unmask"]
pub struct LSU_W<'a> {
    w: &'a mut W,
}
impl<'a> LSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `FSU` reader - Frame start delimiter unmask"]
pub struct FSU_R(crate::FieldReader<u8, u8>);
impl FSU_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSU` writer - Frame start delimiter unmask"]
pub struct FSU_W<'a> {
    w: &'a mut W,
}
impl<'a> FSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Frame end delimiter unmask"]
    #[inline(always)]
    pub fn feu(&self) -> FEU_R {
        FEU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end delimiter unmask"]
    #[inline(always)]
    pub fn leu(&self) -> LEU_R {
        LEU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start delimiter unmask"]
    #[inline(always)]
    pub fn lsu(&self) -> LSU_R {
        LSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Frame start delimiter unmask"]
    #[inline(always)]
    pub fn fsu(&self) -> FSU_R {
        FSU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Frame end delimiter unmask"]
    #[inline(always)]
    pub fn feu(&mut self) -> FEU_W {
        FEU_W { w: self }
    }
    #[doc = "Bits 16:23 - Line end delimiter unmask"]
    #[inline(always)]
    pub fn leu(&mut self) -> LEU_W {
        LEU_W { w: self }
    }
    #[doc = "Bits 8:15 - Line start delimiter unmask"]
    #[inline(always)]
    pub fn lsu(&mut self) -> LSU_W {
        LSU_W { w: self }
    }
    #[doc = "Bits 0:7 - Frame start delimiter unmask"]
    #[inline(always)]
    pub fn fsu(&mut self) -> FSU_W {
        FSU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "embedded synchronization unmask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esur](index.html) module"]
pub struct ESUR_SPEC;
impl crate::RegisterSpec for ESUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esur::R](R) reader structure"]
impl crate::Readable for ESUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esur::W](W) writer structure"]
impl crate::Writable for ESUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESUR to value 0"]
impl crate::Resettable for ESUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
