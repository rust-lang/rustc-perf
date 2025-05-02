#[doc = "Register `TIR` reader"]
pub struct R(crate::R<TIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIR` writer"]
pub struct W(crate::W<TIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIR_SPEC>;
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
impl From<crate::W<TIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STID` reader - STID"]
pub type STID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STID` writer - STID"]
pub type STID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIR_SPEC, u16, u16, 11, O>;
#[doc = "Field `EXID` reader - EXID"]
pub type EXID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EXID` writer - EXID"]
pub type EXID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIR_SPEC, u32, u32, 18, O>;
#[doc = "IDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_A {
    #[doc = "0: Standard identifier"]
    Standard = 0,
    #[doc = "1: Extended identifier"]
    Extended = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader<IDE_A>;
impl IDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::Standard,
            true => IDE_A::Extended,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDE_A::Standard
    }
    #[doc = "Checks if the value of the field is `Extended`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDE_A::Extended
    }
}
#[doc = "Field `IDE` writer - IDE"]
pub type IDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIR_SPEC, IDE_A, O>;
impl<'a, const O: u8> IDE_W<'a, O> {
    #[doc = "Standard identifier"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(IDE_A::Standard)
    }
    #[doc = "Extended identifier"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(IDE_A::Extended)
    }
}
#[doc = "RTR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_A {
    #[doc = "0: Data frame"]
    Data = 0,
    #[doc = "1: Remote frame"]
    Remote = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTR` reader - RTR"]
pub type RTR_R = crate::BitReader<RTR_A>;
impl RTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::Data,
            true => RTR_A::Remote,
        }
    }
    #[doc = "Checks if the value of the field is `Data`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RTR_A::Data
    }
    #[doc = "Checks if the value of the field is `Remote`"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == RTR_A::Remote
    }
}
#[doc = "Field `RTR` writer - RTR"]
pub type RTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIR_SPEC, RTR_A, O>;
impl<'a, const O: u8> RTR_W<'a, O> {
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(RTR_A::Data)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn remote(self) -> &'a mut W {
        self.variant(RTR_A::Remote)
    }
}
#[doc = "Field `TXRQ` reader - TXRQ"]
pub type TXRQ_R = crate::BitReader<bool>;
#[doc = "Field `TXRQ` writer - TXRQ"]
pub type TXRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&mut self) -> STID_W<21> {
        STID_W::new(self)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&mut self) -> EXID_W<3> {
        EXID_W::new(self)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W<2> {
        IDE_W::new(self)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W<1> {
        RTR_W::new(self)
    }
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&mut self) -> TXRQ_W<0> {
        TXRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tir](index.html) module"]
pub struct TIR_SPEC;
impl crate::RegisterSpec for TIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tir::R](R) reader structure"]
impl crate::Readable for TIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tir::W](W) writer structure"]
impl crate::Writable for TIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIR to value 0"]
impl crate::Resettable for TIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
