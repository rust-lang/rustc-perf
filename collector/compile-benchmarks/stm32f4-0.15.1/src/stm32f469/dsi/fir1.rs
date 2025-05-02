#[doc = "Register `FIR1` reader"]
pub struct R(crate::R<FIR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIR1` writer"]
pub struct W(crate::W<FIR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIR1_SPEC>;
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
impl From<crate::W<FIR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FGPRXE` reader - Force Generic Payload Receive Error"]
pub type FGPRXE_R = crate::BitReader<bool>;
#[doc = "Field `FGPRXE` writer - Force Generic Payload Receive Error"]
pub type FGPRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPRDE` reader - Force Generic Payload Read Error"]
pub type FGPRDE_R = crate::BitReader<bool>;
#[doc = "Field `FGPRDE` writer - Force Generic Payload Read Error"]
pub type FGPRDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPTXE` reader - Force Generic Payload Transmit Error"]
pub type FGPTXE_R = crate::BitReader<bool>;
#[doc = "Field `FGPTXE` writer - Force Generic Payload Transmit Error"]
pub type FGPTXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGPWRE` reader - Force Generic Payload Write Error"]
pub type FGPWRE_R = crate::BitReader<bool>;
#[doc = "Field `FGPWRE` writer - Force Generic Payload Write Error"]
pub type FGPWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FGCWRE` reader - Force Generic Command Write Error"]
pub type FGCWRE_R = crate::BitReader<bool>;
#[doc = "Field `FGCWRE` writer - Force Generic Command Write Error"]
pub type FGCWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FLPWRE` reader - Force LTDC Payload Write Error"]
pub type FLPWRE_R = crate::BitReader<bool>;
#[doc = "Field `FLPWRE` writer - Force LTDC Payload Write Error"]
pub type FLPWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FEOTPE` reader - Force EoTp Error"]
pub type FEOTPE_R = crate::BitReader<bool>;
#[doc = "Field `FEOTPE` writer - Force EoTp Error"]
pub type FEOTPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FPSE` reader - Force Packet Size Error"]
pub type FPSE_R = crate::BitReader<bool>;
#[doc = "Field `FPSE` writer - Force Packet Size Error"]
pub type FPSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FCRCE` reader - Force CRC Error"]
pub type FCRCE_R = crate::BitReader<bool>;
#[doc = "Field `FCRCE` writer - Force CRC Error"]
pub type FCRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FECCME` reader - Force ECC Multi-bit Error"]
pub type FECCME_R = crate::BitReader<bool>;
#[doc = "Field `FECCME` writer - Force ECC Multi-bit Error"]
pub type FECCME_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FECCSE` reader - Force ECC Single-bit Error"]
pub type FECCSE_R = crate::BitReader<bool>;
#[doc = "Field `FECCSE` writer - Force ECC Single-bit Error"]
pub type FECCSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FTOLPRX` reader - Force Timeout Low-Power Reception"]
pub type FTOLPRX_R = crate::BitReader<bool>;
#[doc = "Field `FTOLPRX` writer - Force Timeout Low-Power Reception"]
pub type FTOLPRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
#[doc = "Field `FTOHSTX` reader - Force Timeout High-Speed Transmission"]
pub type FTOHSTX_R = crate::BitReader<bool>;
#[doc = "Field `FTOHSTX` writer - Force Timeout High-Speed Transmission"]
pub type FTOHSTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - Force Generic Payload Receive Error"]
    #[inline(always)]
    pub fn fgprxe(&self) -> FGPRXE_R {
        FGPRXE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Force Generic Payload Read Error"]
    #[inline(always)]
    pub fn fgprde(&self) -> FGPRDE_R {
        FGPRDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Force Generic Payload Transmit Error"]
    #[inline(always)]
    pub fn fgptxe(&self) -> FGPTXE_R {
        FGPTXE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Force Generic Payload Write Error"]
    #[inline(always)]
    pub fn fgpwre(&self) -> FGPWRE_R {
        FGPWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Force Generic Command Write Error"]
    #[inline(always)]
    pub fn fgcwre(&self) -> FGCWRE_R {
        FGCWRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Force LTDC Payload Write Error"]
    #[inline(always)]
    pub fn flpwre(&self) -> FLPWRE_R {
        FLPWRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Force EoTp Error"]
    #[inline(always)]
    pub fn feotpe(&self) -> FEOTPE_R {
        FEOTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Force Packet Size Error"]
    #[inline(always)]
    pub fn fpse(&self) -> FPSE_R {
        FPSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Force CRC Error"]
    #[inline(always)]
    pub fn fcrce(&self) -> FCRCE_R {
        FCRCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Force ECC Multi-bit Error"]
    #[inline(always)]
    pub fn feccme(&self) -> FECCME_R {
        FECCME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Force ECC Single-bit Error"]
    #[inline(always)]
    pub fn feccse(&self) -> FECCSE_R {
        FECCSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Force Timeout Low-Power Reception"]
    #[inline(always)]
    pub fn ftolprx(&self) -> FTOLPRX_R {
        FTOLPRX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Force Timeout High-Speed Transmission"]
    #[inline(always)]
    pub fn ftohstx(&self) -> FTOHSTX_R {
        FTOHSTX_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Force Generic Payload Receive Error"]
    #[inline(always)]
    pub fn fgprxe(&mut self) -> FGPRXE_W<12> {
        FGPRXE_W::new(self)
    }
    #[doc = "Bit 11 - Force Generic Payload Read Error"]
    #[inline(always)]
    pub fn fgprde(&mut self) -> FGPRDE_W<11> {
        FGPRDE_W::new(self)
    }
    #[doc = "Bit 10 - Force Generic Payload Transmit Error"]
    #[inline(always)]
    pub fn fgptxe(&mut self) -> FGPTXE_W<10> {
        FGPTXE_W::new(self)
    }
    #[doc = "Bit 9 - Force Generic Payload Write Error"]
    #[inline(always)]
    pub fn fgpwre(&mut self) -> FGPWRE_W<9> {
        FGPWRE_W::new(self)
    }
    #[doc = "Bit 8 - Force Generic Command Write Error"]
    #[inline(always)]
    pub fn fgcwre(&mut self) -> FGCWRE_W<8> {
        FGCWRE_W::new(self)
    }
    #[doc = "Bit 7 - Force LTDC Payload Write Error"]
    #[inline(always)]
    pub fn flpwre(&mut self) -> FLPWRE_W<7> {
        FLPWRE_W::new(self)
    }
    #[doc = "Bit 6 - Force EoTp Error"]
    #[inline(always)]
    pub fn feotpe(&mut self) -> FEOTPE_W<6> {
        FEOTPE_W::new(self)
    }
    #[doc = "Bit 5 - Force Packet Size Error"]
    #[inline(always)]
    pub fn fpse(&mut self) -> FPSE_W<5> {
        FPSE_W::new(self)
    }
    #[doc = "Bit 4 - Force CRC Error"]
    #[inline(always)]
    pub fn fcrce(&mut self) -> FCRCE_W<4> {
        FCRCE_W::new(self)
    }
    #[doc = "Bit 3 - Force ECC Multi-bit Error"]
    #[inline(always)]
    pub fn feccme(&mut self) -> FECCME_W<3> {
        FECCME_W::new(self)
    }
    #[doc = "Bit 2 - Force ECC Single-bit Error"]
    #[inline(always)]
    pub fn feccse(&mut self) -> FECCSE_W<2> {
        FECCSE_W::new(self)
    }
    #[doc = "Bit 1 - Force Timeout Low-Power Reception"]
    #[inline(always)]
    pub fn ftolprx(&mut self) -> FTOLPRX_W<1> {
        FTOLPRX_W::new(self)
    }
    #[doc = "Bit 0 - Force Timeout High-Speed Transmission"]
    #[inline(always)]
    pub fn ftohstx(&mut self) -> FTOHSTX_W<0> {
        FTOHSTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Force Interrupt Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fir1](index.html) module"]
pub struct FIR1_SPEC;
impl crate::RegisterSpec for FIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fir1::R](R) reader structure"]
impl crate::Readable for FIR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fir1::W](W) writer structure"]
impl crate::Writable for FIR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIR1 to value 0"]
impl crate::Resettable for FIR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
