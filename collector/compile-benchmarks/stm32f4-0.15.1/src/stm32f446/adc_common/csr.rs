#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Analog watchdog flag of ADC 3"]
pub use AWD1_A as AWD3_A;
#[doc = "Analog watchdog flag of ADC 2"]
pub use AWD1_A as AWD2_A;
#[doc = "Field `AWD3` reader - Analog watchdog flag of ADC 3"]
pub use AWD1_R as AWD3_R;
#[doc = "Field `AWD2` reader - Analog watchdog flag of ADC 2"]
pub use AWD1_R as AWD2_R;
#[doc = "End of conversion of ADC 3"]
pub use EOC1_A as EOC3_A;
#[doc = "End of conversion of ADC 2"]
pub use EOC1_A as EOC2_A;
#[doc = "Field `EOC3` reader - End of conversion of ADC 3"]
pub use EOC1_R as EOC3_R;
#[doc = "Field `EOC2` reader - End of conversion of ADC 2"]
pub use EOC1_R as EOC2_R;
#[doc = "Injected channel end of conversion of ADC 3"]
pub use JEOC1_A as JEOC3_A;
#[doc = "Injected channel end of conversion of ADC 2"]
pub use JEOC1_A as JEOC2_A;
#[doc = "Field `JEOC3` reader - Injected channel end of conversion of ADC 3"]
pub use JEOC1_R as JEOC3_R;
#[doc = "Field `JEOC2` reader - Injected channel end of conversion of ADC 2"]
pub use JEOC1_R as JEOC2_R;
#[doc = "Injected channel Start flag of ADC 3"]
pub use JSTRT1_A as JSTRT3_A;
#[doc = "Injected channel Start flag of ADC 2"]
pub use JSTRT1_A as JSTRT2_A;
#[doc = "Field `JSTRT3` reader - Injected channel Start flag of ADC 3"]
pub use JSTRT1_R as JSTRT3_R;
#[doc = "Field `JSTRT2` reader - Injected channel Start flag of ADC 2"]
pub use JSTRT1_R as JSTRT2_R;
#[doc = "Overrun flag of ADC3"]
pub use OVR1_A as OVR3_A;
#[doc = "Overrun flag of ADC 2"]
pub use OVR1_A as OVR2_A;
#[doc = "Field `OVR3` reader - Overrun flag of ADC3"]
pub use OVR1_R as OVR3_R;
#[doc = "Field `OVR2` reader - Overrun flag of ADC 2"]
pub use OVR1_R as OVR2_R;
#[doc = "Regular channel Start flag of ADC 3"]
pub use STRT1_A as STRT3_A;
#[doc = "Regular channel Start flag of ADC 2"]
pub use STRT1_A as STRT2_A;
#[doc = "Field `STRT3` reader - Regular channel Start flag of ADC 3"]
pub use STRT1_R as STRT3_R;
#[doc = "Field `STRT2` reader - Regular channel Start flag of ADC 2"]
pub use STRT1_R as STRT2_R;
#[doc = "Overrun flag of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR1_A {
    #[doc = "0: No overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun occurred"]
    Overrun = 1,
}
impl From<OVR1_A> for bool {
    #[inline(always)]
    fn from(variant: OVR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR1` reader - Overrun flag of ADC 1"]
pub type OVR1_R = crate::BitReader<OVR1_A>;
impl OVR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR1_A {
        match self.bits {
            false => OVR1_A::NoOverrun,
            true => OVR1_A::Overrun,
        }
    }
    #[doc = "Checks if the value of the field is `NoOverrun`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR1_A::NoOverrun
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR1_A::Overrun
    }
}
#[doc = "Regular channel Start flag of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT1_A {
    #[doc = "0: No regular channel conversion started"]
    NotStarted = 0,
    #[doc = "1: Regular channel conversion has started"]
    Started = 1,
}
impl From<STRT1_A> for bool {
    #[inline(always)]
    fn from(variant: STRT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT1` reader - Regular channel Start flag of ADC 1"]
pub type STRT1_R = crate::BitReader<STRT1_A>;
impl STRT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRT1_A {
        match self.bits {
            false => STRT1_A::NotStarted,
            true => STRT1_A::Started,
        }
    }
    #[doc = "Checks if the value of the field is `NotStarted`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRT1_A::NotStarted
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRT1_A::Started
    }
}
#[doc = "Injected channel Start flag of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSTRT1_A {
    #[doc = "0: No injected channel conversion started"]
    NotStarted = 0,
    #[doc = "1: Injected channel conversion has started"]
    Started = 1,
}
impl From<JSTRT1_A> for bool {
    #[inline(always)]
    fn from(variant: JSTRT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSTRT1` reader - Injected channel Start flag of ADC 1"]
pub type JSTRT1_R = crate::BitReader<JSTRT1_A>;
impl JSTRT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSTRT1_A {
        match self.bits {
            false => JSTRT1_A::NotStarted,
            true => JSTRT1_A::Started,
        }
    }
    #[doc = "Checks if the value of the field is `NotStarted`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRT1_A::NotStarted
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRT1_A::Started
    }
}
#[doc = "Injected channel end of conversion of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC1_A {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
    Complete = 1,
}
impl From<JEOC1_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC1` reader - Injected channel end of conversion of ADC 1"]
pub type JEOC1_R = crate::BitReader<JEOC1_A>;
impl JEOC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOC1_A {
        match self.bits {
            false => JEOC1_A::NotComplete,
            true => JEOC1_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC1_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC1_A::Complete
    }
}
#[doc = "End of conversion of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC1_A {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
    Complete = 1,
}
impl From<EOC1_A> for bool {
    #[inline(always)]
    fn from(variant: EOC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC1` reader - End of conversion of ADC 1"]
pub type EOC1_R = crate::BitReader<EOC1_A>;
impl EOC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOC1_A {
        match self.bits {
            false => EOC1_A::NotComplete,
            true => EOC1_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC1_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC1_A::Complete
    }
}
#[doc = "Analog watchdog flag of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1_A {
    #[doc = "0: No analog watchdog event occurred"]
    NoEvent = 0,
    #[doc = "1: Analog watchdog event occurred"]
    Event = 1,
}
impl From<AWD1_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` reader - Analog watchdog flag of ADC 1"]
pub type AWD1_R = crate::BitReader<AWD1_A>;
impl AWD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1_A {
        match self.bits {
            false => AWD1_A::NoEvent,
            true => AWD1_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1_A::Event
    }
}
impl R {
    #[doc = "Bit 21 - Overrun flag of ADC3"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Regular channel Start flag of ADC 3"]
    #[inline(always)]
    pub fn strt3(&self) -> STRT3_R {
        STRT3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Injected channel Start flag of ADC 3"]
    #[inline(always)]
    pub fn jstrt3(&self) -> JSTRT3_R {
        JSTRT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Injected channel end of conversion of ADC 3"]
    #[inline(always)]
    pub fn jeoc3(&self) -> JEOC3_R {
        JEOC3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - End of conversion of ADC 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog watchdog flag of ADC 3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13 - Overrun flag of ADC 2"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Regular channel Start flag of ADC 2"]
    #[inline(always)]
    pub fn strt2(&self) -> STRT2_R {
        STRT2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Injected channel Start flag of ADC 2"]
    #[inline(always)]
    pub fn jstrt2(&self) -> JSTRT2_R {
        JSTRT2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected channel end of conversion of ADC 2"]
    #[inline(always)]
    pub fn jeoc2(&self) -> JEOC2_R {
        JEOC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - End of conversion of ADC 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog flag of ADC 2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun flag of ADC 1"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular channel Start flag of ADC 1"]
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Injected channel Start flag of ADC 1"]
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion of ADC 1"]
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - End of conversion of ADC 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Analog watchdog flag of ADC 1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 1) != 0)
    }
}
#[doc = "ADC Common status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
