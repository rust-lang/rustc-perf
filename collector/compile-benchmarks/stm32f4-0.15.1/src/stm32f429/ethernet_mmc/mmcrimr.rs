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
    Unmasked = 0,
    #[doc = "1: Received-crc-error counter half-full interrupt disabled"]
    Masked = 1,
}
impl From<RFCEM_A> for bool {
    #[inline(always)]
    fn from(variant: RFCEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCEM` reader - Received frame CRC error mask"]
pub type RFCEM_R = crate::BitReader<RFCEM_A>;
impl RFCEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCEM_A {
        match self.bits {
            false => RFCEM_A::Unmasked,
            true => RFCEM_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RFCEM_A::Unmasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RFCEM_A::Masked
    }
}
#[doc = "Field `RFCEM` writer - Received frame CRC error mask"]
pub type RFCEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCRIMR_SPEC, RFCEM_A, O>;
impl<'a, const O: u8> RFCEM_W<'a, O> {
    #[doc = "Received-crc-error counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RFCEM_A::Unmasked)
    }
    #[doc = "Received-crc-error counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RFCEM_A::Masked)
    }
}
#[doc = "Received frames alignment error mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFAEM_A {
    #[doc = "0: Received-alignment-error counter half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Received-alignment-error counter half-full interrupt disabled"]
    Masked = 1,
}
impl From<RFAEM_A> for bool {
    #[inline(always)]
    fn from(variant: RFAEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFAEM` reader - Received frames alignment error mask"]
pub type RFAEM_R = crate::BitReader<RFAEM_A>;
impl RFAEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFAEM_A {
        match self.bits {
            false => RFAEM_A::Unmasked,
            true => RFAEM_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RFAEM_A::Unmasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RFAEM_A::Masked
    }
}
#[doc = "Field `RFAEM` writer - Received frames alignment error mask"]
pub type RFAEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCRIMR_SPEC, RFAEM_A, O>;
impl<'a, const O: u8> RFAEM_W<'a, O> {
    #[doc = "Received-alignment-error counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RFAEM_A::Unmasked)
    }
    #[doc = "Received-alignment-error counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RFAEM_A::Masked)
    }
}
#[doc = "Received good Unicast frames mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGUFM_A {
    #[doc = "0: Received-good-unicast counter half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Received-good-unicast counter half-full interrupt disabled"]
    Masked = 1,
}
impl From<RGUFM_A> for bool {
    #[inline(always)]
    fn from(variant: RGUFM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGUFM` reader - Received good Unicast frames mask"]
pub type RGUFM_R = crate::BitReader<RGUFM_A>;
impl RGUFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGUFM_A {
        match self.bits {
            false => RGUFM_A::Unmasked,
            true => RGUFM_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RGUFM_A::Unmasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RGUFM_A::Masked
    }
}
#[doc = "Field `RGUFM` writer - Received good Unicast frames mask"]
pub type RGUFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCRIMR_SPEC, RGUFM_A, O>;
impl<'a, const O: u8> RGUFM_W<'a, O> {
    #[doc = "Received-good-unicast counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(RGUFM_A::Unmasked)
    }
    #[doc = "Received-good-unicast counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RGUFM_A::Masked)
    }
}
impl R {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&self) -> RFCEM_R {
        RFCEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&self) -> RFAEM_R {
        RFAEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good Unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&self) -> RGUFM_R {
        RGUFM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&mut self) -> RFCEM_W<5> {
        RFCEM_W::new(self)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&mut self) -> RFAEM_W<6> {
        RFAEM_W::new(self)
    }
    #[doc = "Bit 17 - Received good Unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&mut self) -> RGUFM_W<17> {
        RGUFM_W::new(self)
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
