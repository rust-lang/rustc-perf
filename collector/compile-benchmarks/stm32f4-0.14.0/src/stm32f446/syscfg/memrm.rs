#[doc = "Register `MEMRM` reader"]
pub struct R(crate::R<MEMRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMRM` writer"]
pub struct W(crate::W<MEMRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMRM_SPEC>;
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
impl From<crate::W<MEMRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection"]
pub struct MEM_MODE_R(crate::FieldReader<u8, u8>);
impl MEM_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEM_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection"]
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `FB_MODE` reader - Flash bank mode selection"]
pub struct FB_MODE_R(crate::FieldReader<bool, bool>);
impl FB_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_MODE` writer - Flash bank mode selection"]
pub struct FB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SWP_FMC` reader - FMC memory mapping swap"]
pub struct SWP_FMC_R(crate::FieldReader<u8, u8>);
impl SWP_FMC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWP_FMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWP_FMC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWP_FMC` writer - FMC memory mapping swap"]
pub struct SWP_FMC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWP_FMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - Flash bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&self) -> FB_MODE_R {
        FB_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - FMC memory mapping swap"]
    #[inline(always)]
    pub fn swp_fmc(&self) -> SWP_FMC_R {
        SWP_FMC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
    #[doc = "Bit 8 - Flash bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&mut self) -> FB_MODE_W {
        FB_MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - FMC memory mapping swap"]
    #[inline(always)]
    pub fn swp_fmc(&mut self) -> SWP_FMC_W {
        SWP_FMC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "memory remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memrm](index.html) module"]
pub struct MEMRM_SPEC;
impl crate::RegisterSpec for MEMRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memrm::R](R) reader structure"]
impl crate::Readable for MEMRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memrm::W](W) writer structure"]
impl crate::Writable for MEMRM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMRM to value 0"]
impl crate::Resettable for MEMRM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
