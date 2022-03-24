#[doc = "Register `BTR` reader"]
pub struct R(crate::R<BTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTR` writer"]
pub struct W(crate::W<BTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTR_SPEC>;
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
impl From<crate::W<BTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SILM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SILM_A {
    #[doc = "0: Normal operation"]
    NORMAL = 0,
    #[doc = "1: Silent Mode"]
    SILENT = 1,
}
impl From<SILM_A> for bool {
    #[inline(always)]
    fn from(variant: SILM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SILM` reader - SILM"]
pub struct SILM_R(crate::FieldReader<bool, SILM_A>);
impl SILM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SILM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SILM_A {
        match self.bits {
            false => SILM_A::NORMAL,
            true => SILM_A::SILENT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SILM_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SILENT`"]
    #[inline(always)]
    pub fn is_silent(&self) -> bool {
        **self == SILM_A::SILENT
    }
}
impl core::ops::Deref for SILM_R {
    type Target = crate::FieldReader<bool, SILM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SILM` writer - SILM"]
pub struct SILM_W<'a> {
    w: &'a mut W,
}
impl<'a> SILM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SILM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SILM_A::NORMAL)
    }
    #[doc = "Silent Mode"]
    #[inline(always)]
    pub fn silent(self) -> &'a mut W {
        self.variant(SILM_A::SILENT)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "LBKM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKM_A {
    #[doc = "0: Loop Back Mode disabled"]
    DISABLED = 0,
    #[doc = "1: Loop Back Mode enabled"]
    ENABLED = 1,
}
impl From<LBKM_A> for bool {
    #[inline(always)]
    fn from(variant: LBKM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKM` reader - LBKM"]
pub struct LBKM_R(crate::FieldReader<bool, LBKM_A>);
impl LBKM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBKM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKM_A {
        match self.bits {
            false => LBKM_A::DISABLED,
            true => LBKM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LBKM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LBKM_A::ENABLED
    }
}
impl core::ops::Deref for LBKM_R {
    type Target = crate::FieldReader<bool, LBKM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBKM` writer - LBKM"]
pub struct LBKM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Loop Back Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBKM_A::DISABLED)
    }
    #[doc = "Loop Back Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBKM_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SJW` reader - SJW"]
pub struct SJW_R(crate::FieldReader<u8, u8>);
impl SJW_R {
    pub(crate) fn new(bits: u8) -> Self {
        SJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SJW` writer - SJW"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `TS2` reader - TS2"]
pub struct TS2_R(crate::FieldReader<u8, u8>);
impl TS2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS2` writer - TS2"]
pub struct TS2_W<'a> {
    w: &'a mut W,
}
impl<'a> TS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `TS1` reader - TS1"]
pub struct TS1_R(crate::FieldReader<u8, u8>);
impl TS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1` writer - TS1"]
pub struct TS1_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `BRP` reader - BRP"]
pub struct BRP_R(crate::FieldReader<u16, u16>);
impl BRP_R {
    pub(crate) fn new(bits: u16) -> Self {
        BRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRP` writer - BRP"]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&self) -> SILM_R {
        SILM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&self) -> LBKM_R {
        LBKM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&mut self) -> SILM_W {
        SILM_W { w: self }
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&mut self) -> LBKM_W {
        LBKM_W { w: self }
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W {
        TS2_W { w: self }
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&mut self) -> TS1_W {
        TS1_W { w: self }
    }
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bit timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr](index.html) module"]
pub struct BTR_SPEC;
impl crate::RegisterSpec for BTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btr::R](R) reader structure"]
impl crate::Readable for BTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btr::W](W) writer structure"]
impl crate::Writable for BTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTR to value 0"]
impl crate::Resettable for BTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
