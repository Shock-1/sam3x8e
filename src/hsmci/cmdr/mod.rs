#[doc = "Writer for register CMDR"]
pub type W = crate::W<u32, super::CMDR>;
#[doc = "Write proxy for field `CMDNB`"]
pub struct CMDNB_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Possible values of the field `RSPTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSPTYP_AW {
    #[doc = "No response"]
    NORESP,
    #[doc = "48-bit response"]
    _48_BIT,
    #[doc = "136-bit response"]
    _136_BIT,
    #[doc = "R1b response type"]
    R1B,
}
impl crate::ToBits<u8> for RSPTYP_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RSPTYP_AW::NORESP => 0,
            RSPTYP_AW::_48_BIT => 1,
            RSPTYP_AW::_136_BIT => 2,
            RSPTYP_AW::R1B => 3,
        }
    }
}
#[doc = "Write proxy for field `RSPTYP`"]
pub struct RSPTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> RSPTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSPTYP_AW) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No response"]
    #[inline(always)]
    pub fn noresp(self) -> &'a mut W {
        self.variant(RSPTYP_AW::NORESP)
    }
    #[doc = "48-bit response"]
    #[inline(always)]
    pub fn _48_bit(self) -> &'a mut W {
        self.variant(RSPTYP_AW::_48_BIT)
    }
    #[doc = "136-bit response"]
    #[inline(always)]
    pub fn _136_bit(self) -> &'a mut W {
        self.variant(RSPTYP_AW::_136_BIT)
    }
    #[doc = "R1b response type"]
    #[inline(always)]
    pub fn r1b(self) -> &'a mut W {
        self.variant(RSPTYP_AW::R1B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `SPCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPCMD_AW {
    #[doc = "Not a special CMD."]
    STD,
    #[doc = "Initialization CMD: 74 clock cycles for initialization sequence."]
    INIT,
    #[doc = "Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    SYNC,
    #[doc = "CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    CE_ATA,
    #[doc = "Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    IT_CMD,
    #[doc = "Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    IT_RESP,
    #[doc = "Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    BOR,
    #[doc = "End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    EBO,
}
impl crate::ToBits<u8> for SPCMD_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SPCMD_AW::STD => 0,
            SPCMD_AW::INIT => 1,
            SPCMD_AW::SYNC => 2,
            SPCMD_AW::CE_ATA => 3,
            SPCMD_AW::IT_CMD => 4,
            SPCMD_AW::IT_RESP => 5,
            SPCMD_AW::BOR => 6,
            SPCMD_AW::EBO => 7,
        }
    }
}
#[doc = "Write proxy for field `SPCMD`"]
pub struct SPCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPCMD_AW) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Not a special CMD."]
    #[inline(always)]
    pub fn std(self) -> &'a mut W {
        self.variant(SPCMD_AW::STD)
    }
    #[doc = "Initialization CMD: 74 clock cycles for initialization sequence."]
    #[inline(always)]
    pub fn init(self) -> &'a mut W {
        self.variant(SPCMD_AW::INIT)
    }
    #[doc = "Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(SPCMD_AW::SYNC)
    }
    #[doc = "CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    #[inline(always)]
    pub fn ce_ata(self) -> &'a mut W {
        self.variant(SPCMD_AW::CE_ATA)
    }
    #[doc = "Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline(always)]
    pub fn it_cmd(self) -> &'a mut W {
        self.variant(SPCMD_AW::IT_CMD)
    }
    #[doc = "Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline(always)]
    pub fn it_resp(self) -> &'a mut W {
        self.variant(SPCMD_AW::IT_RESP)
    }
    #[doc = "Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    #[inline(always)]
    pub fn bor(self) -> &'a mut W {
        self.variant(SPCMD_AW::BOR)
    }
    #[doc = "End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    #[inline(always)]
    pub fn ebo(self) -> &'a mut W {
        self.variant(SPCMD_AW::EBO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `OPDCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPDCMD_AW {
    #[doc = "Push pull command."]
    PUSHPULL,
    #[doc = "Open drain command."]
    OPENDRAIN,
}
impl crate::ToBits<bool> for OPDCMD_AW {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OPDCMD_AW::PUSHPULL => false,
            OPDCMD_AW::OPENDRAIN => true,
        }
    }
}
#[doc = "Write proxy for field `OPDCMD`"]
pub struct OPDCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> OPDCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPDCMD_AW) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Push pull command."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(OPDCMD_AW::PUSHPULL)
    }
    #[doc = "Open drain command."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(OPDCMD_AW::OPENDRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `MAXLAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXLAT_AW {
    #[doc = "5-cycle max latency."]
    _5,
    #[doc = "64-cycle max latency."]
    _64,
}
impl crate::ToBits<bool> for MAXLAT_AW {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MAXLAT_AW::_5 => false,
            MAXLAT_AW::_64 => true,
        }
    }
}
#[doc = "Write proxy for field `MAXLAT`"]
pub struct MAXLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXLAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAXLAT_AW) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "5-cycle max latency."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(MAXLAT_AW::_5)
    }
    #[doc = "64-cycle max latency."]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(MAXLAT_AW::_64)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `TRCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRCMD_AW {
    #[doc = "No data transfer"]
    NO_DATA,
    #[doc = "Start data transfer"]
    START_DATA,
    #[doc = "Stop data transfer"]
    STOP_DATA,
}
impl crate::ToBits<u8> for TRCMD_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TRCMD_AW::NO_DATA => 0,
            TRCMD_AW::START_DATA => 1,
            TRCMD_AW::STOP_DATA => 2,
        }
    }
}
#[doc = "Write proxy for field `TRCMD`"]
pub struct TRCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRCMD_AW) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No data transfer"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(TRCMD_AW::NO_DATA)
    }
    #[doc = "Start data transfer"]
    #[inline(always)]
    pub fn start_data(self) -> &'a mut W {
        self.variant(TRCMD_AW::START_DATA)
    }
    #[doc = "Stop data transfer"]
    #[inline(always)]
    pub fn stop_data(self) -> &'a mut W {
        self.variant(TRCMD_AW::STOP_DATA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `TRDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRDIR_AW {
    #[doc = "Write."]
    WRITE,
    #[doc = "Read."]
    READ,
}
impl crate::ToBits<bool> for TRDIR_AW {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRDIR_AW::WRITE => false,
            TRDIR_AW::READ => true,
        }
    }
}
#[doc = "Write proxy for field `TRDIR`"]
pub struct TRDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRDIR_AW) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write."]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(TRDIR_AW::WRITE)
    }
    #[doc = "Read."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(TRDIR_AW::READ)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `TRTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRTYP_AW {
    #[doc = "MMC/SD Card Single Block"]
    SINGLE,
    #[doc = "MMC/SD Card Multiple Block"]
    MULTIPLE,
    #[doc = "MMC Stream"]
    STREAM,
    #[doc = "SDIO Byte"]
    BYTE,
    #[doc = "SDIO Block"]
    BLOCK,
}
impl crate::ToBits<u8> for TRTYP_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TRTYP_AW::SINGLE => 0,
            TRTYP_AW::MULTIPLE => 1,
            TRTYP_AW::STREAM => 2,
            TRTYP_AW::BYTE => 4,
            TRTYP_AW::BLOCK => 5,
        }
    }
}
#[doc = "Write proxy for field `TRTYP`"]
pub struct TRTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRTYP_AW) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MMC/SD Card Single Block"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TRTYP_AW::SINGLE)
    }
    #[doc = "MMC/SD Card Multiple Block"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(TRTYP_AW::MULTIPLE)
    }
    #[doc = "MMC Stream"]
    #[inline(always)]
    pub fn stream(self) -> &'a mut W {
        self.variant(TRTYP_AW::STREAM)
    }
    #[doc = "SDIO Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(TRTYP_AW::BYTE)
    }
    #[doc = "SDIO Block"]
    #[inline(always)]
    pub fn block(self) -> &'a mut W {
        self.variant(TRTYP_AW::BLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `IOSPCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOSPCMD_AW {
    #[doc = "Not an SDIO Special Command"]
    STD,
    #[doc = "SDIO Suspend Command"]
    SUSPEND,
    #[doc = "SDIO Resume Command"]
    RESUME,
}
impl crate::ToBits<u8> for IOSPCMD_AW {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            IOSPCMD_AW::STD => 0,
            IOSPCMD_AW::SUSPEND => 1,
            IOSPCMD_AW::RESUME => 2,
        }
    }
}
#[doc = "Write proxy for field `IOSPCMD`"]
pub struct IOSPCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSPCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOSPCMD_AW) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Not an SDIO Special Command"]
    #[inline(always)]
    pub fn std(self) -> &'a mut W {
        self.variant(IOSPCMD_AW::STD)
    }
    #[doc = "SDIO Suspend Command"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(IOSPCMD_AW::SUSPEND)
    }
    #[doc = "SDIO Resume Command"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(IOSPCMD_AW::RESUME)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `ATACS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATACS_AW {
    #[doc = "Normal operation mode."]
    NORMAL,
    #[doc = "This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    COMPLETION,
}
impl crate::ToBits<bool> for ATACS_AW {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ATACS_AW::NORMAL => false,
            ATACS_AW::COMPLETION => true,
        }
    }
}
#[doc = "Write proxy for field `ATACS`"]
pub struct ATACS_W<'a> {
    w: &'a mut W,
}
impl<'a> ATACS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATACS_AW) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ATACS_AW::NORMAL)
    }
    #[doc = "This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    #[inline(always)]
    pub fn completion(self) -> &'a mut W {
        self.variant(ATACS_AW::COMPLETION)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write proxy for field `BOOT_ACK`"]
pub struct BOOT_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:5 - Command Number"]
    #[inline(always)]
    pub fn cmdnb(&mut self) -> CMDNB_W {
        CMDNB_W { w: self }
    }
    #[doc = "Bits 6:7 - Response Type"]
    #[inline(always)]
    pub fn rsptyp(&mut self) -> RSPTYP_W {
        RSPTYP_W { w: self }
    }
    #[doc = "Bits 8:10 - Special Command"]
    #[inline(always)]
    pub fn spcmd(&mut self) -> SPCMD_W {
        SPCMD_W { w: self }
    }
    #[doc = "Bit 11 - Open Drain Command"]
    #[inline(always)]
    pub fn opdcmd(&mut self) -> OPDCMD_W {
        OPDCMD_W { w: self }
    }
    #[doc = "Bit 12 - Max Latency for Command to Response"]
    #[inline(always)]
    pub fn maxlat(&mut self) -> MAXLAT_W {
        MAXLAT_W { w: self }
    }
    #[doc = "Bits 16:17 - Transfer Command"]
    #[inline(always)]
    pub fn trcmd(&mut self) -> TRCMD_W {
        TRCMD_W { w: self }
    }
    #[doc = "Bit 18 - Transfer Direction"]
    #[inline(always)]
    pub fn trdir(&mut self) -> TRDIR_W {
        TRDIR_W { w: self }
    }
    #[doc = "Bits 19:21 - Transfer Type"]
    #[inline(always)]
    pub fn trtyp(&mut self) -> TRTYP_W {
        TRTYP_W { w: self }
    }
    #[doc = "Bits 24:25 - SDIO Special Command"]
    #[inline(always)]
    pub fn iospcmd(&mut self) -> IOSPCMD_W {
        IOSPCMD_W { w: self }
    }
    #[doc = "Bit 26 - ATA with Command Completion Signal"]
    #[inline(always)]
    pub fn atacs(&mut self) -> ATACS_W {
        ATACS_W { w: self }
    }
    #[doc = "Bit 27 - Boot Operation Acknowledge"]
    #[inline(always)]
    pub fn boot_ack(&mut self) -> BOOT_ACK_W {
        BOOT_ACK_W { w: self }
    }
}
