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
    CLEAR = 1,
}
impl From<CCEIF_A> for bool {
    #[inline(always)]
    fn from(variant: CCEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCEIF` reader - Clear configuration error interrupt flag"]
pub struct CCEIF_R(crate::FieldReader<bool, CCEIF_A>);
impl CCEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCEIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCEIF_A> {
        match self.bits {
            true => Some(CCEIF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CCEIF_A::CLEAR
    }
}
impl core::ops::Deref for CCEIF_R {
    type Target = crate::FieldReader<bool, CCEIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCEIF` writer - Clear configuration error interrupt flag"]
pub struct CCEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCEIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCEIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the CEIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCEIF_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Clear CLUT transfer complete interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCTCIF_A {
    #[doc = "1: Clear the CTCIF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CCTCIF_A> for bool {
    #[inline(always)]
    fn from(variant: CCTCIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCTCIF` reader - Clear CLUT transfer complete interrupt flag"]
pub struct CCTCIF_R(crate::FieldReader<bool, CCTCIF_A>);
impl CCTCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCTCIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCTCIF_A> {
        match self.bits {
            true => Some(CCTCIF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CCTCIF_A::CLEAR
    }
}
impl core::ops::Deref for CCTCIF_R {
    type Target = crate::FieldReader<bool, CCTCIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCTCIF` writer - Clear CLUT transfer complete interrupt flag"]
pub struct CCTCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCTCIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the CTCIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCTCIF_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Clear CLUT access error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAECIF_A {
    #[doc = "1: Clear the CAEIF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CAECIF_A> for bool {
    #[inline(always)]
    fn from(variant: CAECIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAECIF` reader - Clear CLUT access error interrupt flag"]
pub struct CAECIF_R(crate::FieldReader<bool, CAECIF_A>);
impl CAECIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAECIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAECIF_A> {
        match self.bits {
            true => Some(CAECIF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CAECIF_A::CLEAR
    }
}
impl core::ops::Deref for CAECIF_R {
    type Target = crate::FieldReader<bool, CAECIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAECIF` writer - Clear CLUT access error interrupt flag"]
pub struct CAECIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAECIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAECIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the CAEIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAECIF_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Clear transfer watermark interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTWIF_A {
    #[doc = "1: Clear the TWIF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CTWIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTWIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTWIF` reader - Clear transfer watermark interrupt flag"]
pub struct CTWIF_R(crate::FieldReader<bool, CTWIF_A>);
impl CTWIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTWIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTWIF_A> {
        match self.bits {
            true => Some(CTWIF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CTWIF_A::CLEAR
    }
}
impl core::ops::Deref for CTWIF_R {
    type Target = crate::FieldReader<bool, CTWIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTWIF` writer - Clear transfer watermark interrupt flag"]
pub struct CTWIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTWIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTWIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the TWIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTWIF_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Clear transfer complete interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF_A {
    #[doc = "1: Clear the TCIF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CTCIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIF` reader - Clear transfer complete interrupt flag"]
pub struct CTCIF_R(crate::FieldReader<bool, CTCIF_A>);
impl CTCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTCIF_A> {
        match self.bits {
            true => Some(CTCIF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CTCIF_A::CLEAR
    }
}
impl core::ops::Deref for CTCIF_R {
    type Target = crate::FieldReader<bool, CTCIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF` writer - Clear transfer complete interrupt flag"]
pub struct CTCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the TCIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Clear Transfer error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEIF_A {
    #[doc = "1: Clear the TEIF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CTEIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEIF` reader - Clear Transfer error interrupt flag"]
pub struct CTEIF_R(crate::FieldReader<bool, CTEIF_A>);
impl CTEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTEIF_A> {
        match self.bits {
            true => Some(CTEIF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CTEIF_A::CLEAR
    }
}
impl core::ops::Deref for CTEIF_R {
    type Target = crate::FieldReader<bool, CTEIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF` writer - Clear Transfer error interrupt flag"]
pub struct CTEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the TEIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF_A::CLEAR)
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
    #[doc = "Bit 5 - Clear configuration error interrupt flag"]
    #[inline(always)]
    pub fn cceif(&self) -> CCEIF_R {
        CCEIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag"]
    #[inline(always)]
    pub fn cctcif(&self) -> CCTCIF_R {
        CCTCIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag"]
    #[inline(always)]
    pub fn caecif(&self) -> CAECIF_R {
        CAECIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag"]
    #[inline(always)]
    pub fn ctwif(&self) -> CTWIF_R {
        CTWIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag"]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clear Transfer error interrupt flag"]
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clear configuration error interrupt flag"]
    #[inline(always)]
    pub fn cceif(&mut self) -> CCEIF_W {
        CCEIF_W { w: self }
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag"]
    #[inline(always)]
    pub fn cctcif(&mut self) -> CCTCIF_W {
        CCTCIF_W { w: self }
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag"]
    #[inline(always)]
    pub fn caecif(&mut self) -> CAECIF_W {
        CAECIF_W { w: self }
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag"]
    #[inline(always)]
    pub fn ctwif(&mut self) -> CTWIF_W {
        CTWIF_W { w: self }
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag"]
    #[inline(always)]
    pub fn ctcif(&mut self) -> CTCIF_W {
        CTCIF_W { w: self }
    }
    #[doc = "Bit 0 - Clear Transfer error interrupt flag"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W {
        CTEIF_W { w: self }
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
