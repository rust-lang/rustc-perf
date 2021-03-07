#[doc = "Reader of register DSI_CLTCR"]
pub type R = crate::R<u32, super::DSI_CLTCR>;
#[doc = "Writer for register DSI_CLTCR"]
pub type W = crate::W<u32, super::DSI_CLTCR>;
#[doc = "Register DSI_CLTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_CLTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HS2LP_TIME`"]
pub type HS2LP_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HS2LP_TIME`"]
pub struct HS2LP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> HS2LP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LP2HS_TIME`"]
pub type LP2HS_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LP2HS_TIME`"]
pub struct LP2HS_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> LP2HS_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:25 - High-Speed to Low-Power Time"]
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - Low-Power to High-Speed Time"]
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - High-Speed to Low-Power Time"]
    #[inline(always)]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W {
        HS2LP_TIME_W { w: self }
    }
    #[doc = "Bits 0:9 - Low-Power to High-Speed Time"]
    #[inline(always)]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W {
        LP2HS_TIME_W { w: self }
    }
}
