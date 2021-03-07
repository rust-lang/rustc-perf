#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `DOWN`"]
pub type DOWN_R = crate::R<bool, bool>;
#[doc = "Reader of field `UP`"]
pub type UP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARROK`"]
pub type ARROK_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPOK`"]
pub type CMPOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTTRIG`"]
pub type EXTTRIG_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARRM`"]
pub type ARRM_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPM`"]
pub type CMPM_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 6 - Counter direction change up to down"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK"]
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK"]
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Autoreload match"]
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Compare match"]
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 0x01) != 0)
    }
}
