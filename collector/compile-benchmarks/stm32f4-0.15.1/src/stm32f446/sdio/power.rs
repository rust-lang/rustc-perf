#[doc = "Register `POWER` reader"]
pub struct R(crate::R<POWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER` writer"]
pub struct W(crate::W<POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_SPEC>;
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
impl From<crate::W<POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power supply control bits. These bits are used to define the current functional state of the card clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRCTRL_A {
    #[doc = "0: Power off"]
    PowerOff = 0,
    #[doc = "3: Power on"]
    PowerOn = 3,
}
impl From<PWRCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWRCTRL` reader - Power supply control bits. These bits are used to define the current functional state of the card clock"]
pub type PWRCTRL_R = crate::FieldReader<u8, PWRCTRL_A>;
impl PWRCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWRCTRL_A> {
        match self.bits {
            0 => Some(PWRCTRL_A::PowerOff),
            3 => Some(PWRCTRL_A::PowerOn),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PowerOff`"]
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        *self == PWRCTRL_A::PowerOff
    }
    #[doc = "Checks if the value of the field is `PowerOn`"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == PWRCTRL_A::PowerOn
    }
}
#[doc = "Field `PWRCTRL` writer - Power supply control bits. These bits are used to define the current functional state of the card clock"]
pub type PWRCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POWER_SPEC, u8, PWRCTRL_A, 2, O>;
impl<'a, const O: u8> PWRCTRL_W<'a, O> {
    #[doc = "Power off"]
    #[inline(always)]
    pub fn power_off(self) -> &'a mut W {
        self.variant(PWRCTRL_A::PowerOff)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut W {
        self.variant(PWRCTRL_A::PowerOn)
    }
}
impl R {
    #[doc = "Bits 0:1 - Power supply control bits. These bits are used to define the current functional state of the card clock"]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power supply control bits. These bits are used to define the current functional state of the card clock"]
    #[inline(always)]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<0> {
        PWRCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](index.html) module"]
pub struct POWER_SPEC;
impl crate::RegisterSpec for POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power::R](R) reader structure"]
impl crate::Readable for POWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power::W](W) writer structure"]
impl crate::Writable for POWER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for POWER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
