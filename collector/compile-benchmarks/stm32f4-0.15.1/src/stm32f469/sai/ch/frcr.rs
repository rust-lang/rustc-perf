#[doc = "Register `FRCR` reader"]
pub struct R(crate::R<FRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRCR` writer"]
pub struct W(crate::W<FRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRCR_SPEC>;
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
impl From<crate::W<FRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frame synchronization offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSOFF_A {
    #[doc = "0: FS is asserted on the first bit of the slot 0"]
    OnFirst = 0,
    #[doc = "1: FS is asserted one bit before the first bit of the slot 0"]
    BeforeFirst = 1,
}
impl From<FSOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FSOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSOFF` reader - Frame synchronization offset"]
pub type FSOFF_R = crate::BitReader<FSOFF_A>;
impl FSOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSOFF_A {
        match self.bits {
            false => FSOFF_A::OnFirst,
            true => FSOFF_A::BeforeFirst,
        }
    }
    #[doc = "Checks if the value of the field is `OnFirst`"]
    #[inline(always)]
    pub fn is_on_first(&self) -> bool {
        *self == FSOFF_A::OnFirst
    }
    #[doc = "Checks if the value of the field is `BeforeFirst`"]
    #[inline(always)]
    pub fn is_before_first(&self) -> bool {
        *self == FSOFF_A::BeforeFirst
    }
}
#[doc = "Field `FSOFF` writer - Frame synchronization offset"]
pub type FSOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCR_SPEC, FSOFF_A, O>;
impl<'a, const O: u8> FSOFF_W<'a, O> {
    #[doc = "FS is asserted on the first bit of the slot 0"]
    #[inline(always)]
    pub fn on_first(self) -> &'a mut W {
        self.variant(FSOFF_A::OnFirst)
    }
    #[doc = "FS is asserted one bit before the first bit of the slot 0"]
    #[inline(always)]
    pub fn before_first(self) -> &'a mut W {
        self.variant(FSOFF_A::BeforeFirst)
    }
}
#[doc = "Frame synchronization polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSPOL_A {
    #[doc = "0: FS is active low (falling edge)"]
    FallingEdge = 0,
    #[doc = "1: FS is active high (rising edge)"]
    RisingEdge = 1,
}
impl From<FSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: FSPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSPOL` reader - Frame synchronization polarity"]
pub type FSPOL_R = crate::BitReader<FSPOL_A>;
impl FSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSPOL_A {
        match self.bits {
            false => FSPOL_A::FallingEdge,
            true => FSPOL_A::RisingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == FSPOL_A::FallingEdge
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == FSPOL_A::RisingEdge
    }
}
#[doc = "Field `FSPOL` writer - Frame synchronization polarity"]
pub type FSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCR_SPEC, FSPOL_A, O>;
impl<'a, const O: u8> FSPOL_W<'a, O> {
    #[doc = "FS is active low (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(FSPOL_A::FallingEdge)
    }
    #[doc = "FS is active high (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(FSPOL_A::RisingEdge)
    }
}
#[doc = "Field `FSDEF` reader - Frame synchronization definition"]
pub type FSDEF_R = crate::BitReader<bool>;
#[doc = "Field `FSDEF` writer - Frame synchronization definition"]
pub type FSDEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCR_SPEC, bool, O>;
#[doc = "Field `FSALL` reader - Frame synchronization active level length"]
pub type FSALL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSALL` writer - Frame synchronization active level length"]
pub type FSALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `FRL` reader - Frame length"]
pub type FRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRL` writer - Frame length"]
pub type FRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Frame synchronization definition"]
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    pub fn fsoff(&mut self) -> FSOFF_W<18> {
        FSOFF_W::new(self)
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    pub fn fspol(&mut self) -> FSPOL_W<17> {
        FSPOL_W::new(self)
    }
    #[doc = "Bit 16 - Frame synchronization definition"]
    #[inline(always)]
    pub fn fsdef(&mut self) -> FSDEF_W<16> {
        FSDEF_W::new(self)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    pub fn fsall(&mut self) -> FSALL_W<8> {
        FSALL_W::new(self)
    }
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    pub fn frl(&mut self) -> FRL_W<0> {
        FRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frcr](index.html) module"]
pub struct FRCR_SPEC;
impl crate::RegisterSpec for FRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frcr::R](R) reader structure"]
impl crate::Readable for FRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frcr::W](W) writer structure"]
impl crate::Writable for FRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRCR to value 0x07"]
impl crate::Resettable for FRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
