#[doc = "Register `VEND_SPEC` reader"]
pub type R = crate::R<VendSpecSpec>;
#[doc = "Register `VEND_SPEC` writer"]
pub type W = crate::W<VendSpecSpec>;
#[doc = "Voltage selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vselect {
    #[doc = "0: Change the voltage to high voltage range, around 3.0 V"]
    Vselect0 = 0,
    #[doc = "1: Change the voltage to low voltage range, around 1.8 V"]
    Vselect1 = 1,
}
impl From<Vselect> for bool {
    #[inline(always)]
    fn from(variant: Vselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSELECT` reader - Voltage selection"]
pub type VselectR = crate::BitReader<Vselect>;
impl VselectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vselect {
        match self.bits {
            false => Vselect::Vselect0,
            true => Vselect::Vselect1,
        }
    }
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    #[inline(always)]
    pub fn is_vselect_0(&self) -> bool {
        *self == Vselect::Vselect0
    }
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    #[inline(always)]
    pub fn is_vselect_1(&self) -> bool {
        *self == Vselect::Vselect1
    }
}
#[doc = "Field `VSELECT` writer - Voltage selection"]
pub type VselectW<'a, REG> = crate::BitWriter<'a, REG, Vselect>;
impl<'a, REG> VselectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    #[inline(always)]
    pub fn vselect_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vselect::Vselect0)
    }
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    #[inline(always)]
    pub fn vselect_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vselect::Vselect1)
    }
}
#[doc = "Check busy enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ac12WrChkbusyEn {
    #[doc = "0: Do not check busy after auto CMD12 for write data packet"]
    Ac12WrChkbusyEn0 = 0,
    #[doc = "1: Check busy after auto CMD12 for write data packet"]
    Ac12WrChkbusyEn1 = 1,
}
impl From<Ac12WrChkbusyEn> for bool {
    #[inline(always)]
    fn from(variant: Ac12WrChkbusyEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AC12_WR_CHKBUSY_EN` reader - Check busy enable"]
pub type Ac12WrChkbusyEnR = crate::BitReader<Ac12WrChkbusyEn>;
impl Ac12WrChkbusyEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ac12WrChkbusyEn {
        match self.bits {
            false => Ac12WrChkbusyEn::Ac12WrChkbusyEn0,
            true => Ac12WrChkbusyEn::Ac12WrChkbusyEn1,
        }
    }
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    #[inline(always)]
    pub fn is_ac12_wr_chkbusy_en_0(&self) -> bool {
        *self == Ac12WrChkbusyEn::Ac12WrChkbusyEn0
    }
    #[doc = "Check busy after auto CMD12 for write data packet"]
    #[inline(always)]
    pub fn is_ac12_wr_chkbusy_en_1(&self) -> bool {
        *self == Ac12WrChkbusyEn::Ac12WrChkbusyEn1
    }
}
#[doc = "Field `AC12_WR_CHKBUSY_EN` writer - Check busy enable"]
pub type Ac12WrChkbusyEnW<'a, REG> = crate::BitWriter<'a, REG, Ac12WrChkbusyEn>;
impl<'a, REG> Ac12WrChkbusyEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    #[inline(always)]
    pub fn ac12_wr_chkbusy_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ac12WrChkbusyEn::Ac12WrChkbusyEn0)
    }
    #[doc = "Check busy after auto CMD12 for write data packet"]
    #[inline(always)]
    pub fn ac12_wr_chkbusy_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ac12WrChkbusyEn::Ac12WrChkbusyEn1)
    }
}
#[doc = "Force CLK\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrcSdclkOn {
    #[doc = "0: CLK active or inactive is fully controlled by the hardware."]
    FrcSdclkOn0 = 0,
    #[doc = "1: Force CLK active."]
    FrcSdclkOn1 = 1,
}
impl From<FrcSdclkOn> for bool {
    #[inline(always)]
    fn from(variant: FrcSdclkOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRC_SDCLK_ON` reader - Force CLK"]
pub type FrcSdclkOnR = crate::BitReader<FrcSdclkOn>;
impl FrcSdclkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrcSdclkOn {
        match self.bits {
            false => FrcSdclkOn::FrcSdclkOn0,
            true => FrcSdclkOn::FrcSdclkOn1,
        }
    }
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    #[inline(always)]
    pub fn is_frc_sdclk_on_0(&self) -> bool {
        *self == FrcSdclkOn::FrcSdclkOn0
    }
    #[doc = "Force CLK active."]
    #[inline(always)]
    pub fn is_frc_sdclk_on_1(&self) -> bool {
        *self == FrcSdclkOn::FrcSdclkOn1
    }
}
#[doc = "Field `FRC_SDCLK_ON` writer - Force CLK"]
pub type FrcSdclkOnW<'a, REG> = crate::BitWriter<'a, REG, FrcSdclkOn>;
impl<'a, REG> FrcSdclkOnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    #[inline(always)]
    pub fn frc_sdclk_on_0(self) -> &'a mut crate::W<REG> {
        self.variant(FrcSdclkOn::FrcSdclkOn0)
    }
    #[doc = "Force CLK active."]
    #[inline(always)]
    pub fn frc_sdclk_on_1(self) -> &'a mut crate::W<REG> {
        self.variant(FrcSdclkOn::FrcSdclkOn1)
    }
}
#[doc = "CRC Check Disable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcChkDis {
    #[doc = "0: Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    CrcChkDis0 = 0,
    #[doc = "1: Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    CrcChkDis1 = 1,
}
impl From<CrcChkDis> for bool {
    #[inline(always)]
    fn from(variant: CrcChkDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_CHK_DIS` reader - CRC Check Disable"]
pub type CrcChkDisR = crate::BitReader<CrcChkDis>;
impl CrcChkDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcChkDis {
        match self.bits {
            false => CrcChkDis::CrcChkDis0,
            true => CrcChkDis::CrcChkDis1,
        }
    }
    #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    #[inline(always)]
    pub fn is_crc_chk_dis_0(&self) -> bool {
        *self == CrcChkDis::CrcChkDis0
    }
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    #[inline(always)]
    pub fn is_crc_chk_dis_1(&self) -> bool {
        *self == CrcChkDis::CrcChkDis1
    }
}
#[doc = "Field `CRC_CHK_DIS` writer - CRC Check Disable"]
pub type CrcChkDisW<'a, REG> = crate::BitWriter<'a, REG, CrcChkDis>;
impl<'a, REG> CrcChkDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    #[inline(always)]
    pub fn crc_chk_dis_0(self) -> &'a mut crate::W<REG> {
        self.variant(CrcChkDis::CrcChkDis0)
    }
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    #[inline(always)]
    pub fn crc_chk_dis_1(self) -> &'a mut crate::W<REG> {
        self.variant(CrcChkDis::CrcChkDis1)
    }
}
#[doc = "Byte access\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdByteEn {
    #[doc = "0: Disable"]
    CmdByteEn0 = 0,
    #[doc = "1: Enable"]
    CmdByteEn1 = 1,
}
impl From<CmdByteEn> for bool {
    #[inline(always)]
    fn from(variant: CmdByteEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_BYTE_EN` reader - Byte access"]
pub type CmdByteEnR = crate::BitReader<CmdByteEn>;
impl CmdByteEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdByteEn {
        match self.bits {
            false => CmdByteEn::CmdByteEn0,
            true => CmdByteEn::CmdByteEn1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_cmd_byte_en_0(&self) -> bool {
        *self == CmdByteEn::CmdByteEn0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_cmd_byte_en_1(&self) -> bool {
        *self == CmdByteEn::CmdByteEn1
    }
}
#[doc = "Field `CMD_BYTE_EN` writer - Byte access"]
pub type CmdByteEnW<'a, REG> = crate::BitWriter<'a, REG, CmdByteEn>;
impl<'a, REG> CmdByteEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn cmd_byte_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(CmdByteEn::CmdByteEn0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn cmd_byte_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdByteEn::CmdByteEn1)
    }
}
impl R {
    #[doc = "Bit 1 - Voltage selection"]
    #[inline(always)]
    pub fn vselect(&self) -> VselectR {
        VselectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Check busy enable"]
    #[inline(always)]
    pub fn ac12_wr_chkbusy_en(&self) -> Ac12WrChkbusyEnR {
        Ac12WrChkbusyEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Force CLK"]
    #[inline(always)]
    pub fn frc_sdclk_on(&self) -> FrcSdclkOnR {
        FrcSdclkOnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - CRC Check Disable"]
    #[inline(always)]
    pub fn crc_chk_dis(&self) -> CrcChkDisR {
        CrcChkDisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Byte access"]
    #[inline(always)]
    pub fn cmd_byte_en(&self) -> CmdByteEnR {
        CmdByteEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VEND_SPEC")
            .field("vselect", &self.vselect())
            .field("ac12_wr_chkbusy_en", &self.ac12_wr_chkbusy_en())
            .field("frc_sdclk_on", &self.frc_sdclk_on())
            .field("crc_chk_dis", &self.crc_chk_dis())
            .field("cmd_byte_en", &self.cmd_byte_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Voltage selection"]
    #[inline(always)]
    #[must_use]
    pub fn vselect(&mut self) -> VselectW<VendSpecSpec> {
        VselectW::new(self, 1)
    }
    #[doc = "Bit 3 - Check busy enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac12_wr_chkbusy_en(&mut self) -> Ac12WrChkbusyEnW<VendSpecSpec> {
        Ac12WrChkbusyEnW::new(self, 3)
    }
    #[doc = "Bit 8 - Force CLK"]
    #[inline(always)]
    #[must_use]
    pub fn frc_sdclk_on(&mut self) -> FrcSdclkOnW<VendSpecSpec> {
        FrcSdclkOnW::new(self, 8)
    }
    #[doc = "Bit 15 - CRC Check Disable"]
    #[inline(always)]
    #[must_use]
    pub fn crc_chk_dis(&mut self) -> CrcChkDisW<VendSpecSpec> {
        CrcChkDisW::new(self, 15)
    }
    #[doc = "Bit 31 - Byte access"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_byte_en(&mut self) -> CmdByteEnW<VendSpecSpec> {
        CmdByteEnW::new(self, 31)
    }
}
#[doc = "Vendor Specific Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vend_spec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vend_spec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VendSpecSpec;
impl crate::RegisterSpec for VendSpecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vend_spec::R`](R) reader structure"]
impl crate::Readable for VendSpecSpec {}
#[doc = "`write(|w| ..)` method takes [`vend_spec::W`](W) writer structure"]
impl crate::Writable for VendSpecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VEND_SPEC to value 0x2000_7809"]
impl crate::Resettable for VendSpecSpec {
    const RESET_VALUE: u32 = 0x2000_7809;
}
