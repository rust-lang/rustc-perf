#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Overrun flag of ADC3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR3_A {
    #[doc = "0: No overrun occurred"]
    NOOVERRUN = 0,
    #[doc = "1: Overrun occurred"]
    OVERRUN = 1,
}
impl From<OVR3_A> for bool {
    #[inline(always)]
    fn from(variant: OVR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVR3`"]
pub type OVR3_R = crate::R<bool, OVR3_A>;
impl OVR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR3_A {
        match self.bits {
            false => OVR3_A::NOOVERRUN,
            true => OVR3_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR3_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR3_A::OVERRUN
    }
}
#[doc = "Regular channel Start flag of ADC 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT3_A {
    #[doc = "0: No regular channel conversion started"]
    NOTSTARTED = 0,
    #[doc = "1: Regular channel conversion has started"]
    STARTED = 1,
}
impl From<STRT3_A> for bool {
    #[inline(always)]
    fn from(variant: STRT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STRT3`"]
pub type STRT3_R = crate::R<bool, STRT3_A>;
impl STRT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRT3_A {
        match self.bits {
            false => STRT3_A::NOTSTARTED,
            true => STRT3_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRT3_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRT3_A::STARTED
    }
}
#[doc = "Injected channel Start flag of ADC 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSTRT3_A {
    #[doc = "0: No injected channel conversion started"]
    NOTSTARTED = 0,
    #[doc = "1: Injected channel conversion has started"]
    STARTED = 1,
}
impl From<JSTRT3_A> for bool {
    #[inline(always)]
    fn from(variant: JSTRT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JSTRT3`"]
pub type JSTRT3_R = crate::R<bool, JSTRT3_A>;
impl JSTRT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSTRT3_A {
        match self.bits {
            false => JSTRT3_A::NOTSTARTED,
            true => JSTRT3_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRT3_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRT3_A::STARTED
    }
}
#[doc = "Injected channel end of conversion of ADC 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC3_A {
    #[doc = "0: Conversion is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Conversion complete"]
    COMPLETE = 1,
}
impl From<JEOC3_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JEOC3`"]
pub type JEOC3_R = crate::R<bool, JEOC3_A>;
impl JEOC3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOC3_A {
        match self.bits {
            false => JEOC3_A::NOTCOMPLETE,
            true => JEOC3_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC3_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC3_A::COMPLETE
    }
}
#[doc = "End of conversion of ADC 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC3_A {
    #[doc = "0: Conversion is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Conversion complete"]
    COMPLETE = 1,
}
impl From<EOC3_A> for bool {
    #[inline(always)]
    fn from(variant: EOC3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOC3`"]
pub type EOC3_R = crate::R<bool, EOC3_A>;
impl EOC3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOC3_A {
        match self.bits {
            false => EOC3_A::NOTCOMPLETE,
            true => EOC3_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC3_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC3_A::COMPLETE
    }
}
#[doc = "Analog watchdog flag of ADC 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD3_A {
    #[doc = "0: No analog watchdog event occurred"]
    NOEVENT = 0,
    #[doc = "1: Analog watchdog event occurred"]
    EVENT = 1,
}
impl From<AWD3_A> for bool {
    #[inline(always)]
    fn from(variant: AWD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWD3`"]
pub type AWD3_R = crate::R<bool, AWD3_A>;
impl AWD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD3_A {
        match self.bits {
            false => AWD3_A::NOEVENT,
            true => AWD3_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD3_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD3_A::EVENT
    }
}
#[doc = "Overrun flag of ADC 2"]
pub type OVR2_A = OVR3_A;
#[doc = "Reader of field `OVR2`"]
pub type OVR2_R = crate::R<bool, OVR3_A>;
#[doc = "Regular channel Start flag of ADC 2"]
pub type STRT2_A = STRT3_A;
#[doc = "Reader of field `STRT2`"]
pub type STRT2_R = crate::R<bool, STRT3_A>;
#[doc = "Injected channel Start flag of ADC 2"]
pub type JSTRT2_A = JSTRT3_A;
#[doc = "Reader of field `JSTRT2`"]
pub type JSTRT2_R = crate::R<bool, JSTRT3_A>;
#[doc = "Injected channel end of conversion of ADC 2"]
pub type JEOC2_A = JEOC3_A;
#[doc = "Reader of field `JEOC2`"]
pub type JEOC2_R = crate::R<bool, JEOC3_A>;
#[doc = "End of conversion of ADC 2"]
pub type EOC2_A = EOC3_A;
#[doc = "Reader of field `EOC2`"]
pub type EOC2_R = crate::R<bool, EOC3_A>;
#[doc = "Analog watchdog flag of ADC 2"]
pub type AWD2_A = AWD3_A;
#[doc = "Reader of field `AWD2`"]
pub type AWD2_R = crate::R<bool, AWD3_A>;
#[doc = "Overrun flag of ADC 1"]
pub type OVR1_A = OVR3_A;
#[doc = "Reader of field `OVR1`"]
pub type OVR1_R = crate::R<bool, OVR3_A>;
#[doc = "Regular channel Start flag of ADC 1"]
pub type STRT1_A = STRT3_A;
#[doc = "Reader of field `STRT1`"]
pub type STRT1_R = crate::R<bool, STRT3_A>;
#[doc = "Injected channel Start flag of ADC 1"]
pub type JSTRT1_A = JSTRT3_A;
#[doc = "Reader of field `JSTRT1`"]
pub type JSTRT1_R = crate::R<bool, JSTRT3_A>;
#[doc = "Injected channel end of conversion of ADC 1"]
pub type JEOC1_A = JEOC3_A;
#[doc = "Reader of field `JEOC1`"]
pub type JEOC1_R = crate::R<bool, JEOC3_A>;
#[doc = "End of conversion of ADC 1"]
pub type EOC1_A = EOC3_A;
#[doc = "Reader of field `EOC1`"]
pub type EOC1_R = crate::R<bool, EOC3_A>;
#[doc = "Analog watchdog flag of ADC 1"]
pub type AWD1_A = AWD3_A;
#[doc = "Reader of field `AWD1`"]
pub type AWD1_R = crate::R<bool, AWD3_A>;
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
