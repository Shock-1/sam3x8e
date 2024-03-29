#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `URSTS`"]
pub type URSTS_R = crate::R<bool, bool>;
#[doc = "Possible values of the field `RSTTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTTYP_A {
    #[doc = "First power-up Reset"]
    GENERALRESET,
    #[doc = "Return from Backup Mode"]
    BACKUPRESET,
    #[doc = "Watchdog fault occurred"]
    WATCHDOGRESET,
    #[doc = "Processor reset required by the software"]
    SOFTWARERESET,
    #[doc = "NRST pin detected low"]
    USERRESET,
}
impl crate::ToBits<u8> for RSTTYP_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RSTTYP_A::GENERALRESET => 0,
            RSTTYP_A::BACKUPRESET => 1,
            RSTTYP_A::WATCHDOGRESET => 2,
            RSTTYP_A::SOFTWARERESET => 3,
            RSTTYP_A::USERRESET => 4,
        }
    }
}
#[doc = "Reader of field `RSTTYP`"]
pub type RSTTYP_R = crate::R<u8, RSTTYP_A>;
impl RSTTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSTTYP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSTTYP_A::GENERALRESET),
            1 => Val(RSTTYP_A::BACKUPRESET),
            2 => Val(RSTTYP_A::WATCHDOGRESET),
            3 => Val(RSTTYP_A::SOFTWARERESET),
            4 => Val(RSTTYP_A::USERRESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GENERALRESET`"]
    #[inline(always)]
    pub fn is_general_reset(&self) -> bool {
        *self == RSTTYP_A::GENERALRESET
    }
    #[doc = "Checks if the value of the field is `BACKUPRESET`"]
    #[inline(always)]
    pub fn is_backup_reset(&self) -> bool {
        *self == RSTTYP_A::BACKUPRESET
    }
    #[doc = "Checks if the value of the field is `WATCHDOGRESET`"]
    #[inline(always)]
    pub fn is_watchdog_reset(&self) -> bool {
        *self == RSTTYP_A::WATCHDOGRESET
    }
    #[doc = "Checks if the value of the field is `SOFTWARERESET`"]
    #[inline(always)]
    pub fn is_software_reset(&self) -> bool {
        *self == RSTTYP_A::SOFTWARERESET
    }
    #[doc = "Checks if the value of the field is `USERRESET`"]
    #[inline(always)]
    pub fn is_user_reset(&self) -> bool {
        *self == RSTTYP_A::USERRESET
    }
}
#[doc = "Reader of field `NRSTL`"]
pub type NRSTL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRCMP`"]
pub type SRCMP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - User Reset Status"]
    #[inline(always)]
    pub fn ursts(&self) -> URSTS_R {
        URSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Reset Type"]
    #[inline(always)]
    pub fn rsttyp(&self) -> RSTTYP_R {
        RSTTYP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 16 - NRST Pin Level"]
    #[inline(always)]
    pub fn nrstl(&self) -> NRSTL_R {
        NRSTL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Software Reset Command in Progress"]
    #[inline(always)]
    pub fn srcmp(&self) -> SRCMP_R {
        SRCMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
