#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Possible values of the field `VROFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VROFF_AW {
    #[doc = "no effect."]
    NO_EFFECT,
    #[doc = "if KEY is correct, asserts the vddcore_nreset and stops the voltage regulator."]
    STOP_VREG,
}
impl crate::ToBits<bool> for VROFF_AW {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            VROFF_AW::NO_EFFECT => false,
            VROFF_AW::STOP_VREG => true,
        }
    }
}
#[doc = "Write proxy for field `VROFF`"]
pub struct VROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> VROFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VROFF_AW) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(VROFF_AW::NO_EFFECT)
    }
    #[doc = "if KEY is correct, asserts the vddcore_nreset and stops the voltage regulator."]
    #[inline(always)]
    pub fn stop_vreg(self) -> &'a mut W {
        self.variant(VROFF_AW::STOP_VREG)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `XTALSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALSEL_AW {
    #[doc = "no effect."]
    NO_EFFECT,
    #[doc = "if KEY is correct, switches the slow clock on the crystal oscillator output."]
    CRYSTAL_SEL,
}
impl crate::ToBits<bool> for XTALSEL_AW {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            XTALSEL_AW::NO_EFFECT => false,
            XTALSEL_AW::CRYSTAL_SEL => true,
        }
    }
}
#[doc = "Write proxy for field `XTALSEL`"]
pub struct XTALSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTALSEL_AW) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(XTALSEL_AW::NO_EFFECT)
    }
    #[doc = "if KEY is correct, switches the slow clock on the crystal oscillator output."]
    #[inline(always)]
    pub fn crystal_sel(self) -> &'a mut W {
        self.variant(XTALSEL_AW::CRYSTAL_SEL)
    }
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
    #[doc = "Bit 2 - Voltage Regulator Off"]
    #[inline(always)]
    pub fn vroff(&mut self) -> VROFF_W {
        VROFF_W { w: self }
    }
    #[doc = "Bit 3 - Crystal Oscillator Select"]
    #[inline(always)]
    pub fn xtalsel(&mut self) -> XTALSEL_W {
        XTALSEL_W { w: self }
    }
    #[doc = "Bits 24:31 - Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
