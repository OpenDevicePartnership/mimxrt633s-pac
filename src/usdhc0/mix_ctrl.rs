#[doc = "Register `MIX_CTRL` reader"]
pub type R = crate::R<MixCtrlSpec>;
#[doc = "Register `MIX_CTRL` writer"]
pub type W = crate::W<MixCtrlSpec>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: Disable"]
    Dmaen0 = 0,
    #[doc = "1: Enable"]
    Dmaen1 = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Dmaen0,
            true => Dmaen::Dmaen1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dmaen_0(&self) -> bool {
        *self == Dmaen::Dmaen0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_dmaen_1(&self) -> bool {
        *self == Dmaen::Dmaen1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmaen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Dmaen0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmaen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Dmaen1)
    }
}
#[doc = "Block Count Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcen {
    #[doc = "0: Disable"]
    Bcen0 = 0,
    #[doc = "1: Enable"]
    Bcen1 = 1,
}
impl From<Bcen> for bool {
    #[inline(always)]
    fn from(variant: Bcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCEN` reader - Block Count Enable"]
pub type BcenR = crate::BitReader<Bcen>;
impl BcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcen {
        match self.bits {
            false => Bcen::Bcen0,
            true => Bcen::Bcen1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_bcen_0(&self) -> bool {
        *self == Bcen::Bcen0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_bcen_1(&self) -> bool {
        *self == Bcen::Bcen1
    }
}
#[doc = "Field `BCEN` writer - Block Count Enable"]
pub type BcenW<'a, REG> = crate::BitWriter<'a, REG, Bcen>;
impl<'a, REG> BcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn bcen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bcen::Bcen0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn bcen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcen::Bcen1)
    }
}
#[doc = "Auto CMD12 Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ac12en {
    #[doc = "0: Disable"]
    Ac12en0 = 0,
    #[doc = "1: Enable"]
    Ac12en1 = 1,
}
impl From<Ac12en> for bool {
    #[inline(always)]
    fn from(variant: Ac12en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AC12EN` reader - Auto CMD12 Enable"]
pub type Ac12enR = crate::BitReader<Ac12en>;
impl Ac12enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ac12en {
        match self.bits {
            false => Ac12en::Ac12en0,
            true => Ac12en::Ac12en1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_ac12en_0(&self) -> bool {
        *self == Ac12en::Ac12en0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_ac12en_1(&self) -> bool {
        *self == Ac12en::Ac12en1
    }
}
#[doc = "Field `AC12EN` writer - Auto CMD12 Enable"]
pub type Ac12enW<'a, REG> = crate::BitWriter<'a, REG, Ac12en>;
impl<'a, REG> Ac12enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn ac12en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ac12en::Ac12en0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn ac12en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ac12en::Ac12en1)
    }
}
#[doc = "Field `DDR_EN` reader - Dual Data Rate mode selection"]
pub type DdrEnR = crate::BitReader;
#[doc = "Field `DDR_EN` writer - Dual Data Rate mode selection"]
pub type DdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data Transfer Direction Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtdsel {
    #[doc = "0: Write (Host to Card)"]
    Dtdsel0 = 0,
    #[doc = "1: Read (Card to Host)"]
    Dtdsel1 = 1,
}
impl From<Dtdsel> for bool {
    #[inline(always)]
    fn from(variant: Dtdsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTDSEL` reader - Data Transfer Direction Select"]
pub type DtdselR = crate::BitReader<Dtdsel>;
impl DtdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtdsel {
        match self.bits {
            false => Dtdsel::Dtdsel0,
            true => Dtdsel::Dtdsel1,
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn is_dtdsel_0(&self) -> bool {
        *self == Dtdsel::Dtdsel0
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn is_dtdsel_1(&self) -> bool {
        *self == Dtdsel::Dtdsel1
    }
}
#[doc = "Field `DTDSEL` writer - Data Transfer Direction Select"]
pub type DtdselW<'a, REG> = crate::BitWriter<'a, REG, Dtdsel>;
impl<'a, REG> DtdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn dtdsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtdsel::Dtdsel0)
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn dtdsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtdsel::Dtdsel1)
    }
}
#[doc = "Multi / Single Block Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbsel {
    #[doc = "0: Single Block"]
    Msbsel0 = 0,
    #[doc = "1: Multiple Blocks"]
    Msbsel1 = 1,
}
impl From<Msbsel> for bool {
    #[inline(always)]
    fn from(variant: Msbsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBSEL` reader - Multi / Single Block Select"]
pub type MsbselR = crate::BitReader<Msbsel>;
impl MsbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbsel {
        match self.bits {
            false => Msbsel::Msbsel0,
            true => Msbsel::Msbsel1,
        }
    }
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn is_msbsel_0(&self) -> bool {
        *self == Msbsel::Msbsel0
    }
    #[doc = "Multiple Blocks"]
    #[inline(always)]
    pub fn is_msbsel_1(&self) -> bool {
        *self == Msbsel::Msbsel1
    }
}
#[doc = "Field `MSBSEL` writer - Multi / Single Block Select"]
pub type MsbselW<'a, REG> = crate::BitWriter<'a, REG, Msbsel>;
impl<'a, REG> MsbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn msbsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Msbsel::Msbsel0)
    }
    #[doc = "Multiple Blocks"]
    #[inline(always)]
    pub fn msbsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Msbsel::Msbsel1)
    }
}
#[doc = "Field `NIBBLE_POS` reader - NIBBLE_POS"]
pub type NibblePosR = crate::BitReader;
#[doc = "Field `NIBBLE_POS` writer - NIBBLE_POS"]
pub type NibblePosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC23EN` reader - Auto CMD23 Enable"]
pub type Ac23enR = crate::BitReader;
#[doc = "Field `AC23EN` writer - Auto CMD23 Enable"]
pub type Ac23enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExeTune {
    #[doc = "0: Not Tuned or Tuning Completed"]
    ExeTune0 = 0,
    #[doc = "1: Execute Tuning"]
    ExeTune1 = 1,
}
impl From<ExeTune> for bool {
    #[inline(always)]
    fn from(variant: ExeTune) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXE_TUNE` reader - Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
pub type ExeTuneR = crate::BitReader<ExeTune>;
impl ExeTuneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ExeTune {
        match self.bits {
            false => ExeTune::ExeTune0,
            true => ExeTune::ExeTune1,
        }
    }
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline(always)]
    pub fn is_exe_tune_0(&self) -> bool {
        *self == ExeTune::ExeTune0
    }
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub fn is_exe_tune_1(&self) -> bool {
        *self == ExeTune::ExeTune1
    }
}
#[doc = "Field `EXE_TUNE` writer - Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
pub type ExeTuneW<'a, REG> = crate::BitWriter<'a, REG, ExeTune>;
impl<'a, REG> ExeTuneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline(always)]
    pub fn exe_tune_0(self) -> &'a mut crate::W<REG> {
        self.variant(ExeTune::ExeTune0)
    }
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub fn exe_tune_1(self) -> &'a mut crate::W<REG> {
        self.variant(ExeTune::ExeTune1)
    }
}
#[doc = "SMP_CLK_SEL\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmpClkSel {
    #[doc = "0: Fixed clock is used to sample data / cmd"]
    SmpClkSel0 = 0,
    #[doc = "1: Tuned clock is used to sample data / cmd"]
    SmpClkSel1 = 1,
}
impl From<SmpClkSel> for bool {
    #[inline(always)]
    fn from(variant: SmpClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMP_CLK_SEL` reader - SMP_CLK_SEL"]
pub type SmpClkSelR = crate::BitReader<SmpClkSel>;
impl SmpClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmpClkSel {
        match self.bits {
            false => SmpClkSel::SmpClkSel0,
            true => SmpClkSel::SmpClkSel1,
        }
    }
    #[doc = "Fixed clock is used to sample data / cmd"]
    #[inline(always)]
    pub fn is_smp_clk_sel_0(&self) -> bool {
        *self == SmpClkSel::SmpClkSel0
    }
    #[doc = "Tuned clock is used to sample data / cmd"]
    #[inline(always)]
    pub fn is_smp_clk_sel_1(&self) -> bool {
        *self == SmpClkSel::SmpClkSel1
    }
}
#[doc = "Field `SMP_CLK_SEL` writer - SMP_CLK_SEL"]
pub type SmpClkSelW<'a, REG> = crate::BitWriter<'a, REG, SmpClkSel>;
impl<'a, REG> SmpClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed clock is used to sample data / cmd"]
    #[inline(always)]
    pub fn smp_clk_sel_0(self) -> &'a mut crate::W<REG> {
        self.variant(SmpClkSel::SmpClkSel0)
    }
    #[doc = "Tuned clock is used to sample data / cmd"]
    #[inline(always)]
    pub fn smp_clk_sel_1(self) -> &'a mut crate::W<REG> {
        self.variant(SmpClkSel::SmpClkSel1)
    }
}
#[doc = "Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoTuneEn {
    #[doc = "0: Disable auto tuning"]
    AutoTuneEn0 = 0,
    #[doc = "1: Enable auto tuning"]
    AutoTuneEn1 = 1,
}
impl From<AutoTuneEn> for bool {
    #[inline(always)]
    fn from(variant: AutoTuneEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTO_TUNE_EN` reader - Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
pub type AutoTuneEnR = crate::BitReader<AutoTuneEn>;
impl AutoTuneEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoTuneEn {
        match self.bits {
            false => AutoTuneEn::AutoTuneEn0,
            true => AutoTuneEn::AutoTuneEn1,
        }
    }
    #[doc = "Disable auto tuning"]
    #[inline(always)]
    pub fn is_auto_tune_en_0(&self) -> bool {
        *self == AutoTuneEn::AutoTuneEn0
    }
    #[doc = "Enable auto tuning"]
    #[inline(always)]
    pub fn is_auto_tune_en_1(&self) -> bool {
        *self == AutoTuneEn::AutoTuneEn1
    }
}
#[doc = "Field `AUTO_TUNE_EN` writer - Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
pub type AutoTuneEnW<'a, REG> = crate::BitWriter<'a, REG, AutoTuneEn>;
impl<'a, REG> AutoTuneEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto tuning"]
    #[inline(always)]
    pub fn auto_tune_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(AutoTuneEn::AutoTuneEn0)
    }
    #[doc = "Enable auto tuning"]
    #[inline(always)]
    pub fn auto_tune_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(AutoTuneEn::AutoTuneEn1)
    }
}
#[doc = "Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbclkSel {
    #[doc = "0: Feedback clock comes from the loopback CLK"]
    FbclkSel0 = 0,
    #[doc = "1: Feedback clock comes from the ipp_card_clk_out"]
    FbclkSel1 = 1,
}
impl From<FbclkSel> for bool {
    #[inline(always)]
    fn from(variant: FbclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBCLK_SEL` reader - Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
pub type FbclkSelR = crate::BitReader<FbclkSel>;
impl FbclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FbclkSel {
        match self.bits {
            false => FbclkSel::FbclkSel0,
            true => FbclkSel::FbclkSel1,
        }
    }
    #[doc = "Feedback clock comes from the loopback CLK"]
    #[inline(always)]
    pub fn is_fbclk_sel_0(&self) -> bool {
        *self == FbclkSel::FbclkSel0
    }
    #[doc = "Feedback clock comes from the ipp_card_clk_out"]
    #[inline(always)]
    pub fn is_fbclk_sel_1(&self) -> bool {
        *self == FbclkSel::FbclkSel1
    }
}
#[doc = "Field `FBCLK_SEL` writer - Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
pub type FbclkSelW<'a, REG> = crate::BitWriter<'a, REG, FbclkSel>;
impl<'a, REG> FbclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Feedback clock comes from the loopback CLK"]
    #[inline(always)]
    pub fn fbclk_sel_0(self) -> &'a mut crate::W<REG> {
        self.variant(FbclkSel::FbclkSel0)
    }
    #[doc = "Feedback clock comes from the ipp_card_clk_out"]
    #[inline(always)]
    pub fn fbclk_sel_1(self) -> &'a mut crate::W<REG> {
        self.variant(FbclkSel::FbclkSel1)
    }
}
#[doc = "Field `HS400_MODE` reader - Enable HS400 Mode"]
pub type Hs400ModeR = crate::BitReader;
#[doc = "Field `HS400_MODE` writer - Enable HS400 Mode"]
pub type Hs400ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&self) -> BcenR {
        BcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline(always)]
    pub fn ac12en(&self) -> Ac12enR {
        Ac12enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dual Data Rate mode selection"]
    #[inline(always)]
    pub fn ddr_en(&self) -> DdrEnR {
        DdrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn dtdsel(&self) -> DtdselR {
        DtdselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline(always)]
    pub fn msbsel(&self) -> MsbselR {
        MsbselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NIBBLE_POS"]
    #[inline(always)]
    pub fn nibble_pos(&self) -> NibblePosR {
        NibblePosR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto CMD23 Enable"]
    #[inline(always)]
    pub fn ac23en(&self) -> Ac23enR {
        Ac23enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 22 - Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn exe_tune(&self) -> ExeTuneR {
        ExeTuneR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SMP_CLK_SEL"]
    #[inline(always)]
    pub fn smp_clk_sel(&self) -> SmpClkSelR {
        SmpClkSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn auto_tune_en(&self) -> AutoTuneEnR {
        AutoTuneEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn fbclk_sel(&self) -> FbclkSelR {
        FbclkSelR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable HS400 Mode"]
    #[inline(always)]
    pub fn hs400_mode(&self) -> Hs400ModeR {
        Hs400ModeR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIX_CTRL")
            .field("dmaen", &self.dmaen())
            .field("bcen", &self.bcen())
            .field("ac12en", &self.ac12en())
            .field("ddr_en", &self.ddr_en())
            .field("dtdsel", &self.dtdsel())
            .field("msbsel", &self.msbsel())
            .field("nibble_pos", &self.nibble_pos())
            .field("ac23en", &self.ac23en())
            .field("exe_tune", &self.exe_tune())
            .field("smp_clk_sel", &self.smp_clk_sel())
            .field("auto_tune_en", &self.auto_tune_en())
            .field("fbclk_sel", &self.fbclk_sel())
            .field("hs400_mode", &self.hs400_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<MixCtrlSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&mut self) -> BcenW<MixCtrlSpec> {
        BcenW::new(self, 1)
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline(always)]
    pub fn ac12en(&mut self) -> Ac12enW<MixCtrlSpec> {
        Ac12enW::new(self, 2)
    }
    #[doc = "Bit 3 - Dual Data Rate mode selection"]
    #[inline(always)]
    pub fn ddr_en(&mut self) -> DdrEnW<MixCtrlSpec> {
        DdrEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn dtdsel(&mut self) -> DtdselW<MixCtrlSpec> {
        DtdselW::new(self, 4)
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline(always)]
    pub fn msbsel(&mut self) -> MsbselW<MixCtrlSpec> {
        MsbselW::new(self, 5)
    }
    #[doc = "Bit 6 - NIBBLE_POS"]
    #[inline(always)]
    pub fn nibble_pos(&mut self) -> NibblePosW<MixCtrlSpec> {
        NibblePosW::new(self, 6)
    }
    #[doc = "Bit 7 - Auto CMD23 Enable"]
    #[inline(always)]
    pub fn ac23en(&mut self) -> Ac23enW<MixCtrlSpec> {
        Ac23enW::new(self, 7)
    }
    #[doc = "Bit 22 - Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn exe_tune(&mut self) -> ExeTuneW<MixCtrlSpec> {
        ExeTuneW::new(self, 22)
    }
    #[doc = "Bit 23 - SMP_CLK_SEL"]
    #[inline(always)]
    pub fn smp_clk_sel(&mut self) -> SmpClkSelW<MixCtrlSpec> {
        SmpClkSelW::new(self, 23)
    }
    #[doc = "Bit 24 - Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn auto_tune_en(&mut self) -> AutoTuneEnW<MixCtrlSpec> {
        AutoTuneEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn fbclk_sel(&mut self) -> FbclkSelW<MixCtrlSpec> {
        FbclkSelW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable HS400 Mode"]
    #[inline(always)]
    pub fn hs400_mode(&mut self) -> Hs400ModeW<MixCtrlSpec> {
        Hs400ModeW::new(self, 26)
    }
}
#[doc = "Mixer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mix_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mix_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MixCtrlSpec;
impl crate::RegisterSpec for MixCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mix_ctrl::R`](R) reader structure"]
impl crate::Readable for MixCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mix_ctrl::W`](W) writer structure"]
impl crate::Writable for MixCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIX_CTRL to value 0x8000_0000"]
impl crate::Resettable for MixCtrlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
