#[doc = "Reader of register HSTPIPCFG0_HSBOHSCP"]
pub type R = crate::R<u32, super::HSTPIPCFG0_HSBOHSCP>;
#[doc = "Writer for register HSTPIPCFG0_HSBOHSCP"]
pub type W = crate::W<u32, super::HSTPIPCFG0_HSBOHSCP>;
#[doc = "Reader of field `ALLOC`"]
pub type ALLOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALLOC`"]
pub struct ALLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `PBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBK_A {
    #[doc = "Single-bank pipe"]
    _1_BANK,
    #[doc = "Double-bank pipe"]
    _2_BANK,
    #[doc = "Triple-bank pipe"]
    _3_BANK,
}
impl crate::ToBits<u8> for PBK_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PBK_A::_1_BANK => 0,
            PBK_A::_2_BANK => 1,
            PBK_A::_3_BANK => 2,
        }
    }
}
#[doc = "Reader of field `PBK`"]
pub type PBK_R = crate::R<u8, PBK_A>;
impl PBK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PBK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PBK_A::_1_BANK),
            1 => Val(PBK_A::_2_BANK),
            2 => Val(PBK_A::_3_BANK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_BANK`"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        *self == PBK_A::_1_BANK
    }
    #[doc = "Checks if the value of the field is `_2_BANK`"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        *self == PBK_A::_2_BANK
    }
    #[doc = "Checks if the value of the field is `_3_BANK`"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        *self == PBK_A::_3_BANK
    }
}
#[doc = "Write proxy for field `PBK`"]
pub struct PBK_W<'a> {
    w: &'a mut W,
}
impl<'a> PBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBK_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single-bank pipe"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut W {
        self.variant(PBK_A::_1_BANK)
    }
    #[doc = "Double-bank pipe"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut W {
        self.variant(PBK_A::_2_BANK)
    }
    #[doc = "Triple-bank pipe"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut W {
        self.variant(PBK_A::_3_BANK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `PSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSIZE_A {
    #[doc = "8 bytes"]
    _8_BYTE,
    #[doc = "16 bytes"]
    _16_BYTE,
    #[doc = "32 bytes"]
    _32_BYTE,
    #[doc = "64 bytes"]
    _64_BYTE,
    #[doc = "128 bytes"]
    _128_BYTE,
    #[doc = "256 bytes"]
    _256_BYTE,
    #[doc = "512 bytes"]
    _512_BYTE,
    #[doc = "1024 bytes"]
    _1024_BYTE,
}
impl crate::ToBits<u8> for PSIZE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PSIZE_A::_8_BYTE => 0,
            PSIZE_A::_16_BYTE => 1,
            PSIZE_A::_32_BYTE => 2,
            PSIZE_A::_64_BYTE => 3,
            PSIZE_A::_128_BYTE => 4,
            PSIZE_A::_256_BYTE => 5,
            PSIZE_A::_512_BYTE => 6,
            PSIZE_A::_1024_BYTE => 7,
        }
    }
}
#[doc = "Reader of field `PSIZE`"]
pub type PSIZE_R = crate::R<u8, PSIZE_A>;
impl PSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSIZE_A {
        match self.bits {
            0 => PSIZE_A::_8_BYTE,
            1 => PSIZE_A::_16_BYTE,
            2 => PSIZE_A::_32_BYTE,
            3 => PSIZE_A::_64_BYTE,
            4 => PSIZE_A::_128_BYTE,
            5 => PSIZE_A::_256_BYTE,
            6 => PSIZE_A::_512_BYTE,
            7 => PSIZE_A::_1024_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == PSIZE_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == PSIZE_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == PSIZE_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == PSIZE_A::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_128_BYTE`"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        *self == PSIZE_A::_128_BYTE
    }
    #[doc = "Checks if the value of the field is `_256_BYTE`"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        *self == PSIZE_A::_256_BYTE
    }
    #[doc = "Checks if the value of the field is `_512_BYTE`"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        *self == PSIZE_A::_512_BYTE
    }
    #[doc = "Checks if the value of the field is `_1024_BYTE`"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        *self == PSIZE_A::_1024_BYTE
    }
}
#[doc = "Write proxy for field `PSIZE`"]
pub struct PSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSIZE_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_8_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_16_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_32_BYTE)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_64_BYTE)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_128_BYTE)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_256_BYTE)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_512_BYTE)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut W {
        self.variant(PSIZE_A::_1024_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `PTOKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTOKEN_A {
    #[doc = "SETUP"]
    SETUP,
    #[doc = "IN"]
    IN,
    #[doc = "OUT"]
    OUT,
}
impl crate::ToBits<u8> for PTOKEN_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PTOKEN_A::SETUP => 0,
            PTOKEN_A::IN => 1,
            PTOKEN_A::OUT => 2,
        }
    }
}
#[doc = "Reader of field `PTOKEN`"]
pub type PTOKEN_R = crate::R<u8, PTOKEN_A>;
impl PTOKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PTOKEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PTOKEN_A::SETUP),
            1 => Val(PTOKEN_A::IN),
            2 => Val(PTOKEN_A::OUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SETUP`"]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        *self == PTOKEN_A::SETUP
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == PTOKEN_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == PTOKEN_A::OUT
    }
}
#[doc = "Write proxy for field `PTOKEN`"]
pub struct PTOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTOKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTOKEN_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn setup(self) -> &'a mut W {
        self.variant(PTOKEN_A::SETUP)
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(PTOKEN_A::IN)
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(PTOKEN_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `AUTOSW`"]
pub type AUTOSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSW`"]
pub struct AUTOSW_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `PTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTYPE_A {
    #[doc = "Control"]
    CTRL,
    #[doc = "Bulk"]
    BLK,
}
impl crate::ToBits<u8> for PTYPE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PTYPE_A::CTRL => 0,
            PTYPE_A::BLK => 2,
        }
    }
}
#[doc = "Reader of field `PTYPE`"]
pub type PTYPE_R = crate::R<u8, PTYPE_A>;
impl PTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PTYPE_A::CTRL),
            2 => Val(PTYPE_A::BLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == PTYPE_A::CTRL
    }
    #[doc = "Checks if the value of the field is `BLK`"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == PTYPE_A::BLK
    }
}
#[doc = "Write proxy for field `PTYPE`"]
pub struct PTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTYPE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(PTYPE_A::CTRL)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut W {
        self.variant(PTYPE_A::BLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `PEPNUM`"]
pub type PEPNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PEPNUM`"]
pub struct PEPNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEPNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PINGEN`"]
pub type PINGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINGEN`"]
pub struct PINGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PINGEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `BINTERVAL`"]
pub type BINTERVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BINTERVAL`"]
pub struct BINTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BINTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> ALLOC_R {
        ALLOC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&self) -> PBK_R {
        PBK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PTOKEN_R {
        PTOKEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AUTOSW_R {
        AUTOSW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    pub fn pepnum(&self) -> PEPNUM_R {
        PEPNUM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&self) -> PINGEN_R {
        PINGEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Binterval Parameter for the Bulk-Out/Ping Transaction"]
    #[inline(always)]
    pub fn binterval(&self) -> BINTERVAL_R {
        BINTERVAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&mut self) -> ALLOC_W {
        ALLOC_W { w: self }
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&mut self) -> PBK_W {
        PBK_W { w: self }
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W { w: self }
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&mut self) -> PTOKEN_W {
        PTOKEN_W { w: self }
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&mut self) -> AUTOSW_W {
        AUTOSW_W { w: self }
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&mut self) -> PTYPE_W {
        PTYPE_W { w: self }
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    pub fn pepnum(&mut self) -> PEPNUM_W {
        PEPNUM_W { w: self }
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&mut self) -> PINGEN_W {
        PINGEN_W { w: self }
    }
    #[doc = "Bits 24:31 - Binterval Parameter for the Bulk-Out/Ping Transaction"]
    #[inline(always)]
    pub fn binterval(&mut self) -> BINTERVAL_W {
        BINTERVAL_W { w: self }
    }
}
