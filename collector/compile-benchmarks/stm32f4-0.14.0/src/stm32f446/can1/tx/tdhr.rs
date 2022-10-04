#[doc = "Register `TDHR` reader"]
pub struct R(crate::R<TDHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDHR` writer"]
pub struct W(crate::W<TDHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDHR_SPEC>;
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
impl From<crate::W<TDHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA7` reader - DATA7"]
pub struct DATA7_R(crate::FieldReader<u8, u8>);
impl DATA7_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA7` writer - DATA7"]
pub struct DATA7_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `DATA6` reader - DATA6"]
pub struct DATA6_R(crate::FieldReader<u8, u8>);
impl DATA6_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA6` writer - DATA6"]
pub struct DATA6_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DATA5` reader - DATA5"]
pub struct DATA5_R(crate::FieldReader<u8, u8>);
impl DATA5_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA5` writer - DATA5"]
pub struct DATA5_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DATA4` reader - DATA4"]
pub struct DATA4_R(crate::FieldReader<u8, u8>);
impl DATA4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA4` writer - DATA4"]
pub struct DATA4_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W {
        DATA7_W { w: self }
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W {
        DATA6_W { w: self }
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W {
        DATA5_W { w: self }
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W {
        DATA4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdhr](index.html) module"]
pub struct TDHR_SPEC;
impl crate::RegisterSpec for TDHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdhr::R](R) reader structure"]
impl crate::Readable for TDHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdhr::W](W) writer structure"]
impl crate::Writable for TDHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TDHR to value 0"]
impl crate::Resettable for TDHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
