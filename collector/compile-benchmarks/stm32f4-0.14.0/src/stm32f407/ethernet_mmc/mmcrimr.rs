#[doc = "Register `MMCRIMR` reader"]
pub struct R(crate::R<MMCRIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCRIMR` writer"]
pub struct W(crate::W<MMCRIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCRIMR_SPEC>;
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
impl From<crate::W<MMCRIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCRIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Received frame CRC error mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCEM_A {
    #[doc = "0: Received-crc-error counter half-full interrupt enabled"]
    UNMASKED = 0,
    #[doc = "1: Received-crc-error counter half-full interrupt disabled"]
    MASKED = 1,
}
impl From<RFCEM_A> for bool {
    #[inline(always)]
    fn from(variant: RFCEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCEM` reader - Received frame CRC error mask"]
pub struct RFCEM_R(crate::FieldReader<bool, RFCEM_A>);
impl RFCEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCEM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCEM_A {
        match self.bits {
            false => RFCEM_A::UNMASKED,
            true => RFCEM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == RFCEM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == RFCEM_A::MASKED
    }
}
impl core::ops::Deref for RFCEM_R {
    type Target = crate::FieldReader<bool, RFCEM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCEM` writer - Received frame CRC error mask"]
pub struct RFCEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFCEM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Received-crc-error counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RFCEM_A::UNMASKED)
    }
    #[doc = "Received-crc-error counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RFCEM_A::MASKED)
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
#[doc = "Received frames alignment error mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFAEM_A {
    #[doc = "0: Received-alignment-error counter half-full interrupt enabled"]
    UNMASKED = 0,
    #[doc = "1: Received-alignment-error counter half-full interrupt disabled"]
    MASKED = 1,
}
impl From<RFAEM_A> for bool {
    #[inline(always)]
    fn from(variant: RFAEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFAEM` reader - Received frames alignment error mask"]
pub struct RFAEM_R(crate::FieldReader<bool, RFAEM_A>);
impl RFAEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFAEM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFAEM_A {
        match self.bits {
            false => RFAEM_A::UNMASKED,
            true => RFAEM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == RFAEM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == RFAEM_A::MASKED
    }
}
impl core::ops::Deref for RFAEM_R {
    type Target = crate::FieldReader<bool, RFAEM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFAEM` writer - Received frames alignment error mask"]
pub struct RFAEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFAEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFAEM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Received-alignment-error counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RFAEM_A::UNMASKED)
    }
    #[doc = "Received-alignment-error counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RFAEM_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Received good Unicast frames mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGUFM_A {
    #[doc = "0: Received-good-unicast counter half-full interrupt enabled"]
    UNMASKED = 0,
    #[doc = "1: Received-good-unicast counter half-full interrupt disabled"]
    MASKED = 1,
}
impl From<RGUFM_A> for bool {
    #[inline(always)]
    fn from(variant: RGUFM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGUFM` reader - Received good Unicast frames mask"]
pub struct RGUFM_R(crate::FieldReader<bool, RGUFM_A>);
impl RGUFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RGUFM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGUFM_A {
        match self.bits {
            false => RGUFM_A::UNMASKED,
            true => RGUFM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == RGUFM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == RGUFM_A::MASKED
    }
}
impl core::ops::Deref for RGUFM_R {
    type Target = crate::FieldReader<bool, RGUFM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGUFM` writer - Received good Unicast frames mask"]
pub struct RGUFM_W<'a> {
    w: &'a mut W,
}
impl<'a> RGUFM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGUFM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Received-good-unicast counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RGUFM_A::UNMASKED)
    }
    #[doc = "Received-good-unicast counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RGUFM_A::MASKED)
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
impl R {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&self) -> RFCEM_R {
        RFCEM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&self) -> RFAEM_R {
        RFAEM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Received good Unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&self) -> RGUFM_R {
        RGUFM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&mut self) -> RFCEM_W {
        RFCEM_W { w: self }
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&mut self) -> RFAEM_W {
        RFAEM_W { w: self }
    }
    #[doc = "Bit 17 - Received good Unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&mut self) -> RGUFM_W {
        RGUFM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC receive interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrimr](index.html) module"]
pub struct MMCRIMR_SPEC;
impl crate::RegisterSpec for MMCRIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcrimr::R](R) reader structure"]
impl crate::Readable for MMCRIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmcrimr::W](W) writer structure"]
impl crate::Writable for MMCRIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCRIMR to value 0"]
impl crate::Resettable for MMCRIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
