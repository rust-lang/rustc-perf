#[doc = "Register `BDTR` reader"]
pub struct R(crate::R<BDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDTR` writer"]
pub struct W(crate::W<BDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTR_SPEC>;
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
impl From<crate::W<BDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Main output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOE_A {
    #[doc = "0: OC/OCN are disabled or forced idle depending on OSSI"]
    DisabledIdle = 0,
    #[doc = "1: OC/OCN are enabled if CCxE/CCxNE are set"]
    Enabled = 1,
}
impl From<MOE_A> for bool {
    #[inline(always)]
    fn from(variant: MOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOE` reader - Main output enable"]
pub type MOE_R = crate::BitReader<MOE_A>;
impl MOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOE_A {
        match self.bits {
            false => MOE_A::DisabledIdle,
            true => MOE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `DisabledIdle`"]
    #[inline(always)]
    pub fn is_disabled_idle(&self) -> bool {
        *self == MOE_A::DisabledIdle
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MOE_A::Enabled
    }
}
#[doc = "Field `MOE` writer - Main output enable"]
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, MOE_A, O>;
impl<'a, const O: u8> MOE_W<'a, O> {
    #[doc = "OC/OCN are disabled or forced idle depending on OSSI"]
    #[inline(always)]
    pub fn disabled_idle(self) -> &'a mut W {
        self.variant(MOE_A::DisabledIdle)
    }
    #[doc = "OC/OCN are enabled if CCxE/CCxNE are set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MOE_A::Enabled)
    }
}
#[doc = "Field `AOE` reader - Automatic output enable"]
pub type AOE_R = crate::BitReader<bool>;
#[doc = "Field `AOE` writer - Automatic output enable"]
pub type AOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
#[doc = "Field `BKP` reader - Break polarity"]
pub type BKP_R = crate::BitReader<bool>;
#[doc = "Field `BKP` writer - Break polarity"]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
#[doc = "Field `BKE` reader - Break enable"]
pub type BKE_R = crate::BitReader<bool>;
#[doc = "Field `BKE` writer - Break enable"]
pub type BKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
#[doc = "Off-state selection for Run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSSR_A {
    #[doc = "0: When inactive, OC/OCN outputs are disabled"]
    Disabled = 0,
    #[doc = "1: When inactive, OC/OCN outputs are enabled with their inactive level"]
    IdleLevel = 1,
}
impl From<OSSR_A> for bool {
    #[inline(always)]
    fn from(variant: OSSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSR` reader - Off-state selection for Run mode"]
pub type OSSR_R = crate::BitReader<OSSR_A>;
impl OSSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSSR_A {
        match self.bits {
            false => OSSR_A::Disabled,
            true => OSSR_A::IdleLevel,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `IdleLevel`"]
    #[inline(always)]
    pub fn is_idle_level(&self) -> bool {
        *self == OSSR_A::IdleLevel
    }
}
#[doc = "Field `OSSR` writer - Off-state selection for Run mode"]
pub type OSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, OSSR_A, O>;
impl<'a, const O: u8> OSSR_W<'a, O> {
    #[doc = "When inactive, OC/OCN outputs are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSR_A::Disabled)
    }
    #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level"]
    #[inline(always)]
    pub fn idle_level(self) -> &'a mut W {
        self.variant(OSSR_A::IdleLevel)
    }
}
#[doc = "Off-state selection for Idle mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSSI_A {
    #[doc = "0: When inactive, OC/OCN outputs are disabled"]
    Disabled = 0,
    #[doc = "1: When inactive, OC/OCN outputs are forced to idle level"]
    IdleLevel = 1,
}
impl From<OSSI_A> for bool {
    #[inline(always)]
    fn from(variant: OSSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSI` reader - Off-state selection for Idle mode"]
pub type OSSI_R = crate::BitReader<OSSI_A>;
impl OSSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSSI_A {
        match self.bits {
            false => OSSI_A::Disabled,
            true => OSSI_A::IdleLevel,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSI_A::Disabled
    }
    #[doc = "Checks if the value of the field is `IdleLevel`"]
    #[inline(always)]
    pub fn is_idle_level(&self) -> bool {
        *self == OSSI_A::IdleLevel
    }
}
#[doc = "Field `OSSI` writer - Off-state selection for Idle mode"]
pub type OSSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, OSSI_A, O>;
impl<'a, const O: u8> OSSI_W<'a, O> {
    #[doc = "When inactive, OC/OCN outputs are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSI_A::Disabled)
    }
    #[doc = "When inactive, OC/OCN outputs are forced to idle level"]
    #[inline(always)]
    pub fn idle_level(self) -> &'a mut W {
        self.variant(OSSI_A::IdleLevel)
    }
}
#[doc = "Field `LOCK` reader - Lock configuration"]
pub type LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK` writer - Lock configuration"]
pub type LOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DTG` reader - Dead-time generator setup"]
pub type DTG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTG` writer - Dead-time generator setup"]
pub type DTG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDTR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<15> {
        MOE_W::new(self)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W<14> {
        AOE_W::new(self)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<13> {
        BKP_W::new(self)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W<12> {
        BKE_W::new(self)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W<11> {
        OSSR_W::new(self)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W<10> {
        OSSI_W::new(self)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<8> {
        LOCK_W::new(self)
    }
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W<0> {
        DTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "break and dead-time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtr](index.html) module"]
pub struct BDTR_SPEC;
impl crate::RegisterSpec for BDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdtr::R](R) reader structure"]
impl crate::Readable for BDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdtr::W](W) writer structure"]
impl crate::Writable for BDTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
