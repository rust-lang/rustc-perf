#[doc = "Register `IFCR` reader"]
pub struct R(crate::R<IFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear configuration error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCEIF_A {
    #[doc = "1: Clear the CEIF flag in the ISR register"]
    Clear = 1,
}
impl From<CCEIF_A> for bool {
    #[inline(always)]
    fn from(variant: CCEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCEIF` reader - Clear configuration error interrupt flag"]
pub type CCEIF_R = crate::BitReader<CCEIF_A>;
impl CCEIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCEIF_A> {
        match self.bits {
            true => Some(CCEIF_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCEIF_A::Clear
    }
}
#[doc = "Field `CCEIF` writer - Clear configuration error interrupt flag"]
pub type CCEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CCEIF_A, O>;
impl<'a, const O: u8> CCEIF_W<'a, O> {
    #[doc = "Clear the CEIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCEIF_A::Clear)
    }
}
#[doc = "Clear CLUT transfer complete interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCTCIF_A {
    #[doc = "1: Clear the CTCIF flag in the ISR register"]
    Clear = 1,
}
impl From<CCTCIF_A> for bool {
    #[inline(always)]
    fn from(variant: CCTCIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCTCIF` reader - Clear CLUT transfer complete interrupt flag"]
pub type CCTCIF_R = crate::BitReader<CCTCIF_A>;
impl CCTCIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCTCIF_A> {
        match self.bits {
            true => Some(CCTCIF_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCTCIF_A::Clear
    }
}
#[doc = "Field `CCTCIF` writer - Clear CLUT transfer complete interrupt flag"]
pub type CCTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CCTCIF_A, O>;
impl<'a, const O: u8> CCTCIF_W<'a, O> {
    #[doc = "Clear the CTCIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCTCIF_A::Clear)
    }
}
#[doc = "Clear CLUT access error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAECIF_A {
    #[doc = "1: Clear the CAEIF flag in the ISR register"]
    Clear = 1,
}
impl From<CAECIF_A> for bool {
    #[inline(always)]
    fn from(variant: CAECIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAECIF` reader - Clear CLUT access error interrupt flag"]
pub type CAECIF_R = crate::BitReader<CAECIF_A>;
impl CAECIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAECIF_A> {
        match self.bits {
            true => Some(CAECIF_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CAECIF_A::Clear
    }
}
#[doc = "Field `CAECIF` writer - Clear CLUT access error interrupt flag"]
pub type CAECIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CAECIF_A, O>;
impl<'a, const O: u8> CAECIF_W<'a, O> {
    #[doc = "Clear the CAEIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAECIF_A::Clear)
    }
}
#[doc = "Clear transfer watermark interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTWIF_A {
    #[doc = "1: Clear the TWIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTWIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTWIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTWIF` reader - Clear transfer watermark interrupt flag"]
pub type CTWIF_R = crate::BitReader<CTWIF_A>;
impl CTWIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTWIF_A> {
        match self.bits {
            true => Some(CTWIF_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTWIF_A::Clear
    }
}
#[doc = "Field `CTWIF` writer - Clear transfer watermark interrupt flag"]
pub type CTWIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTWIF_A, O>;
impl<'a, const O: u8> CTWIF_W<'a, O> {
    #[doc = "Clear the TWIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTWIF_A::Clear)
    }
}
#[doc = "Clear transfer complete interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF_A {
    #[doc = "1: Clear the TCIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTCIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIF` reader - Clear transfer complete interrupt flag"]
pub type CTCIF_R = crate::BitReader<CTCIF_A>;
impl CTCIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTCIF_A> {
        match self.bits {
            true => Some(CTCIF_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTCIF_A::Clear
    }
}
#[doc = "Field `CTCIF` writer - Clear transfer complete interrupt flag"]
pub type CTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTCIF_A, O>;
impl<'a, const O: u8> CTCIF_W<'a, O> {
    #[doc = "Clear the TCIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF_A::Clear)
    }
}
#[doc = "Clear Transfer error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEIF_A {
    #[doc = "1: Clear the TEIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTEIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEIF` reader - Clear Transfer error interrupt flag"]
pub type CTEIF_R = crate::BitReader<CTEIF_A>;
impl CTEIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTEIF_A> {
        match self.bits {
            true => Some(CTEIF_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTEIF_A::Clear
    }
}
#[doc = "Field `CTEIF` writer - Clear Transfer error interrupt flag"]
pub type CTEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTEIF_A, O>;
impl<'a, const O: u8> CTEIF_W<'a, O> {
    #[doc = "Clear the TEIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF_A::Clear)
    }
}
impl R {
    #[doc = "Bit 5 - Clear configuration error interrupt flag"]
    #[inline(always)]
    pub fn cceif(&self) -> CCEIF_R {
        CCEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag"]
    #[inline(always)]
    pub fn cctcif(&self) -> CCTCIF_R {
        CCTCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag"]
    #[inline(always)]
    pub fn caecif(&self) -> CAECIF_R {
        CAECIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag"]
    #[inline(always)]
    pub fn ctwif(&self) -> CTWIF_R {
        CTWIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag"]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Clear Transfer error interrupt flag"]
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clear configuration error interrupt flag"]
    #[inline(always)]
    pub fn cceif(&mut self) -> CCEIF_W<5> {
        CCEIF_W::new(self)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag"]
    #[inline(always)]
    pub fn cctcif(&mut self) -> CCTCIF_W<4> {
        CCTCIF_W::new(self)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag"]
    #[inline(always)]
    pub fn caecif(&mut self) -> CAECIF_W<3> {
        CAECIF_W::new(self)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag"]
    #[inline(always)]
    pub fn ctwif(&mut self) -> CTWIF_W<2> {
        CTWIF_W::new(self)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag"]
    #[inline(always)]
    pub fn ctcif(&mut self) -> CTCIF_W<1> {
        CTCIF_W::new(self)
    }
    #[doc = "Bit 0 - Clear Transfer error interrupt flag"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W<0> {
        CTEIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifcr::R](R) reader structure"]
impl crate::Readable for IFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
