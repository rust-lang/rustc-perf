#[doc = "Reader of register SSR"]
pub type R = crate::R<u32, super::SSR>;
#[doc = "Reader of field `SS`"]
pub type SS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
