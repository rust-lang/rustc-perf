#[doc = "Register `CDSR` reader"]
pub struct R(crate::R<CDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Horizontal Synchronization display Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSYNCS_A {
    #[doc = "0: Currently not in HSYNC phase"]
    NotActive = 0,
    #[doc = "1: Currently in HSYNC phase"]
    Active = 1,
}
impl From<HSYNCS_A> for bool {
    #[inline(always)]
    fn from(variant: HSYNCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSYNCS` reader - Horizontal Synchronization display Status"]
pub type HSYNCS_R = crate::BitReader<HSYNCS_A>;
impl HSYNCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSYNCS_A {
        match self.bits {
            false => HSYNCS_A::NotActive,
            true => HSYNCS_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `NotActive`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == HSYNCS_A::NotActive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HSYNCS_A::Active
    }
}
#[doc = "Vertical Synchronization display Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSYNCS_A {
    #[doc = "0: Currently not in VSYNC phase"]
    NotActive = 0,
    #[doc = "1: Currently in VSYNC phase"]
    Active = 1,
}
impl From<VSYNCS_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSYNCS` reader - Vertical Synchronization display Status"]
pub type VSYNCS_R = crate::BitReader<VSYNCS_A>;
impl VSYNCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSYNCS_A {
        match self.bits {
            false => VSYNCS_A::NotActive,
            true => VSYNCS_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `NotActive`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == VSYNCS_A::NotActive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == VSYNCS_A::Active
    }
}
#[doc = "Horizontal Data Enable display Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDES_A {
    #[doc = "0: Currently not in horizontal Data Enable phase"]
    NotActive = 0,
    #[doc = "1: Currently in horizontal Data Enable phase"]
    Active = 1,
}
impl From<HDES_A> for bool {
    #[inline(always)]
    fn from(variant: HDES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDES` reader - Horizontal Data Enable display Status"]
pub type HDES_R = crate::BitReader<HDES_A>;
impl HDES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDES_A {
        match self.bits {
            false => HDES_A::NotActive,
            true => HDES_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `NotActive`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == HDES_A::NotActive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HDES_A::Active
    }
}
#[doc = "Vertical Data Enable display Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDES_A {
    #[doc = "0: Currently not in vertical Data Enable phase"]
    NotActive = 0,
    #[doc = "1: Currently in vertical Data Enable phase"]
    Active = 1,
}
impl From<VDES_A> for bool {
    #[inline(always)]
    fn from(variant: VDES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDES` reader - Vertical Data Enable display Status"]
pub type VDES_R = crate::BitReader<VDES_A>;
impl VDES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDES_A {
        match self.bits {
            false => VDES_A::NotActive,
            true => VDES_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `NotActive`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == VDES_A::NotActive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == VDES_A::Active
    }
}
impl R {
    #[doc = "Bit 3 - Horizontal Synchronization display Status"]
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Vertical Synchronization display Status"]
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Horizontal Data Enable display Status"]
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Vertical Data Enable display Status"]
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Current Display Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdsr](index.html) module"]
pub struct CDSR_SPEC;
impl crate::RegisterSpec for CDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdsr::R](R) reader structure"]
impl crate::Readable for CDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDSR to value 0x0f"]
impl crate::Resettable for CDSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
