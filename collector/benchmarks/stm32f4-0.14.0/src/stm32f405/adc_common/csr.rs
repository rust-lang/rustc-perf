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
#[doc = "Overrun flag of ADC3"]
pub type OVR3_A = OVR1_A;
#[doc = "Field `OVR3` reader - Overrun flag of ADC3"]
pub type OVR3_R = OVR1_R;
#[doc = "Regular channel Start flag of ADC 3"]
pub type STRT3_A = STRT1_A;
#[doc = "Field `STRT3` reader - Regular channel Start flag of ADC 3"]
pub type STRT3_R = STRT1_R;
#[doc = "Injected channel Start flag of ADC 3"]
pub type JSTRT3_A = JSTRT1_A;
#[doc = "Field `JSTRT3` reader - Injected channel Start flag of ADC 3"]
pub type JSTRT3_R = JSTRT1_R;
#[doc = "Injected channel end of conversion of ADC 3"]
pub type JEOC3_A = JEOC1_A;
#[doc = "Field `JEOC3` reader - Injected channel end of conversion of ADC 3"]
pub type JEOC3_R = JEOC1_R;
#[doc = "End of conversion of ADC 3"]
pub type EOC3_A = EOC1_A;
#[doc = "Field `EOC3` reader - End of conversion of ADC 3"]
pub type EOC3_R = EOC1_R;
#[doc = "Analog watchdog flag of ADC 3"]
pub type AWD3_A = AWD1_A;
#[doc = "Field `AWD3` reader - Analog watchdog flag of ADC 3"]
pub type AWD3_R = AWD1_R;
#[doc = "Overrun flag of ADC 2"]
pub type OVR2_A = OVR1_A;
#[doc = "Field `OVR2` reader - Overrun flag of ADC 2"]
pub type OVR2_R = OVR1_R;
#[doc = "Regular channel Start flag of ADC 2"]
pub type STRT2_A = STRT1_A;
#[doc = "Field `STRT2` reader - Regular channel Start flag of ADC 2"]
pub type STRT2_R = STRT1_R;
#[doc = "Injected channel Start flag of ADC 2"]
pub type JSTRT2_A = JSTRT1_A;
#[doc = "Field `JSTRT2` reader - Injected channel Start flag of ADC 2"]
pub type JSTRT2_R = JSTRT1_R;
#[doc = "Injected channel end of conversion of ADC 2"]
pub type JEOC2_A = JEOC1_A;
#[doc = "Field `JEOC2` reader - Injected channel end of conversion of ADC 2"]
pub type JEOC2_R = JEOC1_R;
#[doc = "End of conversion of ADC 2"]
pub type EOC2_A = EOC1_A;
#[doc = "Field `EOC2` reader - End of conversion of ADC 2"]
pub type EOC2_R = EOC1_R;
#[doc = "Analog watchdog flag of ADC 2"]
pub type AWD2_A = AWD1_A;
#[doc = "Field `AWD2` reader - Analog watchdog flag of ADC 2"]
pub type AWD2_R = AWD1_R;
#[doc = "Overrun flag of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR1_A {
    #[doc = "0: No overrun occurred"]
    NOOVERRUN = 0,
    #[doc = "1: Overrun occurred"]
    OVERRUN = 1,
}
impl From<OVR1_A> for bool {
    #[inline(always)]
    fn from(variant: OVR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR1` reader - Overrun flag of ADC 1"]
pub struct OVR1_R(crate::FieldReader<bool, OVR1_A>);
impl OVR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR1_A {
        match self.bits {
            false => OVR1_A::NOOVERRUN,
            true => OVR1_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        **self == OVR1_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        **self == OVR1_A::OVERRUN
    }
}
impl core::ops::Deref for OVR1_R {
    type Target = crate::FieldReader<bool, OVR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Regular channel Start flag of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT1_A {
    #[doc = "0: No regular channel conversion started"]
    NOTSTARTED = 0,
    #[doc = "1: Regular channel conversion has started"]
    STARTED = 1,
}
impl From<STRT1_A> for bool {
    #[inline(always)]
    fn from(variant: STRT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT1` reader - Regular channel Start flag of ADC 1"]
pub struct STRT1_R(crate::FieldReader<bool, STRT1_A>);
impl STRT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        STRT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRT1_A {
        match self.bits {
            false => STRT1_A::NOTSTARTED,
            true => STRT1_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == STRT1_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == STRT1_A::STARTED
    }
}
impl core::ops::Deref for STRT1_R {
    type Target = crate::FieldReader<bool, STRT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Injected channel Start flag of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSTRT1_A {
    #[doc = "0: No injected channel conversion started"]
    NOTSTARTED = 0,
    #[doc = "1: Injected channel conversion has started"]
    STARTED = 1,
}
impl From<JSTRT1_A> for bool {
    #[inline(always)]
    fn from(variant: JSTRT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSTRT1` reader - Injected channel Start flag of ADC 1"]
pub struct JSTRT1_R(crate::FieldReader<bool, JSTRT1_A>);
impl JSTRT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSTRT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSTRT1_A {
        match self.bits {
            false => JSTRT1_A::NOTSTARTED,
            true => JSTRT1_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == JSTRT1_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == JSTRT1_A::STARTED
    }
}
impl core::ops::Deref for JSTRT1_R {
    type Target = crate::FieldReader<bool, JSTRT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Injected channel end of conversion of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC1_A {
    #[doc = "0: Conversion is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Conversion complete"]
    COMPLETE = 1,
}
impl From<JEOC1_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC1` reader - Injected channel end of conversion of ADC 1"]
pub struct JEOC1_R(crate::FieldReader<bool, JEOC1_A>);
impl JEOC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEOC1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOC1_A {
        match self.bits {
            false => JEOC1_A::NOTCOMPLETE,
            true => JEOC1_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == JEOC1_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == JEOC1_A::COMPLETE
    }
}
impl core::ops::Deref for JEOC1_R {
    type Target = crate::FieldReader<bool, JEOC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "End of conversion of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC1_A {
    #[doc = "0: Conversion is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Conversion complete"]
    COMPLETE = 1,
}
impl From<EOC1_A> for bool {
    #[inline(always)]
    fn from(variant: EOC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC1` reader - End of conversion of ADC 1"]
pub struct EOC1_R(crate::FieldReader<bool, EOC1_A>);
impl EOC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOC1_A {
        match self.bits {
            false => EOC1_A::NOTCOMPLETE,
            true => EOC1_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == EOC1_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == EOC1_A::COMPLETE
    }
}
impl core::ops::Deref for EOC1_R {
    type Target = crate::FieldReader<bool, EOC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Analog watchdog flag of ADC 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1_A {
    #[doc = "0: No analog watchdog event occurred"]
    NOEVENT = 0,
    #[doc = "1: Analog watchdog event occurred"]
    EVENT = 1,
}
impl From<AWD1_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` reader - Analog watchdog flag of ADC 1"]
pub struct AWD1_R(crate::FieldReader<bool, AWD1_A>);
impl AWD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1_A {
        match self.bits {
            false => AWD1_A::NOEVENT,
            true => AWD1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == AWD1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == AWD1_A::EVENT
    }
}
impl core::ops::Deref for AWD1_R {
    type Target = crate::FieldReader<bool, AWD1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 21 - Overrun flag of ADC3"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Regular channel Start flag of ADC 3"]
    #[inline(always)]
    pub fn strt3(&self) -> STRT3_R {
        STRT3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Injected channel Start flag of ADC 3"]
    #[inline(always)]
    pub fn jstrt3(&self) -> JSTRT3_R {
        JSTRT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Injected channel end of conversion of ADC 3"]
    #[inline(always)]
    pub fn jeoc3(&self) -> JEOC3_R {
        JEOC3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - End of conversion of ADC 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Analog watchdog flag of ADC 3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Overrun flag of ADC 2"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Regular channel Start flag of ADC 2"]
    #[inline(always)]
    pub fn strt2(&self) -> STRT2_R {
        STRT2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Injected channel Start flag of ADC 2"]
    #[inline(always)]
    pub fn jstrt2(&self) -> JSTRT2_R {
        JSTRT2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Injected channel end of conversion of ADC 2"]
    #[inline(always)]
    pub fn jeoc2(&self) -> JEOC2_R {
        JEOC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - End of conversion of ADC 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog flag of ADC 2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun flag of ADC 1"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Regular channel Start flag of ADC 1"]
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Injected channel Start flag of ADC 1"]
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion of ADC 1"]
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of conversion of ADC 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Analog watchdog flag of ADC 1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 0x01) != 0)
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
