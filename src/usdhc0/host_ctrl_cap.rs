#[doc = "Register `HOST_CTRL_CAP` reader"]
pub type R = crate::R<HostCtrlCapSpec>;
#[doc = "Register `HOST_CTRL_CAP` writer"]
pub type W = crate::W<HostCtrlCapSpec>;
#[doc = "Field `SDR50_SUPPORT` reader - SDR50 support"]
pub type Sdr50SupportR = crate::BitReader;
#[doc = "Field `SDR104_SUPPORT` reader - SDR104 support"]
pub type Sdr104SupportR = crate::BitReader;
#[doc = "Field `DDR50_SUPPORT` reader - DDR50 support"]
pub type Ddr50SupportR = crate::BitReader;
#[doc = "Field `TIME_COUNT_RETUNING` reader - Time Counter for Retuning"]
pub type TimeCountRetuningR = crate::FieldReader;
#[doc = "Field `TIME_COUNT_RETUNING` writer - Time Counter for Retuning"]
pub type TimeCountRetuningW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Use Tuning for SDR50\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UseTuningSdr50 {
    #[doc = "0: SDR does not require tuning"]
    UseTuningSdr50_0 = 0,
    #[doc = "1: SDR50 requires tuning"]
    UseTuningSdr50_1 = 1,
}
impl From<UseTuningSdr50> for bool {
    #[inline(always)]
    fn from(variant: UseTuningSdr50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USE_TUNING_SDR50` reader - Use Tuning for SDR50"]
pub type UseTuningSdr50R = crate::BitReader<UseTuningSdr50>;
impl UseTuningSdr50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UseTuningSdr50 {
        match self.bits {
            false => UseTuningSdr50::UseTuningSdr50_0,
            true => UseTuningSdr50::UseTuningSdr50_1,
        }
    }
    #[doc = "SDR does not require tuning"]
    #[inline(always)]
    pub fn is_use_tuning_sdr50_0(&self) -> bool {
        *self == UseTuningSdr50::UseTuningSdr50_0
    }
    #[doc = "SDR50 requires tuning"]
    #[inline(always)]
    pub fn is_use_tuning_sdr50_1(&self) -> bool {
        *self == UseTuningSdr50::UseTuningSdr50_1
    }
}
#[doc = "Field `USE_TUNING_SDR50` writer - Use Tuning for SDR50"]
pub type UseTuningSdr50W<'a, REG> = crate::BitWriter<'a, REG, UseTuningSdr50>;
impl<'a, REG> UseTuningSdr50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDR does not require tuning"]
    #[inline(always)]
    pub fn use_tuning_sdr50_0(self) -> &'a mut crate::W<REG> {
        self.variant(UseTuningSdr50::UseTuningSdr50_0)
    }
    #[doc = "SDR50 requires tuning"]
    #[inline(always)]
    pub fn use_tuning_sdr50_1(self) -> &'a mut crate::W<REG> {
        self.variant(UseTuningSdr50::UseTuningSdr50_1)
    }
}
#[doc = "Retuning Mode\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RetuningMode {
    #[doc = "0: Mode 1"]
    RetuningMode0 = 0,
    #[doc = "1: Mode 2"]
    RetuningMode1 = 1,
    #[doc = "2: Mode 3"]
    RetuningMode2 = 2,
}
impl From<RetuningMode> for u8 {
    #[inline(always)]
    fn from(variant: RetuningMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RetuningMode {
    type Ux = u8;
}
impl crate::IsEnum for RetuningMode {}
#[doc = "Field `RETUNING_MODE` reader - Retuning Mode"]
pub type RetuningModeR = crate::FieldReader<RetuningMode>;
impl RetuningModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RetuningMode> {
        match self.bits {
            0 => Some(RetuningMode::RetuningMode0),
            1 => Some(RetuningMode::RetuningMode1),
            2 => Some(RetuningMode::RetuningMode2),
            _ => None,
        }
    }
    #[doc = "Mode 1"]
    #[inline(always)]
    pub fn is_retuning_mode_0(&self) -> bool {
        *self == RetuningMode::RetuningMode0
    }
    #[doc = "Mode 2"]
    #[inline(always)]
    pub fn is_retuning_mode_1(&self) -> bool {
        *self == RetuningMode::RetuningMode1
    }
    #[doc = "Mode 3"]
    #[inline(always)]
    pub fn is_retuning_mode_2(&self) -> bool {
        *self == RetuningMode::RetuningMode2
    }
}
#[doc = "Max Block Length\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mbl {
    #[doc = "0: 512 bytes"]
    Mbl0 = 0,
    #[doc = "1: 1024 bytes"]
    Mbl1 = 1,
    #[doc = "2: 2048 bytes"]
    Mbl2 = 2,
    #[doc = "3: 4096 bytes"]
    Mbl3 = 3,
}
impl From<Mbl> for u8 {
    #[inline(always)]
    fn from(variant: Mbl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mbl {
    type Ux = u8;
}
impl crate::IsEnum for Mbl {}
#[doc = "Field `MBL` reader - Max Block Length"]
pub type MblR = crate::FieldReader<Mbl>;
impl MblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mbl> {
        match self.bits {
            0 => Some(Mbl::Mbl0),
            1 => Some(Mbl::Mbl1),
            2 => Some(Mbl::Mbl2),
            3 => Some(Mbl::Mbl3),
            _ => None,
        }
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_mbl_0(&self) -> bool {
        *self == Mbl::Mbl0
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_mbl_1(&self) -> bool {
        *self == Mbl::Mbl1
    }
    #[doc = "2048 bytes"]
    #[inline(always)]
    pub fn is_mbl_2(&self) -> bool {
        *self == Mbl::Mbl2
    }
    #[doc = "4096 bytes"]
    #[inline(always)]
    pub fn is_mbl_3(&self) -> bool {
        *self == Mbl::Mbl3
    }
}
#[doc = "ADMA Support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admas {
    #[doc = "0: Advanced DMA Not supported"]
    Admas0 = 0,
    #[doc = "1: Advanced DMA Supported"]
    Admas1 = 1,
}
impl From<Admas> for bool {
    #[inline(always)]
    fn from(variant: Admas) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMAS` reader - ADMA Support"]
pub type AdmasR = crate::BitReader<Admas>;
impl AdmasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admas {
        match self.bits {
            false => Admas::Admas0,
            true => Admas::Admas1,
        }
    }
    #[doc = "Advanced DMA Not supported"]
    #[inline(always)]
    pub fn is_admas_0(&self) -> bool {
        *self == Admas::Admas0
    }
    #[doc = "Advanced DMA Supported"]
    #[inline(always)]
    pub fn is_admas_1(&self) -> bool {
        *self == Admas::Admas1
    }
}
#[doc = "High Speed Support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hss {
    #[doc = "0: High Speed Not Supported"]
    Hss0 = 0,
    #[doc = "1: High Speed Supported"]
    Hss1 = 1,
}
impl From<Hss> for bool {
    #[inline(always)]
    fn from(variant: Hss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSS` reader - High Speed Support"]
pub type HssR = crate::BitReader<Hss>;
impl HssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hss {
        match self.bits {
            false => Hss::Hss0,
            true => Hss::Hss1,
        }
    }
    #[doc = "High Speed Not Supported"]
    #[inline(always)]
    pub fn is_hss_0(&self) -> bool {
        *self == Hss::Hss0
    }
    #[doc = "High Speed Supported"]
    #[inline(always)]
    pub fn is_hss_1(&self) -> bool {
        *self == Hss::Hss1
    }
}
#[doc = "DMA Support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmas {
    #[doc = "0: DMA not supported"]
    Dmas0 = 0,
    #[doc = "1: DMA Supported"]
    Dmas1 = 1,
}
impl From<Dmas> for bool {
    #[inline(always)]
    fn from(variant: Dmas) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAS` reader - DMA Support"]
pub type DmasR = crate::BitReader<Dmas>;
impl DmasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmas {
        match self.bits {
            false => Dmas::Dmas0,
            true => Dmas::Dmas1,
        }
    }
    #[doc = "DMA not supported"]
    #[inline(always)]
    pub fn is_dmas_0(&self) -> bool {
        *self == Dmas::Dmas0
    }
    #[doc = "DMA Supported"]
    #[inline(always)]
    pub fn is_dmas_1(&self) -> bool {
        *self == Dmas::Dmas1
    }
}
#[doc = "Suspend / Resume Support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srs {
    #[doc = "0: Not supported"]
    Srs0 = 0,
    #[doc = "1: Supported"]
    Srs1 = 1,
}
impl From<Srs> for bool {
    #[inline(always)]
    fn from(variant: Srs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRS` reader - Suspend / Resume Support"]
pub type SrsR = crate::BitReader<Srs>;
impl SrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srs {
        match self.bits {
            false => Srs::Srs0,
            true => Srs::Srs1,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_srs_0(&self) -> bool {
        *self == Srs::Srs0
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_srs_1(&self) -> bool {
        *self == Srs::Srs1
    }
}
#[doc = "Voltage Support 3.3V\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vs33 {
    #[doc = "0: 3.3V not supported"]
    Vs33_0 = 0,
    #[doc = "1: 3.3V supported"]
    Vs33_1 = 1,
}
impl From<Vs33> for bool {
    #[inline(always)]
    fn from(variant: Vs33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VS33` reader - Voltage Support 3.3V"]
pub type Vs33R = crate::BitReader<Vs33>;
impl Vs33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vs33 {
        match self.bits {
            false => Vs33::Vs33_0,
            true => Vs33::Vs33_1,
        }
    }
    #[doc = "3.3V not supported"]
    #[inline(always)]
    pub fn is_vs33_0(&self) -> bool {
        *self == Vs33::Vs33_0
    }
    #[doc = "3.3V supported"]
    #[inline(always)]
    pub fn is_vs33_1(&self) -> bool {
        *self == Vs33::Vs33_1
    }
}
#[doc = "Voltage Support 3.0 V\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vs30 {
    #[doc = "0: 3.0V not supported"]
    Vs30_0 = 0,
    #[doc = "1: 3.0V supported"]
    Vs30_1 = 1,
}
impl From<Vs30> for bool {
    #[inline(always)]
    fn from(variant: Vs30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VS30` reader - Voltage Support 3.0 V"]
pub type Vs30R = crate::BitReader<Vs30>;
impl Vs30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vs30 {
        match self.bits {
            false => Vs30::Vs30_0,
            true => Vs30::Vs30_1,
        }
    }
    #[doc = "3.0V not supported"]
    #[inline(always)]
    pub fn is_vs30_0(&self) -> bool {
        *self == Vs30::Vs30_0
    }
    #[doc = "3.0V supported"]
    #[inline(always)]
    pub fn is_vs30_1(&self) -> bool {
        *self == Vs30::Vs30_1
    }
}
#[doc = "Voltage Support 1.8 V\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vs18 {
    #[doc = "0: 1.8V not supported"]
    Vs18_0 = 0,
    #[doc = "1: 1.8V supported"]
    Vs18_1 = 1,
}
impl From<Vs18> for bool {
    #[inline(always)]
    fn from(variant: Vs18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VS18` reader - Voltage Support 1.8 V"]
pub type Vs18R = crate::BitReader<Vs18>;
impl Vs18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vs18 {
        match self.bits {
            false => Vs18::Vs18_0,
            true => Vs18::Vs18_1,
        }
    }
    #[doc = "1.8V not supported"]
    #[inline(always)]
    pub fn is_vs18_0(&self) -> bool {
        *self == Vs18::Vs18_0
    }
    #[doc = "1.8V supported"]
    #[inline(always)]
    pub fn is_vs18_1(&self) -> bool {
        *self == Vs18::Vs18_1
    }
}
impl R {
    #[doc = "Bit 0 - SDR50 support"]
    #[inline(always)]
    pub fn sdr50_support(&self) -> Sdr50SupportR {
        Sdr50SupportR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 support"]
    #[inline(always)]
    pub fn sdr104_support(&self) -> Sdr104SupportR {
        Sdr104SupportR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 support"]
    #[inline(always)]
    pub fn ddr50_support(&self) -> Ddr50SupportR {
        Ddr50SupportR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Time Counter for Retuning"]
    #[inline(always)]
    pub fn time_count_retuning(&self) -> TimeCountRetuningR {
        TimeCountRetuningR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn use_tuning_sdr50(&self) -> UseTuningSdr50R {
        UseTuningSdr50R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Retuning Mode"]
    #[inline(always)]
    pub fn retuning_mode(&self) -> RetuningModeR {
        RetuningModeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Max Block Length"]
    #[inline(always)]
    pub fn mbl(&self) -> MblR {
        MblR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - ADMA Support"]
    #[inline(always)]
    pub fn admas(&self) -> AdmasR {
        AdmasR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn hss(&self) -> HssR {
        HssR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA Support"]
    #[inline(always)]
    pub fn dmas(&self) -> DmasR {
        DmasR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend / Resume Support"]
    #[inline(always)]
    pub fn srs(&self) -> SrsR {
        SrsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline(always)]
    pub fn vs33(&self) -> Vs33R {
        Vs33R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0 V"]
    #[inline(always)]
    pub fn vs30(&self) -> Vs30R {
        Vs30R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8 V"]
    #[inline(always)]
    pub fn vs18(&self) -> Vs18R {
        Vs18R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_CTRL_CAP")
            .field("sdr50_support", &self.sdr50_support())
            .field("sdr104_support", &self.sdr104_support())
            .field("ddr50_support", &self.ddr50_support())
            .field("time_count_retuning", &self.time_count_retuning())
            .field("use_tuning_sdr50", &self.use_tuning_sdr50())
            .field("retuning_mode", &self.retuning_mode())
            .field("mbl", &self.mbl())
            .field("admas", &self.admas())
            .field("hss", &self.hss())
            .field("dmas", &self.dmas())
            .field("srs", &self.srs())
            .field("vs33", &self.vs33())
            .field("vs30", &self.vs30())
            .field("vs18", &self.vs18())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:11 - Time Counter for Retuning"]
    #[inline(always)]
    pub fn time_count_retuning(&mut self) -> TimeCountRetuningW<HostCtrlCapSpec> {
        TimeCountRetuningW::new(self, 8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn use_tuning_sdr50(&mut self) -> UseTuningSdr50W<HostCtrlCapSpec> {
        UseTuningSdr50W::new(self, 13)
    }
}
#[doc = "Host Controller Capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctrl_cap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctrl_cap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCtrlCapSpec;
impl crate::RegisterSpec for HostCtrlCapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ctrl_cap::R`](R) reader structure"]
impl crate::Readable for HostCtrlCapSpec {}
#[doc = "`write(|w| ..)` method takes [`host_ctrl_cap::W`](W) writer structure"]
impl crate::Writable for HostCtrlCapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_CTRL_CAP to value 0x07f3_b407"]
impl crate::Resettable for HostCtrlCapSpec {
    const RESET_VALUE: u32 = 0x07f3_b407;
}
