#[doc = "Reader of register DTOR"]
pub type R = crate::R<u32, super::DTOR>;
#[doc = "Writer for register DTOR"]
pub type W = crate::W<u32, super::DTOR>;
#[doc = "Register DTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::DTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTOCYC`"]
pub type DTOCYC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTOCYC`"]
pub struct DTOCYC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOCYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Possible values of the field `DTOMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOMUL_A {
    #[doc = "DTOCYC"]
    _1,
    #[doc = "DTOCYC x 16"]
    _16,
    #[doc = "DTOCYC x 128"]
    _128,
    #[doc = "DTOCYC x 256"]
    _256,
    #[doc = "DTOCYC x 1024"]
    _1024,
    #[doc = "DTOCYC x 4096"]
    _4096,
    #[doc = "DTOCYC x 65536"]
    _65536,
    #[doc = "DTOCYC x 1048576"]
    _1048576,
}
impl crate::ToBits<u8> for DTOMUL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DTOMUL_A::_1 => 0,
            DTOMUL_A::_16 => 1,
            DTOMUL_A::_128 => 2,
            DTOMUL_A::_256 => 3,
            DTOMUL_A::_1024 => 4,
            DTOMUL_A::_4096 => 5,
            DTOMUL_A::_65536 => 6,
            DTOMUL_A::_1048576 => 7,
        }
    }
}
#[doc = "Reader of field `DTOMUL`"]
pub type DTOMUL_R = crate::R<u8, DTOMUL_A>;
impl DTOMUL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOMUL_A {
        match self.bits {
            0 => DTOMUL_A::_1,
            1 => DTOMUL_A::_16,
            2 => DTOMUL_A::_128,
            3 => DTOMUL_A::_256,
            4 => DTOMUL_A::_1024,
            5 => DTOMUL_A::_4096,
            6 => DTOMUL_A::_65536,
            7 => DTOMUL_A::_1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTOMUL_A::_1
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == DTOMUL_A::_16
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == DTOMUL_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == DTOMUL_A::_256
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == DTOMUL_A::_1024
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == DTOMUL_A::_4096
    }
    #[doc = "Checks if the value of the field is `_65536`"]
    #[inline(always)]
    pub fn is_65536(&self) -> bool {
        *self == DTOMUL_A::_65536
    }
    #[doc = "Checks if the value of the field is `_1048576`"]
    #[inline(always)]
    pub fn is_1048576(&self) -> bool {
        *self == DTOMUL_A::_1048576
    }
}
#[doc = "Write proxy for field `DTOMUL`"]
pub struct DTOMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOMUL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOMUL_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DTOCYC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOMUL_A::_1)
    }
    #[doc = "DTOCYC x 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(DTOMUL_A::_16)
    }
    #[doc = "DTOCYC x 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(DTOMUL_A::_128)
    }
    #[doc = "DTOCYC x 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(DTOMUL_A::_256)
    }
    #[doc = "DTOCYC x 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(DTOMUL_A::_1024)
    }
    #[doc = "DTOCYC x 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut W {
        self.variant(DTOMUL_A::_4096)
    }
    #[doc = "DTOCYC x 65536"]
    #[inline(always)]
    pub fn _65536(self) -> &'a mut W {
        self.variant(DTOMUL_A::_65536)
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline(always)]
    pub fn _1048576(self) -> &'a mut W {
        self.variant(DTOMUL_A::_1048576)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&self) -> DTOCYC_R {
        DTOCYC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&self) -> DTOMUL_R {
        DTOMUL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&mut self) -> DTOCYC_W {
        DTOCYC_W { w: self }
    }
    #[doc = "Bits 4:6 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&mut self) -> DTOMUL_W {
        DTOMUL_W { w: self }
    }
}
