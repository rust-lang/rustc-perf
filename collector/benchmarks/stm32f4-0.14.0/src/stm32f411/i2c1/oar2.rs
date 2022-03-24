#[doc = "Register `OAR2` reader"]
pub struct R(crate::R<OAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OAR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OAR2` writer"]
pub struct W(crate::W<OAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OAR2_SPEC>;
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
impl From<crate::W<OAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OAR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD2` reader - Interface address"]
pub struct ADD2_R(crate::FieldReader<u8, u8>);
impl ADD2_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD2` writer - Interface address"]
pub struct ADD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
#[doc = "Dual addressing mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDUAL_A {
    #[doc = "0: Single addressing mode"]
    SINGLE = 0,
    #[doc = "1: Dual addressing mode"]
    DUAL = 1,
}
impl From<ENDUAL_A> for bool {
    #[inline(always)]
    fn from(variant: ENDUAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDUAL` reader - Dual addressing mode enable"]
pub struct ENDUAL_R(crate::FieldReader<bool, ENDUAL_A>);
impl ENDUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDUAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDUAL_A {
        match self.bits {
            false => ENDUAL_A::SINGLE,
            true => ENDUAL_A::DUAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == ENDUAL_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        **self == ENDUAL_A::DUAL
    }
}
impl core::ops::Deref for ENDUAL_R {
    type Target = crate::FieldReader<bool, ENDUAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDUAL` writer - Dual addressing mode enable"]
pub struct ENDUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDUAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDUAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single addressing mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ENDUAL_A::SINGLE)
    }
    #[doc = "Dual addressing mode"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(ENDUAL_A::DUAL)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn endual(&self) -> ENDUAL_R {
        ENDUAL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add2(&mut self) -> ADD2_W {
        ADD2_W { w: self }
    }
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn endual(&mut self) -> ENDUAL_W {
        ENDUAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Own address register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar2](index.html) module"]
pub struct OAR2_SPEC;
impl crate::RegisterSpec for OAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oar2::R](R) reader structure"]
impl crate::Readable for OAR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oar2::W](W) writer structure"]
impl crate::Writable for OAR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OAR2 to value 0"]
impl crate::Resettable for OAR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
