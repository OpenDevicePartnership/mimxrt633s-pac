#[doc = "Register `MMC_BOOT` reader"]
pub type R = crate::R<MmcBootSpec>;
#[doc = "Register `MMC_BOOT` writer"]
pub type W = crate::W<MmcBootSpec>;
#[doc = "DTOCV_ACK\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DtocvAck {
    #[doc = "0: SDCLK x 2^14"]
    DtocvAck0 = 0,
    #[doc = "1: SDCLK x 2^15"]
    DtocvAck1 = 1,
    #[doc = "2: SDCLK x 2^16"]
    DtocvAck2 = 2,
    #[doc = "3: SDCLK x 2^17"]
    DtocvAck3 = 3,
    #[doc = "4: SDCLK x 2^18"]
    DtocvAck4 = 4,
    #[doc = "5: SDCLK x 2^19"]
    DtocvAck5 = 5,
    #[doc = "6: SDCLK x 2^20"]
    DtocvAck6 = 6,
    #[doc = "7: SDCLK x 2^21"]
    DtocvAck7 = 7,
    #[doc = "14: SDCLK x 2^28"]
    DtocvAck14 = 14,
    #[doc = "15: SDCLK x 2^29"]
    DtocvAck15 = 15,
}
impl From<DtocvAck> for u8 {
    #[inline(always)]
    fn from(variant: DtocvAck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DtocvAck {
    type Ux = u8;
}
impl crate::IsEnum for DtocvAck {}
#[doc = "Field `DTOCV_ACK` reader - DTOCV_ACK"]
pub type DtocvAckR = crate::FieldReader<DtocvAck>;
impl DtocvAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DtocvAck> {
        match self.bits {
            0 => Some(DtocvAck::DtocvAck0),
            1 => Some(DtocvAck::DtocvAck1),
            2 => Some(DtocvAck::DtocvAck2),
            3 => Some(DtocvAck::DtocvAck3),
            4 => Some(DtocvAck::DtocvAck4),
            5 => Some(DtocvAck::DtocvAck5),
            6 => Some(DtocvAck::DtocvAck6),
            7 => Some(DtocvAck::DtocvAck7),
            14 => Some(DtocvAck::DtocvAck14),
            15 => Some(DtocvAck::DtocvAck15),
            _ => None,
        }
    }
    #[doc = "SDCLK x 2^14"]
    #[inline(always)]
    pub fn is_dtocv_ack_0(&self) -> bool {
        *self == DtocvAck::DtocvAck0
    }
    #[doc = "SDCLK x 2^15"]
    #[inline(always)]
    pub fn is_dtocv_ack_1(&self) -> bool {
        *self == DtocvAck::DtocvAck1
    }
    #[doc = "SDCLK x 2^16"]
    #[inline(always)]
    pub fn is_dtocv_ack_2(&self) -> bool {
        *self == DtocvAck::DtocvAck2
    }
    #[doc = "SDCLK x 2^17"]
    #[inline(always)]
    pub fn is_dtocv_ack_3(&self) -> bool {
        *self == DtocvAck::DtocvAck3
    }
    #[doc = "SDCLK x 2^18"]
    #[inline(always)]
    pub fn is_dtocv_ack_4(&self) -> bool {
        *self == DtocvAck::DtocvAck4
    }
    #[doc = "SDCLK x 2^19"]
    #[inline(always)]
    pub fn is_dtocv_ack_5(&self) -> bool {
        *self == DtocvAck::DtocvAck5
    }
    #[doc = "SDCLK x 2^20"]
    #[inline(always)]
    pub fn is_dtocv_ack_6(&self) -> bool {
        *self == DtocvAck::DtocvAck6
    }
    #[doc = "SDCLK x 2^21"]
    #[inline(always)]
    pub fn is_dtocv_ack_7(&self) -> bool {
        *self == DtocvAck::DtocvAck7
    }
    #[doc = "SDCLK x 2^28"]
    #[inline(always)]
    pub fn is_dtocv_ack_14(&self) -> bool {
        *self == DtocvAck::DtocvAck14
    }
    #[doc = "SDCLK x 2^29"]
    #[inline(always)]
    pub fn is_dtocv_ack_15(&self) -> bool {
        *self == DtocvAck::DtocvAck15
    }
}
#[doc = "Field `DTOCV_ACK` writer - DTOCV_ACK"]
pub type DtocvAckW<'a, REG> = crate::FieldWriter<'a, REG, 4, DtocvAck>;
impl<'a, REG> DtocvAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDCLK x 2^14"]
    #[inline(always)]
    pub fn dtocv_ack_0(self) -> &'a mut crate::W<REG> {
        self.variant(DtocvAck::DtocvAck0)
    }
    #[doc = "SDCLK x 2^15"]
    #[inline(always)]
    pub fn dtocv_ack_1(self) -> &'a mut crate::W<REG> {
        self.variant(DtocvAck::DtocvAck1)
    }
    #[doc = "SDCLK x 2^16"]
    #[inline(always)]
    pub fn dtocv_ack_2(self) -> &'a mut crate::W<REG> {
        self.variant(DtocvAck::DtocvAck2)
    }
    #[doc = "SDCLK x 2^17"]
    #[inline(always)]
    pub fn dtocv_ack_3(self) -> &'a mut crate::W<REG> {
        self.variant(DtocvAck::DtocvAck3)
    }
    #[doc = "SDCLK x 2^18"]
    #[inline(always)]
    pub fn dtocv_ack_4(self) -> &'a mut crate::W<REG> {
        self.variant(DtocvAck::DtocvAck4)
    }
    #[doc = "SDCLK x 2^19"]
    #[inline(always)]
    pub fn dtocv_ack_5(self) -> &'a mut crate::W<REG> {
        self.variant(DtocvAck::DtocvAck5)
    }
    #[doc = "SDCLK x 2^20"]
    #[inline(always)]
    pub fn dtocv_ack_6(self) -> &'a mut crate::W<REG> {
        self.variant(DtocvAck::DtocvAck6)
    }
    #[doc = "SDCLK x 2^21"]
    #[inline(always)]
    pub fn dtocv_ack_7(self) -> &'a mut crate::W<REG> {
        self.variant(DtocvAck::DtocvAck7)
    }
    #[doc = "SDCLK x 2^28"]
    #[inline(always)]
    pub fn dtocv_ack_14(self) -> &'a mut crate::W<REG> {
        self.variant(DtocvAck::DtocvAck14)
    }
    #[doc = "SDCLK x 2^29"]
    #[inline(always)]
    pub fn dtocv_ack_15(self) -> &'a mut crate::W<REG> {
        self.variant(DtocvAck::DtocvAck15)
    }
}
#[doc = "BOOT_ACK\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootAck {
    #[doc = "0: No ack"]
    BootAck0 = 0,
    #[doc = "1: Ack"]
    BootAck1 = 1,
}
impl From<BootAck> for bool {
    #[inline(always)]
    fn from(variant: BootAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOT_ACK` reader - BOOT_ACK"]
pub type BootAckR = crate::BitReader<BootAck>;
impl BootAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BootAck {
        match self.bits {
            false => BootAck::BootAck0,
            true => BootAck::BootAck1,
        }
    }
    #[doc = "No ack"]
    #[inline(always)]
    pub fn is_boot_ack_0(&self) -> bool {
        *self == BootAck::BootAck0
    }
    #[doc = "Ack"]
    #[inline(always)]
    pub fn is_boot_ack_1(&self) -> bool {
        *self == BootAck::BootAck1
    }
}
#[doc = "Field `BOOT_ACK` writer - BOOT_ACK"]
pub type BootAckW<'a, REG> = crate::BitWriter<'a, REG, BootAck>;
impl<'a, REG> BootAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ack"]
    #[inline(always)]
    pub fn boot_ack_0(self) -> &'a mut crate::W<REG> {
        self.variant(BootAck::BootAck0)
    }
    #[doc = "Ack"]
    #[inline(always)]
    pub fn boot_ack_1(self) -> &'a mut crate::W<REG> {
        self.variant(BootAck::BootAck1)
    }
}
#[doc = "BOOT_MODE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootMode {
    #[doc = "0: Normal boot"]
    BootMode0 = 0,
    #[doc = "1: Alternative boot"]
    BootMode1 = 1,
}
impl From<BootMode> for bool {
    #[inline(always)]
    fn from(variant: BootMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOT_MODE` reader - BOOT_MODE"]
pub type BootModeR = crate::BitReader<BootMode>;
impl BootModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BootMode {
        match self.bits {
            false => BootMode::BootMode0,
            true => BootMode::BootMode1,
        }
    }
    #[doc = "Normal boot"]
    #[inline(always)]
    pub fn is_boot_mode_0(&self) -> bool {
        *self == BootMode::BootMode0
    }
    #[doc = "Alternative boot"]
    #[inline(always)]
    pub fn is_boot_mode_1(&self) -> bool {
        *self == BootMode::BootMode1
    }
}
#[doc = "Field `BOOT_MODE` writer - BOOT_MODE"]
pub type BootModeW<'a, REG> = crate::BitWriter<'a, REG, BootMode>;
impl<'a, REG> BootModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal boot"]
    #[inline(always)]
    pub fn boot_mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(BootMode::BootMode0)
    }
    #[doc = "Alternative boot"]
    #[inline(always)]
    pub fn boot_mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(BootMode::BootMode1)
    }
}
#[doc = "BOOT_EN\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootEn {
    #[doc = "0: Fast boot disable"]
    BootEn0 = 0,
    #[doc = "1: Fast boot enable"]
    BootEn1 = 1,
}
impl From<BootEn> for bool {
    #[inline(always)]
    fn from(variant: BootEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOT_EN` reader - BOOT_EN"]
pub type BootEnR = crate::BitReader<BootEn>;
impl BootEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BootEn {
        match self.bits {
            false => BootEn::BootEn0,
            true => BootEn::BootEn1,
        }
    }
    #[doc = "Fast boot disable"]
    #[inline(always)]
    pub fn is_boot_en_0(&self) -> bool {
        *self == BootEn::BootEn0
    }
    #[doc = "Fast boot enable"]
    #[inline(always)]
    pub fn is_boot_en_1(&self) -> bool {
        *self == BootEn::BootEn1
    }
}
#[doc = "Field `BOOT_EN` writer - BOOT_EN"]
pub type BootEnW<'a, REG> = crate::BitWriter<'a, REG, BootEn>;
impl<'a, REG> BootEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast boot disable"]
    #[inline(always)]
    pub fn boot_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(BootEn::BootEn0)
    }
    #[doc = "Fast boot enable"]
    #[inline(always)]
    pub fn boot_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(BootEn::BootEn1)
    }
}
#[doc = "Field `AUTO_SABG_EN` reader - AUTO_SABG_EN"]
pub type AutoSabgEnR = crate::BitReader;
#[doc = "Field `AUTO_SABG_EN` writer - AUTO_SABG_EN"]
pub type AutoSabgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Disable Time Out\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableTimeOut {
    #[doc = "0: Enable time out"]
    DisableTimeOut0 = 0,
    #[doc = "1: Disable time out"]
    DisableTimeOut1 = 1,
}
impl From<DisableTimeOut> for bool {
    #[inline(always)]
    fn from(variant: DisableTimeOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLE_TIME_OUT` reader - Disable Time Out"]
pub type DisableTimeOutR = crate::BitReader<DisableTimeOut>;
impl DisableTimeOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisableTimeOut {
        match self.bits {
            false => DisableTimeOut::DisableTimeOut0,
            true => DisableTimeOut::DisableTimeOut1,
        }
    }
    #[doc = "Enable time out"]
    #[inline(always)]
    pub fn is_disable_time_out_0(&self) -> bool {
        *self == DisableTimeOut::DisableTimeOut0
    }
    #[doc = "Disable time out"]
    #[inline(always)]
    pub fn is_disable_time_out_1(&self) -> bool {
        *self == DisableTimeOut::DisableTimeOut1
    }
}
#[doc = "Field `DISABLE_TIME_OUT` writer - Disable Time Out"]
pub type DisableTimeOutW<'a, REG> = crate::BitWriter<'a, REG, DisableTimeOut>;
impl<'a, REG> DisableTimeOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable time out"]
    #[inline(always)]
    pub fn disable_time_out_0(self) -> &'a mut crate::W<REG> {
        self.variant(DisableTimeOut::DisableTimeOut0)
    }
    #[doc = "Disable time out"]
    #[inline(always)]
    pub fn disable_time_out_1(self) -> &'a mut crate::W<REG> {
        self.variant(DisableTimeOut::DisableTimeOut1)
    }
}
#[doc = "Field `BOOT_BLK_CNT` reader - BOOT_BLK_CNT"]
pub type BootBlkCntR = crate::FieldReader<u16>;
#[doc = "Field `BOOT_BLK_CNT` writer - BOOT_BLK_CNT"]
pub type BootBlkCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - DTOCV_ACK"]
    #[inline(always)]
    pub fn dtocv_ack(&self) -> DtocvAckR {
        DtocvAckR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - BOOT_ACK"]
    #[inline(always)]
    pub fn boot_ack(&self) -> BootAckR {
        BootAckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BOOT_MODE"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BootModeR {
        BootModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BOOT_EN"]
    #[inline(always)]
    pub fn boot_en(&self) -> BootEnR {
        BootEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AUTO_SABG_EN"]
    #[inline(always)]
    pub fn auto_sabg_en(&self) -> AutoSabgEnR {
        AutoSabgEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable Time Out"]
    #[inline(always)]
    pub fn disable_time_out(&self) -> DisableTimeOutR {
        DisableTimeOutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - BOOT_BLK_CNT"]
    #[inline(always)]
    pub fn boot_blk_cnt(&self) -> BootBlkCntR {
        BootBlkCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_BOOT")
            .field("dtocv_ack", &self.dtocv_ack())
            .field("boot_ack", &self.boot_ack())
            .field("boot_mode", &self.boot_mode())
            .field("boot_en", &self.boot_en())
            .field("auto_sabg_en", &self.auto_sabg_en())
            .field("disable_time_out", &self.disable_time_out())
            .field("boot_blk_cnt", &self.boot_blk_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - DTOCV_ACK"]
    #[inline(always)]
    pub fn dtocv_ack(&mut self) -> DtocvAckW<MmcBootSpec> {
        DtocvAckW::new(self, 0)
    }
    #[doc = "Bit 4 - BOOT_ACK"]
    #[inline(always)]
    pub fn boot_ack(&mut self) -> BootAckW<MmcBootSpec> {
        BootAckW::new(self, 4)
    }
    #[doc = "Bit 5 - BOOT_MODE"]
    #[inline(always)]
    pub fn boot_mode(&mut self) -> BootModeW<MmcBootSpec> {
        BootModeW::new(self, 5)
    }
    #[doc = "Bit 6 - BOOT_EN"]
    #[inline(always)]
    pub fn boot_en(&mut self) -> BootEnW<MmcBootSpec> {
        BootEnW::new(self, 6)
    }
    #[doc = "Bit 7 - AUTO_SABG_EN"]
    #[inline(always)]
    pub fn auto_sabg_en(&mut self) -> AutoSabgEnW<MmcBootSpec> {
        AutoSabgEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Disable Time Out"]
    #[inline(always)]
    pub fn disable_time_out(&mut self) -> DisableTimeOutW<MmcBootSpec> {
        DisableTimeOutW::new(self, 8)
    }
    #[doc = "Bits 16:31 - BOOT_BLK_CNT"]
    #[inline(always)]
    pub fn boot_blk_cnt(&mut self) -> BootBlkCntW<MmcBootSpec> {
        BootBlkCntW::new(self, 16)
    }
}
#[doc = "MMC Boot Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_boot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_boot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcBootSpec;
impl crate::RegisterSpec for MmcBootSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_boot::R`](R) reader structure"]
impl crate::Readable for MmcBootSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_boot::W`](W) writer structure"]
impl crate::Writable for MmcBootSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_BOOT to value 0"]
impl crate::Resettable for MmcBootSpec {
    const RESET_VALUE: u32 = 0;
}
