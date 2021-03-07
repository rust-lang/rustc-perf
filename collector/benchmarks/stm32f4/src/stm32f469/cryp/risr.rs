#[doc = "Reader of register RISR"]
pub type R = crate::R<u32, super::RISR>;
#[doc = "Reader of field `OUTRIS`"]
pub type OUTRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRIS`"]
pub type INRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Output FIFO service raw interrupt status"]
    #[inline(always)]
    pub fn outris(&self) -> OUTRIS_R {
        OUTRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Input FIFO service raw interrupt status"]
    #[inline(always)]
    pub fn inris(&self) -> INRIS_R {
        INRIS_R::new((self.bits & 0x01) != 0)
    }
}
