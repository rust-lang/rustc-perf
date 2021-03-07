#[doc = "Reader of register RESP3"]
pub type R = crate::R<u32, super::RESP3>;
#[doc = "Reader of field `CARDSTATUS3`"]
pub type CARDSTATUS3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Card Status"]
    #[inline(always)]
    pub fn cardstatus3(&self) -> CARDSTATUS3_R {
        CARDSTATUS3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
