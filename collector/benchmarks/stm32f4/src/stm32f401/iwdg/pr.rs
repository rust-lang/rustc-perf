#[doc = "Reader of register PR"]
pub type R = crate::R<u32, super::PR>;
#[doc = "Writer for register PR"]
pub type W = crate::W<u32, super::PR>;
#[doc = "Register PR `reset()`'s with value 0"]
impl crate::ResetValue for super::PR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescaler divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PR_A {
    #[doc = "0: Divider /4"]
    DIVIDEBY4 = 0,
    #[doc = "1: Divider /8"]
    DIVIDEBY8 = 1,
    #[doc = "2: Divider /16"]
    DIVIDEBY16 = 2,
    #[doc = "3: Divider /32"]
    DIVIDEBY32 = 3,
    #[doc = "4: Divider /64"]
    DIVIDEBY64 = 4,
    #[doc = "5: Divider /128"]
    DIVIDEBY128 = 5,
    #[doc = "6: Divider /256"]
    DIVIDEBY256 = 6,
    #[doc = "7: Divider /256"]
    DIVIDEBY256BIS = 7,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<u8, PR_A>;
impl PR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR_A {
        match self.bits {
            0 => PR_A::DIVIDEBY4,
            1 => PR_A::DIVIDEBY8,
            2 => PR_A::DIVIDEBY16,
            3 => PR_A::DIVIDEBY32,
            4 => PR_A::DIVIDEBY64,
            5 => PR_A::DIVIDEBY128,
            6 => PR_A::DIVIDEBY256,
            7 => PR_A::DIVIDEBY256BIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY4`"]
    #[inline(always)]
    pub fn is_divide_by4(&self) -> bool {
        *self == PR_A::DIVIDEBY4
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY8`"]
    #[inline(always)]
    pub fn is_divide_by8(&self) -> bool {
        *self == PR_A::DIVIDEBY8
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY16`"]
    #[inline(always)]
    pub fn is_divide_by16(&self) -> bool {
        *self == PR_A::DIVIDEBY16
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY32`"]
    #[inline(always)]
    pub fn is_divide_by32(&self) -> bool {
        *self == PR_A::DIVIDEBY32
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY64`"]
    #[inline(always)]
    pub fn is_divide_by64(&self) -> bool {
        *self == PR_A::DIVIDEBY64
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY128`"]
    #[inline(always)]
    pub fn is_divide_by128(&self) -> bool {
        *self == PR_A::DIVIDEBY128
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY256`"]
    #[inline(always)]
    pub fn is_divide_by256(&self) -> bool {
        *self == PR_A::DIVIDEBY256
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY256BIS`"]
    #[inline(always)]
    pub fn is_divide_by256bis(&self) -> bool {
        *self == PR_A::DIVIDEBY256BIS
    }
}
#[doc = "Write proxy for field `PR`"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divider /4"]
    #[inline(always)]
    pub fn divide_by4(self) -> &'a mut W {
        self.variant(PR_A::DIVIDEBY4)
    }
    #[doc = "Divider /8"]
    #[inline(always)]
    pub fn divide_by8(self) -> &'a mut W {
        self.variant(PR_A::DIVIDEBY8)
    }
    #[doc = "Divider /16"]
    #[inline(always)]
    pub fn divide_by16(self) -> &'a mut W {
        self.variant(PR_A::DIVIDEBY16)
    }
    #[doc = "Divider /32"]
    #[inline(always)]
    pub fn divide_by32(self) -> &'a mut W {
        self.variant(PR_A::DIVIDEBY32)
    }
    #[doc = "Divider /64"]
    #[inline(always)]
    pub fn divide_by64(self) -> &'a mut W {
        self.variant(PR_A::DIVIDEBY64)
    }
    #[doc = "Divider /128"]
    #[inline(always)]
    pub fn divide_by128(self) -> &'a mut W {
        self.variant(PR_A::DIVIDEBY128)
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn divide_by256(self) -> &'a mut W {
        self.variant(PR_A::DIVIDEBY256)
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn divide_by256bis(self) -> &'a mut W {
        self.variant(PR_A::DIVIDEBY256BIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler divider"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler divider"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
}
