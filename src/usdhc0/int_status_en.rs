#[doc = "Register `INT_STATUS_EN` reader"]
pub type R = crate::R<IntStatusEnSpec>;
#[doc = "Register `INT_STATUS_EN` writer"]
pub type W = crate::W<IntStatusEnSpec>;
#[doc = "Command Complete Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccsen {
    #[doc = "0: Masked"]
    Ccsen0 = 0,
    #[doc = "1: Enabled"]
    Ccsen1 = 1,
}
impl From<Ccsen> for bool {
    #[inline(always)]
    fn from(variant: Ccsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCSEN` reader - Command Complete Status Enable"]
pub type CcsenR = crate::BitReader<Ccsen>;
impl CcsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccsen {
        match self.bits {
            false => Ccsen::Ccsen0,
            true => Ccsen::Ccsen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_ccsen_0(&self) -> bool {
        *self == Ccsen::Ccsen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_ccsen_1(&self) -> bool {
        *self == Ccsen::Ccsen1
    }
}
#[doc = "Field `CCSEN` writer - Command Complete Status Enable"]
pub type CcsenW<'a, REG> = crate::BitWriter<'a, REG, Ccsen>;
impl<'a, REG> CcsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ccsen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccsen::Ccsen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ccsen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccsen::Ccsen1)
    }
}
#[doc = "Transfer Complete Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcsen {
    #[doc = "0: Masked"]
    Tcsen0 = 0,
    #[doc = "1: Enabled"]
    Tcsen1 = 1,
}
impl From<Tcsen> for bool {
    #[inline(always)]
    fn from(variant: Tcsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCSEN` reader - Transfer Complete Status Enable"]
pub type TcsenR = crate::BitReader<Tcsen>;
impl TcsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcsen {
        match self.bits {
            false => Tcsen::Tcsen0,
            true => Tcsen::Tcsen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_tcsen_0(&self) -> bool {
        *self == Tcsen::Tcsen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_tcsen_1(&self) -> bool {
        *self == Tcsen::Tcsen1
    }
}
#[doc = "Field `TCSEN` writer - Transfer Complete Status Enable"]
pub type TcsenW<'a, REG> = crate::BitWriter<'a, REG, Tcsen>;
impl<'a, REG> TcsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tcsen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcsen::Tcsen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tcsen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcsen::Tcsen1)
    }
}
#[doc = "Block Gap Event Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bgesen {
    #[doc = "0: Masked"]
    Bgesen0 = 0,
    #[doc = "1: Enabled"]
    Bgesen1 = 1,
}
impl From<Bgesen> for bool {
    #[inline(always)]
    fn from(variant: Bgesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGESEN` reader - Block Gap Event Status Enable"]
pub type BgesenR = crate::BitReader<Bgesen>;
impl BgesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bgesen {
        match self.bits {
            false => Bgesen::Bgesen0,
            true => Bgesen::Bgesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_bgesen_0(&self) -> bool {
        *self == Bgesen::Bgesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_bgesen_1(&self) -> bool {
        *self == Bgesen::Bgesen1
    }
}
#[doc = "Field `BGESEN` writer - Block Gap Event Status Enable"]
pub type BgesenW<'a, REG> = crate::BitWriter<'a, REG, Bgesen>;
impl<'a, REG> BgesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn bgesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bgesen::Bgesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bgesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bgesen::Bgesen1)
    }
}
#[doc = "DMA Interrupt Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dintsen {
    #[doc = "0: Masked"]
    Dintsen0 = 0,
    #[doc = "1: Enabled"]
    Dintsen1 = 1,
}
impl From<Dintsen> for bool {
    #[inline(always)]
    fn from(variant: Dintsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINTSEN` reader - DMA Interrupt Status Enable"]
pub type DintsenR = crate::BitReader<Dintsen>;
impl DintsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dintsen {
        match self.bits {
            false => Dintsen::Dintsen0,
            true => Dintsen::Dintsen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_dintsen_0(&self) -> bool {
        *self == Dintsen::Dintsen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_dintsen_1(&self) -> bool {
        *self == Dintsen::Dintsen1
    }
}
#[doc = "Field `DINTSEN` writer - DMA Interrupt Status Enable"]
pub type DintsenW<'a, REG> = crate::BitWriter<'a, REG, Dintsen>;
impl<'a, REG> DintsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dintsen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dintsen::Dintsen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dintsen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dintsen::Dintsen1)
    }
}
#[doc = "Buffer Write Ready Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwrsen {
    #[doc = "0: Masked"]
    Bwrsen0 = 0,
    #[doc = "1: Enabled"]
    Bwrsen1 = 1,
}
impl From<Bwrsen> for bool {
    #[inline(always)]
    fn from(variant: Bwrsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWRSEN` reader - Buffer Write Ready Status Enable"]
pub type BwrsenR = crate::BitReader<Bwrsen>;
impl BwrsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwrsen {
        match self.bits {
            false => Bwrsen::Bwrsen0,
            true => Bwrsen::Bwrsen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_bwrsen_0(&self) -> bool {
        *self == Bwrsen::Bwrsen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_bwrsen_1(&self) -> bool {
        *self == Bwrsen::Bwrsen1
    }
}
#[doc = "Field `BWRSEN` writer - Buffer Write Ready Status Enable"]
pub type BwrsenW<'a, REG> = crate::BitWriter<'a, REG, Bwrsen>;
impl<'a, REG> BwrsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn bwrsen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwrsen::Bwrsen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bwrsen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwrsen::Bwrsen1)
    }
}
#[doc = "Buffer Read Ready Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brrsen {
    #[doc = "0: Masked"]
    Brrsen0 = 0,
    #[doc = "1: Enabled"]
    Brrsen1 = 1,
}
impl From<Brrsen> for bool {
    #[inline(always)]
    fn from(variant: Brrsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRRSEN` reader - Buffer Read Ready Status Enable"]
pub type BrrsenR = crate::BitReader<Brrsen>;
impl BrrsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brrsen {
        match self.bits {
            false => Brrsen::Brrsen0,
            true => Brrsen::Brrsen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_brrsen_0(&self) -> bool {
        *self == Brrsen::Brrsen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_brrsen_1(&self) -> bool {
        *self == Brrsen::Brrsen1
    }
}
#[doc = "Field `BRRSEN` writer - Buffer Read Ready Status Enable"]
pub type BrrsenW<'a, REG> = crate::BitWriter<'a, REG, Brrsen>;
impl<'a, REG> BrrsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn brrsen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Brrsen::Brrsen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn brrsen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Brrsen::Brrsen1)
    }
}
#[doc = "Card Insertion Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cinssen {
    #[doc = "0: Masked"]
    Cinssen0 = 0,
    #[doc = "1: Enabled"]
    Cinssen1 = 1,
}
impl From<Cinssen> for bool {
    #[inline(always)]
    fn from(variant: Cinssen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINSSEN` reader - Card Insertion Status Enable"]
pub type CinssenR = crate::BitReader<Cinssen>;
impl CinssenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cinssen {
        match self.bits {
            false => Cinssen::Cinssen0,
            true => Cinssen::Cinssen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_cinssen_0(&self) -> bool {
        *self == Cinssen::Cinssen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_cinssen_1(&self) -> bool {
        *self == Cinssen::Cinssen1
    }
}
#[doc = "Field `CINSSEN` writer - Card Insertion Status Enable"]
pub type CinssenW<'a, REG> = crate::BitWriter<'a, REG, Cinssen>;
impl<'a, REG> CinssenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cinssen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cinssen::Cinssen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cinssen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cinssen::Cinssen1)
    }
}
#[doc = "Card Removal Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crmsen {
    #[doc = "0: Masked"]
    Crmsen0 = 0,
    #[doc = "1: Enabled"]
    Crmsen1 = 1,
}
impl From<Crmsen> for bool {
    #[inline(always)]
    fn from(variant: Crmsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRMSEN` reader - Card Removal Status Enable"]
pub type CrmsenR = crate::BitReader<Crmsen>;
impl CrmsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crmsen {
        match self.bits {
            false => Crmsen::Crmsen0,
            true => Crmsen::Crmsen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_crmsen_0(&self) -> bool {
        *self == Crmsen::Crmsen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_crmsen_1(&self) -> bool {
        *self == Crmsen::Crmsen1
    }
}
#[doc = "Field `CRMSEN` writer - Card Removal Status Enable"]
pub type CrmsenW<'a, REG> = crate::BitWriter<'a, REG, Crmsen>;
impl<'a, REG> CrmsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn crmsen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Crmsen::Crmsen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn crmsen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Crmsen::Crmsen1)
    }
}
#[doc = "Card Interrupt Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cintsen {
    #[doc = "0: Masked"]
    Cintsen0 = 0,
    #[doc = "1: Enabled"]
    Cintsen1 = 1,
}
impl From<Cintsen> for bool {
    #[inline(always)]
    fn from(variant: Cintsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINTSEN` reader - Card Interrupt Status Enable"]
pub type CintsenR = crate::BitReader<Cintsen>;
impl CintsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cintsen {
        match self.bits {
            false => Cintsen::Cintsen0,
            true => Cintsen::Cintsen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_cintsen_0(&self) -> bool {
        *self == Cintsen::Cintsen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_cintsen_1(&self) -> bool {
        *self == Cintsen::Cintsen1
    }
}
#[doc = "Field `CINTSEN` writer - Card Interrupt Status Enable"]
pub type CintsenW<'a, REG> = crate::BitWriter<'a, REG, Cintsen>;
impl<'a, REG> CintsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cintsen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cintsen::Cintsen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cintsen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cintsen::Cintsen1)
    }
}
#[doc = "Re-Tuning Event Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtesen {
    #[doc = "0: Masked"]
    Rtesen0 = 0,
    #[doc = "1: Enabled"]
    Rtesen1 = 1,
}
impl From<Rtesen> for bool {
    #[inline(always)]
    fn from(variant: Rtesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTESEN` reader - Re-Tuning Event Status Enable"]
pub type RtesenR = crate::BitReader<Rtesen>;
impl RtesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtesen {
        match self.bits {
            false => Rtesen::Rtesen0,
            true => Rtesen::Rtesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_rtesen_0(&self) -> bool {
        *self == Rtesen::Rtesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_rtesen_1(&self) -> bool {
        *self == Rtesen::Rtesen1
    }
}
#[doc = "Field `RTESEN` writer - Re-Tuning Event Status Enable"]
pub type RtesenW<'a, REG> = crate::BitWriter<'a, REG, Rtesen>;
impl<'a, REG> RtesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn rtesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtesen::Rtesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn rtesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtesen::Rtesen1)
    }
}
#[doc = "Tuning Pass Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpsen {
    #[doc = "0: Masked"]
    Tpsen0 = 0,
    #[doc = "1: Enabled"]
    Tpsen1 = 1,
}
impl From<Tpsen> for bool {
    #[inline(always)]
    fn from(variant: Tpsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPSEN` reader - Tuning Pass Status Enable"]
pub type TpsenR = crate::BitReader<Tpsen>;
impl TpsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpsen {
        match self.bits {
            false => Tpsen::Tpsen0,
            true => Tpsen::Tpsen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_tpsen_0(&self) -> bool {
        *self == Tpsen::Tpsen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_tpsen_1(&self) -> bool {
        *self == Tpsen::Tpsen1
    }
}
#[doc = "Field `TPSEN` writer - Tuning Pass Status Enable"]
pub type TpsenW<'a, REG> = crate::BitWriter<'a, REG, Tpsen>;
impl<'a, REG> TpsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tpsen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpsen::Tpsen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tpsen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpsen::Tpsen1)
    }
}
#[doc = "Command Timeout Error Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctoesen {
    #[doc = "0: Masked"]
    Ctoesen0 = 0,
    #[doc = "1: Enabled"]
    Ctoesen1 = 1,
}
impl From<Ctoesen> for bool {
    #[inline(always)]
    fn from(variant: Ctoesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTOESEN` reader - Command Timeout Error Status Enable"]
pub type CtoesenR = crate::BitReader<Ctoesen>;
impl CtoesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctoesen {
        match self.bits {
            false => Ctoesen::Ctoesen0,
            true => Ctoesen::Ctoesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_ctoesen_0(&self) -> bool {
        *self == Ctoesen::Ctoesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_ctoesen_1(&self) -> bool {
        *self == Ctoesen::Ctoesen1
    }
}
#[doc = "Field `CTOESEN` writer - Command Timeout Error Status Enable"]
pub type CtoesenW<'a, REG> = crate::BitWriter<'a, REG, Ctoesen>;
impl<'a, REG> CtoesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ctoesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctoesen::Ctoesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ctoesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctoesen::Ctoesen1)
    }
}
#[doc = "Command CRC Error Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccesen {
    #[doc = "0: Masked"]
    Ccesen0 = 0,
    #[doc = "1: Enabled"]
    Ccesen1 = 1,
}
impl From<Ccesen> for bool {
    #[inline(always)]
    fn from(variant: Ccesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCESEN` reader - Command CRC Error Status Enable"]
pub type CcesenR = crate::BitReader<Ccesen>;
impl CcesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccesen {
        match self.bits {
            false => Ccesen::Ccesen0,
            true => Ccesen::Ccesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_ccesen_0(&self) -> bool {
        *self == Ccesen::Ccesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_ccesen_1(&self) -> bool {
        *self == Ccesen::Ccesen1
    }
}
#[doc = "Field `CCESEN` writer - Command CRC Error Status Enable"]
pub type CcesenW<'a, REG> = crate::BitWriter<'a, REG, Ccesen>;
impl<'a, REG> CcesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ccesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccesen::Ccesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ccesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccesen::Ccesen1)
    }
}
#[doc = "Command End Bit Error Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cebesen {
    #[doc = "0: Masked"]
    Cebesen0 = 0,
    #[doc = "1: Enabled"]
    Cebesen1 = 1,
}
impl From<Cebesen> for bool {
    #[inline(always)]
    fn from(variant: Cebesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEBESEN` reader - Command End Bit Error Status Enable"]
pub type CebesenR = crate::BitReader<Cebesen>;
impl CebesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cebesen {
        match self.bits {
            false => Cebesen::Cebesen0,
            true => Cebesen::Cebesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_cebesen_0(&self) -> bool {
        *self == Cebesen::Cebesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_cebesen_1(&self) -> bool {
        *self == Cebesen::Cebesen1
    }
}
#[doc = "Field `CEBESEN` writer - Command End Bit Error Status Enable"]
pub type CebesenW<'a, REG> = crate::BitWriter<'a, REG, Cebesen>;
impl<'a, REG> CebesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cebesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cebesen::Cebesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cebesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cebesen::Cebesen1)
    }
}
#[doc = "Command Index Error Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ciesen {
    #[doc = "0: Masked"]
    Ciesen0 = 0,
    #[doc = "1: Enabled"]
    Ciesen1 = 1,
}
impl From<Ciesen> for bool {
    #[inline(always)]
    fn from(variant: Ciesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIESEN` reader - Command Index Error Status Enable"]
pub type CiesenR = crate::BitReader<Ciesen>;
impl CiesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ciesen {
        match self.bits {
            false => Ciesen::Ciesen0,
            true => Ciesen::Ciesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_ciesen_0(&self) -> bool {
        *self == Ciesen::Ciesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_ciesen_1(&self) -> bool {
        *self == Ciesen::Ciesen1
    }
}
#[doc = "Field `CIESEN` writer - Command Index Error Status Enable"]
pub type CiesenW<'a, REG> = crate::BitWriter<'a, REG, Ciesen>;
impl<'a, REG> CiesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ciesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ciesen::Ciesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ciesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ciesen::Ciesen1)
    }
}
#[doc = "Data Timeout Error Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtoesen {
    #[doc = "0: Masked"]
    Dtoesen0 = 0,
    #[doc = "1: Enabled"]
    Dtoesen1 = 1,
}
impl From<Dtoesen> for bool {
    #[inline(always)]
    fn from(variant: Dtoesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTOESEN` reader - Data Timeout Error Status Enable"]
pub type DtoesenR = crate::BitReader<Dtoesen>;
impl DtoesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtoesen {
        match self.bits {
            false => Dtoesen::Dtoesen0,
            true => Dtoesen::Dtoesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_dtoesen_0(&self) -> bool {
        *self == Dtoesen::Dtoesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_dtoesen_1(&self) -> bool {
        *self == Dtoesen::Dtoesen1
    }
}
#[doc = "Field `DTOESEN` writer - Data Timeout Error Status Enable"]
pub type DtoesenW<'a, REG> = crate::BitWriter<'a, REG, Dtoesen>;
impl<'a, REG> DtoesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dtoesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtoesen::Dtoesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dtoesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtoesen::Dtoesen1)
    }
}
#[doc = "Data CRC Error Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcesen {
    #[doc = "0: Masked"]
    Dcesen0 = 0,
    #[doc = "1: Enabled"]
    Dcesen1 = 1,
}
impl From<Dcesen> for bool {
    #[inline(always)]
    fn from(variant: Dcesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCESEN` reader - Data CRC Error Status Enable"]
pub type DcesenR = crate::BitReader<Dcesen>;
impl DcesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcesen {
        match self.bits {
            false => Dcesen::Dcesen0,
            true => Dcesen::Dcesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_dcesen_0(&self) -> bool {
        *self == Dcesen::Dcesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_dcesen_1(&self) -> bool {
        *self == Dcesen::Dcesen1
    }
}
#[doc = "Field `DCESEN` writer - Data CRC Error Status Enable"]
pub type DcesenW<'a, REG> = crate::BitWriter<'a, REG, Dcesen>;
impl<'a, REG> DcesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dcesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcesen::Dcesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dcesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcesen::Dcesen1)
    }
}
#[doc = "Data End Bit Error Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debesen {
    #[doc = "0: Masked"]
    Debesen0 = 0,
    #[doc = "1: Enabled"]
    Debesen1 = 1,
}
impl From<Debesen> for bool {
    #[inline(always)]
    fn from(variant: Debesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBESEN` reader - Data End Bit Error Status Enable"]
pub type DebesenR = crate::BitReader<Debesen>;
impl DebesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debesen {
        match self.bits {
            false => Debesen::Debesen0,
            true => Debesen::Debesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_debesen_0(&self) -> bool {
        *self == Debesen::Debesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_debesen_1(&self) -> bool {
        *self == Debesen::Debesen1
    }
}
#[doc = "Field `DEBESEN` writer - Data End Bit Error Status Enable"]
pub type DebesenW<'a, REG> = crate::BitWriter<'a, REG, Debesen>;
impl<'a, REG> DebesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn debesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Debesen::Debesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn debesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Debesen::Debesen1)
    }
}
#[doc = "Auto CMD12 Error Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ac12esen {
    #[doc = "0: Masked"]
    Ac12esen0 = 0,
    #[doc = "1: Enabled"]
    Ac12esen1 = 1,
}
impl From<Ac12esen> for bool {
    #[inline(always)]
    fn from(variant: Ac12esen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AC12ESEN` reader - Auto CMD12 Error Status Enable"]
pub type Ac12esenR = crate::BitReader<Ac12esen>;
impl Ac12esenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ac12esen {
        match self.bits {
            false => Ac12esen::Ac12esen0,
            true => Ac12esen::Ac12esen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_ac12esen_0(&self) -> bool {
        *self == Ac12esen::Ac12esen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_ac12esen_1(&self) -> bool {
        *self == Ac12esen::Ac12esen1
    }
}
#[doc = "Field `AC12ESEN` writer - Auto CMD12 Error Status Enable"]
pub type Ac12esenW<'a, REG> = crate::BitWriter<'a, REG, Ac12esen>;
impl<'a, REG> Ac12esenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ac12esen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ac12esen::Ac12esen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ac12esen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ac12esen::Ac12esen1)
    }
}
#[doc = "Tuning Error Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnesen {
    #[doc = "0: Masked"]
    Tnesen0 = 0,
    #[doc = "1: Enabled"]
    Tnesen1 = 1,
}
impl From<Tnesen> for bool {
    #[inline(always)]
    fn from(variant: Tnesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNESEN` reader - Tuning Error Status Enable"]
pub type TnesenR = crate::BitReader<Tnesen>;
impl TnesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnesen {
        match self.bits {
            false => Tnesen::Tnesen0,
            true => Tnesen::Tnesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_tnesen_0(&self) -> bool {
        *self == Tnesen::Tnesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_tnesen_1(&self) -> bool {
        *self == Tnesen::Tnesen1
    }
}
#[doc = "Field `TNESEN` writer - Tuning Error Status Enable"]
pub type TnesenW<'a, REG> = crate::BitWriter<'a, REG, Tnesen>;
impl<'a, REG> TnesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tnesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tnesen::Tnesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tnesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tnesen::Tnesen1)
    }
}
#[doc = "DMA Error Status Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaesen {
    #[doc = "0: Masked"]
    Dmaesen0 = 0,
    #[doc = "1: Enabled"]
    Dmaesen1 = 1,
}
impl From<Dmaesen> for bool {
    #[inline(always)]
    fn from(variant: Dmaesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAESEN` reader - DMA Error Status Enable"]
pub type DmaesenR = crate::BitReader<Dmaesen>;
impl DmaesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaesen {
        match self.bits {
            false => Dmaesen::Dmaesen0,
            true => Dmaesen::Dmaesen1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_dmaesen_0(&self) -> bool {
        *self == Dmaesen::Dmaesen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_dmaesen_1(&self) -> bool {
        *self == Dmaesen::Dmaesen1
    }
}
#[doc = "Field `DMAESEN` writer - DMA Error Status Enable"]
pub type DmaesenW<'a, REG> = crate::BitWriter<'a, REG, Dmaesen>;
impl<'a, REG> DmaesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dmaesen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaesen::Dmaesen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dmaesen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaesen::Dmaesen1)
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    pub fn ccsen(&self) -> CcsenR {
        CcsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn tcsen(&self) -> TcsenR {
        TcsenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn bgesen(&self) -> BgesenR {
        BgesenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    pub fn dintsen(&self) -> DintsenR {
        DintsenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn bwrsen(&self) -> BwrsenR {
        BwrsenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn brrsen(&self) -> BrrsenR {
        BrrsenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    pub fn cinssen(&self) -> CinssenR {
        CinssenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    pub fn crmsen(&self) -> CrmsenR {
        CrmsenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    pub fn cintsen(&self) -> CintsenR {
        CintsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event Status Enable"]
    #[inline(always)]
    pub fn rtesen(&self) -> RtesenR {
        RtesenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Tuning Pass Status Enable"]
    #[inline(always)]
    pub fn tpsen(&self) -> TpsenR {
        TpsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable"]
    #[inline(always)]
    pub fn ctoesen(&self) -> CtoesenR {
        CtoesenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn ccesen(&self) -> CcesenR {
        CcesenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cebesen(&self) -> CebesenR {
        CebesenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn ciesen(&self) -> CiesenR {
        CiesenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dtoesen(&self) -> DtoesenR {
        DtoesenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn dcesen(&self) -> DcesenR {
        DcesenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn debesen(&self) -> DebesenR {
        DebesenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn ac12esen(&self) -> Ac12esenR {
        Ac12esenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    pub fn tnesen(&self) -> TnesenR {
        TnesenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Error Status Enable"]
    #[inline(always)]
    pub fn dmaesen(&self) -> DmaesenR {
        DmaesenR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS_EN")
            .field("ccsen", &self.ccsen())
            .field("tcsen", &self.tcsen())
            .field("bgesen", &self.bgesen())
            .field("dintsen", &self.dintsen())
            .field("bwrsen", &self.bwrsen())
            .field("brrsen", &self.brrsen())
            .field("cinssen", &self.cinssen())
            .field("crmsen", &self.crmsen())
            .field("cintsen", &self.cintsen())
            .field("rtesen", &self.rtesen())
            .field("tpsen", &self.tpsen())
            .field("ctoesen", &self.ctoesen())
            .field("ccesen", &self.ccesen())
            .field("cebesen", &self.cebesen())
            .field("ciesen", &self.ciesen())
            .field("dtoesen", &self.dtoesen())
            .field("dcesen", &self.dcesen())
            .field("debesen", &self.debesen())
            .field("ac12esen", &self.ac12esen())
            .field("tnesen", &self.tnesen())
            .field("dmaesen", &self.dmaesen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    pub fn ccsen(&mut self) -> CcsenW<IntStatusEnSpec> {
        CcsenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn tcsen(&mut self) -> TcsenW<IntStatusEnSpec> {
        TcsenW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn bgesen(&mut self) -> BgesenW<IntStatusEnSpec> {
        BgesenW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    pub fn dintsen(&mut self) -> DintsenW<IntStatusEnSpec> {
        DintsenW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn bwrsen(&mut self) -> BwrsenW<IntStatusEnSpec> {
        BwrsenW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn brrsen(&mut self) -> BrrsenW<IntStatusEnSpec> {
        BrrsenW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    pub fn cinssen(&mut self) -> CinssenW<IntStatusEnSpec> {
        CinssenW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    pub fn crmsen(&mut self) -> CrmsenW<IntStatusEnSpec> {
        CrmsenW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    pub fn cintsen(&mut self) -> CintsenW<IntStatusEnSpec> {
        CintsenW::new(self, 8)
    }
    #[doc = "Bit 12 - Re-Tuning Event Status Enable"]
    #[inline(always)]
    pub fn rtesen(&mut self) -> RtesenW<IntStatusEnSpec> {
        RtesenW::new(self, 12)
    }
    #[doc = "Bit 14 - Tuning Pass Status Enable"]
    #[inline(always)]
    pub fn tpsen(&mut self) -> TpsenW<IntStatusEnSpec> {
        TpsenW::new(self, 14)
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable"]
    #[inline(always)]
    pub fn ctoesen(&mut self) -> CtoesenW<IntStatusEnSpec> {
        CtoesenW::new(self, 16)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn ccesen(&mut self) -> CcesenW<IntStatusEnSpec> {
        CcesenW::new(self, 17)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cebesen(&mut self) -> CebesenW<IntStatusEnSpec> {
        CebesenW::new(self, 18)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn ciesen(&mut self) -> CiesenW<IntStatusEnSpec> {
        CiesenW::new(self, 19)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dtoesen(&mut self) -> DtoesenW<IntStatusEnSpec> {
        DtoesenW::new(self, 20)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn dcesen(&mut self) -> DcesenW<IntStatusEnSpec> {
        DcesenW::new(self, 21)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn debesen(&mut self) -> DebesenW<IntStatusEnSpec> {
        DebesenW::new(self, 22)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn ac12esen(&mut self) -> Ac12esenW<IntStatusEnSpec> {
        Ac12esenW::new(self, 24)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    pub fn tnesen(&mut self) -> TnesenW<IntStatusEnSpec> {
        TnesenW::new(self, 26)
    }
    #[doc = "Bit 28 - DMA Error Status Enable"]
    #[inline(always)]
    pub fn dmaesen(&mut self) -> DmaesenW<IntStatusEnSpec> {
        DmaesenW::new(self, 28)
    }
}
#[doc = "Interrupt Status Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusEnSpec;
impl crate::RegisterSpec for IntStatusEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status_en::R`](R) reader structure"]
impl crate::Readable for IntStatusEnSpec {}
#[doc = "`write(|w| ..)` method takes [`int_status_en::W`](W) writer structure"]
impl crate::Writable for IntStatusEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_STATUS_EN to value 0"]
impl crate::Resettable for IntStatusEnSpec {
    const RESET_VALUE: u32 = 0;
}
