#[doc = "Reader of register RESP2"]
pub type R = crate::R<u32, super::RESP2>;
#[doc = "Reader of field `CARDSTATUS2`"]
pub type CARDSTATUS2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Card Status"]
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
