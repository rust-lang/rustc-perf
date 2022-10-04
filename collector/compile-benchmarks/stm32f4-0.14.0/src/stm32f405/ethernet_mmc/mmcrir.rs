#[doc = "Register `MMCRIR` reader"]
pub struct R(crate::R<MMCRIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCRIR` writer"]
pub struct W(crate::W<MMCRIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCRIR_SPEC>;
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
impl From<crate::W<MMCRIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCRIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFCES` reader - RFCES"]
pub struct RFCES_R(crate::FieldReader<bool, bool>);
impl RFCES_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCES` writer - RFCES"]
pub struct RFCES_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RFAES` reader - RFAES"]
pub struct RFAES_R(crate::FieldReader<bool, bool>);
impl RFAES_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFAES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFAES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFAES` writer - RFAES"]
pub struct RFAES_W<'a> {
    w: &'a mut W,
}
impl<'a> RFAES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RGUFS` reader - RGUFS"]
pub struct RGUFS_R(crate::FieldReader<bool, bool>);
impl RGUFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RGUFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RGUFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGUFS` writer - RGUFS"]
pub struct RGUFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RGUFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - RFCES"]
    #[inline(always)]
    pub fn rfces(&self) -> RFCES_R {
        RFCES_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RFAES"]
    #[inline(always)]
    pub fn rfaes(&self) -> RFAES_R {
        RFAES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RGUFS"]
    #[inline(always)]
    pub fn rgufs(&self) -> RGUFS_R {
        RGUFS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - RFCES"]
    #[inline(always)]
    pub fn rfces(&mut self) -> RFCES_W {
        RFCES_W { w: self }
    }
    #[doc = "Bit 6 - RFAES"]
    #[inline(always)]
    pub fn rfaes(&mut self) -> RFAES_W {
        RFAES_W { w: self }
    }
    #[doc = "Bit 17 - RGUFS"]
    #[inline(always)]
    pub fn rgufs(&mut self) -> RGUFS_W {
        RGUFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC receive interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrir](index.html) module"]
pub struct MMCRIR_SPEC;
impl crate::RegisterSpec for MMCRIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcrir::R](R) reader structure"]
impl crate::Readable for MMCRIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmcrir::W](W) writer structure"]
impl crate::Writable for MMCRIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCRIR to value 0"]
impl crate::Resettable for MMCRIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
