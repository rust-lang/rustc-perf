#[doc = "Register `LIFCR` writer"]
pub struct W(crate::W<LIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIFCR_SPEC>;
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
impl From<crate::W<LIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub use CDMEIF0_AW as CDMEIF3_AW;
#[doc = "Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub use CDMEIF0_AW as CDMEIF2_AW;
#[doc = "Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub use CDMEIF0_AW as CDMEIF1_AW;
#[doc = "Field `CDMEIF3` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub use CDMEIF0_W as CDMEIF3_W;
#[doc = "Field `CDMEIF2` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub use CDMEIF0_W as CDMEIF2_W;
#[doc = "Field `CDMEIF1` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub use CDMEIF0_W as CDMEIF1_W;
#[doc = "Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub use CFEIF0_AW as CFEIF3_AW;
#[doc = "Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub use CFEIF0_AW as CFEIF2_AW;
#[doc = "Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub use CFEIF0_AW as CFEIF1_AW;
#[doc = "Field `CFEIF3` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub use CFEIF0_W as CFEIF3_W;
#[doc = "Field `CFEIF2` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub use CFEIF0_W as CFEIF2_W;
#[doc = "Field `CFEIF1` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub use CFEIF0_W as CFEIF1_W;
#[doc = "Stream x clear half transfer interrupt flag (x = 3..0)"]
pub use CHTIF0_AW as CHTIF3_AW;
#[doc = "Stream x clear half transfer interrupt flag (x = 3..0)"]
pub use CHTIF0_AW as CHTIF2_AW;
#[doc = "Stream x clear half transfer interrupt flag (x = 3..0)"]
pub use CHTIF0_AW as CHTIF1_AW;
#[doc = "Field `CHTIF3` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub use CHTIF0_W as CHTIF3_W;
#[doc = "Field `CHTIF2` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub use CHTIF0_W as CHTIF2_W;
#[doc = "Field `CHTIF1` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub use CHTIF0_W as CHTIF1_W;
#[doc = "Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub use CTCIF0_AW as CTCIF3_AW;
#[doc = "Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub use CTCIF0_AW as CTCIF2_AW;
#[doc = "Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub use CTCIF0_AW as CTCIF1_AW;
#[doc = "Field `CTCIF3` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub use CTCIF0_W as CTCIF3_W;
#[doc = "Field `CTCIF2` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub use CTCIF0_W as CTCIF2_W;
#[doc = "Field `CTCIF1` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub use CTCIF0_W as CTCIF1_W;
#[doc = "Stream x clear transfer error interrupt flag (x = 3..0)"]
pub use CTEIF0_AW as CTEIF3_AW;
#[doc = "Stream x clear transfer error interrupt flag (x = 3..0)"]
pub use CTEIF0_AW as CTEIF2_AW;
#[doc = "Stream x clear transfer error interrupt flag (x = 3..0)"]
pub use CTEIF0_AW as CTEIF1_AW;
#[doc = "Field `CTEIF3` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub use CTEIF0_W as CTEIF3_W;
#[doc = "Field `CTEIF2` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub use CTEIF0_W as CTEIF2_W;
#[doc = "Field `CTEIF1` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub use CTEIF0_W as CTEIF1_W;
#[doc = "Stream x clear transfer complete interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF0_AW {
    #[doc = "1: Clear the corresponding TCIFx flag"]
    Clear = 1,
}
impl From<CTCIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CTCIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIF0` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIFCR_SPEC, CTCIF0_AW, O>;
impl<'a, const O: u8> CTCIF0_W<'a, O> {
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF0_AW::Clear)
    }
}
#[doc = "Stream x clear half transfer interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHTIF0_AW {
    #[doc = "1: Clear the corresponding HTIFx flag"]
    Clear = 1,
}
impl From<CHTIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CHTIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHTIF0` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIFCR_SPEC, CHTIF0_AW, O>;
impl<'a, const O: u8> CHTIF0_W<'a, O> {
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF0_AW::Clear)
    }
}
#[doc = "Stream x clear transfer error interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEIF0_AW {
    #[doc = "1: Clear the corresponding TEIFx flag"]
    Clear = 1,
}
impl From<CTEIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEIF0` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIFCR_SPEC, CTEIF0_AW, O>;
impl<'a, const O: u8> CTEIF0_W<'a, O> {
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF0_AW::Clear)
    }
}
#[doc = "Stream x clear direct mode error interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDMEIF0_AW {
    #[doc = "1: Clear the corresponding DMEIFx flag"]
    Clear = 1,
}
impl From<CDMEIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CDMEIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDMEIF0` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIFCR_SPEC, CDMEIF0_AW, O>;
impl<'a, const O: u8> CDMEIF0_W<'a, O> {
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF0_AW::Clear)
    }
}
#[doc = "Stream x clear FIFO error interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFEIF0_AW {
    #[doc = "1: Clear the corresponding CFEIFx flag"]
    Clear = 1,
}
impl From<CFEIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CFEIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFEIF0` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIFCR_SPEC, CFEIF0_AW, O>;
impl<'a, const O: u8> CFEIF0_W<'a, O> {
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF0_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W<27> {
        CTCIF3_W::new(self)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W<26> {
        CHTIF3_W::new(self)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W<25> {
        CTEIF3_W::new(self)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W<24> {
        CDMEIF3_W::new(self)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif3(&mut self) -> CFEIF3_W<22> {
        CFEIF3_W::new(self)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W<21> {
        CTCIF2_W::new(self)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W<20> {
        CHTIF2_W::new(self)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W<19> {
        CTEIF2_W::new(self)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W<18> {
        CDMEIF2_W::new(self)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif2(&mut self) -> CFEIF2_W<16> {
        CFEIF2_W::new(self)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W<11> {
        CTCIF1_W::new(self)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W<10> {
        CHTIF1_W::new(self)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W<9> {
        CTEIF1_W::new(self)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W<8> {
        CDMEIF1_W::new(self)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif1(&mut self) -> CFEIF1_W<6> {
        CFEIF1_W::new(self)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF0_W<5> {
        CTCIF0_W::new(self)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF0_W<4> {
        CHTIF0_W::new(self)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W<3> {
        CTEIF0_W::new(self)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W<2> {
        CDMEIF0_W::new(self)
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif0(&mut self) -> CFEIF0_W<0> {
        CFEIF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "low interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lifcr](index.html) module"]
pub struct LIFCR_SPEC;
impl crate::RegisterSpec for LIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lifcr::W](W) writer structure"]
impl crate::Writable for LIFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIFCR to value 0"]
impl crate::Resettable for LIFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
