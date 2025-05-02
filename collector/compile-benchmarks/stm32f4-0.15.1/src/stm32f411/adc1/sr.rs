#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    #[doc = "0: No overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun occurred"]
    Overrun = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - Overrun"]
pub type OVR_R = crate::BitReader<OVR_A>;
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NoOverrun,
            true => OVR_A::Overrun,
        }
    }
    #[doc = "Checks if the value of the field is `NoOverrun`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NoOverrun
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::Overrun
    }
}
#[doc = "Field `OVR` writer - Overrun"]
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, OVR_A, O>;
impl<'a, const O: u8> OVR_W<'a, O> {
    #[doc = "No overrun occurred"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OVR_A::NoOverrun)
    }
    #[doc = "Overrun occurred"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OVR_A::Overrun)
    }
}
#[doc = "Regular channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT_A {
    #[doc = "0: No regular channel conversion started"]
    NotStarted = 0,
    #[doc = "1: Regular channel conversion has started"]
    Started = 1,
}
impl From<STRT_A> for bool {
    #[inline(always)]
    fn from(variant: STRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT` reader - Regular channel start flag"]
pub type STRT_R = crate::BitReader<STRT_A>;
impl STRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRT_A {
        match self.bits {
            false => STRT_A::NotStarted,
            true => STRT_A::Started,
        }
    }
    #[doc = "Checks if the value of the field is `NotStarted`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRT_A::NotStarted
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRT_A::Started
    }
}
#[doc = "Field `STRT` writer - Regular channel start flag"]
pub type STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, STRT_A, O>;
impl<'a, const O: u8> STRT_W<'a, O> {
    #[doc = "No regular channel conversion started"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(STRT_A::NotStarted)
    }
    #[doc = "Regular channel conversion has started"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(STRT_A::Started)
    }
}
#[doc = "Injected channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSTRT_A {
    #[doc = "0: No injected channel conversion started"]
    NotStarted = 0,
    #[doc = "1: Injected channel conversion has started"]
    Started = 1,
}
impl From<JSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: JSTRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSTRT` reader - Injected channel start flag"]
pub type JSTRT_R = crate::BitReader<JSTRT_A>;
impl JSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSTRT_A {
        match self.bits {
            false => JSTRT_A::NotStarted,
            true => JSTRT_A::Started,
        }
    }
    #[doc = "Checks if the value of the field is `NotStarted`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRT_A::NotStarted
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRT_A::Started
    }
}
#[doc = "Field `JSTRT` writer - Injected channel start flag"]
pub type JSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, JSTRT_A, O>;
impl<'a, const O: u8> JSTRT_W<'a, O> {
    #[doc = "No injected channel conversion started"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(JSTRT_A::NotStarted)
    }
    #[doc = "Injected channel conversion has started"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(JSTRT_A::Started)
    }
}
#[doc = "Injected channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC_A {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
    Complete = 1,
}
impl From<JEOC_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC` reader - Injected channel end of conversion"]
pub type JEOC_R = crate::BitReader<JEOC_A>;
impl JEOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOC_A {
        match self.bits {
            false => JEOC_A::NotComplete,
            true => JEOC_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC_A::Complete
    }
}
#[doc = "Field `JEOC` writer - Injected channel end of conversion"]
pub type JEOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, JEOC_A, O>;
impl<'a, const O: u8> JEOC_W<'a, O> {
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn not_complete(self) -> &'a mut W {
        self.variant(JEOC_A::NotComplete)
    }
    #[doc = "Conversion complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(JEOC_A::Complete)
    }
}
#[doc = "Regular channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_A {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
    Complete = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` reader - Regular channel end of conversion"]
pub type EOC_R = crate::BitReader<EOC_A>;
impl EOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::NotComplete,
            true => EOC_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC_A::Complete
    }
}
#[doc = "Field `EOC` writer - Regular channel end of conversion"]
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, EOC_A, O>;
impl<'a, const O: u8> EOC_W<'a, O> {
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn not_complete(self) -> &'a mut W {
        self.variant(EOC_A::NotComplete)
    }
    #[doc = "Conversion complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(EOC_A::Complete)
    }
}
#[doc = "Analog watchdog flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD_A {
    #[doc = "0: No analog watchdog event occurred"]
    NoEvent = 0,
    #[doc = "1: Analog watchdog event occurred"]
    Event = 1,
}
impl From<AWD_A> for bool {
    #[inline(always)]
    fn from(variant: AWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD` reader - Analog watchdog flag"]
pub type AWD_R = crate::BitReader<AWD_A>;
impl AWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD_A {
        match self.bits {
            false => AWD_A::NoEvent,
            true => AWD_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD_A::Event
    }
}
#[doc = "Field `AWD` writer - Analog watchdog flag"]
pub type AWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, AWD_A, O>;
impl<'a, const O: u8> AWD_W<'a, O> {
    #[doc = "No analog watchdog event occurred"]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(AWD_A::NoEvent)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(AWD_A::Event)
    }
}
impl R {
    #[doc = "Bit 5 - Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Overrun"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<5> {
        OVR_W::new(self)
    }
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<4> {
        STRT_W::new(self)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    pub fn jstrt(&mut self) -> JSTRT_W<3> {
        JSTRT_W::new(self)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W<2> {
        JEOC_W::new(self)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<1> {
        EOC_W::new(self)
    }
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&mut self) -> AWD_W<0> {
        AWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
