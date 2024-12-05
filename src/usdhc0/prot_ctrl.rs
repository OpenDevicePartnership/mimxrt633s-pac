#[doc = "Register `PROT_CTRL` reader"]
pub type R = crate::R<ProtCtrlSpec>;
#[doc = "Register `PROT_CTRL` writer"]
pub type W = crate::W<ProtCtrlSpec>;
#[doc = "Data Transfer Width\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtw {
    #[doc = "0: 1-bit mode"]
    Dtw0 = 0,
    #[doc = "1: 4-bit mode"]
    Dtw1 = 1,
    #[doc = "2: 8-bit mode"]
    Dtw2 = 2,
}
impl From<Dtw> for u8 {
    #[inline(always)]
    fn from(variant: Dtw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtw {
    type Ux = u8;
}
impl crate::IsEnum for Dtw {}
#[doc = "Field `DTW` reader - Data Transfer Width"]
pub type DtwR = crate::FieldReader<Dtw>;
impl DtwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtw> {
        match self.bits {
            0 => Some(Dtw::Dtw0),
            1 => Some(Dtw::Dtw1),
            2 => Some(Dtw::Dtw2),
            _ => None,
        }
    }
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn is_dtw_0(&self) -> bool {
        *self == Dtw::Dtw0
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn is_dtw_1(&self) -> bool {
        *self == Dtw::Dtw1
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn is_dtw_2(&self) -> bool {
        *self == Dtw::Dtw2
    }
}
#[doc = "Field `DTW` writer - Data Transfer Width"]
pub type DtwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dtw>;
impl<'a, REG> DtwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn dtw_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtw::Dtw0)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn dtw_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtw::Dtw1)
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn dtw_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dtw::Dtw2)
    }
}
#[doc = "DATA3 as Card Detection Pin\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3cd {
    #[doc = "0: DATA3 does not monitor Card Insertion"]
    D3cd0 = 0,
    #[doc = "1: DATA3 as Card Detection Pin"]
    D3cd1 = 1,
}
impl From<D3cd> for bool {
    #[inline(always)]
    fn from(variant: D3cd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D3CD` reader - DATA3 as Card Detection Pin"]
pub type D3cdR = crate::BitReader<D3cd>;
impl D3cdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D3cd {
        match self.bits {
            false => D3cd::D3cd0,
            true => D3cd::D3cd1,
        }
    }
    #[doc = "DATA3 does not monitor Card Insertion"]
    #[inline(always)]
    pub fn is_d3cd_0(&self) -> bool {
        *self == D3cd::D3cd0
    }
    #[doc = "DATA3 as Card Detection Pin"]
    #[inline(always)]
    pub fn is_d3cd_1(&self) -> bool {
        *self == D3cd::D3cd1
    }
}
#[doc = "Field `D3CD` writer - DATA3 as Card Detection Pin"]
pub type D3cdW<'a, REG> = crate::BitWriter<'a, REG, D3cd>;
impl<'a, REG> D3cdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DATA3 does not monitor Card Insertion"]
    #[inline(always)]
    pub fn d3cd_0(self) -> &'a mut crate::W<REG> {
        self.variant(D3cd::D3cd0)
    }
    #[doc = "DATA3 as Card Detection Pin"]
    #[inline(always)]
    pub fn d3cd_1(self) -> &'a mut crate::W<REG> {
        self.variant(D3cd::D3cd1)
    }
}
#[doc = "Endian Mode\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emode {
    #[doc = "0: Big Endian Mode"]
    Emode0 = 0,
    #[doc = "1: Half Word Big Endian Mode"]
    Emode1 = 1,
    #[doc = "2: Little Endian Mode"]
    Emode2 = 2,
}
impl From<Emode> for u8 {
    #[inline(always)]
    fn from(variant: Emode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emode {
    type Ux = u8;
}
impl crate::IsEnum for Emode {}
#[doc = "Field `EMODE` reader - Endian Mode"]
pub type EmodeR = crate::FieldReader<Emode>;
impl EmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Emode> {
        match self.bits {
            0 => Some(Emode::Emode0),
            1 => Some(Emode::Emode1),
            2 => Some(Emode::Emode2),
            _ => None,
        }
    }
    #[doc = "Big Endian Mode"]
    #[inline(always)]
    pub fn is_emode_0(&self) -> bool {
        *self == Emode::Emode0
    }
    #[doc = "Half Word Big Endian Mode"]
    #[inline(always)]
    pub fn is_emode_1(&self) -> bool {
        *self == Emode::Emode1
    }
    #[doc = "Little Endian Mode"]
    #[inline(always)]
    pub fn is_emode_2(&self) -> bool {
        *self == Emode::Emode2
    }
}
#[doc = "Field `EMODE` writer - Endian Mode"]
pub type EmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Emode>;
impl<'a, REG> EmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Big Endian Mode"]
    #[inline(always)]
    pub fn emode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Emode::Emode0)
    }
    #[doc = "Half Word Big Endian Mode"]
    #[inline(always)]
    pub fn emode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Emode::Emode1)
    }
    #[doc = "Little Endian Mode"]
    #[inline(always)]
    pub fn emode_2(self) -> &'a mut crate::W<REG> {
        self.variant(Emode::Emode2)
    }
}
#[doc = "Card Detect Test Level\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdtl {
    #[doc = "0: Card Detect Test Level is 0, no card inserted"]
    Cdtl0 = 0,
    #[doc = "1: Card Detect Test Level is 1, card inserted"]
    Cdtl1 = 1,
}
impl From<Cdtl> for bool {
    #[inline(always)]
    fn from(variant: Cdtl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDTL` reader - Card Detect Test Level"]
pub type CdtlR = crate::BitReader<Cdtl>;
impl CdtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdtl {
        match self.bits {
            false => Cdtl::Cdtl0,
            true => Cdtl::Cdtl1,
        }
    }
    #[doc = "Card Detect Test Level is 0, no card inserted"]
    #[inline(always)]
    pub fn is_cdtl_0(&self) -> bool {
        *self == Cdtl::Cdtl0
    }
    #[doc = "Card Detect Test Level is 1, card inserted"]
    #[inline(always)]
    pub fn is_cdtl_1(&self) -> bool {
        *self == Cdtl::Cdtl1
    }
}
#[doc = "Field `CDTL` writer - Card Detect Test Level"]
pub type CdtlW<'a, REG> = crate::BitWriter<'a, REG, Cdtl>;
impl<'a, REG> CdtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card Detect Test Level is 0, no card inserted"]
    #[inline(always)]
    pub fn cdtl_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdtl::Cdtl0)
    }
    #[doc = "Card Detect Test Level is 1, card inserted"]
    #[inline(always)]
    pub fn cdtl_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdtl::Cdtl1)
    }
}
#[doc = "Card Detect Signal Selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdss {
    #[doc = "0: Card Detection Level is selected (for normal purpose)."]
    Cdss0 = 0,
    #[doc = "1: Card Detection Test Level is selected (for test purpose)."]
    Cdss1 = 1,
}
impl From<Cdss> for bool {
    #[inline(always)]
    fn from(variant: Cdss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDSS` reader - Card Detect Signal Selection"]
pub type CdssR = crate::BitReader<Cdss>;
impl CdssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdss {
        match self.bits {
            false => Cdss::Cdss0,
            true => Cdss::Cdss1,
        }
    }
    #[doc = "Card Detection Level is selected (for normal purpose)."]
    #[inline(always)]
    pub fn is_cdss_0(&self) -> bool {
        *self == Cdss::Cdss0
    }
    #[doc = "Card Detection Test Level is selected (for test purpose)."]
    #[inline(always)]
    pub fn is_cdss_1(&self) -> bool {
        *self == Cdss::Cdss1
    }
}
#[doc = "Field `CDSS` writer - Card Detect Signal Selection"]
pub type CdssW<'a, REG> = crate::BitWriter<'a, REG, Cdss>;
impl<'a, REG> CdssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card Detection Level is selected (for normal purpose)."]
    #[inline(always)]
    pub fn cdss_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdss::Cdss0)
    }
    #[doc = "Card Detection Test Level is selected (for test purpose)."]
    #[inline(always)]
    pub fn cdss_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdss::Cdss1)
    }
}
#[doc = "DMA Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmasel {
    #[doc = "0: No DMA or Simple DMA is selected"]
    Dmasel0 = 0,
    #[doc = "1: ADMA1 is selected"]
    Dmasel1 = 1,
    #[doc = "2: ADMA2 is selected"]
    Dmasel2 = 2,
}
impl From<Dmasel> for u8 {
    #[inline(always)]
    fn from(variant: Dmasel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmasel {
    type Ux = u8;
}
impl crate::IsEnum for Dmasel {}
#[doc = "Field `DMASEL` reader - DMA Select"]
pub type DmaselR = crate::FieldReader<Dmasel>;
impl DmaselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmasel> {
        match self.bits {
            0 => Some(Dmasel::Dmasel0),
            1 => Some(Dmasel::Dmasel1),
            2 => Some(Dmasel::Dmasel2),
            _ => None,
        }
    }
    #[doc = "No DMA or Simple DMA is selected"]
    #[inline(always)]
    pub fn is_dmasel_0(&self) -> bool {
        *self == Dmasel::Dmasel0
    }
    #[doc = "ADMA1 is selected"]
    #[inline(always)]
    pub fn is_dmasel_1(&self) -> bool {
        *self == Dmasel::Dmasel1
    }
    #[doc = "ADMA2 is selected"]
    #[inline(always)]
    pub fn is_dmasel_2(&self) -> bool {
        *self == Dmasel::Dmasel2
    }
}
#[doc = "Field `DMASEL` writer - DMA Select"]
pub type DmaselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmasel>;
impl<'a, REG> DmaselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No DMA or Simple DMA is selected"]
    #[inline(always)]
    pub fn dmasel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasel::Dmasel0)
    }
    #[doc = "ADMA1 is selected"]
    #[inline(always)]
    pub fn dmasel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasel::Dmasel1)
    }
    #[doc = "ADMA2 is selected"]
    #[inline(always)]
    pub fn dmasel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasel::Dmasel2)
    }
}
#[doc = "Stop At Block Gap Request\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sabgreq {
    #[doc = "0: Transfer"]
    Sabgreq0 = 0,
    #[doc = "1: Stop"]
    Sabgreq1 = 1,
}
impl From<Sabgreq> for bool {
    #[inline(always)]
    fn from(variant: Sabgreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SABGREQ` reader - Stop At Block Gap Request"]
pub type SabgreqR = crate::BitReader<Sabgreq>;
impl SabgreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sabgreq {
        match self.bits {
            false => Sabgreq::Sabgreq0,
            true => Sabgreq::Sabgreq1,
        }
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn is_sabgreq_0(&self) -> bool {
        *self == Sabgreq::Sabgreq0
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_sabgreq_1(&self) -> bool {
        *self == Sabgreq::Sabgreq1
    }
}
#[doc = "Field `SABGREQ` writer - Stop At Block Gap Request"]
pub type SabgreqW<'a, REG> = crate::BitWriter<'a, REG, Sabgreq>;
impl<'a, REG> SabgreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn sabgreq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sabgreq::Sabgreq0)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn sabgreq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sabgreq::Sabgreq1)
    }
}
#[doc = "Continue Request\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Creq {
    #[doc = "0: No effect"]
    Creq0 = 0,
    #[doc = "1: Restart"]
    Creq1 = 1,
}
impl From<Creq> for bool {
    #[inline(always)]
    fn from(variant: Creq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CREQ` reader - Continue Request"]
pub type CreqR = crate::BitReader<Creq>;
impl CreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Creq {
        match self.bits {
            false => Creq::Creq0,
            true => Creq::Creq1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_creq_0(&self) -> bool {
        *self == Creq::Creq0
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn is_creq_1(&self) -> bool {
        *self == Creq::Creq1
    }
}
#[doc = "Field `CREQ` writer - Continue Request"]
pub type CreqW<'a, REG> = crate::BitWriter<'a, REG, Creq>;
impl<'a, REG> CreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn creq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Creq::Creq0)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn creq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Creq::Creq1)
    }
}
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwctl {
    #[doc = "0: Disable Read Wait Control, and stop SD Clock at block gap when SABGREQ bit is set"]
    Rwctl0 = 0,
    #[doc = "1: Enable Read Wait Control, and assert Read Wait without stopping SD Clock at block gap when SABGREQ bit is set"]
    Rwctl1 = 1,
}
impl From<Rwctl> for bool {
    #[inline(always)]
    fn from(variant: Rwctl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWCTL` reader - Read Wait Control"]
pub type RwctlR = crate::BitReader<Rwctl>;
impl RwctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwctl {
        match self.bits {
            false => Rwctl::Rwctl0,
            true => Rwctl::Rwctl1,
        }
    }
    #[doc = "Disable Read Wait Control, and stop SD Clock at block gap when SABGREQ bit is set"]
    #[inline(always)]
    pub fn is_rwctl_0(&self) -> bool {
        *self == Rwctl::Rwctl0
    }
    #[doc = "Enable Read Wait Control, and assert Read Wait without stopping SD Clock at block gap when SABGREQ bit is set"]
    #[inline(always)]
    pub fn is_rwctl_1(&self) -> bool {
        *self == Rwctl::Rwctl1
    }
}
#[doc = "Field `RWCTL` writer - Read Wait Control"]
pub type RwctlW<'a, REG> = crate::BitWriter<'a, REG, Rwctl>;
impl<'a, REG> RwctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Read Wait Control, and stop SD Clock at block gap when SABGREQ bit is set"]
    #[inline(always)]
    pub fn rwctl_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwctl::Rwctl0)
    }
    #[doc = "Enable Read Wait Control, and assert Read Wait without stopping SD Clock at block gap when SABGREQ bit is set"]
    #[inline(always)]
    pub fn rwctl_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwctl::Rwctl1)
    }
}
#[doc = "Interrupt At Block Gap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iabg {
    #[doc = "0: Disabled"]
    Iabg0 = 0,
    #[doc = "1: Enabled"]
    Iabg1 = 1,
}
impl From<Iabg> for bool {
    #[inline(always)]
    fn from(variant: Iabg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IABG` reader - Interrupt At Block Gap"]
pub type IabgR = crate::BitReader<Iabg>;
impl IabgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iabg {
        match self.bits {
            false => Iabg::Iabg0,
            true => Iabg::Iabg1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_iabg_0(&self) -> bool {
        *self == Iabg::Iabg0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_iabg_1(&self) -> bool {
        *self == Iabg::Iabg1
    }
}
#[doc = "Field `IABG` writer - Interrupt At Block Gap"]
pub type IabgW<'a, REG> = crate::BitWriter<'a, REG, Iabg>;
impl<'a, REG> IabgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn iabg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Iabg::Iabg0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn iabg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Iabg::Iabg1)
    }
}
#[doc = "Field `RD_DONE_NO_8CLK` reader - RD_DONE_NO_8CLK"]
pub type RdDoneNo8clkR = crate::BitReader;
#[doc = "Field `RD_DONE_NO_8CLK` writer - RD_DONE_NO_8CLK"]
pub type RdDoneNo8clkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_WAIT_POINT` reader - Read wait point"]
pub type RdWaitPointR = crate::FieldReader;
#[doc = "Field `RD_WAIT_POINT` writer - Read wait point"]
pub type RdWaitPointW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Wakeup Event Enable On Card Interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wecint {
    #[doc = "0: Disable"]
    Wecint0 = 0,
    #[doc = "1: Enable"]
    Wecint1 = 1,
}
impl From<Wecint> for bool {
    #[inline(always)]
    fn from(variant: Wecint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WECINT` reader - Wakeup Event Enable On Card Interrupt"]
pub type WecintR = crate::BitReader<Wecint>;
impl WecintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wecint {
        match self.bits {
            false => Wecint::Wecint0,
            true => Wecint::Wecint1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_wecint_0(&self) -> bool {
        *self == Wecint::Wecint0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_wecint_1(&self) -> bool {
        *self == Wecint::Wecint1
    }
}
#[doc = "Field `WECINT` writer - Wakeup Event Enable On Card Interrupt"]
pub type WecintW<'a, REG> = crate::BitWriter<'a, REG, Wecint>;
impl<'a, REG> WecintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wecint_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wecint::Wecint0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wecint_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wecint::Wecint1)
    }
}
#[doc = "Wakeup Event Enable On SD Card Insertion\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wecins {
    #[doc = "0: Disable"]
    Wecins0 = 0,
    #[doc = "1: Enable"]
    Wecins1 = 1,
}
impl From<Wecins> for bool {
    #[inline(always)]
    fn from(variant: Wecins) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WECINS` reader - Wakeup Event Enable On SD Card Insertion"]
pub type WecinsR = crate::BitReader<Wecins>;
impl WecinsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wecins {
        match self.bits {
            false => Wecins::Wecins0,
            true => Wecins::Wecins1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_wecins_0(&self) -> bool {
        *self == Wecins::Wecins0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_wecins_1(&self) -> bool {
        *self == Wecins::Wecins1
    }
}
#[doc = "Field `WECINS` writer - Wakeup Event Enable On SD Card Insertion"]
pub type WecinsW<'a, REG> = crate::BitWriter<'a, REG, Wecins>;
impl<'a, REG> WecinsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wecins_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wecins::Wecins0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wecins_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wecins::Wecins1)
    }
}
#[doc = "Wakeup Event Enable On SD Card Removal\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wecrm {
    #[doc = "0: Disable"]
    Wecrm0 = 0,
    #[doc = "1: Enable"]
    Wecrm1 = 1,
}
impl From<Wecrm> for bool {
    #[inline(always)]
    fn from(variant: Wecrm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WECRM` reader - Wakeup Event Enable On SD Card Removal"]
pub type WecrmR = crate::BitReader<Wecrm>;
impl WecrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wecrm {
        match self.bits {
            false => Wecrm::Wecrm0,
            true => Wecrm::Wecrm1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_wecrm_0(&self) -> bool {
        *self == Wecrm::Wecrm0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_wecrm_1(&self) -> bool {
        *self == Wecrm::Wecrm1
    }
}
#[doc = "Field `WECRM` writer - Wakeup Event Enable On SD Card Removal"]
pub type WecrmW<'a, REG> = crate::BitWriter<'a, REG, Wecrm>;
impl<'a, REG> WecrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wecrm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wecrm::Wecrm0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wecrm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wecrm::Wecrm1)
    }
}
#[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BurstLenEn {
    #[doc = "1: Burst length is enabled for INCR"]
    BurstLenEn1 = 1,
}
impl From<BurstLenEn> for u8 {
    #[inline(always)]
    fn from(variant: BurstLenEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BurstLenEn {
    type Ux = u8;
}
impl crate::IsEnum for BurstLenEn {}
#[doc = "Field `BURST_LEN_EN` reader - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
pub type BurstLenEnR = crate::FieldReader<BurstLenEn>;
impl BurstLenEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BurstLenEn> {
        match self.bits {
            1 => Some(BurstLenEn::BurstLenEn1),
            _ => None,
        }
    }
    #[doc = "Burst length is enabled for INCR"]
    #[inline(always)]
    pub fn is_burst_len_en_1(&self) -> bool {
        *self == BurstLenEn::BurstLenEn1
    }
}
#[doc = "Field `BURST_LEN_EN` writer - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
pub type BurstLenEnW<'a, REG> = crate::FieldWriter<'a, REG, 3, BurstLenEn>;
impl<'a, REG> BurstLenEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Burst length is enabled for INCR"]
    #[inline(always)]
    pub fn burst_len_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(BurstLenEn::BurstLenEn1)
    }
}
#[doc = "NON_EXACT_BLK_RD\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NonExactBlkRd {
    #[doc = "0: The block read is exact block read. Host driver doesn't need to issue abort command to terminate this multi-block read."]
    NonExactBlkRd0 = 0,
    #[doc = "1: The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    NonExactBlkRd1 = 1,
}
impl From<NonExactBlkRd> for bool {
    #[inline(always)]
    fn from(variant: NonExactBlkRd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NON_EXACT_BLK_RD` reader - NON_EXACT_BLK_RD"]
pub type NonExactBlkRdR = crate::BitReader<NonExactBlkRd>;
impl NonExactBlkRdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NonExactBlkRd {
        match self.bits {
            false => NonExactBlkRd::NonExactBlkRd0,
            true => NonExactBlkRd::NonExactBlkRd1,
        }
    }
    #[doc = "The block read is exact block read. Host driver doesn't need to issue abort command to terminate this multi-block read."]
    #[inline(always)]
    pub fn is_non_exact_blk_rd_0(&self) -> bool {
        *self == NonExactBlkRd::NonExactBlkRd0
    }
    #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    #[inline(always)]
    pub fn is_non_exact_blk_rd_1(&self) -> bool {
        *self == NonExactBlkRd::NonExactBlkRd1
    }
}
#[doc = "Field `NON_EXACT_BLK_RD` writer - NON_EXACT_BLK_RD"]
pub type NonExactBlkRdW<'a, REG> = crate::BitWriter<'a, REG, NonExactBlkRd>;
impl<'a, REG> NonExactBlkRdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The block read is exact block read. Host driver doesn't need to issue abort command to terminate this multi-block read."]
    #[inline(always)]
    pub fn non_exact_blk_rd_0(self) -> &'a mut crate::W<REG> {
        self.variant(NonExactBlkRd::NonExactBlkRd0)
    }
    #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    #[inline(always)]
    pub fn non_exact_blk_rd_1(self) -> &'a mut crate::W<REG> {
        self.variant(NonExactBlkRd::NonExactBlkRd1)
    }
}
#[doc = "RD_NO8CLK_EN\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdNo8clkEn {
    #[doc = "0: Disable S/W RD_DONE_NO_8CLK, uSHDC determines if 8 clocks are needed automatically."]
    RdNo8clkEn0 = 0,
    #[doc = "1: S/W RD_DONE_NO_8CLK is enabled."]
    RdNo8clkEn1 = 1,
}
impl From<RdNo8clkEn> for bool {
    #[inline(always)]
    fn from(variant: RdNo8clkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_NO8CLK_EN` reader - RD_NO8CLK_EN"]
pub type RdNo8clkEnR = crate::BitReader<RdNo8clkEn>;
impl RdNo8clkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RdNo8clkEn {
        match self.bits {
            false => RdNo8clkEn::RdNo8clkEn0,
            true => RdNo8clkEn::RdNo8clkEn1,
        }
    }
    #[doc = "Disable S/W RD_DONE_NO_8CLK, uSHDC determines if 8 clocks are needed automatically."]
    #[inline(always)]
    pub fn is_rd_no8clk_en_0(&self) -> bool {
        *self == RdNo8clkEn::RdNo8clkEn0
    }
    #[doc = "S/W RD_DONE_NO_8CLK is enabled."]
    #[inline(always)]
    pub fn is_rd_no8clk_en_1(&self) -> bool {
        *self == RdNo8clkEn::RdNo8clkEn1
    }
}
#[doc = "Field `RD_NO8CLK_EN` writer - RD_NO8CLK_EN"]
pub type RdNo8clkEnW<'a, REG> = crate::BitWriter<'a, REG, RdNo8clkEn>;
impl<'a, REG> RdNo8clkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable S/W RD_DONE_NO_8CLK, uSHDC determines if 8 clocks are needed automatically."]
    #[inline(always)]
    pub fn rd_no8clk_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(RdNo8clkEn::RdNo8clkEn0)
    }
    #[doc = "S/W RD_DONE_NO_8CLK is enabled."]
    #[inline(always)]
    pub fn rd_no8clk_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(RdNo8clkEn::RdNo8clkEn1)
    }
}
impl R {
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline(always)]
    pub fn dtw(&self) -> DtwR {
        DtwR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - DATA3 as Card Detection Pin"]
    #[inline(always)]
    pub fn d3cd(&self) -> D3cdR {
        D3cdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline(always)]
    pub fn emode(&self) -> EmodeR {
        EmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtl(&self) -> CdtlR {
        CdtlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn cdss(&self) -> CdssR {
        CdssR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&self) -> DmaselR {
        DmaselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn sabgreq(&self) -> SabgreqR {
        SabgreqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn creq(&self) -> CreqR {
        CreqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctl(&self) -> RwctlR {
        RwctlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn iabg(&self) -> IabgR {
        IabgR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RD_DONE_NO_8CLK"]
    #[inline(always)]
    pub fn rd_done_no_8clk(&self) -> RdDoneNo8clkR {
        RdDoneNo8clkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Read wait point"]
    #[inline(always)]
    pub fn rd_wait_point(&self) -> RdWaitPointR {
        RdWaitPointR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wecint(&self) -> WecintR {
        WecintR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wecins(&self) -> WecinsR {
        WecinsR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wecrm(&self) -> WecrmR {
        WecrmR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline(always)]
    pub fn burst_len_en(&self) -> BurstLenEnR {
        BurstLenEnR::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - NON_EXACT_BLK_RD"]
    #[inline(always)]
    pub fn non_exact_blk_rd(&self) -> NonExactBlkRdR {
        NonExactBlkRdR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RD_NO8CLK_EN"]
    #[inline(always)]
    pub fn rd_no8clk_en(&self) -> RdNo8clkEnR {
        RdNo8clkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PROT_CTRL")
            .field("dtw", &self.dtw())
            .field("d3cd", &self.d3cd())
            .field("emode", &self.emode())
            .field("cdtl", &self.cdtl())
            .field("cdss", &self.cdss())
            .field("dmasel", &self.dmasel())
            .field("sabgreq", &self.sabgreq())
            .field("creq", &self.creq())
            .field("rwctl", &self.rwctl())
            .field("iabg", &self.iabg())
            .field("rd_done_no_8clk", &self.rd_done_no_8clk())
            .field("rd_wait_point", &self.rd_wait_point())
            .field("wecint", &self.wecint())
            .field("wecins", &self.wecins())
            .field("wecrm", &self.wecrm())
            .field("burst_len_en", &self.burst_len_en())
            .field("non_exact_blk_rd", &self.non_exact_blk_rd())
            .field("rd_no8clk_en", &self.rd_no8clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline(always)]
    pub fn dtw(&mut self) -> DtwW<ProtCtrlSpec> {
        DtwW::new(self, 1)
    }
    #[doc = "Bit 3 - DATA3 as Card Detection Pin"]
    #[inline(always)]
    pub fn d3cd(&mut self) -> D3cdW<ProtCtrlSpec> {
        D3cdW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline(always)]
    pub fn emode(&mut self) -> EmodeW<ProtCtrlSpec> {
        EmodeW::new(self, 4)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtl(&mut self) -> CdtlW<ProtCtrlSpec> {
        CdtlW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn cdss(&mut self) -> CdssW<ProtCtrlSpec> {
        CdssW::new(self, 7)
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&mut self) -> DmaselW<ProtCtrlSpec> {
        DmaselW::new(self, 8)
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn sabgreq(&mut self) -> SabgreqW<ProtCtrlSpec> {
        SabgreqW::new(self, 16)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn creq(&mut self) -> CreqW<ProtCtrlSpec> {
        CreqW::new(self, 17)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctl(&mut self) -> RwctlW<ProtCtrlSpec> {
        RwctlW::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn iabg(&mut self) -> IabgW<ProtCtrlSpec> {
        IabgW::new(self, 19)
    }
    #[doc = "Bit 20 - RD_DONE_NO_8CLK"]
    #[inline(always)]
    pub fn rd_done_no_8clk(&mut self) -> RdDoneNo8clkW<ProtCtrlSpec> {
        RdDoneNo8clkW::new(self, 20)
    }
    #[doc = "Bits 21:23 - Read wait point"]
    #[inline(always)]
    pub fn rd_wait_point(&mut self) -> RdWaitPointW<ProtCtrlSpec> {
        RdWaitPointW::new(self, 21)
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wecint(&mut self) -> WecintW<ProtCtrlSpec> {
        WecintW::new(self, 24)
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wecins(&mut self) -> WecinsW<ProtCtrlSpec> {
        WecinsW::new(self, 25)
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wecrm(&mut self) -> WecrmW<ProtCtrlSpec> {
        WecrmW::new(self, 26)
    }
    #[doc = "Bits 27:29 - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline(always)]
    pub fn burst_len_en(&mut self) -> BurstLenEnW<ProtCtrlSpec> {
        BurstLenEnW::new(self, 27)
    }
    #[doc = "Bit 30 - NON_EXACT_BLK_RD"]
    #[inline(always)]
    pub fn non_exact_blk_rd(&mut self) -> NonExactBlkRdW<ProtCtrlSpec> {
        NonExactBlkRdW::new(self, 30)
    }
    #[doc = "Bit 31 - RD_NO8CLK_EN"]
    #[inline(always)]
    pub fn rd_no8clk_en(&mut self) -> RdNo8clkEnW<ProtCtrlSpec> {
        RdNo8clkEnW::new(self, 31)
    }
}
#[doc = "Protocol Control\n\nYou can [`read`](crate::Reg::read) this register and get [`prot_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prot_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProtCtrlSpec;
impl crate::RegisterSpec for ProtCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prot_ctrl::R`](R) reader structure"]
impl crate::Readable for ProtCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`prot_ctrl::W`](W) writer structure"]
impl crate::Writable for ProtCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROT_CTRL to value 0x0880_0020"]
impl crate::Resettable for ProtCtrlSpec {
    const RESET_VALUE: u32 = 0x0880_0020;
}
