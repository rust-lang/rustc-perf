#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMPI2C1_SCL` reader - Forces FM+ drive capability on I2CFMP1_SCL pin"]
pub struct FMPI2C1_SCL_R(crate::FieldReader<bool, bool>);
impl FMPI2C1_SCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMPI2C1_SCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMPI2C1_SCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMPI2C1_SCL` writer - Forces FM+ drive capability on I2CFMP1_SCL pin"]
pub struct FMPI2C1_SCL_W<'a> {
    w: &'a mut W,
}
impl<'a> FMPI2C1_SCL_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `FMPI2C1_SDA` reader - Forces FM+ drive capability on I2CFMP1_SCL pin"]
pub struct FMPI2C1_SDA_R(crate::FieldReader<bool, bool>);
impl FMPI2C1_SDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMPI2C1_SDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMPI2C1_SDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMPI2C1_SDA` writer - Forces FM+ drive capability on I2CFMP1_SCL pin"]
pub struct FMPI2C1_SDA_W<'a> {
    w: &'a mut W,
}
impl<'a> FMPI2C1_SDA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Forces FM+ drive capability on I2CFMP1_SCL pin"]
    #[inline(always)]
    pub fn fmpi2c1_scl(&self) -> FMPI2C1_SCL_R {
        FMPI2C1_SCL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Forces FM+ drive capability on I2CFMP1_SCL pin"]
    #[inline(always)]
    pub fn fmpi2c1_sda(&self) -> FMPI2C1_SDA_R {
        FMPI2C1_SDA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Forces FM+ drive capability on I2CFMP1_SCL pin"]
    #[inline(always)]
    pub fn fmpi2c1_scl(&mut self) -> FMPI2C1_SCL_W {
        FMPI2C1_SCL_W { w: self }
    }
    #[doc = "Bit 1 - Forces FM+ drive capability on I2CFMP1_SCL pin"]
    #[inline(always)]
    pub fn fmpi2c1_sda(&mut self) -> FMPI2C1_SDA_W {
        FMPI2C1_SDA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
