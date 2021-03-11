#[doc = "Reader of register DOUT"]
pub type R = crate::R<u32, super::DOUT>;
#[doc = "Reader of field `DATAOUT`"]
pub type DATAOUT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data output"]
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
