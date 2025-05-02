#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKD_A {
    #[doc = "0: t_DTS = t_CK_INT"]
    Div1 = 0,
    #[doc = "1: t_DTS = 2 × t_CK_INT"]
    Div2 = 1,
    #[doc = "2: t_DTS = 4 × t_CK_INT"]
    Div4 = 2,
}
impl From<CKD_A> for u8 {
    #[inline(always)]
    fn from(variant: CKD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKD` reader - Clock division"]
pub type CKD_R = crate::FieldReader<u8, CKD_A>;
impl CKD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKD_A> {
        match self.bits {
            0 => Some(CKD_A::Div1),
            1 => Some(CKD_A::Div2),
            2 => Some(CKD_A::Div4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CKD_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CKD_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CKD_A::Div4
    }
}
#[doc = "Field `CKD` writer - Clock division"]
pub type CKD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, CKD_A, 2, O>;
impl<'a, const O: u8> CKD_W<'a, O> {
    #[doc = "t_DTS = t_CK_INT"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CKD_A::Div1)
    }
    #[doc = "t_DTS = 2 × t_CK_INT"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CKD_A::Div2)
    }
    #[doc = "t_DTS = 4 × t_CK_INT"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CKD_A::Div4)
    }
}
#[doc = "Auto-reload preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARPE_A {
    #[doc = "0: TIMx_APRR register is not buffered"]
    Disabled = 0,
    #[doc = "1: TIMx_APRR register is buffered"]
    Enabled = 1,
}
impl From<ARPE_A> for bool {
    #[inline(always)]
    fn from(variant: ARPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARPE` reader - Auto-reload preload enable"]
pub type ARPE_R = crate::BitReader<ARPE_A>;
impl ARPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARPE_A {
        match self.bits {
            false => ARPE_A::Disabled,
            true => ARPE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARPE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARPE_A::Enabled
    }
}
#[doc = "Field `ARPE` writer - Auto-reload preload enable"]
pub type ARPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, ARPE_A, O>;
impl<'a, const O: u8> ARPE_W<'a, O> {
    #[doc = "TIMx_APRR register is not buffered"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARPE_A::Disabled)
    }
    #[doc = "TIMx_APRR register is buffered"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARPE_A::Enabled)
    }
}
#[doc = "Center-aligned mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMS_A {
    #[doc = "0: The counter counts up or down depending on the direction bit"]
    EdgeAligned = 0,
    #[doc = "1: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
    CenterAligned1 = 1,
    #[doc = "2: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
    CenterAligned2 = 2,
    #[doc = "3: The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
    CenterAligned3 = 3,
}
impl From<CMS_A> for u8 {
    #[inline(always)]
    fn from(variant: CMS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMS` reader - Center-aligned mode selection"]
pub type CMS_R = crate::FieldReader<u8, CMS_A>;
impl CMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMS_A {
        match self.bits {
            0 => CMS_A::EdgeAligned,
            1 => CMS_A::CenterAligned1,
            2 => CMS_A::CenterAligned2,
            3 => CMS_A::CenterAligned3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EdgeAligned`"]
    #[inline(always)]
    pub fn is_edge_aligned(&self) -> bool {
        *self == CMS_A::EdgeAligned
    }
    #[doc = "Checks if the value of the field is `CenterAligned1`"]
    #[inline(always)]
    pub fn is_center_aligned1(&self) -> bool {
        *self == CMS_A::CenterAligned1
    }
    #[doc = "Checks if the value of the field is `CenterAligned2`"]
    #[inline(always)]
    pub fn is_center_aligned2(&self) -> bool {
        *self == CMS_A::CenterAligned2
    }
    #[doc = "Checks if the value of the field is `CenterAligned3`"]
    #[inline(always)]
    pub fn is_center_aligned3(&self) -> bool {
        *self == CMS_A::CenterAligned3
    }
}
#[doc = "Field `CMS` writer - Center-aligned mode selection"]
pub type CMS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, CMS_A, 2, O>;
impl<'a, const O: u8> CMS_W<'a, O> {
    #[doc = "The counter counts up or down depending on the direction bit"]
    #[inline(always)]
    pub fn edge_aligned(self) -> &'a mut W {
        self.variant(CMS_A::EdgeAligned)
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
    #[inline(always)]
    pub fn center_aligned1(self) -> &'a mut W {
        self.variant(CMS_A::CenterAligned1)
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
    #[inline(always)]
    pub fn center_aligned2(self) -> &'a mut W {
        self.variant(CMS_A::CenterAligned2)
    }
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
    #[inline(always)]
    pub fn center_aligned3(self) -> &'a mut W {
        self.variant(CMS_A::CenterAligned3)
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Counter used as upcounter"]
    Up = 0,
    #[doc = "1: Counter used as downcounter"]
    Down = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::Up,
            true => DIR_A::Down,
        }
    }
    #[doc = "Checks if the value of the field is `Up`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == DIR_A::Up
    }
    #[doc = "Checks if the value of the field is `Down`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == DIR_A::Down
    }
}
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, DIR_A, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "Counter used as upcounter"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(DIR_A::Up)
    }
    #[doc = "Counter used as downcounter"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(DIR_A::Down)
    }
}
#[doc = "One-pulse mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPM_A {
    #[doc = "0: Counter is not stopped at update event"]
    Disabled = 0,
    #[doc = "1: Counter stops counting at the next update event (clearing the CEN bit)"]
    Enabled = 1,
}
impl From<OPM_A> for bool {
    #[inline(always)]
    fn from(variant: OPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPM` reader - One-pulse mode"]
pub type OPM_R = crate::BitReader<OPM_A>;
impl OPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPM_A {
        match self.bits {
            false => OPM_A::Disabled,
            true => OPM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPM_A::Enabled
    }
}
#[doc = "Field `OPM` writer - One-pulse mode"]
pub type OPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, OPM_A, O>;
impl<'a, const O: u8> OPM_W<'a, O> {
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPM_A::Disabled)
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPM_A::Enabled)
    }
}
#[doc = "Update request source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URS_A {
    #[doc = "0: Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    AnyEvent = 0,
    #[doc = "1: Only counter overflow/underflow generates an update interrupt or DMA request"]
    CounterOnly = 1,
}
impl From<URS_A> for bool {
    #[inline(always)]
    fn from(variant: URS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `URS` reader - Update request source"]
pub type URS_R = crate::BitReader<URS_A>;
impl URS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> URS_A {
        match self.bits {
            false => URS_A::AnyEvent,
            true => URS_A::CounterOnly,
        }
    }
    #[doc = "Checks if the value of the field is `AnyEvent`"]
    #[inline(always)]
    pub fn is_any_event(&self) -> bool {
        *self == URS_A::AnyEvent
    }
    #[doc = "Checks if the value of the field is `CounterOnly`"]
    #[inline(always)]
    pub fn is_counter_only(&self) -> bool {
        *self == URS_A::CounterOnly
    }
}
#[doc = "Field `URS` writer - Update request source"]
pub type URS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, URS_A, O>;
impl<'a, const O: u8> URS_W<'a, O> {
    #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn any_event(self) -> &'a mut W {
        self.variant(URS_A::AnyEvent)
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn counter_only(self) -> &'a mut W {
        self.variant(URS_A::CounterOnly)
    }
}
#[doc = "Update disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDIS_A {
    #[doc = "0: Update event enabled"]
    Enabled = 0,
    #[doc = "1: Update event disabled"]
    Disabled = 1,
}
impl From<UDIS_A> for bool {
    #[inline(always)]
    fn from(variant: UDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDIS` reader - Update disable"]
pub type UDIS_R = crate::BitReader<UDIS_A>;
impl UDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDIS_A {
        match self.bits {
            false => UDIS_A::Enabled,
            true => UDIS_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDIS_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDIS_A::Disabled
    }
}
#[doc = "Field `UDIS` writer - Update disable"]
pub type UDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, UDIS_A, O>;
impl<'a, const O: u8> UDIS_W<'a, O> {
    #[doc = "Update event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDIS_A::Enabled)
    }
    #[doc = "Update event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDIS_A::Disabled)
    }
}
#[doc = "Counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN_A {
    #[doc = "0: Counter disabled"]
    Disabled = 0,
    #[doc = "1: Counter enabled"]
    Enabled = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Counter enable"]
pub type CEN_R = crate::BitReader<CEN_A>;
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::Disabled,
            true => CEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEN_A::Enabled
    }
}
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CEN_A, O>;
impl<'a, const O: u8> CEN_W<'a, O> {
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEN_A::Disabled)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckd(&mut self) -> CKD_W<8> {
        CKD_W::new(self)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&mut self) -> ARPE_W<7> {
        ARPE_W::new(self)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn cms(&mut self) -> CMS_W<5> {
        CMS_W::new(self)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W<3> {
        OPM_W::new(self)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W<2> {
        URS_W::new(self)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&mut self) -> UDIS_W<1> {
        UDIS_W::new(self)
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
