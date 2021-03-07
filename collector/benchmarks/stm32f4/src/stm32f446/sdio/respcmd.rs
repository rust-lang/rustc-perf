#[doc = "Reader of register RESPCMD"]
pub type R = crate::R<u32, super::RESPCMD>;
#[doc = "Reader of field `RESPCMD`"]
pub type RESPCMD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Response command index Read-only bit field. Contains the command index of the last command response received."]
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
