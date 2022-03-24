#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC channel2 DMA underrun flag"]
pub type DMAUDR2_A = DMAUDR1_A;
#[doc = "Field `DMAUDR2` reader - DAC channel2 DMA underrun flag"]
pub type DMAUDR2_R = DMAUDR1_R;
#[doc = "Field `DMAUDR2` writer - DAC channel2 DMA underrun flag"]
pub struct DMAUDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut W {
        self.variant(DMAUDR2_A::NOUNDERRUN)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut W {
        self.variant(DMAUDR2_A::UNDERRUN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "DAC channel1 DMA underrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDR1_A {
    #[doc = "0: No DMA underrun error condition occurred for DAC channel X"]
    NOUNDERRUN = 0,
    #[doc = "1: DMA underrun error condition occurred for DAC channel X"]
    UNDERRUN = 1,
}
impl From<DMAUDR1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDR1` reader - DAC channel1 DMA underrun flag"]
pub struct DMAUDR1_R(crate::FieldReader<bool, DMAUDR1_A>);
impl DMAUDR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDR1_A {
        match self.bits {
            false => DMAUDR1_A::NOUNDERRUN,
            true => DMAUDR1_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOUNDERRUN`"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        **self == DMAUDR1_A::NOUNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        **self == DMAUDR1_A::UNDERRUN
    }
}
impl core::ops::Deref for DMAUDR1_R {
    type Target = crate::FieldReader<bool, DMAUDR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAUDR1` writer - DAC channel1 DMA underrun flag"]
pub struct DMAUDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut W {
        self.variant(DMAUDR1_A::NOUNDERRUN)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut W {
        self.variant(DMAUDR1_A::UNDERRUN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W {
        DMAUDR2_W { w: self }
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W {
        DMAUDR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
