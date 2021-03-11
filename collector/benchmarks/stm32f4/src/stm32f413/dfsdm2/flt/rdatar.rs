#[doc = "Reader of register RDATAR"]
pub type R = crate::R<u32, super::RDATAR>;
#[doc = "Reader of field `RDATA`"]
pub type RDATA_R = crate::R<u32, u32>;
#[doc = "Reader of field `RPEND`"]
pub type RPEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDATACH`"]
pub type RDATACH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:31 - Regular channel conversion data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 4 - Regular channel pending data"]
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Regular channel most recently converted"]
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 0x07) as u8)
    }
}
