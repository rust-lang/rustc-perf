#[doc = "Register `ESR` reader"]
pub struct R(crate::R<ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESR` writer"]
pub struct W(crate::W<ESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESR_SPEC>;
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
impl From<crate::W<ESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REC` reader - REC"]
pub type REC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEC` reader - TEC"]
pub type TEC_R = crate::FieldReader<u8, u8>;
#[doc = "LEC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEC_A {
    #[doc = "0: No Error"]
    NoError = 0,
    #[doc = "1: Stuff Error"]
    Stuff = 1,
    #[doc = "2: Form Error"]
    Form = 2,
    #[doc = "3: Acknowledgment Error"]
    Ack = 3,
    #[doc = "4: Bit recessive Error"]
    BitRecessive = 4,
    #[doc = "5: Bit dominant Error"]
    BitDominant = 5,
    #[doc = "6: CRC Error"]
    Crc = 6,
    #[doc = "7: Set by software"]
    Custom = 7,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEC` reader - LEC"]
pub type LEC_R = crate::FieldReader<u8, LEC_A>;
impl LEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::NoError,
            1 => LEC_A::Stuff,
            2 => LEC_A::Form,
            3 => LEC_A::Ack,
            4 => LEC_A::BitRecessive,
            5 => LEC_A::BitDominant,
            6 => LEC_A::Crc,
            7 => LEC_A::Custom,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LEC_A::NoError
    }
    #[doc = "Checks if the value of the field is `Stuff`"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == LEC_A::Stuff
    }
    #[doc = "Checks if the value of the field is `Form`"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == LEC_A::Form
    }
    #[doc = "Checks if the value of the field is `Ack`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LEC_A::Ack
    }
    #[doc = "Checks if the value of the field is `BitRecessive`"]
    #[inline(always)]
    pub fn is_bit_recessive(&self) -> bool {
        *self == LEC_A::BitRecessive
    }
    #[doc = "Checks if the value of the field is `BitDominant`"]
    #[inline(always)]
    pub fn is_bit_dominant(&self) -> bool {
        *self == LEC_A::BitDominant
    }
    #[doc = "Checks if the value of the field is `Crc`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == LEC_A::Crc
    }
    #[doc = "Checks if the value of the field is `Custom`"]
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == LEC_A::Custom
    }
}
#[doc = "Field `LEC` writer - LEC"]
pub type LEC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ESR_SPEC, u8, LEC_A, 3, O>;
impl<'a, const O: u8> LEC_W<'a, O> {
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(LEC_A::NoError)
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn stuff(self) -> &'a mut W {
        self.variant(LEC_A::Stuff)
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn form(self) -> &'a mut W {
        self.variant(LEC_A::Form)
    }
    #[doc = "Acknowledgment Error"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(LEC_A::Ack)
    }
    #[doc = "Bit recessive Error"]
    #[inline(always)]
    pub fn bit_recessive(self) -> &'a mut W {
        self.variant(LEC_A::BitRecessive)
    }
    #[doc = "Bit dominant Error"]
    #[inline(always)]
    pub fn bit_dominant(self) -> &'a mut W {
        self.variant(LEC_A::BitDominant)
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(LEC_A::Crc)
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn custom(self) -> &'a mut W {
        self.variant(LEC_A::Custom)
    }
}
#[doc = "Field `BOFF` reader - BOFF"]
pub type BOFF_R = crate::BitReader<bool>;
#[doc = "Field `EPVF` reader - EPVF"]
pub type EPVF_R = crate::BitReader<bool>;
#[doc = "Field `EWGF` reader - EWGF"]
pub type EWGF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 24:31 - REC"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 2 - BOFF"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - EPVF"]
    #[inline(always)]
    pub fn epvf(&self) -> EPVF_R {
        EPVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - EWGF"]
    #[inline(always)]
    pub fn ewgf(&self) -> EWGF_R {
        EWGF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W<4> {
        LEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](index.html) module"]
pub struct ESR_SPEC;
impl crate::RegisterSpec for ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr::R](R) reader structure"]
impl crate::Readable for ESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esr::W](W) writer structure"]
impl crate::Writable for ESR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESR to value 0"]
impl crate::Resettable for ESR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
