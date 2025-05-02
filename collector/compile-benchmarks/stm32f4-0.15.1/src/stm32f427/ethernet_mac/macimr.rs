#[doc = "Register `MACIMR` reader"]
pub struct R(crate::R<MACIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACIMR` writer"]
pub struct W(crate::W<MACIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACIMR_SPEC>;
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
impl From<crate::W<MACIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PMT interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMTIM_A {
    #[doc = "0: PMT Status interrupt generation enabled"]
    Unmasked = 0,
    #[doc = "1: PMT Status interrupt generation disabled"]
    Masked = 1,
}
impl From<PMTIM_A> for bool {
    #[inline(always)]
    fn from(variant: PMTIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMTIM` reader - PMT interrupt mask"]
pub type PMTIM_R = crate::BitReader<PMTIM_A>;
impl PMTIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMTIM_A {
        match self.bits {
            false => PMTIM_A::Unmasked,
            true => PMTIM_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == PMTIM_A::Unmasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMTIM_A::Masked
    }
}
#[doc = "Field `PMTIM` writer - PMT interrupt mask"]
pub type PMTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIMR_SPEC, PMTIM_A, O>;
impl<'a, const O: u8> PMTIM_W<'a, O> {
    #[doc = "PMT Status interrupt generation enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(PMTIM_A::Unmasked)
    }
    #[doc = "PMT Status interrupt generation disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMTIM_A::Masked)
    }
}
#[doc = "Time stamp trigger interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTIM_A {
    #[doc = "0: Time stamp interrupt generation enabled"]
    Unmasked = 0,
    #[doc = "1: Time stamp interrupt generation disabled"]
    Masked = 1,
}
impl From<TSTIM_A> for bool {
    #[inline(always)]
    fn from(variant: TSTIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTIM` reader - Time stamp trigger interrupt mask"]
pub type TSTIM_R = crate::BitReader<TSTIM_A>;
impl TSTIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTIM_A {
        match self.bits {
            false => TSTIM_A::Unmasked,
            true => TSTIM_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TSTIM_A::Unmasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TSTIM_A::Masked
    }
}
#[doc = "Field `TSTIM` writer - Time stamp trigger interrupt mask"]
pub type TSTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIMR_SPEC, TSTIM_A, O>;
impl<'a, const O: u8> TSTIM_W<'a, O> {
    #[doc = "Time stamp interrupt generation enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TSTIM_A::Unmasked)
    }
    #[doc = "Time stamp interrupt generation disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TSTIM_A::Masked)
    }
}
impl R {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    pub fn tstim(&self) -> TSTIM_R {
        TSTIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pmtim(&mut self) -> PMTIM_W<3> {
        PMTIM_W::new(self)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    pub fn tstim(&mut self) -> TSTIM_W<9> {
        TSTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macimr](index.html) module"]
pub struct MACIMR_SPEC;
impl crate::RegisterSpec for MACIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macimr::R](R) reader structure"]
impl crate::Readable for MACIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macimr::W](W) writer structure"]
impl crate::Writable for MACIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACIMR to value 0"]
impl crate::Resettable for MACIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
