#[doc = "Writer for register FCR"]
pub type W = crate::W<u32, super::FCR>;
#[doc = "Possible values of the field `FCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCMD_AW {
    #[doc = "Get Flash Descriptor"]
    GETD,
    #[doc = "Write page"]
    WP,
    #[doc = "Write page and lock"]
    WPL,
    #[doc = "Erase page and write page"]
    EWP,
    #[doc = "Erase page and write page then lock"]
    EWPL,
    #[doc = "Erase all"]
    EA,
    #[doc = "Set Lock Bit"]
    SLB,
    #[doc = "Clear Lock Bit"]
    CLB,
    #[doc = "Get Lock Bit"]
    GLB,
    #[doc = "Set GPNVM Bit"]
    SGPB,
    #[doc = "Clear GPNVM Bit"]
    CGPB,
    #[doc = "Get GPNVM Bit"]
    GGPB,
    #[doc = "Start Read Unique Identifier"]
    STUI,
    #[doc = "Stop Read Unique Identifier"]
    SPUI,
    #[doc = "Get CALIB Bit"]
    GCALB,
}
impl crate::ToBits<u8> for FCMD_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FCMD_AW::GETD => 0,
            FCMD_AW::WP => 1,
            FCMD_AW::WPL => 2,
            FCMD_AW::EWP => 3,
            FCMD_AW::EWPL => 4,
            FCMD_AW::EA => 5,
            FCMD_AW::SLB => 8,
            FCMD_AW::CLB => 9,
            FCMD_AW::GLB => 10,
            FCMD_AW::SGPB => 11,
            FCMD_AW::CGPB => 12,
            FCMD_AW::GGPB => 13,
            FCMD_AW::STUI => 14,
            FCMD_AW::SPUI => 15,
            FCMD_AW::GCALB => 16,
        }
    }
}
#[doc = "Write proxy for field `FCMD`"]
pub struct FCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCMD_AW) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Get Flash Descriptor"]
    #[inline(always)]
    pub fn getd(self) -> &'a mut W {
        self.variant(FCMD_AW::GETD)
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(FCMD_AW::WP)
    }
    #[doc = "Write page and lock"]
    #[inline(always)]
    pub fn wpl(self) -> &'a mut W {
        self.variant(FCMD_AW::WPL)
    }
    #[doc = "Erase page and write page"]
    #[inline(always)]
    pub fn ewp(self) -> &'a mut W {
        self.variant(FCMD_AW::EWP)
    }
    #[doc = "Erase page and write page then lock"]
    #[inline(always)]
    pub fn ewpl(self) -> &'a mut W {
        self.variant(FCMD_AW::EWPL)
    }
    #[doc = "Erase all"]
    #[inline(always)]
    pub fn ea(self) -> &'a mut W {
        self.variant(FCMD_AW::EA)
    }
    #[doc = "Set Lock Bit"]
    #[inline(always)]
    pub fn slb(self) -> &'a mut W {
        self.variant(FCMD_AW::SLB)
    }
    #[doc = "Clear Lock Bit"]
    #[inline(always)]
    pub fn clb(self) -> &'a mut W {
        self.variant(FCMD_AW::CLB)
    }
    #[doc = "Get Lock Bit"]
    #[inline(always)]
    pub fn glb(self) -> &'a mut W {
        self.variant(FCMD_AW::GLB)
    }
    #[doc = "Set GPNVM Bit"]
    #[inline(always)]
    pub fn sgpb(self) -> &'a mut W {
        self.variant(FCMD_AW::SGPB)
    }
    #[doc = "Clear GPNVM Bit"]
    #[inline(always)]
    pub fn cgpb(self) -> &'a mut W {
        self.variant(FCMD_AW::CGPB)
    }
    #[doc = "Get GPNVM Bit"]
    #[inline(always)]
    pub fn ggpb(self) -> &'a mut W {
        self.variant(FCMD_AW::GGPB)
    }
    #[doc = "Start Read Unique Identifier"]
    #[inline(always)]
    pub fn stui(self) -> &'a mut W {
        self.variant(FCMD_AW::STUI)
    }
    #[doc = "Stop Read Unique Identifier"]
    #[inline(always)]
    pub fn spui(self) -> &'a mut W {
        self.variant(FCMD_AW::SPUI)
    }
    #[doc = "Get CALIB Bit"]
    #[inline(always)]
    pub fn gcalb(self) -> &'a mut W {
        self.variant(FCMD_AW::GCALB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `FARG`"]
pub struct FARG_W<'a> {
    w: &'a mut W,
}
impl<'a> FARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `FKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FKEY_AW {
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    PASSWD,
}
impl crate::ToBits<u8> for FKEY_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FKEY_AW::PASSWD => 90,
        }
    }
}
#[doc = "Write proxy for field `FKEY`"]
pub struct FKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> FKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FKEY_AW) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(FKEY_AW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash Command"]
    #[inline(always)]
    pub fn fcmd(&mut self) -> FCMD_W {
        FCMD_W { w: self }
    }
    #[doc = "Bits 8:23 - Flash Command Argument"]
    #[inline(always)]
    pub fn farg(&mut self) -> FARG_W {
        FARG_W { w: self }
    }
    #[doc = "Bits 24:31 - Flash Writing Protection Key"]
    #[inline(always)]
    pub fn fkey(&mut self) -> FKEY_W {
        FKEY_W { w: self }
    }
}
