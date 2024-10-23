#[doc = "Register `SCAPABILITIES` reader"]
pub type R = crate::R<ScapabilitiesSpec>;
#[doc = "ID 48b handler\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idena {
    #[doc = "0: APPLICATION: Application handles ID 48b"]
    Application = 0,
    #[doc = "1: HW: Hardware handles ID 48b"]
    Hw = 1,
    #[doc = "2: HW_BUT: in hardware but the I3C module instance handles ID 48b."]
    HwBut = 2,
    #[doc = "3: PARTNO: a part number register (PARTNO) handles ID 48b"]
    Partno = 3,
}
impl From<Idena> for u8 {
    #[inline(always)]
    fn from(variant: Idena) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idena {
    type Ux = u8;
}
impl crate::IsEnum for Idena {}
#[doc = "Field `IDENA` reader - ID 48b handler"]
pub type IdenaR = crate::FieldReader<Idena>;
impl IdenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idena {
        match self.bits {
            0 => Idena::Application,
            1 => Idena::Hw,
            2 => Idena::HwBut,
            3 => Idena::Partno,
            _ => unreachable!(),
        }
    }
    #[doc = "APPLICATION: Application handles ID 48b"]
    #[inline(always)]
    pub fn is_application(&self) -> bool {
        *self == Idena::Application
    }
    #[doc = "HW: Hardware handles ID 48b"]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        *self == Idena::Hw
    }
    #[doc = "HW_BUT: in hardware but the I3C module instance handles ID 48b."]
    #[inline(always)]
    pub fn is_hw_but(&self) -> bool {
        *self == Idena::HwBut
    }
    #[doc = "PARTNO: a part number register (PARTNO) handles ID 48b"]
    #[inline(always)]
    pub fn is_partno(&self) -> bool {
        *self == Idena::Partno
    }
}
#[doc = "Field `IDREG` reader - ID register"]
pub type IdregR = crate::FieldReader;
#[doc = "Field `HDRSUPP` reader - HDR support"]
pub type HdrsuppR = crate::FieldReader;
#[doc = "Master\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Master {
    #[doc = "0: MASTERNOTSUPPORTED: master capability is not supported."]
    Masternotsupported = 0,
    #[doc = "1: MASTERSUPPORTED: master capability is supported."]
    Mastersupported = 1,
}
impl From<Master> for bool {
    #[inline(always)]
    fn from(variant: Master) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER` reader - Master"]
pub type MasterR = crate::BitReader<Master>;
impl MasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Master {
        match self.bits {
            false => Master::Masternotsupported,
            true => Master::Mastersupported,
        }
    }
    #[doc = "MASTERNOTSUPPORTED: master capability is not supported."]
    #[inline(always)]
    pub fn is_masternotsupported(&self) -> bool {
        *self == Master::Masternotsupported
    }
    #[doc = "MASTERSUPPORTED: master capability is supported."]
    #[inline(always)]
    pub fn is_mastersupported(&self) -> bool {
        *self == Master::Mastersupported
    }
}
#[doc = "Static address\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Saddr {
    #[doc = "0: NO_STATIC: No static address"]
    NoStatic = 0,
    #[doc = "1: STATIC: Static address is fixed in hardware"]
    Static = 1,
    #[doc = "2: HW_CONTROL: Hardware controls the static address dynamically (for example, from the pin strap)"]
    HwControl = 2,
    #[doc = "3: CONFIG: SCONFIG register supplies the static address"]
    Config = 3,
}
impl From<Saddr> for u8 {
    #[inline(always)]
    fn from(variant: Saddr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Saddr {
    type Ux = u8;
}
impl crate::IsEnum for Saddr {}
#[doc = "Field `SADDR` reader - Static address"]
pub type SaddrR = crate::FieldReader<Saddr>;
impl SaddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Saddr {
        match self.bits {
            0 => Saddr::NoStatic,
            1 => Saddr::Static,
            2 => Saddr::HwControl,
            3 => Saddr::Config,
            _ => unreachable!(),
        }
    }
    #[doc = "NO_STATIC: No static address"]
    #[inline(always)]
    pub fn is_no_static(&self) -> bool {
        *self == Saddr::NoStatic
    }
    #[doc = "STATIC: Static address is fixed in hardware"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == Saddr::Static
    }
    #[doc = "HW_CONTROL: Hardware controls the static address dynamically (for example, from the pin strap)"]
    #[inline(always)]
    pub fn is_hw_control(&self) -> bool {
        *self == Saddr::HwControl
    }
    #[doc = "CONFIG: SCONFIG register supplies the static address"]
    #[inline(always)]
    pub fn is_config(&self) -> bool {
        *self == Saddr::Config
    }
}
#[doc = "Field `CCCHANDLE` reader - Common Command Codes (CCC) handling"]
pub type CcchandleR = crate::FieldReader;
#[doc = "Field `IBI_MR_HJ` reader - In-Band Interrupts, Master Requests, Hot Join events"]
pub type IbiMrHjR = crate::FieldReader;
#[doc = "Time control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timectrl {
    #[doc = "0: NO_TIME_CONTROL_TYPE: No time control is enabled"]
    NoTimeControlType = 0,
    #[doc = "1: ATLEAST1_TIME_CONTROL: at least one time-control type is supported"]
    Atleast1TimeControl = 1,
}
impl From<Timectrl> for bool {
    #[inline(always)]
    fn from(variant: Timectrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMECTRL` reader - Time control"]
pub type TimectrlR = crate::BitReader<Timectrl>;
impl TimectrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timectrl {
        match self.bits {
            false => Timectrl::NoTimeControlType,
            true => Timectrl::Atleast1TimeControl,
        }
    }
    #[doc = "NO_TIME_CONTROL_TYPE: No time control is enabled"]
    #[inline(always)]
    pub fn is_no_time_control_type(&self) -> bool {
        *self == Timectrl::NoTimeControlType
    }
    #[doc = "ATLEAST1_TIME_CONTROL: at least one time-control type is supported"]
    #[inline(always)]
    pub fn is_atleast1_time_control(&self) -> bool {
        *self == Timectrl::Atleast1TimeControl
    }
}
#[doc = "External FIFO\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extfifo {
    #[doc = "1: STD_EXT_FIFO: standard available/free external FIFO"]
    StdExtFifo = 1,
}
impl From<Extfifo> for u8 {
    #[inline(always)]
    fn from(variant: Extfifo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extfifo {
    type Ux = u8;
}
impl crate::IsEnum for Extfifo {}
#[doc = "Field `EXTFIFO` reader - External FIFO"]
pub type ExtfifoR = crate::FieldReader<Extfifo>;
impl ExtfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extfifo> {
        match self.bits {
            1 => Some(Extfifo::StdExtFifo),
            _ => None,
        }
    }
    #[doc = "STD_EXT_FIFO: standard available/free external FIFO"]
    #[inline(always)]
    pub fn is_std_ext_fifo(&self) -> bool {
        *self == Extfifo::StdExtFifo
    }
}
#[doc = "FIFO transmit\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fifotx {
    #[doc = "0: FIFO_2BYTE: 2-byte TX FIFO, the default FIFO transmit value (FIFOTX)"]
    Fifo2byte = 0,
    #[doc = "1: FIFO_4BYTE: 4-byte TX FIFO"]
    Fifo4byte = 1,
    #[doc = "2: FIFO_8BYTE: 8-byte TX FIFO"]
    Fifo8byte = 2,
    #[doc = "3: FIFO_16BYTE: 16-byte TX FIFO"]
    Fifo16byte = 3,
}
impl From<Fifotx> for u8 {
    #[inline(always)]
    fn from(variant: Fifotx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fifotx {
    type Ux = u8;
}
impl crate::IsEnum for Fifotx {}
#[doc = "Field `FIFOTX` reader - FIFO transmit"]
pub type FifotxR = crate::FieldReader<Fifotx>;
impl FifotxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifotx {
        match self.bits {
            0 => Fifotx::Fifo2byte,
            1 => Fifotx::Fifo4byte,
            2 => Fifotx::Fifo8byte,
            3 => Fifotx::Fifo16byte,
            _ => unreachable!(),
        }
    }
    #[doc = "FIFO_2BYTE: 2-byte TX FIFO, the default FIFO transmit value (FIFOTX)"]
    #[inline(always)]
    pub fn is_fifo_2byte(&self) -> bool {
        *self == Fifotx::Fifo2byte
    }
    #[doc = "FIFO_4BYTE: 4-byte TX FIFO"]
    #[inline(always)]
    pub fn is_fifo_4byte(&self) -> bool {
        *self == Fifotx::Fifo4byte
    }
    #[doc = "FIFO_8BYTE: 8-byte TX FIFO"]
    #[inline(always)]
    pub fn is_fifo_8byte(&self) -> bool {
        *self == Fifotx::Fifo8byte
    }
    #[doc = "FIFO_16BYTE: 16-byte TX FIFO"]
    #[inline(always)]
    pub fn is_fifo_16byte(&self) -> bool {
        *self == Fifotx::Fifo16byte
    }
}
#[doc = "FIFO receive\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fiforx {
    #[doc = "0: FIFO_2BYTE: 2 (or 3)-byte RX FIFO, the default FIFO receive value (FIFORX)"]
    Fifo2byte = 0,
    #[doc = "1: FIFO_4BYTE: 4-byte RX FIFO"]
    Fifo4byte = 1,
    #[doc = "2: FIFO_8BYTE: 8-byte RX FIFO"]
    Fifo8byte = 2,
    #[doc = "3: FIFO_16BYTE: 16-byte RX FIFO"]
    Fifo16byte = 3,
}
impl From<Fiforx> for u8 {
    #[inline(always)]
    fn from(variant: Fiforx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fiforx {
    type Ux = u8;
}
impl crate::IsEnum for Fiforx {}
#[doc = "Field `FIFORX` reader - FIFO receive"]
pub type FiforxR = crate::FieldReader<Fiforx>;
impl FiforxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fiforx {
        match self.bits {
            0 => Fiforx::Fifo2byte,
            1 => Fiforx::Fifo4byte,
            2 => Fiforx::Fifo8byte,
            3 => Fiforx::Fifo16byte,
            _ => unreachable!(),
        }
    }
    #[doc = "FIFO_2BYTE: 2 (or 3)-byte RX FIFO, the default FIFO receive value (FIFORX)"]
    #[inline(always)]
    pub fn is_fifo_2byte(&self) -> bool {
        *self == Fiforx::Fifo2byte
    }
    #[doc = "FIFO_4BYTE: 4-byte RX FIFO"]
    #[inline(always)]
    pub fn is_fifo_4byte(&self) -> bool {
        *self == Fiforx::Fifo4byte
    }
    #[doc = "FIFO_8BYTE: 8-byte RX FIFO"]
    #[inline(always)]
    pub fn is_fifo_8byte(&self) -> bool {
        *self == Fiforx::Fifo8byte
    }
    #[doc = "FIFO_16BYTE: 16-byte RX FIFO"]
    #[inline(always)]
    pub fn is_fifo_16byte(&self) -> bool {
        *self == Fiforx::Fifo16byte
    }
}
#[doc = "INT\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int {
    #[doc = "0: Interrupts are not supported"]
    Interruptsno = 0,
    #[doc = "1: Interrupts are supported"]
    Interruptsyes = 1,
}
impl From<Int> for bool {
    #[inline(always)]
    fn from(variant: Int) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT` reader - INT"]
pub type IntR = crate::BitReader<Int>;
impl IntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int {
        match self.bits {
            false => Int::Interruptsno,
            true => Int::Interruptsyes,
        }
    }
    #[doc = "Interrupts are not supported"]
    #[inline(always)]
    pub fn is_interruptsno(&self) -> bool {
        *self == Int::Interruptsno
    }
    #[doc = "Interrupts are supported"]
    #[inline(always)]
    pub fn is_interruptsyes(&self) -> bool {
        *self == Int::Interruptsyes
    }
}
#[doc = "DMA\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "0: DMA is not supported"]
    Dmano = 0,
    #[doc = "1: DMA is supported"]
    Dmayes = 1,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            false => Dma::Dmano,
            true => Dma::Dmayes,
        }
    }
    #[doc = "DMA is not supported"]
    #[inline(always)]
    pub fn is_dmano(&self) -> bool {
        *self == Dma::Dmano
    }
    #[doc = "DMA is supported"]
    #[inline(always)]
    pub fn is_dmayes(&self) -> bool {
        *self == Dma::Dmayes
    }
}
impl R {
    #[doc = "Bits 0:1 - ID 48b handler"]
    #[inline(always)]
    pub fn idena(&self) -> IdenaR {
        IdenaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - ID register"]
    #[inline(always)]
    pub fn idreg(&self) -> IdregR {
        IdregR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:8 - HDR support"]
    #[inline(always)]
    pub fn hdrsupp(&self) -> HdrsuppR {
        HdrsuppR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Master"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Static address"]
    #[inline(always)]
    pub fn saddr(&self) -> SaddrR {
        SaddrR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Common Command Codes (CCC) handling"]
    #[inline(always)]
    pub fn ccchandle(&self) -> CcchandleR {
        CcchandleR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - In-Band Interrupts, Master Requests, Hot Join events"]
    #[inline(always)]
    pub fn ibi_mr_hj(&self) -> IbiMrHjR {
        IbiMrHjR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Time control"]
    #[inline(always)]
    pub fn timectrl(&self) -> TimectrlR {
        TimectrlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 23:25 - External FIFO"]
    #[inline(always)]
    pub fn extfifo(&self) -> ExtfifoR {
        ExtfifoR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:27 - FIFO transmit"]
    #[inline(always)]
    pub fn fifotx(&self) -> FifotxR {
        FifotxR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - FIFO receive"]
    #[inline(always)]
    pub fn fiforx(&self) -> FiforxR {
        FiforxR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - INT"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCAPABILITIES")
            .field("idena", &self.idena())
            .field("idreg", &self.idreg())
            .field("hdrsupp", &self.hdrsupp())
            .field("master", &self.master())
            .field("saddr", &self.saddr())
            .field("ccchandle", &self.ccchandle())
            .field("ibi_mr_hj", &self.ibi_mr_hj())
            .field("timectrl", &self.timectrl())
            .field("extfifo", &self.extfifo())
            .field("fifotx", &self.fifotx())
            .field("fiforx", &self.fiforx())
            .field("int", &self.int())
            .field("dma", &self.dma())
            .finish()
    }
}
#[doc = "Slave Capabilities Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scapabilities::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScapabilitiesSpec;
impl crate::RegisterSpec for ScapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scapabilities::R`](R) reader structure"]
impl crate::Readable for ScapabilitiesSpec {}
#[doc = "`reset()` method sets SCAPABILITIES to value 0xe83f_fe78"]
impl crate::Resettable for ScapabilitiesSpec {
    const RESET_VALUE: u32 = 0xe83f_fe78;
}
