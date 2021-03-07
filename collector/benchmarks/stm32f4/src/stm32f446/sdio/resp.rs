#[doc = "Reader of register RESP%s"]
pub type R = crate::R<u32, super::RESP>;
#[doc = "Reader of field `CARDSTATUS`"]
pub type CARDSTATUS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - see Table404."]
    #[inline(always)]
    pub fn cardstatus(&self) -> CARDSTATUS_R {
        CARDSTATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
