#[doc = "Register `MACA3HR` reader"]
pub struct R(crate::R<MACA3HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA3HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA3HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA3HR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA3HR` writer"]
pub struct W(crate::W<MACA3HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA3HR_SPEC>;
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
impl From<crate::W<MACA3HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA3HR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACA3H` reader - MACA3H"]
pub struct MACA3H_R(crate::FieldReader<u16, u16>);
impl MACA3H_R {
    pub(crate) fn new(bits: u16) -> Self {
        MACA3H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACA3H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACA3H` writer - MACA3H"]
pub struct MACA3H_W<'a> {
    w: &'a mut W,
}
impl<'a> MACA3H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `MBC` reader - MBC"]
pub struct MBC_R(crate::FieldReader<u8, u8>);
impl MBC_R {
    pub(crate) fn new(bits: u8) -> Self {
        MBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MBC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MBC` writer - MBC"]
pub struct MBC_W<'a> {
    w: &'a mut W,
}
impl<'a> MBC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `SA` reader - SA"]
pub struct SA_R(crate::FieldReader<bool, bool>);
impl SA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SA` writer - SA"]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `AE` reader - AE"]
pub struct AE_R(crate::FieldReader<bool, bool>);
impl AE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE` writer - AE"]
pub struct AE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MACA3H"]
    #[inline(always)]
    pub fn maca3h(&self) -> MACA3H_R {
        MACA3H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MACA3H"]
    #[inline(always)]
    pub fn maca3h(&mut self) -> MACA3H_W {
        MACA3H_W { w: self }
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W {
        MBC_W { w: self }
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W {
        AE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address 3 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca3hr](index.html) module"]
pub struct MACA3HR_SPEC;
impl crate::RegisterSpec for MACA3HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca3hr::R](R) reader structure"]
impl crate::Readable for MACA3HR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca3hr::W](W) writer structure"]
impl crate::Writable for MACA3HR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACA3HR to value 0xffff"]
impl crate::Resettable for MACA3HR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
