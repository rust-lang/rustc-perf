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
#[doc = "Field `INIT` writer - Initialize message digest calculation"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAE` reader - DMA enable"]
pub type DMAE_R = crate::BitReader<bool>;
#[doc = "Field `DMAE` writer - DMA enable"]
pub type DMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DATATYPE` reader - Data type selection"]
pub type DATATYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATYPE` writer - Data type selection"]
pub type DATATYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE` reader - Mode selection"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Mode selection"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ALGO0` reader - Algorithm selection"]
pub type ALGO0_R = crate::BitReader<bool>;
#[doc = "Field `ALGO0` writer - Algorithm selection"]
pub type ALGO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `NBW` reader - Number of words already pushed"]
pub type NBW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DINNE` reader - DIN not empty"]
pub type DINNE_R = crate::BitReader<bool>;
#[doc = "Field `MDMAT` reader - Multiple DMA Transfers"]
pub type MDMAT_R = crate::BitReader<bool>;
#[doc = "Field `MDMAT` writer - Multiple DMA Transfers"]
pub type MDMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LKEY` reader - Long key selection"]
pub type LKEY_R = crate::BitReader<bool>;
#[doc = "Field `LKEY` writer - Long key selection"]
pub type LKEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ALGO1` reader - ALGO"]
pub type ALGO1_R = crate::BitReader<bool>;
#[doc = "Field `ALGO1` writer - ALGO"]
pub type ALGO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Mode selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Algorithm selection"]
    #[inline(always)]
    pub fn algo0(&self) -> ALGO0_R {
        ALGO0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of words already pushed"]
    #[inline(always)]
    pub fn nbw(&self) -> NBW_R {
        NBW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DIN not empty"]
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Multiple DMA Transfers"]
    #[inline(always)]
    pub fn mdmat(&self) -> MDMAT_R {
        MDMAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Long key selection"]
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - ALGO"]
    #[inline(always)]
    pub fn algo1(&self) -> ALGO1_R {
        ALGO1_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Initialize message digest calculation"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<2> {
        INIT_W::new(self)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W<3> {
        DMAE_W::new(self)
    }
    #[doc = "Bits 4:5 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<4> {
        DATATYPE_W::new(self)
    }
    #[doc = "Bit 6 - Mode selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<6> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - Algorithm selection"]
    #[inline(always)]
    pub fn algo0(&mut self) -> ALGO0_W<7> {
        ALGO0_W::new(self)
    }
    #[doc = "Bit 13 - Multiple DMA Transfers"]
    #[inline(always)]
    pub fn mdmat(&mut self) -> MDMAT_W<13> {
        MDMAT_W::new(self)
    }
    #[doc = "Bit 16 - Long key selection"]
    #[inline(always)]
    pub fn lkey(&mut self) -> LKEY_W<16> {
        LKEY_W::new(self)
    }
    #[doc = "Bit 18 - ALGO"]
    #[inline(always)]
    pub fn algo1(&mut self) -> ALGO1_W<18> {
        ALGO1_W::new(self)
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
