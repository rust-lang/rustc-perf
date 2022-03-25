#[doc = "Register `PMC` reader"]
pub struct R(crate::R<PMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC` writer"]
pub struct W(crate::W<PMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_SPEC>;
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
impl From<crate::W<PMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MII_RMII_SEL` reader - Ethernet PHY interface selection"]
pub struct MII_RMII_SEL_R(crate::FieldReader<bool, bool>);
impl MII_RMII_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MII_RMII_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MII_RMII_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MII_RMII_SEL` writer - Ethernet PHY interface selection"]
pub struct MII_RMII_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MII_RMII_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ADC1DC2` reader - ADC1DC2"]
pub struct ADC1DC2_R(crate::FieldReader<bool, bool>);
impl ADC1DC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC1DC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1DC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1DC2` writer - ADC1DC2"]
pub struct ADC1DC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1DC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ADC2DC2` reader - ADC2DC2"]
pub struct ADC2DC2_R(crate::FieldReader<bool, bool>);
impl ADC2DC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC2DC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2DC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2DC2` writer - ADC2DC2"]
pub struct ADC2DC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2DC2_W<'a> {
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
#[doc = "Field `ADC3DC2` reader - ADC3DC2"]
pub struct ADC3DC2_R(crate::FieldReader<bool, bool>);
impl ADC3DC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC3DC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC3DC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC3DC2` writer - ADC3DC2"]
pub struct ADC3DC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC3DC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - Ethernet PHY interface selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC1DC2"]
    #[inline(always)]
    pub fn adc1dc2(&self) -> ADC1DC2_R {
        ADC1DC2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    pub fn adc2dc2(&self) -> ADC2DC2_R {
        ADC2DC2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    pub fn adc3dc2(&self) -> ADC3DC2_R {
        ADC3DC2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Ethernet PHY interface selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W {
        MII_RMII_SEL_W { w: self }
    }
    #[doc = "Bit 16 - ADC1DC2"]
    #[inline(always)]
    pub fn adc1dc2(&mut self) -> ADC1DC2_W {
        ADC1DC2_W { w: self }
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    pub fn adc2dc2(&mut self) -> ADC2DC2_W {
        ADC2DC2_W { w: self }
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    pub fn adc3dc2(&mut self) -> ADC3DC2_W {
        ADC3DC2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc](index.html) module"]
pub struct PMC_SPEC;
impl crate::RegisterSpec for PMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc::R](R) reader structure"]
impl crate::Readable for PMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc::W](W) writer structure"]
impl crate::Writable for PMC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC to value 0"]
impl crate::Resettable for PMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
