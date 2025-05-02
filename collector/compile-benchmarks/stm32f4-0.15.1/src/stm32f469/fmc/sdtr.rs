#[doc = "Register `SDTR%s` reader"]
pub struct R(crate::R<SDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDTR%s` writer"]
pub struct W(crate::W<SDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDTR_SPEC>;
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
impl From<crate::W<SDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMRD` reader - Load Mode Register to Active"]
pub type TMRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMRD` writer - Load Mode Register to Active"]
pub type TMRD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXSR` reader - Exit self-refresh delay"]
pub type TXSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXSR` writer - Exit self-refresh delay"]
pub type TXSR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRAS` reader - Self refresh time"]
pub type TRAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRAS` writer - Self refresh time"]
pub type TRAS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRC` reader - Row cycle delay"]
pub type TRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRC` writer - Row cycle delay"]
pub type TRC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TWR` reader - Recovery delay"]
pub type TWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWR` writer - Recovery delay"]
pub type TWR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRP` reader - Row precharge delay"]
pub type TRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRP` writer - Row precharge delay"]
pub type TRP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRCD` reader - Row to column delay"]
pub type TRCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRCD` writer - Row to column delay"]
pub type TRCD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDTR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Load Mode Register to Active"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Exit self-refresh delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Self refresh time"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row cycle delay"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Recovery delay"]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Row precharge delay"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Row to column delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load Mode Register to Active"]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W<0> {
        TMRD_W::new(self)
    }
    #[doc = "Bits 4:7 - Exit self-refresh delay"]
    #[inline(always)]
    pub fn txsr(&mut self) -> TXSR_W<4> {
        TXSR_W::new(self)
    }
    #[doc = "Bits 8:11 - Self refresh time"]
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W<8> {
        TRAS_W::new(self)
    }
    #[doc = "Bits 12:15 - Row cycle delay"]
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W<12> {
        TRC_W::new(self)
    }
    #[doc = "Bits 16:19 - Recovery delay"]
    #[inline(always)]
    pub fn twr(&mut self) -> TWR_W<16> {
        TWR_W::new(self)
    }
    #[doc = "Bits 20:23 - Row precharge delay"]
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W<20> {
        TRP_W::new(self)
    }
    #[doc = "Bits 24:27 - Row to column delay"]
    #[inline(always)]
    pub fn trcd(&mut self) -> TRCD_W<24> {
        TRCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdtr](index.html) module"]
pub struct SDTR_SPEC;
impl crate::RegisterSpec for SDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdtr::R](R) reader structure"]
impl crate::Readable for SDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdtr::W](W) writer structure"]
impl crate::Writable for SDTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDTR%s to value 0x0fff_ffff"]
impl crate::Resettable for SDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_ffff
    }
}
