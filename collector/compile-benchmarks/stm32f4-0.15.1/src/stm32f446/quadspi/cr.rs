#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - Clock prescaler"]
pub type PRESCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESCALER` writer - Clock prescaler"]
pub type PRESCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PMM` reader - Polling match mode"]
pub type PMM_R = crate::BitReader<bool>;
#[doc = "Field `PMM` writer - Polling match mode"]
pub type PMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `APMS` reader - Automatic poll mode stop"]
pub type APMS_R = crate::BitReader<bool>;
#[doc = "Field `APMS` writer - Automatic poll mode stop"]
pub type APMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TOIE` reader - TimeOut interrupt enable"]
pub type TOIE_R = crate::BitReader<bool>;
#[doc = "Field `TOIE` writer - TimeOut interrupt enable"]
pub type TOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SMIE` reader - Status match interrupt enable"]
pub type SMIE_R = crate::BitReader<bool>;
#[doc = "Field `SMIE` writer - Status match interrupt enable"]
pub type SMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FTIE` reader - FIFO threshold interrupt enable"]
pub type FTIE_R = crate::BitReader<bool>;
#[doc = "Field `FTIE` writer - FIFO threshold interrupt enable"]
pub type FTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader<bool>;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FTHRES` reader - IFO threshold level"]
pub type FTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTHRES` writer - IFO threshold level"]
pub type FTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 5, O>;
#[doc = "Field `FSEL` reader - FLASH memory selection"]
pub type FSEL_R = crate::BitReader<bool>;
#[doc = "Field `FSEL` writer - FLASH memory selection"]
pub type FSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DFM` reader - Dual-flash mode"]
pub type DFM_R = crate::BitReader<bool>;
#[doc = "Field `DFM` writer - Dual-flash mode"]
pub type DFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SSHIFT` reader - Sample shift"]
pub type SSHIFT_R = crate::BitReader<bool>;
#[doc = "Field `SSHIFT` writer - Sample shift"]
pub type SSHIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TCEN` reader - Timeout counter enable"]
pub type TCEN_R = crate::BitReader<bool>;
#[doc = "Field `TCEN` writer - Timeout counter enable"]
pub type TCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ABORT` reader - Abort request"]
pub type ABORT_R = crate::BitReader<bool>;
#[doc = "Field `ABORT` writer - Abort request"]
pub type ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 24:31 - Clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23 - Polling match mode"]
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - Automatic poll mode stop"]
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 20 - TimeOut interrupt enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Status match interrupt enable"]
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:12 - IFO threshold level"]
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - FLASH memory selection"]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Dual-flash mode"]
    #[inline(always)]
    pub fn dfm(&self) -> DFM_R {
        DFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Sample shift"]
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout counter enable"]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Abort request"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - Clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<24> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bit 23 - Polling match mode"]
    #[inline(always)]
    pub fn pmm(&mut self) -> PMM_W<23> {
        PMM_W::new(self)
    }
    #[doc = "Bit 22 - Automatic poll mode stop"]
    #[inline(always)]
    pub fn apms(&mut self) -> APMS_W<22> {
        APMS_W::new(self)
    }
    #[doc = "Bit 20 - TimeOut interrupt enable"]
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W<20> {
        TOIE_W::new(self)
    }
    #[doc = "Bit 19 - Status match interrupt enable"]
    #[inline(always)]
    pub fn smie(&mut self) -> SMIE_W<19> {
        SMIE_W::new(self)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn ftie(&mut self) -> FTIE_W<18> {
        FTIE_W::new(self)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<17> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<16> {
        TEIE_W::new(self)
    }
    #[doc = "Bits 8:12 - IFO threshold level"]
    #[inline(always)]
    pub fn fthres(&mut self) -> FTHRES_W<8> {
        FTHRES_W::new(self)
    }
    #[doc = "Bit 7 - FLASH memory selection"]
    #[inline(always)]
    pub fn fsel(&mut self) -> FSEL_W<7> {
        FSEL_W::new(self)
    }
    #[doc = "Bit 6 - Dual-flash mode"]
    #[inline(always)]
    pub fn dfm(&mut self) -> DFM_W<6> {
        DFM_W::new(self)
    }
    #[doc = "Bit 4 - Sample shift"]
    #[inline(always)]
    pub fn sshift(&mut self) -> SSHIFT_W<4> {
        SSHIFT_W::new(self)
    }
    #[doc = "Bit 3 - Timeout counter enable"]
    #[inline(always)]
    pub fn tcen(&mut self) -> TCEN_W<3> {
        TCEN_W::new(self)
    }
    #[doc = "Bit 2 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<2> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Abort request"]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W<1> {
        ABORT_W::new(self)
    }
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
