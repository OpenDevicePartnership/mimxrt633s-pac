#[doc = "Register `INT_SIGNAL_EN` reader"]
pub type R = crate::R<IntSignalEnSpec>;
#[doc = "Register `INT_SIGNAL_EN` writer"]
pub type W = crate::W<IntSignalEnSpec>;
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccien {
    #[doc = "0: Masked"]
    Ccien0 = 0,
    #[doc = "1: Enabled"]
    Ccien1 = 1,
}
impl From<Ccien> for bool {
    #[inline(always)]
    fn from(variant: Ccien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIEN` reader - Command Complete Interrupt Enable"]
pub type CcienR = crate::BitReader<Ccien>;
impl CcienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccien {
        match self.bits {
            false => Ccien::Ccien0,
            true => Ccien::Ccien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_ccien_0(&self) -> bool {
        *self == Ccien::Ccien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_ccien_1(&self) -> bool {
        *self == Ccien::Ccien1
    }
}
#[doc = "Field `CCIEN` writer - Command Complete Interrupt Enable"]
pub type CcienW<'a, REG> = crate::BitWriter<'a, REG, Ccien>;
impl<'a, REG> CcienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ccien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccien::Ccien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ccien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccien::Ccien1)
    }
}
#[doc = "Transfer Complete Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcien {
    #[doc = "0: Masked"]
    Tcien0 = 0,
    #[doc = "1: Enabled"]
    Tcien1 = 1,
}
impl From<Tcien> for bool {
    #[inline(always)]
    fn from(variant: Tcien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIEN` reader - Transfer Complete Interrupt Enable"]
pub type TcienR = crate::BitReader<Tcien>;
impl TcienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcien {
        match self.bits {
            false => Tcien::Tcien0,
            true => Tcien::Tcien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_tcien_0(&self) -> bool {
        *self == Tcien::Tcien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_tcien_1(&self) -> bool {
        *self == Tcien::Tcien1
    }
}
#[doc = "Field `TCIEN` writer - Transfer Complete Interrupt Enable"]
pub type TcienW<'a, REG> = crate::BitWriter<'a, REG, Tcien>;
impl<'a, REG> TcienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tcien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcien::Tcien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tcien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcien::Tcien1)
    }
}
#[doc = "Block Gap Event Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bgeien {
    #[doc = "0: Masked"]
    Bgeien0 = 0,
    #[doc = "1: Enabled"]
    Bgeien1 = 1,
}
impl From<Bgeien> for bool {
    #[inline(always)]
    fn from(variant: Bgeien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGEIEN` reader - Block Gap Event Interrupt Enable"]
pub type BgeienR = crate::BitReader<Bgeien>;
impl BgeienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bgeien {
        match self.bits {
            false => Bgeien::Bgeien0,
            true => Bgeien::Bgeien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_bgeien_0(&self) -> bool {
        *self == Bgeien::Bgeien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_bgeien_1(&self) -> bool {
        *self == Bgeien::Bgeien1
    }
}
#[doc = "Field `BGEIEN` writer - Block Gap Event Interrupt Enable"]
pub type BgeienW<'a, REG> = crate::BitWriter<'a, REG, Bgeien>;
impl<'a, REG> BgeienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn bgeien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bgeien::Bgeien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bgeien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bgeien::Bgeien1)
    }
}
#[doc = "DMA Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dintien {
    #[doc = "0: Masked"]
    Dintien0 = 0,
    #[doc = "1: Enabled"]
    Dintien1 = 1,
}
impl From<Dintien> for bool {
    #[inline(always)]
    fn from(variant: Dintien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINTIEN` reader - DMA Interrupt Enable"]
pub type DintienR = crate::BitReader<Dintien>;
impl DintienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dintien {
        match self.bits {
            false => Dintien::Dintien0,
            true => Dintien::Dintien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_dintien_0(&self) -> bool {
        *self == Dintien::Dintien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_dintien_1(&self) -> bool {
        *self == Dintien::Dintien1
    }
}
#[doc = "Field `DINTIEN` writer - DMA Interrupt Enable"]
pub type DintienW<'a, REG> = crate::BitWriter<'a, REG, Dintien>;
impl<'a, REG> DintienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dintien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dintien::Dintien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dintien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dintien::Dintien1)
    }
}
#[doc = "Buffer Write Ready Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwrien {
    #[doc = "0: Masked"]
    Bwrien0 = 0,
    #[doc = "1: Enabled"]
    Bwrien1 = 1,
}
impl From<Bwrien> for bool {
    #[inline(always)]
    fn from(variant: Bwrien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWRIEN` reader - Buffer Write Ready Interrupt Enable"]
pub type BwrienR = crate::BitReader<Bwrien>;
impl BwrienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwrien {
        match self.bits {
            false => Bwrien::Bwrien0,
            true => Bwrien::Bwrien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_bwrien_0(&self) -> bool {
        *self == Bwrien::Bwrien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_bwrien_1(&self) -> bool {
        *self == Bwrien::Bwrien1
    }
}
#[doc = "Field `BWRIEN` writer - Buffer Write Ready Interrupt Enable"]
pub type BwrienW<'a, REG> = crate::BitWriter<'a, REG, Bwrien>;
impl<'a, REG> BwrienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn bwrien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwrien::Bwrien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bwrien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwrien::Bwrien1)
    }
}
#[doc = "Buffer Read Ready Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brrien {
    #[doc = "0: Masked"]
    Brrien0 = 0,
    #[doc = "1: Enabled"]
    Brrien1 = 1,
}
impl From<Brrien> for bool {
    #[inline(always)]
    fn from(variant: Brrien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRRIEN` reader - Buffer Read Ready Interrupt Enable"]
pub type BrrienR = crate::BitReader<Brrien>;
impl BrrienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brrien {
        match self.bits {
            false => Brrien::Brrien0,
            true => Brrien::Brrien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_brrien_0(&self) -> bool {
        *self == Brrien::Brrien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_brrien_1(&self) -> bool {
        *self == Brrien::Brrien1
    }
}
#[doc = "Field `BRRIEN` writer - Buffer Read Ready Interrupt Enable"]
pub type BrrienW<'a, REG> = crate::BitWriter<'a, REG, Brrien>;
impl<'a, REG> BrrienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn brrien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Brrien::Brrien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn brrien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Brrien::Brrien1)
    }
}
#[doc = "Card Insertion Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cinsien {
    #[doc = "0: Masked"]
    Cinsien0 = 0,
    #[doc = "1: Enabled"]
    Cinsien1 = 1,
}
impl From<Cinsien> for bool {
    #[inline(always)]
    fn from(variant: Cinsien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINSIEN` reader - Card Insertion Interrupt Enable"]
pub type CinsienR = crate::BitReader<Cinsien>;
impl CinsienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cinsien {
        match self.bits {
            false => Cinsien::Cinsien0,
            true => Cinsien::Cinsien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_cinsien_0(&self) -> bool {
        *self == Cinsien::Cinsien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_cinsien_1(&self) -> bool {
        *self == Cinsien::Cinsien1
    }
}
#[doc = "Field `CINSIEN` writer - Card Insertion Interrupt Enable"]
pub type CinsienW<'a, REG> = crate::BitWriter<'a, REG, Cinsien>;
impl<'a, REG> CinsienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cinsien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsien::Cinsien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cinsien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsien::Cinsien1)
    }
}
#[doc = "Card Removal Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crmien {
    #[doc = "0: Masked"]
    Crmien0 = 0,
    #[doc = "1: Enabled"]
    Crmien1 = 1,
}
impl From<Crmien> for bool {
    #[inline(always)]
    fn from(variant: Crmien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRMIEN` reader - Card Removal Interrupt Enable"]
pub type CrmienR = crate::BitReader<Crmien>;
impl CrmienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crmien {
        match self.bits {
            false => Crmien::Crmien0,
            true => Crmien::Crmien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_crmien_0(&self) -> bool {
        *self == Crmien::Crmien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_crmien_1(&self) -> bool {
        *self == Crmien::Crmien1
    }
}
#[doc = "Field `CRMIEN` writer - Card Removal Interrupt Enable"]
pub type CrmienW<'a, REG> = crate::BitWriter<'a, REG, Crmien>;
impl<'a, REG> CrmienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn crmien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Crmien::Crmien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn crmien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Crmien::Crmien1)
    }
}
#[doc = "Card Interrupt Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cintien {
    #[doc = "0: Masked"]
    Cintien0 = 0,
    #[doc = "1: Enabled"]
    Cintien1 = 1,
}
impl From<Cintien> for bool {
    #[inline(always)]
    fn from(variant: Cintien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINTIEN` reader - Card Interrupt Interrupt Enable"]
pub type CintienR = crate::BitReader<Cintien>;
impl CintienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cintien {
        match self.bits {
            false => Cintien::Cintien0,
            true => Cintien::Cintien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_cintien_0(&self) -> bool {
        *self == Cintien::Cintien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_cintien_1(&self) -> bool {
        *self == Cintien::Cintien1
    }
}
#[doc = "Field `CINTIEN` writer - Card Interrupt Interrupt Enable"]
pub type CintienW<'a, REG> = crate::BitWriter<'a, REG, Cintien>;
impl<'a, REG> CintienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cintien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cintien::Cintien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cintien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cintien::Cintien1)
    }
}
#[doc = "Re-Tuning Event Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rteien {
    #[doc = "0: Masked"]
    Rteien0 = 0,
    #[doc = "1: Enabled"]
    Rteien1 = 1,
}
impl From<Rteien> for bool {
    #[inline(always)]
    fn from(variant: Rteien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTEIEN` reader - Re-Tuning Event Interrupt Enable"]
pub type RteienR = crate::BitReader<Rteien>;
impl RteienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rteien {
        match self.bits {
            false => Rteien::Rteien0,
            true => Rteien::Rteien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_rteien_0(&self) -> bool {
        *self == Rteien::Rteien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_rteien_1(&self) -> bool {
        *self == Rteien::Rteien1
    }
}
#[doc = "Field `RTEIEN` writer - Re-Tuning Event Interrupt Enable"]
pub type RteienW<'a, REG> = crate::BitWriter<'a, REG, Rteien>;
impl<'a, REG> RteienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn rteien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rteien::Rteien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn rteien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rteien::Rteien1)
    }
}
#[doc = "Tuning Pass Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpien {
    #[doc = "0: Masked"]
    Tpien0 = 0,
    #[doc = "1: Enabled"]
    Tpien1 = 1,
}
impl From<Tpien> for bool {
    #[inline(always)]
    fn from(variant: Tpien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPIEN` reader - Tuning Pass Interrupt Enable"]
pub type TpienR = crate::BitReader<Tpien>;
impl TpienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpien {
        match self.bits {
            false => Tpien::Tpien0,
            true => Tpien::Tpien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_tpien_0(&self) -> bool {
        *self == Tpien::Tpien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_tpien_1(&self) -> bool {
        *self == Tpien::Tpien1
    }
}
#[doc = "Field `TPIEN` writer - Tuning Pass Interrupt Enable"]
pub type TpienW<'a, REG> = crate::BitWriter<'a, REG, Tpien>;
impl<'a, REG> TpienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tpien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpien::Tpien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tpien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpien::Tpien1)
    }
}
#[doc = "Command Timeout Error Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctoeien {
    #[doc = "0: Masked"]
    Ctoeien0 = 0,
    #[doc = "1: Enabled"]
    Ctoeien1 = 1,
}
impl From<Ctoeien> for bool {
    #[inline(always)]
    fn from(variant: Ctoeien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTOEIEN` reader - Command Timeout Error Interrupt Enable"]
pub type CtoeienR = crate::BitReader<Ctoeien>;
impl CtoeienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctoeien {
        match self.bits {
            false => Ctoeien::Ctoeien0,
            true => Ctoeien::Ctoeien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_ctoeien_0(&self) -> bool {
        *self == Ctoeien::Ctoeien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_ctoeien_1(&self) -> bool {
        *self == Ctoeien::Ctoeien1
    }
}
#[doc = "Field `CTOEIEN` writer - Command Timeout Error Interrupt Enable"]
pub type CtoeienW<'a, REG> = crate::BitWriter<'a, REG, Ctoeien>;
impl<'a, REG> CtoeienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ctoeien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctoeien::Ctoeien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ctoeien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctoeien::Ctoeien1)
    }
}
#[doc = "Command CRC Error Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cceien {
    #[doc = "0: Masked"]
    Cceien0 = 0,
    #[doc = "1: Enabled"]
    Cceien1 = 1,
}
impl From<Cceien> for bool {
    #[inline(always)]
    fn from(variant: Cceien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCEIEN` reader - Command CRC Error Interrupt Enable"]
pub type CceienR = crate::BitReader<Cceien>;
impl CceienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cceien {
        match self.bits {
            false => Cceien::Cceien0,
            true => Cceien::Cceien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_cceien_0(&self) -> bool {
        *self == Cceien::Cceien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_cceien_1(&self) -> bool {
        *self == Cceien::Cceien1
    }
}
#[doc = "Field `CCEIEN` writer - Command CRC Error Interrupt Enable"]
pub type CceienW<'a, REG> = crate::BitWriter<'a, REG, Cceien>;
impl<'a, REG> CceienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cceien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cceien::Cceien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cceien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cceien::Cceien1)
    }
}
#[doc = "Command End Bit Error Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cebeien {
    #[doc = "0: Masked"]
    Cebeien0 = 0,
    #[doc = "1: Enabled"]
    Cebeien1 = 1,
}
impl From<Cebeien> for bool {
    #[inline(always)]
    fn from(variant: Cebeien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEBEIEN` reader - Command End Bit Error Interrupt Enable"]
pub type CebeienR = crate::BitReader<Cebeien>;
impl CebeienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cebeien {
        match self.bits {
            false => Cebeien::Cebeien0,
            true => Cebeien::Cebeien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_cebeien_0(&self) -> bool {
        *self == Cebeien::Cebeien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_cebeien_1(&self) -> bool {
        *self == Cebeien::Cebeien1
    }
}
#[doc = "Field `CEBEIEN` writer - Command End Bit Error Interrupt Enable"]
pub type CebeienW<'a, REG> = crate::BitWriter<'a, REG, Cebeien>;
impl<'a, REG> CebeienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cebeien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cebeien::Cebeien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cebeien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cebeien::Cebeien1)
    }
}
#[doc = "Command Index Error Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cieien {
    #[doc = "0: Masked"]
    Cieien0 = 0,
    #[doc = "1: Enabled"]
    Cieien1 = 1,
}
impl From<Cieien> for bool {
    #[inline(always)]
    fn from(variant: Cieien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIEIEN` reader - Command Index Error Interrupt Enable"]
pub type CieienR = crate::BitReader<Cieien>;
impl CieienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cieien {
        match self.bits {
            false => Cieien::Cieien0,
            true => Cieien::Cieien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_cieien_0(&self) -> bool {
        *self == Cieien::Cieien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_cieien_1(&self) -> bool {
        *self == Cieien::Cieien1
    }
}
#[doc = "Field `CIEIEN` writer - Command Index Error Interrupt Enable"]
pub type CieienW<'a, REG> = crate::BitWriter<'a, REG, Cieien>;
impl<'a, REG> CieienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cieien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cieien::Cieien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cieien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cieien::Cieien1)
    }
}
#[doc = "Data Timeout Error Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtoeien {
    #[doc = "0: Masked"]
    Dtoeien0 = 0,
    #[doc = "1: Enabled"]
    Dtoeien1 = 1,
}
impl From<Dtoeien> for bool {
    #[inline(always)]
    fn from(variant: Dtoeien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTOEIEN` reader - Data Timeout Error Interrupt Enable"]
pub type DtoeienR = crate::BitReader<Dtoeien>;
impl DtoeienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtoeien {
        match self.bits {
            false => Dtoeien::Dtoeien0,
            true => Dtoeien::Dtoeien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_dtoeien_0(&self) -> bool {
        *self == Dtoeien::Dtoeien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_dtoeien_1(&self) -> bool {
        *self == Dtoeien::Dtoeien1
    }
}
#[doc = "Field `DTOEIEN` writer - Data Timeout Error Interrupt Enable"]
pub type DtoeienW<'a, REG> = crate::BitWriter<'a, REG, Dtoeien>;
impl<'a, REG> DtoeienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dtoeien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtoeien::Dtoeien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dtoeien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtoeien::Dtoeien1)
    }
}
#[doc = "Data CRC Error Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dceien {
    #[doc = "0: Masked"]
    Dceien0 = 0,
    #[doc = "1: Enabled"]
    Dceien1 = 1,
}
impl From<Dceien> for bool {
    #[inline(always)]
    fn from(variant: Dceien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEIEN` reader - Data CRC Error Interrupt Enable"]
pub type DceienR = crate::BitReader<Dceien>;
impl DceienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dceien {
        match self.bits {
            false => Dceien::Dceien0,
            true => Dceien::Dceien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_dceien_0(&self) -> bool {
        *self == Dceien::Dceien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_dceien_1(&self) -> bool {
        *self == Dceien::Dceien1
    }
}
#[doc = "Field `DCEIEN` writer - Data CRC Error Interrupt Enable"]
pub type DceienW<'a, REG> = crate::BitWriter<'a, REG, Dceien>;
impl<'a, REG> DceienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dceien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dceien::Dceien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dceien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dceien::Dceien1)
    }
}
#[doc = "Data End Bit Error Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debeien {
    #[doc = "0: Masked"]
    Debeien0 = 0,
    #[doc = "1: Enabled"]
    Debeien1 = 1,
}
impl From<Debeien> for bool {
    #[inline(always)]
    fn from(variant: Debeien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBEIEN` reader - Data End Bit Error Interrupt Enable"]
pub type DebeienR = crate::BitReader<Debeien>;
impl DebeienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debeien {
        match self.bits {
            false => Debeien::Debeien0,
            true => Debeien::Debeien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_debeien_0(&self) -> bool {
        *self == Debeien::Debeien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_debeien_1(&self) -> bool {
        *self == Debeien::Debeien1
    }
}
#[doc = "Field `DEBEIEN` writer - Data End Bit Error Interrupt Enable"]
pub type DebeienW<'a, REG> = crate::BitWriter<'a, REG, Debeien>;
impl<'a, REG> DebeienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn debeien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Debeien::Debeien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn debeien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Debeien::Debeien1)
    }
}
#[doc = "Auto CMD12 Error Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ac12eien {
    #[doc = "0: Masked"]
    Ac12eien0 = 0,
    #[doc = "1: Enabled"]
    Ac12eien1 = 1,
}
impl From<Ac12eien> for bool {
    #[inline(always)]
    fn from(variant: Ac12eien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AC12EIEN` reader - Auto CMD12 Error Interrupt Enable"]
pub type Ac12eienR = crate::BitReader<Ac12eien>;
impl Ac12eienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ac12eien {
        match self.bits {
            false => Ac12eien::Ac12eien0,
            true => Ac12eien::Ac12eien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_ac12eien_0(&self) -> bool {
        *self == Ac12eien::Ac12eien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_ac12eien_1(&self) -> bool {
        *self == Ac12eien::Ac12eien1
    }
}
#[doc = "Field `AC12EIEN` writer - Auto CMD12 Error Interrupt Enable"]
pub type Ac12eienW<'a, REG> = crate::BitWriter<'a, REG, Ac12eien>;
impl<'a, REG> Ac12eienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ac12eien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ac12eien::Ac12eien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ac12eien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ac12eien::Ac12eien1)
    }
}
#[doc = "Tuning Error Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tneien {
    #[doc = "0: Masked"]
    Tneien0 = 0,
    #[doc = "1: Enabled"]
    Tneien1 = 1,
}
impl From<Tneien> for bool {
    #[inline(always)]
    fn from(variant: Tneien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNEIEN` reader - Tuning Error Interrupt Enable"]
pub type TneienR = crate::BitReader<Tneien>;
impl TneienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tneien {
        match self.bits {
            false => Tneien::Tneien0,
            true => Tneien::Tneien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_tneien_0(&self) -> bool {
        *self == Tneien::Tneien0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_tneien_1(&self) -> bool {
        *self == Tneien::Tneien1
    }
}
#[doc = "Field `TNEIEN` writer - Tuning Error Interrupt Enable"]
pub type TneienW<'a, REG> = crate::BitWriter<'a, REG, Tneien>;
impl<'a, REG> TneienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tneien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tneien::Tneien0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tneien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tneien::Tneien1)
    }
}
#[doc = "DMA Error Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaeien {
    #[doc = "0: Masked"]
    Dmaeien0 = 0,
    #[doc = "1: Enable"]
    Dmaeien1 = 1,
}
impl From<Dmaeien> for bool {
    #[inline(always)]
    fn from(variant: Dmaeien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEIEN` reader - DMA Error Interrupt Enable"]
pub type DmaeienR = crate::BitReader<Dmaeien>;
impl DmaeienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaeien {
        match self.bits {
            false => Dmaeien::Dmaeien0,
            true => Dmaeien::Dmaeien1,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_dmaeien_0(&self) -> bool {
        *self == Dmaeien::Dmaeien0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_dmaeien_1(&self) -> bool {
        *self == Dmaeien::Dmaeien1
    }
}
#[doc = "Field `DMAEIEN` writer - DMA Error Interrupt Enable"]
pub type DmaeienW<'a, REG> = crate::BitWriter<'a, REG, Dmaeien>;
impl<'a, REG> DmaeienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dmaeien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaeien::Dmaeien0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmaeien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaeien::Dmaeien1)
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccien(&self) -> CcienR {
        CcienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TcienR {
        TcienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Interrupt Enable"]
    #[inline(always)]
    pub fn bgeien(&self) -> BgeienR {
        BgeienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dintien(&self) -> DintienR {
        DintienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bwrien(&self) -> BwrienR {
        BwrienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Interrupt Enable"]
    #[inline(always)]
    pub fn brrien(&self) -> BrrienR {
        BrrienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Interrupt Enable"]
    #[inline(always)]
    pub fn cinsien(&self) -> CinsienR {
        CinsienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Interrupt Enable"]
    #[inline(always)]
    pub fn crmien(&self) -> CrmienR {
        CrmienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Interrupt Enable"]
    #[inline(always)]
    pub fn cintien(&self) -> CintienR {
        CintienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event Interrupt Enable"]
    #[inline(always)]
    pub fn rteien(&self) -> RteienR {
        RteienR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Tuning Pass Interrupt Enable"]
    #[inline(always)]
    pub fn tpien(&self) -> TpienR {
        TpienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn ctoeien(&self) -> CtoeienR {
        CtoeienR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn cceien(&self) -> CceienR {
        CceienR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn cebeien(&self) -> CebeienR {
        CebeienR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Interrupt Enable"]
    #[inline(always)]
    pub fn cieien(&self) -> CieienR {
        CieienR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn dtoeien(&self) -> DtoeienR {
        DtoeienR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dceien(&self) -> DceienR {
        DceienR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn debeien(&self) -> DebeienR {
        DebeienR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Interrupt Enable"]
    #[inline(always)]
    pub fn ac12eien(&self) -> Ac12eienR {
        Ac12eienR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Interrupt Enable"]
    #[inline(always)]
    pub fn tneien(&self) -> TneienR {
        TneienR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Error Interrupt Enable"]
    #[inline(always)]
    pub fn dmaeien(&self) -> DmaeienR {
        DmaeienR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_SIGNAL_EN")
            .field("ccien", &self.ccien())
            .field("tcien", &self.tcien())
            .field("bgeien", &self.bgeien())
            .field("dintien", &self.dintien())
            .field("bwrien", &self.bwrien())
            .field("brrien", &self.brrien())
            .field("cinsien", &self.cinsien())
            .field("crmien", &self.crmien())
            .field("cintien", &self.cintien())
            .field("rteien", &self.rteien())
            .field("tpien", &self.tpien())
            .field("ctoeien", &self.ctoeien())
            .field("cceien", &self.cceien())
            .field("cebeien", &self.cebeien())
            .field("cieien", &self.cieien())
            .field("dtoeien", &self.dtoeien())
            .field("dceien", &self.dceien())
            .field("debeien", &self.debeien())
            .field("ac12eien", &self.ac12eien())
            .field("tneien", &self.tneien())
            .field("dmaeien", &self.dmaeien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccien(&mut self) -> CcienW<IntSignalEnSpec> {
        CcienW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcien(&mut self) -> TcienW<IntSignalEnSpec> {
        TcienW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Interrupt Enable"]
    #[inline(always)]
    pub fn bgeien(&mut self) -> BgeienW<IntSignalEnSpec> {
        BgeienW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dintien(&mut self) -> DintienW<IntSignalEnSpec> {
        DintienW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bwrien(&mut self) -> BwrienW<IntSignalEnSpec> {
        BwrienW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Interrupt Enable"]
    #[inline(always)]
    pub fn brrien(&mut self) -> BrrienW<IntSignalEnSpec> {
        BrrienW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Interrupt Enable"]
    #[inline(always)]
    pub fn cinsien(&mut self) -> CinsienW<IntSignalEnSpec> {
        CinsienW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Interrupt Enable"]
    #[inline(always)]
    pub fn crmien(&mut self) -> CrmienW<IntSignalEnSpec> {
        CrmienW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Interrupt Enable"]
    #[inline(always)]
    pub fn cintien(&mut self) -> CintienW<IntSignalEnSpec> {
        CintienW::new(self, 8)
    }
    #[doc = "Bit 12 - Re-Tuning Event Interrupt Enable"]
    #[inline(always)]
    pub fn rteien(&mut self) -> RteienW<IntSignalEnSpec> {
        RteienW::new(self, 12)
    }
    #[doc = "Bit 14 - Tuning Pass Interrupt Enable"]
    #[inline(always)]
    pub fn tpien(&mut self) -> TpienW<IntSignalEnSpec> {
        TpienW::new(self, 14)
    }
    #[doc = "Bit 16 - Command Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn ctoeien(&mut self) -> CtoeienW<IntSignalEnSpec> {
        CtoeienW::new(self, 16)
    }
    #[doc = "Bit 17 - Command CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn cceien(&mut self) -> CceienW<IntSignalEnSpec> {
        CceienW::new(self, 17)
    }
    #[doc = "Bit 18 - Command End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn cebeien(&mut self) -> CebeienW<IntSignalEnSpec> {
        CebeienW::new(self, 18)
    }
    #[doc = "Bit 19 - Command Index Error Interrupt Enable"]
    #[inline(always)]
    pub fn cieien(&mut self) -> CieienW<IntSignalEnSpec> {
        CieienW::new(self, 19)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn dtoeien(&mut self) -> DtoeienW<IntSignalEnSpec> {
        DtoeienW::new(self, 20)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dceien(&mut self) -> DceienW<IntSignalEnSpec> {
        DceienW::new(self, 21)
    }
    #[doc = "Bit 22 - Data End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn debeien(&mut self) -> DebeienW<IntSignalEnSpec> {
        DebeienW::new(self, 22)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Interrupt Enable"]
    #[inline(always)]
    pub fn ac12eien(&mut self) -> Ac12eienW<IntSignalEnSpec> {
        Ac12eienW::new(self, 24)
    }
    #[doc = "Bit 26 - Tuning Error Interrupt Enable"]
    #[inline(always)]
    pub fn tneien(&mut self) -> TneienW<IntSignalEnSpec> {
        TneienW::new(self, 26)
    }
    #[doc = "Bit 28 - DMA Error Interrupt Enable"]
    #[inline(always)]
    pub fn dmaeien(&mut self) -> DmaeienW<IntSignalEnSpec> {
        DmaeienW::new(self, 28)
    }
}
#[doc = "Interrupt Signal Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`int_signal_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_signal_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSignalEnSpec;
impl crate::RegisterSpec for IntSignalEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_signal_en::R`](R) reader structure"]
impl crate::Readable for IntSignalEnSpec {}
#[doc = "`write(|w| ..)` method takes [`int_signal_en::W`](W) writer structure"]
impl crate::Writable for IntSignalEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_SIGNAL_EN to value 0"]
impl crate::Resettable for IntSignalEnSpec {
    const RESET_VALUE: u32 = 0;
}
