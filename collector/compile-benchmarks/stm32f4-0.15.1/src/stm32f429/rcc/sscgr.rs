#[doc = "Register `SSCGR` reader"]
pub struct R(crate::R<SSCGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSCGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSCGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSCGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSCGR` writer"]
pub struct W(crate::W<SSCGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSCGR_SPEC>;
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
impl From<crate::W<SSCGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSCGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Spread spectrum modulation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSCGEN_A {
    #[doc = "0: Spread spectrum modulation disabled"]
    Disabled = 0,
    #[doc = "1: Spread spectrum modulation enabled"]
    Enabled = 1,
}
impl From<SSCGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSCGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCGEN` reader - Spread spectrum modulation enable"]
pub type SSCGEN_R = crate::BitReader<SSCGEN_A>;
impl SSCGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSCGEN_A {
        match self.bits {
            false => SSCGEN_A::Disabled,
            true => SSCGEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSCGEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSCGEN_A::Enabled
    }
}
#[doc = "Field `SSCGEN` writer - Spread spectrum modulation enable"]
pub type SSCGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCGR_SPEC, SSCGEN_A, O>;
impl<'a, const O: u8> SSCGEN_W<'a, O> {
    #[doc = "Spread spectrum modulation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSCGEN_A::Disabled)
    }
    #[doc = "Spread spectrum modulation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSCGEN_A::Enabled)
    }
}
#[doc = "Spread Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPREADSEL_A {
    #[doc = "0: Center spread"]
    Center = 0,
    #[doc = "1: Down spread"]
    Down = 1,
}
impl From<SPREADSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SPREADSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPREADSEL` reader - Spread Select"]
pub type SPREADSEL_R = crate::BitReader<SPREADSEL_A>;
impl SPREADSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPREADSEL_A {
        match self.bits {
            false => SPREADSEL_A::Center,
            true => SPREADSEL_A::Down,
        }
    }
    #[doc = "Checks if the value of the field is `Center`"]
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        *self == SPREADSEL_A::Center
    }
    #[doc = "Checks if the value of the field is `Down`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == SPREADSEL_A::Down
    }
}
#[doc = "Field `SPREADSEL` writer - Spread Select"]
pub type SPREADSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCGR_SPEC, SPREADSEL_A, O>;
impl<'a, const O: u8> SPREADSEL_W<'a, O> {
    #[doc = "Center spread"]
    #[inline(always)]
    pub fn center(self) -> &'a mut W {
        self.variant(SPREADSEL_A::Center)
    }
    #[doc = "Down spread"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(SPREADSEL_A::Down)
    }
}
#[doc = "Field `INCSTEP` reader - Incrementation step"]
pub type INCSTEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INCSTEP` writer - Incrementation step"]
pub type INCSTEP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SSCGR_SPEC, u16, u16, 15, O>;
#[doc = "Field `MODPER` reader - Modulation period"]
pub type MODPER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MODPER` writer - Modulation period"]
pub type MODPER_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SSCGR_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&self) -> SSCGEN_R {
        SSCGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&self) -> SPREADSEL_R {
        SPREADSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&self) -> INCSTEP_R {
        INCSTEP_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&self) -> MODPER_R {
        MODPER_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&mut self) -> SSCGEN_W<31> {
        SSCGEN_W::new(self)
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&mut self) -> SPREADSEL_W<30> {
        SPREADSEL_W::new(self)
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&mut self) -> INCSTEP_W<13> {
        INCSTEP_W::new(self)
    }
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&mut self) -> MODPER_W<0> {
        MODPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spread spectrum clock generation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sscgr](index.html) module"]
pub struct SSCGR_SPEC;
impl crate::RegisterSpec for SSCGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sscgr::R](R) reader structure"]
impl crate::Readable for SSCGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sscgr::W](W) writer structure"]
impl crate::Writable for SSCGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSCGR to value 0"]
impl crate::Resettable for SSCGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
