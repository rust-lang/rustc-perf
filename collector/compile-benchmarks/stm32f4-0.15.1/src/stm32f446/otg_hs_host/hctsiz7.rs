#[doc = "Register `HCTSIZ7` reader"]
pub struct R(crate::R<HCTSIZ7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCTSIZ7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCTSIZ7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCTSIZ7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCTSIZ7` writer"]
pub struct W(crate::W<HCTSIZ7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCTSIZ7_SPEC>;
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
impl From<crate::W<HCTSIZ7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCTSIZ7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XFRSIZ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XFRSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCTSIZ7_SPEC, u32, u32, 19, O>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCTSIZ7_SPEC, u16, u16, 10, O>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPID` writer - Data PID"]
pub type DPID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCTSIZ7_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<0> {
        XFRSIZ_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<19> {
        PKTCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&mut self) -> DPID_W<29> {
        DPID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS host channel-7 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz7](index.html) module"]
pub struct HCTSIZ7_SPEC;
impl crate::RegisterSpec for HCTSIZ7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hctsiz7::R](R) reader structure"]
impl crate::Readable for HCTSIZ7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hctsiz7::W](W) writer structure"]
impl crate::Writable for HCTSIZ7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCTSIZ7 to value 0"]
impl crate::Resettable for HCTSIZ7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
