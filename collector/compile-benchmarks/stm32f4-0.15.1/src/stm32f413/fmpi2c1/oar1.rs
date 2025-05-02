#[doc = "Register `OAR1` reader"]
pub struct R(crate::R<OAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OAR1` writer"]
pub struct W(crate::W<OAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OAR1_SPEC>;
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
impl From<crate::W<OAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA1` reader - OA1"]
pub type OA1_R = crate::BitReader<bool>;
#[doc = "Field `OA1` writer - OA1"]
pub type OA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, bool, O>;
#[doc = "Field `OA11_7` reader - OA11_7"]
pub type OA11_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA11_7` writer - OA11_7"]
pub type OA11_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OAR1_SPEC, u8, u8, 7, O>;
#[doc = "Field `OA18_9` reader - OA18_9"]
pub type OA18_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA18_9` writer - OA18_9"]
pub type OA18_9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OAR1_SPEC, u8, u8, 2, O>;
#[doc = "OA1MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA1MODE_A {
    #[doc = "0: Own address 1 is a 7-bit address"]
    Bit7 = 0,
    #[doc = "1: Own address 1 is a 10-bit address"]
    Bit10 = 1,
}
impl From<OA1MODE_A> for bool {
    #[inline(always)]
    fn from(variant: OA1MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA1MODE` reader - OA1MODE"]
pub type OA1MODE_R = crate::BitReader<OA1MODE_A>;
impl OA1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA1MODE_A {
        match self.bits {
            false => OA1MODE_A::Bit7,
            true => OA1MODE_A::Bit10,
        }
    }
    #[doc = "Checks if the value of the field is `Bit7`"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == OA1MODE_A::Bit7
    }
    #[doc = "Checks if the value of the field is `Bit10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == OA1MODE_A::Bit10
    }
}
#[doc = "Field `OA1MODE` writer - OA1MODE"]
pub type OA1MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, OA1MODE_A, O>;
impl<'a, const O: u8> OA1MODE_W<'a, O> {
    #[doc = "Own address 1 is a 7-bit address"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(OA1MODE_A::Bit7)
    }
    #[doc = "Own address 1 is a 10-bit address"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(OA1MODE_A::Bit10)
    }
}
#[doc = "OA1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA1EN_A {
    #[doc = "0: Own address 1 disabled. The received slave address OA1 is NACKed"]
    Disabled = 0,
    #[doc = "1: Own address 1 enabled. The received slave address OA1 is ACKed"]
    Enabled = 1,
}
impl From<OA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA1EN` reader - OA1EN"]
pub type OA1EN_R = crate::BitReader<OA1EN_A>;
impl OA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA1EN_A {
        match self.bits {
            false => OA1EN_A::Disabled,
            true => OA1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OA1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OA1EN_A::Enabled
    }
}
#[doc = "Field `OA1EN` writer - OA1EN"]
pub type OA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, OA1EN_A, O>;
impl<'a, const O: u8> OA1EN_W<'a, O> {
    #[doc = "Own address 1 disabled. The received slave address OA1 is NACKed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OA1EN_A::Disabled)
    }
    #[doc = "Own address 1 enabled. The received slave address OA1 is ACKed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OA1EN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - OA1"]
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - OA11_7"]
    #[inline(always)]
    pub fn oa11_7(&self) -> OA11_7_R {
        OA11_7_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - OA18_9"]
    #[inline(always)]
    pub fn oa18_9(&self) -> OA18_9_R {
        OA18_9_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OA1MODE"]
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - OA1EN"]
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OA1"]
    #[inline(always)]
    pub fn oa1(&mut self) -> OA1_W<0> {
        OA1_W::new(self)
    }
    #[doc = "Bits 1:7 - OA11_7"]
    #[inline(always)]
    pub fn oa11_7(&mut self) -> OA11_7_W<1> {
        OA11_7_W::new(self)
    }
    #[doc = "Bits 8:9 - OA18_9"]
    #[inline(always)]
    pub fn oa18_9(&mut self) -> OA18_9_W<8> {
        OA18_9_W::new(self)
    }
    #[doc = "Bit 10 - OA1MODE"]
    #[inline(always)]
    pub fn oa1mode(&mut self) -> OA1MODE_W<10> {
        OA1MODE_W::new(self)
    }
    #[doc = "Bit 15 - OA1EN"]
    #[inline(always)]
    pub fn oa1en(&mut self) -> OA1EN_W<15> {
        OA1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Own address register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar1](index.html) module"]
pub struct OAR1_SPEC;
impl crate::RegisterSpec for OAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oar1::R](R) reader structure"]
impl crate::Readable for OAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oar1::W](W) writer structure"]
impl crate::Writable for OAR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for OAR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
