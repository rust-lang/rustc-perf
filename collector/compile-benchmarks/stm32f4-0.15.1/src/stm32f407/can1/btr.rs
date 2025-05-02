#[doc = "Register `BTR` reader"]
pub struct R(crate::R<BTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTR` writer"]
pub struct W(crate::W<BTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTR_SPEC>;
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
impl From<crate::W<BTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SILM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SILM_A {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Silent Mode"]
    Silent = 1,
}
impl From<SILM_A> for bool {
    #[inline(always)]
    fn from(variant: SILM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SILM` reader - SILM"]
pub type SILM_R = crate::BitReader<SILM_A>;
impl SILM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SILM_A {
        match self.bits {
            false => SILM_A::Normal,
            true => SILM_A::Silent,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SILM_A::Normal
    }
    #[doc = "Checks if the value of the field is `Silent`"]
    #[inline(always)]
    pub fn is_silent(&self) -> bool {
        *self == SILM_A::Silent
    }
}
#[doc = "Field `SILM` writer - SILM"]
pub type SILM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTR_SPEC, SILM_A, O>;
impl<'a, const O: u8> SILM_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SILM_A::Normal)
    }
    #[doc = "Silent Mode"]
    #[inline(always)]
    pub fn silent(self) -> &'a mut W {
        self.variant(SILM_A::Silent)
    }
}
#[doc = "LBKM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKM_A {
    #[doc = "0: Loop Back Mode disabled"]
    Disabled = 0,
    #[doc = "1: Loop Back Mode enabled"]
    Enabled = 1,
}
impl From<LBKM_A> for bool {
    #[inline(always)]
    fn from(variant: LBKM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKM` reader - LBKM"]
pub type LBKM_R = crate::BitReader<LBKM_A>;
impl LBKM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKM_A {
        match self.bits {
            false => LBKM_A::Disabled,
            true => LBKM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBKM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBKM_A::Enabled
    }
}
#[doc = "Field `LBKM` writer - LBKM"]
pub type LBKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BTR_SPEC, LBKM_A, O>;
impl<'a, const O: u8> LBKM_W<'a, O> {
    #[doc = "Loop Back Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBKM_A::Disabled)
    }
    #[doc = "Loop Back Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBKM_A::Enabled)
    }
}
#[doc = "Field `SJW` reader - SJW"]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - SJW"]
pub type SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TS2` reader - TS2"]
pub type TS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS2` writer - TS2"]
pub type TS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TS1` reader - TS1"]
pub type TS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS1` writer - TS1"]
pub type TS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BRP` reader - BRP"]
pub type BRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRP` writer - BRP"]
pub type BRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&self) -> SILM_R {
        SILM_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&self) -> LBKM_R {
        LBKM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&mut self) -> SILM_W<31> {
        SILM_W::new(self)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&mut self) -> LBKM_W<30> {
        LBKM_W::new(self)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W<24> {
        SJW_W::new(self)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W<20> {
        TS2_W::new(self)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&mut self) -> TS1_W<16> {
        TS1_W::new(self)
    }
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W<0> {
        BRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bit timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr](index.html) module"]
pub struct BTR_SPEC;
impl crate::RegisterSpec for BTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btr::R](R) reader structure"]
impl crate::Readable for BTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btr::W](W) writer structure"]
impl crate::Writable for BTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTR to value 0"]
impl crate::Resettable for BTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
