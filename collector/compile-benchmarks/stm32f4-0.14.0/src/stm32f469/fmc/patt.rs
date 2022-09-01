#[doc = "Register `PATT` reader"]
pub struct R(crate::R<PATT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATT` writer"]
pub struct W(crate::W<PATT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATT_SPEC>;
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
impl From<crate::W<PATT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTHIZ` reader - ATTHIZx"]
pub struct ATTHIZ_R(crate::FieldReader<u8, u8>);
impl ATTHIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTHIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTHIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTHIZ` writer - ATTHIZx"]
pub struct ATTHIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `ATTHOLD` reader - ATTHOLDx"]
pub struct ATTHOLD_R(crate::FieldReader<u8, u8>);
impl ATTHOLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTHOLD` writer - ATTHOLDx"]
pub struct ATTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ATTWAIT` reader - ATTWAITx"]
pub struct ATTWAIT_R(crate::FieldReader<u8, u8>);
impl ATTWAIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTWAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTWAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTWAIT` writer - ATTWAITx"]
pub struct ATTWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ATTSET` reader - ATTSETx"]
pub struct ATTSET_R(crate::FieldReader<u8, u8>);
impl ATTSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTSET` writer - ATTSETx"]
pub struct ATTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    pub fn atthiz(&mut self) -> ATTHIZ_W {
        ATTHIZ_W { w: self }
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    pub fn atthold(&mut self) -> ATTHOLD_W {
        ATTHOLD_W { w: self }
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    pub fn attwait(&mut self) -> ATTWAIT_W {
        ATTWAIT_W { w: self }
    }
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    pub fn attset(&mut self) -> ATTSET_W {
        ATTSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Attribute memory space timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt](index.html) module"]
pub struct PATT_SPEC;
impl crate::RegisterSpec for PATT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [patt::R](R) reader structure"]
impl crate::Readable for PATT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [patt::W](W) writer structure"]
impl crate::Writable for PATT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PATT to value 0xfcfc_fcfc"]
impl crate::Resettable for PATT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
