#[doc = "Register `PMEM3` reader"]
pub struct R(crate::R<PMEM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMEM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMEM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMEM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMEM3` writer"]
pub struct W(crate::W<PMEM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMEM3_SPEC>;
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
impl From<crate::W<PMEM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMEM3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMHIZ` reader - MEMHIZx"]
pub struct MEMHIZ_R(crate::FieldReader<u8, u8>);
impl MEMHIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEMHIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMHIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMHIZ` writer - MEMHIZx"]
pub struct MEMHIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMHIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `MEMHOLD` reader - MEMHOLDx"]
pub struct MEMHOLD_R(crate::FieldReader<u8, u8>);
impl MEMHOLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEMHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMHOLD` writer - MEMHOLDx"]
pub struct MEMHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `MEMWAIT` reader - MEMWAITx"]
pub struct MEMWAIT_R(crate::FieldReader<u8, u8>);
impl MEMWAIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEMWAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMWAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMWAIT` writer - MEMWAITx"]
pub struct MEMWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `MEMSET` reader - MEMSETx"]
pub struct MEMSET_R(crate::FieldReader<u8, u8>);
impl MEMSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEMSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMSET` writer - MEMSETx"]
pub struct MEMSET_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhiz(&self) -> MEMHIZ_R {
        MEMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memhold(&self) -> MEMHOLD_R {
        MEMHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhiz(&mut self) -> MEMHIZ_W {
        MEMHIZ_W { w: self }
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memhold(&mut self) -> MEMHOLD_W {
        MEMHOLD_W { w: self }
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwait(&mut self) -> MEMWAIT_W {
        MEMWAIT_W { w: self }
    }
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memset(&mut self) -> MEMSET_W {
        MEMSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common memory space timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem3](index.html) module"]
pub struct PMEM3_SPEC;
impl crate::RegisterSpec for PMEM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmem3::R](R) reader structure"]
impl crate::Readable for PMEM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmem3::W](W) writer structure"]
impl crate::Writable for PMEM3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMEM3 to value 0xfcfc_fcfc"]
impl crate::Resettable for PMEM3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
