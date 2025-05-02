#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CEATAEND flag clear bit"]
pub use CCRCFAILC_A as CEATAENDC_A;
#[doc = "SDIOIT flag clear bit"]
pub use CCRCFAILC_A as SDIOITC_A;
#[doc = "DBCKEND flag clear bit"]
pub use CCRCFAILC_A as DBCKENDC_A;
#[doc = "STBITERR flag clear bit"]
pub use CCRCFAILC_A as STBITERRC_A;
#[doc = "DATAEND flag clear bit"]
pub use CCRCFAILC_A as DATAENDC_A;
#[doc = "CMDSENT flag clear bit"]
pub use CCRCFAILC_A as CMDSENTC_A;
#[doc = "CMDREND flag clear bit"]
pub use CCRCFAILC_A as CMDRENDC_A;
#[doc = "RXOVERR flag clear bit"]
pub use CCRCFAILC_A as RXOVERRC_A;
#[doc = "TXUNDERR flag clear bit"]
pub use CCRCFAILC_A as TXUNDERRC_A;
#[doc = "DTIMEOUT flag clear bit"]
pub use CCRCFAILC_A as DTIMEOUTC_A;
#[doc = "CTIMEOUT flag clear bit"]
pub use CCRCFAILC_A as CTIMEOUTC_A;
#[doc = "DCRCFAIL flag clear bit"]
pub use CCRCFAILC_A as DCRCFAILC_A;
#[doc = "Field `CEATAENDC` reader - CEATAEND flag clear bit"]
pub use CCRCFAILC_R as CEATAENDC_R;
#[doc = "Field `SDIOITC` reader - SDIOIT flag clear bit"]
pub use CCRCFAILC_R as SDIOITC_R;
#[doc = "Field `DBCKENDC` reader - DBCKEND flag clear bit"]
pub use CCRCFAILC_R as DBCKENDC_R;
#[doc = "Field `STBITERRC` reader - STBITERR flag clear bit"]
pub use CCRCFAILC_R as STBITERRC_R;
#[doc = "Field `DATAENDC` reader - DATAEND flag clear bit"]
pub use CCRCFAILC_R as DATAENDC_R;
#[doc = "Field `CMDSENTC` reader - CMDSENT flag clear bit"]
pub use CCRCFAILC_R as CMDSENTC_R;
#[doc = "Field `CMDRENDC` reader - CMDREND flag clear bit"]
pub use CCRCFAILC_R as CMDRENDC_R;
#[doc = "Field `RXOVERRC` reader - RXOVERR flag clear bit"]
pub use CCRCFAILC_R as RXOVERRC_R;
#[doc = "Field `TXUNDERRC` reader - TXUNDERR flag clear bit"]
pub use CCRCFAILC_R as TXUNDERRC_R;
#[doc = "Field `DTIMEOUTC` reader - DTIMEOUT flag clear bit"]
pub use CCRCFAILC_R as DTIMEOUTC_R;
#[doc = "Field `CTIMEOUTC` reader - CTIMEOUT flag clear bit"]
pub use CCRCFAILC_R as CTIMEOUTC_R;
#[doc = "Field `DCRCFAILC` reader - DCRCFAIL flag clear bit"]
pub use CCRCFAILC_R as DCRCFAILC_R;
#[doc = "Field `CEATAENDC` writer - CEATAEND flag clear bit"]
pub use CCRCFAILC_W as CEATAENDC_W;
#[doc = "Field `SDIOITC` writer - SDIOIT flag clear bit"]
pub use CCRCFAILC_W as SDIOITC_W;
#[doc = "Field `DBCKENDC` writer - DBCKEND flag clear bit"]
pub use CCRCFAILC_W as DBCKENDC_W;
#[doc = "Field `STBITERRC` writer - STBITERR flag clear bit"]
pub use CCRCFAILC_W as STBITERRC_W;
#[doc = "Field `DATAENDC` writer - DATAEND flag clear bit"]
pub use CCRCFAILC_W as DATAENDC_W;
#[doc = "Field `CMDSENTC` writer - CMDSENT flag clear bit"]
pub use CCRCFAILC_W as CMDSENTC_W;
#[doc = "Field `CMDRENDC` writer - CMDREND flag clear bit"]
pub use CCRCFAILC_W as CMDRENDC_W;
#[doc = "Field `RXOVERRC` writer - RXOVERR flag clear bit"]
pub use CCRCFAILC_W as RXOVERRC_W;
#[doc = "Field `TXUNDERRC` writer - TXUNDERR flag clear bit"]
pub use CCRCFAILC_W as TXUNDERRC_W;
#[doc = "Field `DTIMEOUTC` writer - DTIMEOUT flag clear bit"]
pub use CCRCFAILC_W as DTIMEOUTC_W;
#[doc = "Field `CTIMEOUTC` writer - CTIMEOUT flag clear bit"]
pub use CCRCFAILC_W as CTIMEOUTC_W;
#[doc = "Field `DCRCFAILC` writer - DCRCFAIL flag clear bit"]
pub use CCRCFAILC_W as DCRCFAILC_W;
#[doc = "CCRCFAIL flag clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCRCFAILC_A {
    #[doc = "1: Clear flag"]
    Clear = 1,
}
impl From<CCRCFAILC_A> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAILC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRCFAILC` reader - CCRCFAIL flag clear bit"]
pub type CCRCFAILC_R = crate::BitReader<CCRCFAILC_A>;
impl CCRCFAILC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCRCFAILC_A> {
        match self.bits {
            true => Some(CCRCFAILC_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCRCFAILC_A::Clear
    }
}
#[doc = "Field `CCRCFAILC` writer - CCRCFAIL flag clear bit"]
pub type CCRCFAILC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CCRCFAILC_A, O>;
impl<'a, const O: u8> CCRCFAILC_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::Clear)
    }
}
impl R {
    #[doc = "Bit 23 - CEATAEND flag clear bit"]
    #[inline(always)]
    pub fn ceataendc(&self) -> CEATAENDC_R {
        CEATAENDC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOIT flag clear bit"]
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKEND flag clear bit"]
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERR flag clear bit"]
    #[inline(always)]
    pub fn stbiterrc(&self) -> STBITERRC_R {
        STBITERRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAEND flag clear bit"]
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENT flag clear bit"]
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDREND flag clear bit"]
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERR flag clear bit"]
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERR flag clear bit"]
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAIL flag clear bit"]
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - CCRCFAIL flag clear bit"]
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - CEATAEND flag clear bit"]
    #[inline(always)]
    pub fn ceataendc(&mut self) -> CEATAENDC_W<23> {
        CEATAENDC_W::new(self)
    }
    #[doc = "Bit 22 - SDIOIT flag clear bit"]
    #[inline(always)]
    pub fn sdioitc(&mut self) -> SDIOITC_W<22> {
        SDIOITC_W::new(self)
    }
    #[doc = "Bit 10 - DBCKEND flag clear bit"]
    #[inline(always)]
    pub fn dbckendc(&mut self) -> DBCKENDC_W<10> {
        DBCKENDC_W::new(self)
    }
    #[doc = "Bit 9 - STBITERR flag clear bit"]
    #[inline(always)]
    pub fn stbiterrc(&mut self) -> STBITERRC_W<9> {
        STBITERRC_W::new(self)
    }
    #[doc = "Bit 8 - DATAEND flag clear bit"]
    #[inline(always)]
    pub fn dataendc(&mut self) -> DATAENDC_W<8> {
        DATAENDC_W::new(self)
    }
    #[doc = "Bit 7 - CMDSENT flag clear bit"]
    #[inline(always)]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W<7> {
        CMDSENTC_W::new(self)
    }
    #[doc = "Bit 6 - CMDREND flag clear bit"]
    #[inline(always)]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W<6> {
        CMDRENDC_W::new(self)
    }
    #[doc = "Bit 5 - RXOVERR flag clear bit"]
    #[inline(always)]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W<5> {
        RXOVERRC_W::new(self)
    }
    #[doc = "Bit 4 - TXUNDERR flag clear bit"]
    #[inline(always)]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W<4> {
        TXUNDERRC_W::new(self)
    }
    #[doc = "Bit 3 - DTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W<3> {
        DTIMEOUTC_W::new(self)
    }
    #[doc = "Bit 2 - CTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W<2> {
        CTIMEOUTC_W::new(self)
    }
    #[doc = "Bit 1 - DCRCFAIL flag clear bit"]
    #[inline(always)]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W<1> {
        DCRCFAILC_W::new(self)
    }
    #[doc = "Bit 0 - CCRCFAIL flag clear bit"]
    #[inline(always)]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W<0> {
        CCRCFAILC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
