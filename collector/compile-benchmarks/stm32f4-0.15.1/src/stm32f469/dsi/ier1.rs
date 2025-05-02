#[doc = "Register `IER1` reader"]
pub struct R(crate::R<IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER1` writer"]
pub struct W(crate::W<IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER1_SPEC>;
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
impl From<crate::W<IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPRXEIE` reader - Generic Payload Receive Error Interrupt Enable"]
pub type GPRXEIE_R = crate::BitReader<bool>;
#[doc = "Field `GPRXEIE` writer - Generic Payload Receive Error Interrupt Enable"]
pub type GPRXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `GPRDEIE` reader - Generic Payload Read Error Interrupt Enable"]
pub type GPRDEIE_R = crate::BitReader<bool>;
#[doc = "Field `GPRDEIE` writer - Generic Payload Read Error Interrupt Enable"]
pub type GPRDEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `GPTXEIE` reader - Generic Payload Transmit Error Interrupt Enable"]
pub type GPTXEIE_R = crate::BitReader<bool>;
#[doc = "Field `GPTXEIE` writer - Generic Payload Transmit Error Interrupt Enable"]
pub type GPTXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `GPWREIE` reader - Generic Payload Write Error Interrupt Enable"]
pub type GPWREIE_R = crate::BitReader<bool>;
#[doc = "Field `GPWREIE` writer - Generic Payload Write Error Interrupt Enable"]
pub type GPWREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `GCWREIE` reader - Generic Command Write Error Interrupt Enable"]
pub type GCWREIE_R = crate::BitReader<bool>;
#[doc = "Field `GCWREIE` writer - Generic Command Write Error Interrupt Enable"]
pub type GCWREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `LPWREIE` reader - LTDC Payload Write Error Interrupt Enable"]
pub type LPWREIE_R = crate::BitReader<bool>;
#[doc = "Field `LPWREIE` writer - LTDC Payload Write Error Interrupt Enable"]
pub type LPWREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `EOTPEIE` reader - EoTp Error Interrupt Enable"]
pub type EOTPEIE_R = crate::BitReader<bool>;
#[doc = "Field `EOTPEIE` writer - EoTp Error Interrupt Enable"]
pub type EOTPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `PSEIE` reader - Packet Size Error Interrupt Enable"]
pub type PSEIE_R = crate::BitReader<bool>;
#[doc = "Field `PSEIE` writer - Packet Size Error Interrupt Enable"]
pub type PSEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `CRCEIE` reader - CRC Error Interrupt Enable"]
pub type CRCEIE_R = crate::BitReader<bool>;
#[doc = "Field `CRCEIE` writer - CRC Error Interrupt Enable"]
pub type CRCEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `ECCMEIE` reader - ECC Multi-bit Error Interrupt Enable"]
pub type ECCMEIE_R = crate::BitReader<bool>;
#[doc = "Field `ECCMEIE` writer - ECC Multi-bit Error Interrupt Enable"]
pub type ECCMEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `ECCSEIE` reader - ECC Single-bit Error Interrupt Enable"]
pub type ECCSEIE_R = crate::BitReader<bool>;
#[doc = "Field `ECCSEIE` writer - ECC Single-bit Error Interrupt Enable"]
pub type ECCSEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `TOLPRXIE` reader - Timeout Low-Power Reception Interrupt Enable"]
pub type TOLPRXIE_R = crate::BitReader<bool>;
#[doc = "Field `TOLPRXIE` writer - Timeout Low-Power Reception Interrupt Enable"]
pub type TOLPRXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `TOHSTXIE` reader - Timeout High-Speed Transmission Interrupt Enable"]
pub type TOHSTXIE_R = crate::BitReader<bool>;
#[doc = "Field `TOHSTXIE` writer - Timeout High-Speed Transmission Interrupt Enable"]
pub type TOHSTXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - Generic Payload Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprxeie(&self) -> GPRXEIE_R {
        GPRXEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic Payload Read Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprdeie(&self) -> GPRDEIE_R {
        GPRDEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic Payload Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub fn gptxeie(&self) -> GPTXEIE_R {
        GPTXEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gpwreie(&self) -> GPWREIE_R {
        GPWREIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic Command Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gcwreie(&self) -> GCWREIE_R {
        GCWREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - LTDC Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn lpwreie(&self) -> LPWREIE_R {
        LPWREIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - EoTp Error Interrupt Enable"]
    #[inline(always)]
    pub fn eotpeie(&self) -> EOTPEIE_R {
        EOTPEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Packet Size Error Interrupt Enable"]
    #[inline(always)]
    pub fn pseie(&self) -> PSEIE_R {
        PSEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC Multi-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccmeie(&self) -> ECCMEIE_R {
        ECCMEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC Single-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Timeout Low-Power Reception Interrupt Enable"]
    #[inline(always)]
    pub fn tolprxie(&self) -> TOLPRXIE_R {
        TOLPRXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Timeout High-Speed Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tohstxie(&self) -> TOHSTXIE_R {
        TOHSTXIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Generic Payload Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprxeie(&mut self) -> GPRXEIE_W<12> {
        GPRXEIE_W::new(self)
    }
    #[doc = "Bit 11 - Generic Payload Read Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprdeie(&mut self) -> GPRDEIE_W<11> {
        GPRDEIE_W::new(self)
    }
    #[doc = "Bit 10 - Generic Payload Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub fn gptxeie(&mut self) -> GPTXEIE_W<10> {
        GPTXEIE_W::new(self)
    }
    #[doc = "Bit 9 - Generic Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gpwreie(&mut self) -> GPWREIE_W<9> {
        GPWREIE_W::new(self)
    }
    #[doc = "Bit 8 - Generic Command Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gcwreie(&mut self) -> GCWREIE_W<8> {
        GCWREIE_W::new(self)
    }
    #[doc = "Bit 7 - LTDC Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn lpwreie(&mut self) -> LPWREIE_W<7> {
        LPWREIE_W::new(self)
    }
    #[doc = "Bit 6 - EoTp Error Interrupt Enable"]
    #[inline(always)]
    pub fn eotpeie(&mut self) -> EOTPEIE_W<6> {
        EOTPEIE_W::new(self)
    }
    #[doc = "Bit 5 - Packet Size Error Interrupt Enable"]
    #[inline(always)]
    pub fn pseie(&mut self) -> PSEIE_W<5> {
        PSEIE_W::new(self)
    }
    #[doc = "Bit 4 - CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crceie(&mut self) -> CRCEIE_W<4> {
        CRCEIE_W::new(self)
    }
    #[doc = "Bit 3 - ECC Multi-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccmeie(&mut self) -> ECCMEIE_W<3> {
        ECCMEIE_W::new(self)
    }
    #[doc = "Bit 2 - ECC Single-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccseie(&mut self) -> ECCSEIE_W<2> {
        ECCSEIE_W::new(self)
    }
    #[doc = "Bit 1 - Timeout Low-Power Reception Interrupt Enable"]
    #[inline(always)]
    pub fn tolprxie(&mut self) -> TOLPRXIE_W<1> {
        TOLPRXIE_W::new(self)
    }
    #[doc = "Bit 0 - Timeout High-Speed Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tohstxie(&mut self) -> TOHSTXIE_W<0> {
        TOHSTXIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Interrupt Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](index.html) module"]
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier1::R](R) reader structure"]
impl crate::Readable for IER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier1::W](W) writer structure"]
impl crate::Writable for IER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER1 to value 0"]
impl crate::Resettable for IER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
