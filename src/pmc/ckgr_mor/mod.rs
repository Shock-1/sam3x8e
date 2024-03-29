#[doc = "Reader of register CKGR_MOR"]
pub type R = crate::R<u32, super::CKGR_MOR>;
#[doc = "Writer for register CKGR_MOR"]
pub type W = crate::W<u32, super::CKGR_MOR>;
#[doc = "Register CKGR_MOR `reset()`'s with value 0x08"]
impl crate::ResetValue for super::CKGR_MOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `MOSCXTEN`"]
pub type MOSCXTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSCXTEN`"]
pub struct MOSCXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCXTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `MOSCXTBY`"]
pub type MOSCXTBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSCXTBY`"]
pub struct MOSCXTBY_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCXTBY_W<'a> {
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
#[doc = "Reader of field `MOSCRCEN`"]
pub type MOSCRCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSCRCEN`"]
pub struct MOSCRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCRCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `MOSCRCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOSCRCF_A {
    #[doc = "The Fast RC Oscillator Frequency is at 4 MHz (default)"]
    _4_MHZ,
    #[doc = "The Fast RC Oscillator Frequency is at 8 MHz"]
    _8_MHZ,
    #[doc = "The Fast RC Oscillator Frequency is at 12 MHz"]
    _12_MHZ,
}
impl crate::ToBits<u8> for MOSCRCF_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MOSCRCF_A::_4_MHZ => 0,
            MOSCRCF_A::_8_MHZ => 1,
            MOSCRCF_A::_12_MHZ => 2,
        }
    }
}
#[doc = "Reader of field `MOSCRCF`"]
pub type MOSCRCF_R = crate::R<u8, MOSCRCF_A>;
impl MOSCRCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MOSCRCF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MOSCRCF_A::_4_MHZ),
            1 => Val(MOSCRCF_A::_8_MHZ),
            2 => Val(MOSCRCF_A::_12_MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4_MHZ`"]
    #[inline(always)]
    pub fn is_4_mhz(&self) -> bool {
        *self == MOSCRCF_A::_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_8_MHZ`"]
    #[inline(always)]
    pub fn is_8_mhz(&self) -> bool {
        *self == MOSCRCF_A::_8_MHZ
    }
    #[doc = "Checks if the value of the field is `_12_MHZ`"]
    #[inline(always)]
    pub fn is_12_mhz(&self) -> bool {
        *self == MOSCRCF_A::_12_MHZ
    }
}
#[doc = "Write proxy for field `MOSCRCF`"]
pub struct MOSCRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCRCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOSCRCF_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The Fast RC Oscillator Frequency is at 4 MHz (default)"]
    #[inline(always)]
    pub fn _4_mhz(self) -> &'a mut W {
        self.variant(MOSCRCF_A::_4_MHZ)
    }
    #[doc = "The Fast RC Oscillator Frequency is at 8 MHz"]
    #[inline(always)]
    pub fn _8_mhz(self) -> &'a mut W {
        self.variant(MOSCRCF_A::_8_MHZ)
    }
    #[doc = "The Fast RC Oscillator Frequency is at 12 MHz"]
    #[inline(always)]
    pub fn _12_mhz(self) -> &'a mut W {
        self.variant(MOSCRCF_A::_12_MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `MOSCXTST`"]
pub type MOSCXTST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MOSCXTST`"]
pub struct MOSCXTST_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCXTST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_A {
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD,
}
impl crate::ToBits<u8> for KEY_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            KEY_A::PASSWD => 55,
        }
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u8, KEY_A>;
impl KEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KEY_A> {
        use crate::Variant::*;
        match self.bits {
            55 => Val(KEY_A::PASSWD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == KEY_A::PASSWD
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_A::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MOSCSEL`"]
pub type MOSCSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSCSEL`"]
pub struct MOSCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CFDEN`"]
pub type CFDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFDEN`"]
pub struct CFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&self) -> MOSCXTEN_R {
        MOSCXTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&self) -> MOSCXTBY_R {
        MOSCXTBY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Main On-Chip RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&self) -> MOSCRCEN_R {
        MOSCRCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Main On-Chip RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&self) -> MOSCRCF_R {
        MOSCRCF_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Start-up Time"]
    #[inline(always)]
    pub fn moscxtst(&self) -> MOSCXTST_R {
        MOSCXTST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Main Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&self) -> MOSCSEL_R {
        MOSCSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&mut self) -> MOSCXTEN_W {
        MOSCXTEN_W { w: self }
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&mut self) -> MOSCXTBY_W {
        MOSCXTBY_W { w: self }
    }
    #[doc = "Bit 3 - Main On-Chip RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&mut self) -> MOSCRCEN_W {
        MOSCRCEN_W { w: self }
    }
    #[doc = "Bits 4:6 - Main On-Chip RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&mut self) -> MOSCRCF_W {
        MOSCRCF_W { w: self }
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Start-up Time"]
    #[inline(always)]
    pub fn moscxtst(&mut self) -> MOSCXTST_W {
        MOSCXTST_W { w: self }
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Bit 24 - Main Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&mut self) -> MOSCSEL_W {
        MOSCSEL_W { w: self }
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W {
        CFDEN_W { w: self }
    }
}
