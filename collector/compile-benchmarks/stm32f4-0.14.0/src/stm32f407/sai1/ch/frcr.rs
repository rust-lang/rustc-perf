#[doc = "Register `FRCR` reader"]
pub struct R(crate::R<FRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRCR` writer"]
pub struct W(crate::W<FRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRCR_SPEC>;
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
impl From<crate::W<FRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRL` reader - Frame length"]
pub struct FRL_R(crate::FieldReader<u8, u8>);
impl FRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRL` writer - Frame length"]
pub struct FRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `FSALL` reader - Frame synchronization active level length"]
pub struct FSALL_R(crate::FieldReader<u8, u8>);
impl FSALL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSALL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSALL` writer - Frame synchronization active level length"]
pub struct FSALL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `FSDEF` reader - Frame synchronization definition"]
pub struct FSDEF_R(crate::FieldReader<bool, bool>);
impl FSDEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSDEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSDEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Frame synchronization polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSPOL_A {
    #[doc = "0: FS is active low (falling edge)"]
    FALLINGEDGE = 0,
    #[doc = "1: FS is active high (rising edge)"]
    RISINGEDGE = 1,
}
impl From<FSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: FSPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSPOL` reader - Frame synchronization polarity"]
pub struct FSPOL_R(crate::FieldReader<bool, FSPOL_A>);
impl FSPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSPOL_A {
        match self.bits {
            false => FSPOL_A::FALLINGEDGE,
            true => FSPOL_A::RISINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == FSPOL_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == FSPOL_A::RISINGEDGE
    }
}
impl core::ops::Deref for FSPOL_R {
    type Target = crate::FieldReader<bool, FSPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSPOL` writer - Frame synchronization polarity"]
pub struct FSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FS is active low (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(FSPOL_A::FALLINGEDGE)
    }
    #[doc = "FS is active high (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(FSPOL_A::RISINGEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Frame synchronization offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSOFF_A {
    #[doc = "0: FS is asserted on the first bit of the slot 0"]
    ONFIRST = 0,
    #[doc = "1: FS is asserted one bit before the first bit of the slot 0"]
    BEFOREFIRST = 1,
}
impl From<FSOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FSOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSOFF` reader - Frame synchronization offset"]
pub struct FSOFF_R(crate::FieldReader<bool, FSOFF_A>);
impl FSOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSOFF_A {
        match self.bits {
            false => FSOFF_A::ONFIRST,
            true => FSOFF_A::BEFOREFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `ONFIRST`"]
    #[inline(always)]
    pub fn is_on_first(&self) -> bool {
        **self == FSOFF_A::ONFIRST
    }
    #[doc = "Checks if the value of the field is `BEFOREFIRST`"]
    #[inline(always)]
    pub fn is_before_first(&self) -> bool {
        **self == FSOFF_A::BEFOREFIRST
    }
}
impl core::ops::Deref for FSOFF_R {
    type Target = crate::FieldReader<bool, FSOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSOFF` writer - Frame synchronization offset"]
pub struct FSOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FSOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FS is asserted on the first bit of the slot 0"]
    #[inline(always)]
    pub fn on_first(self) -> &'a mut W {
        self.variant(FSOFF_A::ONFIRST)
    }
    #[doc = "FS is asserted one bit before the first bit of the slot 0"]
    #[inline(always)]
    pub fn before_first(self) -> &'a mut W {
        self.variant(FSOFF_A::BEFOREFIRST)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Frame synchronization definition"]
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    pub fn frl(&mut self) -> FRL_W {
        FRL_W { w: self }
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    pub fn fsall(&mut self) -> FSALL_W {
        FSALL_W { w: self }
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    pub fn fspol(&mut self) -> FSPOL_W {
        FSPOL_W { w: self }
    }
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    pub fn fsoff(&mut self) -> FSOFF_W {
        FSOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI AFrame configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frcr](index.html) module"]
pub struct FRCR_SPEC;
impl crate::RegisterSpec for FRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frcr::R](R) reader structure"]
impl crate::Readable for FRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frcr::W](W) writer structure"]
impl crate::Writable for FRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRCR to value 0x07"]
impl crate::Resettable for FRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
