#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Write proxy for field `WDRSTT`"]
pub struct WDRSTT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRSTT_W<'a> {
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
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_AW {
    #[doc = "Writing any other value in this field aborts the write operation."]
    PASSWD,
}
impl crate::ToBits<u8> for KEY_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            KEY_AW::PASSWD => 165,
        }
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_AW) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_AW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline(always)]
    pub fn wdrstt(&mut self) -> WDRSTT_W {
        WDRSTT_W { w: self }
    }
    #[doc = "Bits 24:31 - Password."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
