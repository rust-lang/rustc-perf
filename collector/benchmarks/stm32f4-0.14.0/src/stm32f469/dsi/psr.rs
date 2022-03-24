#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSR` writer"]
pub struct W(crate::W<PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_SPEC>;
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
impl From<crate::W<PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UAN1` reader - ULPS Active Not lane 1"]
pub struct UAN1_R(crate::FieldReader<bool, bool>);
impl UAN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UAN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UAN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UAN1` writer - ULPS Active Not lane 1"]
pub struct UAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> UAN1_W<'a> {
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
#[doc = "Field `PSS1` reader - PHY Stop State lane 1"]
pub struct PSS1_R(crate::FieldReader<bool, bool>);
impl PSS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSS1` writer - PHY Stop State lane 1"]
pub struct PSS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RUE0` reader - RX ULPS Escape lane 0"]
pub struct RUE0_R(crate::FieldReader<bool, bool>);
impl RUE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUE0` writer - RX ULPS Escape lane 0"]
pub struct RUE0_W<'a> {
    w: &'a mut W,
}
impl<'a> RUE0_W<'a> {
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
#[doc = "Field `UAN0` reader - ULPS Active Not lane 1"]
pub struct UAN0_R(crate::FieldReader<bool, bool>);
impl UAN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UAN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UAN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UAN0` writer - ULPS Active Not lane 1"]
pub struct UAN0_W<'a> {
    w: &'a mut W,
}
impl<'a> UAN0_W<'a> {
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
#[doc = "Field `PSS0` reader - PHY Stop State lane 0"]
pub struct PSS0_R(crate::FieldReader<bool, bool>);
impl PSS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSS0` writer - PHY Stop State lane 0"]
pub struct PSS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PSS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `UANC` reader - ULPS Active Not Clock lane"]
pub struct UANC_R(crate::FieldReader<bool, bool>);
impl UANC_R {
    pub(crate) fn new(bits: bool) -> Self {
        UANC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UANC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UANC` writer - ULPS Active Not Clock lane"]
pub struct UANC_W<'a> {
    w: &'a mut W,
}
impl<'a> UANC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PSSC` reader - PHY Stop State Clock lane"]
pub struct PSSC_R(crate::FieldReader<bool, bool>);
impl PSSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSSC` writer - PHY Stop State Clock lane"]
pub struct PSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PD` reader - PHY Direction"]
pub struct PD_R(crate::FieldReader<bool, bool>);
impl PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD` writer - PHY Direction"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
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
    #[doc = "Bit 8 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PHY Stop State lane 1"]
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX ULPS Escape lane 0"]
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PHY Stop State lane 0"]
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ULPS Active Not Clock lane"]
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PHY Stop State Clock lane"]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - PHY Direction"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan1(&mut self) -> UAN1_W {
        UAN1_W { w: self }
    }
    #[doc = "Bit 7 - PHY Stop State lane 1"]
    #[inline(always)]
    pub fn pss1(&mut self) -> PSS1_W {
        PSS1_W { w: self }
    }
    #[doc = "Bit 6 - RX ULPS Escape lane 0"]
    #[inline(always)]
    pub fn rue0(&mut self) -> RUE0_W {
        RUE0_W { w: self }
    }
    #[doc = "Bit 5 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan0(&mut self) -> UAN0_W {
        UAN0_W { w: self }
    }
    #[doc = "Bit 4 - PHY Stop State lane 0"]
    #[inline(always)]
    pub fn pss0(&mut self) -> PSS0_W {
        PSS0_W { w: self }
    }
    #[doc = "Bit 3 - ULPS Active Not Clock lane"]
    #[inline(always)]
    pub fn uanc(&mut self) -> UANC_W {
        UANC_W { w: self }
    }
    #[doc = "Bit 2 - PHY Stop State Clock lane"]
    #[inline(always)]
    pub fn pssc(&mut self) -> PSSC_W {
        PSSC_W { w: self }
    }
    #[doc = "Bit 1 - PHY Direction"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host PHY Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psr::W](W) writer structure"]
impl crate::Writable for PSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSR to value 0x1528"]
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1528
    }
}
