#[doc = "Reader of register EXTICR1"]
pub type R = crate::R<u32, super::EXTICR1>;
#[doc = "Writer for register EXTICR1"]
pub type W = crate::W<u32, super::EXTICR1>;
#[doc = "Register EXTICR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTICR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI3`"]
pub type EXTI3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI3`"]
pub struct EXTI3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `EXTI2`"]
pub type EXTI2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI2`"]
pub struct EXTI2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI1`"]
pub type EXTI1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI1`"]
pub struct EXTI1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXTI0`"]
pub type EXTI0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI0`"]
pub struct EXTI0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti3(&mut self) -> EXTI3_W {
        EXTI3_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti2(&mut self) -> EXTI2_W {
        EXTI2_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti1(&mut self) -> EXTI1_W {
        EXTI1_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn exti0(&mut self) -> EXTI0_W {
        EXTI0_W { w: self }
    }
}
