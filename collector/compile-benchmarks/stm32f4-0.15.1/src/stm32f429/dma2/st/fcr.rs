#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FIFO error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: FE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: FE interrupt enabled"]
    Enabled = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIE` reader - FIFO error interrupt enable"]
pub type FEIE_R = crate::BitReader<FEIE_A>;
impl FEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::Disabled,
            true => FEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FEIE_A::Enabled
    }
}
#[doc = "Field `FEIE` writer - FIFO error interrupt enable"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, FEIE_A, O>;
impl<'a, const O: u8> FEIE_W<'a, O> {
    #[doc = "FE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FEIE_A::Disabled)
    }
    #[doc = "FE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FEIE_A::Enabled)
    }
}
#[doc = "FIFO status\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FS_A {
    #[doc = "0: 0 < fifo_level < 1/4"]
    Quarter1 = 0,
    #[doc = "1: 1/4 <= fifo_level < 1/2"]
    Quarter2 = 1,
    #[doc = "2: 1/2 <= fifo_level < 3/4"]
    Quarter3 = 2,
    #[doc = "3: 3/4 <= fifo_level < full"]
    Quarter4 = 3,
    #[doc = "4: FIFO is empty"]
    Empty = 4,
    #[doc = "5: FIFO is full"]
    Full = 5,
}
impl From<FS_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FS` reader - FIFO status"]
pub type FS_R = crate::FieldReader<u8, FS_A>;
impl FS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FS_A> {
        match self.bits {
            0 => Some(FS_A::Quarter1),
            1 => Some(FS_A::Quarter2),
            2 => Some(FS_A::Quarter3),
            3 => Some(FS_A::Quarter4),
            4 => Some(FS_A::Empty),
            5 => Some(FS_A::Full),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Quarter1`"]
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FS_A::Quarter1
    }
    #[doc = "Checks if the value of the field is `Quarter2`"]
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FS_A::Quarter2
    }
    #[doc = "Checks if the value of the field is `Quarter3`"]
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FS_A::Quarter3
    }
    #[doc = "Checks if the value of the field is `Quarter4`"]
    #[inline(always)]
    pub fn is_quarter4(&self) -> bool {
        *self == FS_A::Quarter4
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FS_A::Empty
    }
    #[doc = "Checks if the value of the field is `Full`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FS_A::Full
    }
}
#[doc = "Direct mode disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMDIS_A {
    #[doc = "0: Direct mode is enabled"]
    Enabled = 0,
    #[doc = "1: Direct mode is disabled"]
    Disabled = 1,
}
impl From<DMDIS_A> for bool {
    #[inline(always)]
    fn from(variant: DMDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMDIS` reader - Direct mode disable"]
pub type DMDIS_R = crate::BitReader<DMDIS_A>;
impl DMDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMDIS_A {
        match self.bits {
            false => DMDIS_A::Enabled,
            true => DMDIS_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMDIS_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMDIS_A::Disabled
    }
}
#[doc = "Field `DMDIS` writer - Direct mode disable"]
pub type DMDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, DMDIS_A, O>;
impl<'a, const O: u8> DMDIS_W<'a, O> {
    #[doc = "Direct mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMDIS_A::Enabled)
    }
    #[doc = "Direct mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMDIS_A::Disabled)
    }
}
#[doc = "FIFO threshold selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTH_A {
    #[doc = "0: 1/4 full FIFO"]
    Quarter = 0,
    #[doc = "1: 1/2 full FIFO"]
    Half = 1,
    #[doc = "2: 3/4 full FIFO"]
    ThreeQuarters = 2,
    #[doc = "3: Full FIFO"]
    Full = 3,
}
impl From<FTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTH` reader - FIFO threshold selection"]
pub type FTH_R = crate::FieldReader<u8, FTH_A>;
impl FTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTH_A {
        match self.bits {
            0 => FTH_A::Quarter,
            1 => FTH_A::Half,
            2 => FTH_A::ThreeQuarters,
            3 => FTH_A::Full,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Quarter`"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FTH_A::Quarter
    }
    #[doc = "Checks if the value of the field is `Half`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FTH_A::Half
    }
    #[doc = "Checks if the value of the field is `ThreeQuarters`"]
    #[inline(always)]
    pub fn is_three_quarters(&self) -> bool {
        *self == FTH_A::ThreeQuarters
    }
    #[doc = "Checks if the value of the field is `Full`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTH_A::Full
    }
}
#[doc = "Field `FTH` writer - FIFO threshold selection"]
pub type FTH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCR_SPEC, u8, FTH_A, 2, O>;
impl<'a, const O: u8> FTH_W<'a, O> {
    #[doc = "1/4 full FIFO"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(FTH_A::Quarter)
    }
    #[doc = "1/2 full FIFO"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FTH_A::Half)
    }
    #[doc = "3/4 full FIFO"]
    #[inline(always)]
    pub fn three_quarters(self) -> &'a mut W {
        self.variant(FTH_A::ThreeQuarters)
    }
    #[doc = "Full FIFO"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(FTH_A::Full)
    }
}
impl R {
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W<7> {
        FEIE_W::new(self)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W<2> {
        DMDIS_W::new(self)
    }
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<0> {
        FTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR to value 0x21"]
impl crate::Resettable for FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
