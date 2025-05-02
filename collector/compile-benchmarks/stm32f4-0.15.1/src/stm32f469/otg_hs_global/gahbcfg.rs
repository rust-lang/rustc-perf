#[doc = "Register `GAHBCFG` reader"]
pub struct R(crate::R<GAHBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAHBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAHBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAHBCFG` writer"]
pub struct W(crate::W<GAHBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAHBCFG_SPEC>;
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
impl From<crate::W<GAHBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAHBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GINT` reader - Global interrupt mask"]
pub type GINT_R = crate::BitReader<bool>;
#[doc = "Field `GINT` writer - Global interrupt mask"]
pub type GINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `HBSTLEN` reader - Burst length/type"]
pub type HBSTLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HBSTLEN` writer - Burst length/type"]
pub type HBSTLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAHBCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `TXFELVL` reader - TxFIFO empty level"]
pub type TXFELVL_R = crate::BitReader<bool>;
#[doc = "Field `TXFELVL` writer - TxFIFO empty level"]
pub type TXFELVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `PTXFELVL` reader - Periodic TxFIFO empty level"]
pub type PTXFELVL_R = crate::BitReader<bool>;
#[doc = "Field `PTXFELVL` writer - Periodic TxFIFO empty level"]
pub type PTXFELVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Burst length/type"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn gint(&mut self) -> GINT_W<0> {
        GINT_W::new(self)
    }
    #[doc = "Bits 1:4 - Burst length/type"]
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<1> {
        HBSTLEN_W::new(self)
    }
    #[doc = "Bit 5 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<5> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(&mut self) -> TXFELVL_W<7> {
        TXFELVL_W::new(self)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<8> {
        PTXFELVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS AHB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gahbcfg](index.html) module"]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gahbcfg::R](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
