#[doc = "Register `MACFCR` reader"]
pub struct R(crate::R<MACFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACFCR` writer"]
pub struct W(crate::W<MACFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFCR_SPEC>;
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
impl From<crate::W<MACFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCB` reader - FCB"]
pub type FCB_R = crate::BitReader<bool>;
#[doc = "Field `FCB` writer - FCB"]
pub type FCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, bool, O>;
#[doc = "Field `TFCE` reader - TFCE"]
pub type TFCE_R = crate::BitReader<bool>;
#[doc = "Field `TFCE` writer - TFCE"]
pub type TFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, bool, O>;
#[doc = "Field `RFCE` reader - RFCE"]
pub type RFCE_R = crate::BitReader<bool>;
#[doc = "Field `RFCE` writer - RFCE"]
pub type RFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, bool, O>;
#[doc = "Field `UPFD` reader - UPFD"]
pub type UPFD_R = crate::BitReader<bool>;
#[doc = "Field `UPFD` writer - UPFD"]
pub type UPFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, bool, O>;
#[doc = "Field `PLT` reader - PLT"]
pub type PLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLT` writer - PLT"]
pub type PLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACFCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ZQPD` reader - ZQPD"]
pub type ZQPD_R = crate::BitReader<bool>;
#[doc = "Field `ZQPD` writer - ZQPD"]
pub type ZQPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, bool, O>;
#[doc = "Field `PT` reader - PT"]
pub type PT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PT` writer - PT"]
pub type PT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACFCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - FCB"]
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFCE"]
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RFCE"]
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UPFD"]
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PLT"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - ZQPD"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FCB"]
    #[inline(always)]
    pub fn fcb(&mut self) -> FCB_W<0> {
        FCB_W::new(self)
    }
    #[doc = "Bit 1 - TFCE"]
    #[inline(always)]
    pub fn tfce(&mut self) -> TFCE_W<1> {
        TFCE_W::new(self)
    }
    #[doc = "Bit 2 - RFCE"]
    #[inline(always)]
    pub fn rfce(&mut self) -> RFCE_W<2> {
        RFCE_W::new(self)
    }
    #[doc = "Bit 3 - UPFD"]
    #[inline(always)]
    pub fn upfd(&mut self) -> UPFD_W<3> {
        UPFD_W::new(self)
    }
    #[doc = "Bits 4:5 - PLT"]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 7 - ZQPD"]
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZQPD_W<7> {
        ZQPD_W::new(self)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<16> {
        PT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macfcr](index.html) module"]
pub struct MACFCR_SPEC;
impl crate::RegisterSpec for MACFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macfcr::R](R) reader structure"]
impl crate::Readable for MACFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macfcr::W](W) writer structure"]
impl crate::Writable for MACFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACFCR to value 0"]
impl crate::Resettable for MACFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
