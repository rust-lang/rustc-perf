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
#[doc = "Addressing mode (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDMODE_A {
    #[doc = "0: 7-bit slave address"]
    ADD7 = 0,
    #[doc = "1: 10-bit slave address"]
    ADD10 = 1,
}
impl From<ADDMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDMODE` reader - Addressing mode (slave mode)"]
pub struct ADDMODE_R(crate::FieldReader<bool, ADDMODE_A>);
impl ADDMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDMODE_A {
        match self.bits {
            false => ADDMODE_A::ADD7,
            true => ADDMODE_A::ADD10,
        }
    }
    #[doc = "Checks if the value of the field is `ADD7`"]
    #[inline(always)]
    pub fn is_add7(&self) -> bool {
        **self == ADDMODE_A::ADD7
    }
    #[doc = "Checks if the value of the field is `ADD10`"]
    #[inline(always)]
    pub fn is_add10(&self) -> bool {
        **self == ADDMODE_A::ADD10
    }
}
impl core::ops::Deref for ADDMODE_R {
    type Target = crate::FieldReader<bool, ADDMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDMODE` writer - Addressing mode (slave mode)"]
pub struct ADDMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "7-bit slave address"]
    #[inline(always)]
    pub fn add7(self) -> &'a mut W {
        self.variant(ADDMODE_A::ADD7)
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn add10(self) -> &'a mut W {
        self.variant(ADDMODE_A::ADD10)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ADD` reader - Interface address"]
pub struct ADD_R(crate::FieldReader<u16, u16>);
impl ADD_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD` writer - Interface address"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&mut self) -> ADDMODE_W {
        ADDMODE_W { w: self }
    }
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
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
