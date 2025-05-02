#[doc = "Register `SR%s` reader"]
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
#[doc = "Register `SR%s` writer"]
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
#[doc = "FEMPT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEMPT_A {
    #[doc = "0: FIFO not empty"]
    NotEmpty = 0,
    #[doc = "1: FIFO empty"]
    Empty = 1,
}
impl From<FEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: FEMPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEMPT` reader - FEMPT"]
pub type FEMPT_R = crate::BitReader<FEMPT_A>;
impl FEMPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEMPT_A {
        match self.bits {
            false => FEMPT_A::NotEmpty,
            true => FEMPT_A::Empty,
        }
    }
    #[doc = "Checks if the value of the field is `NotEmpty`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == FEMPT_A::NotEmpty
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FEMPT_A::Empty
    }
}
#[doc = "IFEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFEN_A {
    #[doc = "0: Interrupt falling edge detection request disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt falling edge detection request enabled"]
    Enabled = 1,
}
impl From<IFEN_A> for bool {
    #[inline(always)]
    fn from(variant: IFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFEN` reader - IFEN"]
pub type IFEN_R = crate::BitReader<IFEN_A>;
impl IFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFEN_A {
        match self.bits {
            false => IFEN_A::Disabled,
            true => IFEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IFEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IFEN_A::Enabled
    }
}
#[doc = "Field `IFEN` writer - IFEN"]
pub type IFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, IFEN_A, O>;
impl<'a, const O: u8> IFEN_W<'a, O> {
    #[doc = "Interrupt falling edge detection request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IFEN_A::Disabled)
    }
    #[doc = "Interrupt falling edge detection request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IFEN_A::Enabled)
    }
}
#[doc = "ILEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILEN_A {
    #[doc = "0: Interrupt high-level detection request disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt high-level detection request enabled"]
    Enabled = 1,
}
impl From<ILEN_A> for bool {
    #[inline(always)]
    fn from(variant: ILEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILEN` reader - ILEN"]
pub type ILEN_R = crate::BitReader<ILEN_A>;
impl ILEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILEN_A {
        match self.bits {
            false => ILEN_A::Disabled,
            true => ILEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ILEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ILEN_A::Enabled
    }
}
#[doc = "Field `ILEN` writer - ILEN"]
pub type ILEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, ILEN_A, O>;
impl<'a, const O: u8> ILEN_W<'a, O> {
    #[doc = "Interrupt high-level detection request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ILEN_A::Disabled)
    }
    #[doc = "Interrupt high-level detection request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ILEN_A::Enabled)
    }
}
#[doc = "IREN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREN_A {
    #[doc = "0: Interrupt rising edge detection request disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt rising edge detection request enabled"]
    Enabled = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IREN"]
pub type IREN_R = crate::BitReader<IREN_A>;
impl IREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::Disabled,
            true => IREN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN_A::Enabled
    }
}
#[doc = "Field `IREN` writer - IREN"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, IREN_A, O>;
impl<'a, const O: u8> IREN_W<'a, O> {
    #[doc = "Interrupt rising edge detection request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IREN_A::Disabled)
    }
    #[doc = "Interrupt rising edge detection request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IREN_A::Enabled)
    }
}
#[doc = "IFS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFS_A {
    #[doc = "0: Interrupt falling edge did not occur"]
    DidNotOccur = 0,
    #[doc = "1: Interrupt falling edge occurred"]
    Occurred = 1,
}
impl From<IFS_A> for bool {
    #[inline(always)]
    fn from(variant: IFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFS` reader - IFS"]
pub type IFS_R = crate::BitReader<IFS_A>;
impl IFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFS_A {
        match self.bits {
            false => IFS_A::DidNotOccur,
            true => IFS_A::Occurred,
        }
    }
    #[doc = "Checks if the value of the field is `DidNotOccur`"]
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        *self == IFS_A::DidNotOccur
    }
    #[doc = "Checks if the value of the field is `Occurred`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == IFS_A::Occurred
    }
}
#[doc = "Field `IFS` writer - IFS"]
pub type IFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, IFS_A, O>;
impl<'a, const O: u8> IFS_W<'a, O> {
    #[doc = "Interrupt falling edge did not occur"]
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut W {
        self.variant(IFS_A::DidNotOccur)
    }
    #[doc = "Interrupt falling edge occurred"]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(IFS_A::Occurred)
    }
}
#[doc = "ILS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILS_A {
    #[doc = "0: Interrupt high-level did not occur"]
    DidNotOccur = 0,
    #[doc = "1: Interrupt high-level occurred"]
    Occurred = 1,
}
impl From<ILS_A> for bool {
    #[inline(always)]
    fn from(variant: ILS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILS` reader - ILS"]
pub type ILS_R = crate::BitReader<ILS_A>;
impl ILS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILS_A {
        match self.bits {
            false => ILS_A::DidNotOccur,
            true => ILS_A::Occurred,
        }
    }
    #[doc = "Checks if the value of the field is `DidNotOccur`"]
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        *self == ILS_A::DidNotOccur
    }
    #[doc = "Checks if the value of the field is `Occurred`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == ILS_A::Occurred
    }
}
#[doc = "Field `ILS` writer - ILS"]
pub type ILS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, ILS_A, O>;
impl<'a, const O: u8> ILS_W<'a, O> {
    #[doc = "Interrupt high-level did not occur"]
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut W {
        self.variant(ILS_A::DidNotOccur)
    }
    #[doc = "Interrupt high-level occurred"]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(ILS_A::Occurred)
    }
}
#[doc = "IRS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRS_A {
    #[doc = "0: Interrupt rising edge did not occur"]
    DidNotOccur = 0,
    #[doc = "1: Interrupt rising edge occurred"]
    Occurred = 1,
}
impl From<IRS_A> for bool {
    #[inline(always)]
    fn from(variant: IRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRS` reader - IRS"]
pub type IRS_R = crate::BitReader<IRS_A>;
impl IRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRS_A {
        match self.bits {
            false => IRS_A::DidNotOccur,
            true => IRS_A::Occurred,
        }
    }
    #[doc = "Checks if the value of the field is `DidNotOccur`"]
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        *self == IRS_A::DidNotOccur
    }
    #[doc = "Checks if the value of the field is `Occurred`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == IRS_A::Occurred
    }
}
#[doc = "Field `IRS` writer - IRS"]
pub type IRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, IRS_A, O>;
impl<'a, const O: u8> IRS_W<'a, O> {
    #[doc = "Interrupt rising edge did not occur"]
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut W {
        self.variant(IRS_A::DidNotOccur)
    }
    #[doc = "Interrupt rising edge occurred"]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(IRS_A::Occurred)
    }
}
impl R {
    #[doc = "Bit 6 - FEMPT"]
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&mut self) -> IFEN_W<5> {
        IFEN_W::new(self)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&mut self) -> ILEN_W<4> {
        ILEN_W::new(self)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<3> {
        IREN_W::new(self)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&mut self) -> IFS_W<2> {
        IFS_W::new(self)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&mut self) -> ILS_W<1> {
        ILS_W::new(self)
    }
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&mut self) -> IRS_W<0> {
        IRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO status and interrupt register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
#[doc = "`reset()` method sets SR%s to value 0x40"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
