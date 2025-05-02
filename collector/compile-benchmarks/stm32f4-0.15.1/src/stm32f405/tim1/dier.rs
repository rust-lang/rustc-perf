#[doc = "Register `DIER` reader"]
pub struct R(crate::R<DIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIER` writer"]
pub struct W(crate::W<DIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIER_SPEC>;
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
impl From<crate::W<DIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trigger DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDE_A {
    #[doc = "0: Trigger DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Trigger DMA request enabled"]
    Enabled = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDE` reader - Trigger DMA request enable"]
pub type TDE_R = crate::BitReader<TDE_A>;
impl TDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::Disabled,
            true => TDE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDE_A::Enabled
    }
}
#[doc = "Field `TDE` writer - Trigger DMA request enable"]
pub type TDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, TDE_A, O>;
impl<'a, const O: u8> TDE_W<'a, O> {
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDE_A::Disabled)
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDE_A::Enabled)
    }
}
#[doc = "Field `COMDE` reader - COM DMA request enable"]
pub type COMDE_R = crate::BitReader<bool>;
#[doc = "Field `COMDE` writer - COM DMA request enable"]
pub type COMDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Capture/Compare 4 DMA request enable"]
pub use CC1DE_A as CC4DE_A;
#[doc = "Capture/Compare 3 DMA request enable"]
pub use CC1DE_A as CC3DE_A;
#[doc = "Capture/Compare 2 DMA request enable"]
pub use CC1DE_A as CC2DE_A;
#[doc = "Field `CC4DE` reader - Capture/Compare 4 DMA request enable"]
pub use CC1DE_R as CC4DE_R;
#[doc = "Field `CC3DE` reader - Capture/Compare 3 DMA request enable"]
pub use CC1DE_R as CC3DE_R;
#[doc = "Field `CC2DE` reader - Capture/Compare 2 DMA request enable"]
pub use CC1DE_R as CC2DE_R;
#[doc = "Field `CC4DE` writer - Capture/Compare 4 DMA request enable"]
pub use CC1DE_W as CC4DE_W;
#[doc = "Field `CC3DE` writer - Capture/Compare 3 DMA request enable"]
pub use CC1DE_W as CC3DE_W;
#[doc = "Field `CC2DE` writer - Capture/Compare 2 DMA request enable"]
pub use CC1DE_W as CC2DE_W;
#[doc = "Capture/Compare 1 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1DE_A {
    #[doc = "0: CCx DMA request disabled"]
    Disabled = 0,
    #[doc = "1: CCx DMA request enabled"]
    Enabled = 1,
}
impl From<CC1DE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1DE` reader - Capture/Compare 1 DMA request enable"]
pub type CC1DE_R = crate::BitReader<CC1DE_A>;
impl CC1DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1DE_A {
        match self.bits {
            false => CC1DE_A::Disabled,
            true => CC1DE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1DE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1DE_A::Enabled
    }
}
#[doc = "Field `CC1DE` writer - Capture/Compare 1 DMA request enable"]
pub type CC1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, CC1DE_A, O>;
impl<'a, const O: u8> CC1DE_W<'a, O> {
    #[doc = "CCx DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1DE_A::Disabled)
    }
    #[doc = "CCx DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1DE_A::Enabled)
    }
}
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDE_A {
    #[doc = "0: Update DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Update DMA request enabled"]
    Enabled = 1,
}
impl From<UDE_A> for bool {
    #[inline(always)]
    fn from(variant: UDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UDE_R = crate::BitReader<UDE_A>;
impl UDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDE_A {
        match self.bits {
            false => UDE_A::Disabled,
            true => UDE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDE_A::Enabled
    }
}
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, UDE_A, O>;
impl<'a, const O: u8> UDE_W<'a, O> {
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDE_A::Disabled)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDE_A::Enabled)
    }
}
#[doc = "Trigger interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Trigger interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Trigger interrupt enabled"]
    Enabled = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Trigger interrupt enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::Disabled,
            true => TIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIE_A::Enabled
    }
}
#[doc = "Field `TIE` writer - Trigger interrupt enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIE_A::Disabled)
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIE_A::Enabled)
    }
}
#[doc = "Capture/Compare 4 interrupt enable"]
pub use CC1IE_A as CC4IE_A;
#[doc = "Capture/Compare 3 interrupt enable"]
pub use CC1IE_A as CC3IE_A;
#[doc = "Capture/Compare 2 interrupt enable"]
pub use CC1IE_A as CC2IE_A;
#[doc = "Field `CC4IE` reader - Capture/Compare 4 interrupt enable"]
pub use CC1IE_R as CC4IE_R;
#[doc = "Field `CC3IE` reader - Capture/Compare 3 interrupt enable"]
pub use CC1IE_R as CC3IE_R;
#[doc = "Field `CC2IE` reader - Capture/Compare 2 interrupt enable"]
pub use CC1IE_R as CC2IE_R;
#[doc = "Field `CC4IE` writer - Capture/Compare 4 interrupt enable"]
pub use CC1IE_W as CC4IE_W;
#[doc = "Field `CC3IE` writer - Capture/Compare 3 interrupt enable"]
pub use CC1IE_W as CC3IE_W;
#[doc = "Field `CC2IE` writer - Capture/Compare 2 interrupt enable"]
pub use CC1IE_W as CC2IE_W;
#[doc = "Capture/Compare 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IE_A {
    #[doc = "0: CCx interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CCx interrupt enabled"]
    Enabled = 1,
}
impl From<CC1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IE` reader - Capture/Compare 1 interrupt enable"]
pub type CC1IE_R = crate::BitReader<CC1IE_A>;
impl CC1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1IE_A {
        match self.bits {
            false => CC1IE_A::Disabled,
            true => CC1IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1IE_A::Enabled
    }
}
#[doc = "Field `CC1IE` writer - Capture/Compare 1 interrupt enable"]
pub type CC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, CC1IE_A, O>;
impl<'a, const O: u8> CC1IE_W<'a, O> {
    #[doc = "CCx interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1IE_A::Disabled)
    }
    #[doc = "CCx interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1IE_A::Enabled)
    }
}
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIE_A {
    #[doc = "0: Update interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Update interrupt enabled"]
    Enabled = 1,
}
impl From<UIE_A> for bool {
    #[inline(always)]
    fn from(variant: UIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UIE_R = crate::BitReader<UIE_A>;
impl UIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIE_A {
        match self.bits {
            false => UIE_A::Disabled,
            true => UIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UIE_A::Enabled
    }
}
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, UIE_A, O>;
impl<'a, const O: u8> UIE_W<'a, O> {
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UIE_A::Disabled)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UIE_A::Enabled)
    }
}
#[doc = "Field `BIE` reader - Break interrupt enable"]
pub type BIE_R = crate::BitReader<bool>;
#[doc = "Field `BIE` writer - Break interrupt enable"]
pub type BIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
#[doc = "Field `COMIE` reader - COM interrupt enable"]
pub type COMIE_R = crate::BitReader<bool>;
#[doc = "Field `COMIE` writer - COM interrupt enable"]
pub type COMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<14> {
        TDE_W::new(self)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comde(&mut self) -> COMDE_W<13> {
        COMDE_W::new(self)
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub fn cc4de(&mut self) -> CC4DE_W<12> {
        CC4DE_W::new(self)
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn cc3de(&mut self) -> CC3DE_W<11> {
        CC3DE_W::new(self)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W<10> {
        CC2DE_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<9> {
        CC1DE_W::new(self)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<8> {
        UDE_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<6> {
        TIE_W::new(self)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W<4> {
        CC4IE_W::new(self)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W<3> {
        CC3IE_W::new(self)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<2> {
        CC2IE_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<1> {
        CC1IE_W::new(self)
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<0> {
        UIE_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W<7> {
        BIE_W::new(self)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W<5> {
        COMIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dier](index.html) module"]
pub struct DIER_SPEC;
impl crate::RegisterSpec for DIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dier::R](R) reader structure"]
impl crate::Readable for DIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dier::W](W) writer structure"]
impl crate::Writable for DIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
