#[doc = "Reader of register PTPTSCR"]
pub type R = crate::R<u32, super::PTPTSCR>;
#[doc = "Writer for register PTPTSCR"]
pub type W = crate::W<u32, super::PTPTSCR>;
#[doc = "Register PTPTSCR `reset()`'s with value 0x2000"]
impl crate::ResetValue for super::PTPTSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000
    }
}
#[doc = "Reader of field `TSE`"]
pub type TSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSE`"]
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TSFCU`"]
pub type TSFCU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSFCU`"]
pub struct TSFCU_W<'a> {
    w: &'a mut W,
}
impl<'a> TSFCU_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TSPTPPSV2E`"]
pub type TSPTPPSV2E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSPTPPSV2E`"]
pub struct TSPTPPSV2E_W<'a> {
    w: &'a mut W,
}
impl<'a> TSPTPPSV2E_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TSSPTPOEFE`"]
pub type TSSPTPOEFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSSPTPOEFE`"]
pub struct TSSPTPOEFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSPTPOEFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TSSIPV6FE`"]
pub type TSSIPV6FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSSIPV6FE`"]
pub struct TSSIPV6FE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSIPV6FE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TSSIPV4FE`"]
pub type TSSIPV4FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSSIPV4FE`"]
pub struct TSSIPV4FE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSIPV4FE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TSSEME`"]
pub type TSSEME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSSEME`"]
pub struct TSSEME_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSEME_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TSSMRME`"]
pub type TSSMRME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSSMRME`"]
pub struct TSSMRME_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSMRME_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TSCNT`"]
pub type TSCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSCNT`"]
pub struct TSCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `TSPFFMAE`"]
pub type TSPFFMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSPFFMAE`"]
pub struct TSPFFMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSPFFMAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TSSTI`"]
pub type TSSTI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSSTI`"]
pub struct TSSTI_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSTI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TSSTU`"]
pub type TSSTU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSSTU`"]
pub struct TSSTU_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSTU_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TSITE`"]
pub type TSITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSITE`"]
pub struct TSITE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSITE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TTSARU`"]
pub type TTSARU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TTSARU`"]
pub struct TTSARU_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSARU_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TSSARFE`"]
pub type TSSARFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSSARFE`"]
pub struct TSSARFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSARFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TSSSR`"]
pub type TSSSR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSSSR`"]
pub struct TSSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSSR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TSFCU"]
    #[inline(always)]
    pub fn tsfcu(&self) -> TSFCU_R {
        TSFCU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TSPTPPSV2E"]
    #[inline(always)]
    pub fn tsptppsv2e(&self) -> TSPTPPSV2E_R {
        TSPTPPSV2E_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TSSPTPOEFE"]
    #[inline(always)]
    pub fn tssptpoefe(&self) -> TSSPTPOEFE_R {
        TSSPTPOEFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TSSIPV6FE"]
    #[inline(always)]
    pub fn tssipv6fe(&self) -> TSSIPV6FE_R {
        TSSIPV6FE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TSSIPV4FE"]
    #[inline(always)]
    pub fn tssipv4fe(&self) -> TSSIPV4FE_R {
        TSSIPV4FE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TSSEME"]
    #[inline(always)]
    pub fn tsseme(&self) -> TSSEME_R {
        TSSEME_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TSSMRME"]
    #[inline(always)]
    pub fn tssmrme(&self) -> TSSMRME_R {
        TSSMRME_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - TSCNT"]
    #[inline(always)]
    pub fn tscnt(&self) -> TSCNT_R {
        TSCNT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - TSPFFMAE"]
    #[inline(always)]
    pub fn tspffmae(&self) -> TSPFFMAE_R {
        TSPFFMAE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TSSTI"]
    #[inline(always)]
    pub fn tssti(&self) -> TSSTI_R {
        TSSTI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSSTU"]
    #[inline(always)]
    pub fn tsstu(&self) -> TSSTU_R {
        TSSTU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TSITE"]
    #[inline(always)]
    pub fn tsite(&self) -> TSITE_R {
        TSITE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TTSARU"]
    #[inline(always)]
    pub fn ttsaru(&self) -> TTSARU_R {
        TTSARU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TSSARFE"]
    #[inline(always)]
    pub fn tssarfe(&self) -> TSSARFE_R {
        TSSARFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSSSR"]
    #[inline(always)]
    pub fn tsssr(&self) -> TSSSR_R {
        TSSSR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    #[doc = "Bit 1 - TSFCU"]
    #[inline(always)]
    pub fn tsfcu(&mut self) -> TSFCU_W {
        TSFCU_W { w: self }
    }
    #[doc = "Bit 10 - TSPTPPSV2E"]
    #[inline(always)]
    pub fn tsptppsv2e(&mut self) -> TSPTPPSV2E_W {
        TSPTPPSV2E_W { w: self }
    }
    #[doc = "Bit 11 - TSSPTPOEFE"]
    #[inline(always)]
    pub fn tssptpoefe(&mut self) -> TSSPTPOEFE_W {
        TSSPTPOEFE_W { w: self }
    }
    #[doc = "Bit 12 - TSSIPV6FE"]
    #[inline(always)]
    pub fn tssipv6fe(&mut self) -> TSSIPV6FE_W {
        TSSIPV6FE_W { w: self }
    }
    #[doc = "Bit 13 - TSSIPV4FE"]
    #[inline(always)]
    pub fn tssipv4fe(&mut self) -> TSSIPV4FE_W {
        TSSIPV4FE_W { w: self }
    }
    #[doc = "Bit 14 - TSSEME"]
    #[inline(always)]
    pub fn tsseme(&mut self) -> TSSEME_W {
        TSSEME_W { w: self }
    }
    #[doc = "Bit 15 - TSSMRME"]
    #[inline(always)]
    pub fn tssmrme(&mut self) -> TSSMRME_W {
        TSSMRME_W { w: self }
    }
    #[doc = "Bits 16:17 - TSCNT"]
    #[inline(always)]
    pub fn tscnt(&mut self) -> TSCNT_W {
        TSCNT_W { w: self }
    }
    #[doc = "Bit 18 - TSPFFMAE"]
    #[inline(always)]
    pub fn tspffmae(&mut self) -> TSPFFMAE_W {
        TSPFFMAE_W { w: self }
    }
    #[doc = "Bit 2 - TSSTI"]
    #[inline(always)]
    pub fn tssti(&mut self) -> TSSTI_W {
        TSSTI_W { w: self }
    }
    #[doc = "Bit 3 - TSSTU"]
    #[inline(always)]
    pub fn tsstu(&mut self) -> TSSTU_W {
        TSSTU_W { w: self }
    }
    #[doc = "Bit 4 - TSITE"]
    #[inline(always)]
    pub fn tsite(&mut self) -> TSITE_W {
        TSITE_W { w: self }
    }
    #[doc = "Bit 5 - TTSARU"]
    #[inline(always)]
    pub fn ttsaru(&mut self) -> TTSARU_W {
        TTSARU_W { w: self }
    }
    #[doc = "Bit 8 - TSSARFE"]
    #[inline(always)]
    pub fn tssarfe(&mut self) -> TSSARFE_W {
        TSSARFE_W { w: self }
    }
    #[doc = "Bit 9 - TSSSR"]
    #[inline(always)]
    pub fn tsssr(&mut self) -> TSSSR_W {
        TSSSR_W { w: self }
    }
}
