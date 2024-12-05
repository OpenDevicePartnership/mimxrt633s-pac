#[doc = "Register `INT_STATUS` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Register `INT_STATUS` writer"]
pub type W = crate::W<IntStatusSpec>;
#[doc = "Command Complete\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc {
    #[doc = "0: Command not complete"]
    Cc0 = 0,
    #[doc = "1: Command complete"]
    Cc1 = 1,
}
impl From<Cc> for bool {
    #[inline(always)]
    fn from(variant: Cc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC` reader - Command Complete"]
pub type CcR = crate::BitReader<Cc>;
impl CcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc {
        match self.bits {
            false => Cc::Cc0,
            true => Cc::Cc1,
        }
    }
    #[doc = "Command not complete"]
    #[inline(always)]
    pub fn is_cc_0(&self) -> bool {
        *self == Cc::Cc0
    }
    #[doc = "Command complete"]
    #[inline(always)]
    pub fn is_cc_1(&self) -> bool {
        *self == Cc::Cc1
    }
}
#[doc = "Field `CC` writer - Command Complete"]
pub type CcW<'a, REG> = crate::BitWriter1C<'a, REG, Cc>;
impl<'a, REG> CcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command not complete"]
    #[inline(always)]
    pub fn cc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::Cc0)
    }
    #[doc = "Command complete"]
    #[inline(always)]
    pub fn cc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::Cc1)
    }
}
#[doc = "Transfer Complete\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc {
    #[doc = "0: Transfer not complete"]
    Tc0 = 0,
    #[doc = "1: Transfer complete"]
    Tc1 = 1,
}
impl From<Tc> for bool {
    #[inline(always)]
    fn from(variant: Tc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC` reader - Transfer Complete"]
pub type TcR = crate::BitReader<Tc>;
impl TcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc {
        match self.bits {
            false => Tc::Tc0,
            true => Tc::Tc1,
        }
    }
    #[doc = "Transfer not complete"]
    #[inline(always)]
    pub fn is_tc_0(&self) -> bool {
        *self == Tc::Tc0
    }
    #[doc = "Transfer complete"]
    #[inline(always)]
    pub fn is_tc_1(&self) -> bool {
        *self == Tc::Tc1
    }
}
#[doc = "Field `TC` writer - Transfer Complete"]
pub type TcW<'a, REG> = crate::BitWriter1C<'a, REG, Tc>;
impl<'a, REG> TcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer not complete"]
    #[inline(always)]
    pub fn tc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tc::Tc0)
    }
    #[doc = "Transfer complete"]
    #[inline(always)]
    pub fn tc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tc::Tc1)
    }
}
#[doc = "Block Gap Event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bge {
    #[doc = "0: No block gap event"]
    Bge0 = 0,
    #[doc = "1: Transaction stopped at block gap"]
    Bge1 = 1,
}
impl From<Bge> for bool {
    #[inline(always)]
    fn from(variant: Bge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGE` reader - Block Gap Event"]
pub type BgeR = crate::BitReader<Bge>;
impl BgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bge {
        match self.bits {
            false => Bge::Bge0,
            true => Bge::Bge1,
        }
    }
    #[doc = "No block gap event"]
    #[inline(always)]
    pub fn is_bge_0(&self) -> bool {
        *self == Bge::Bge0
    }
    #[doc = "Transaction stopped at block gap"]
    #[inline(always)]
    pub fn is_bge_1(&self) -> bool {
        *self == Bge::Bge1
    }
}
#[doc = "Field `BGE` writer - Block Gap Event"]
pub type BgeW<'a, REG> = crate::BitWriter1C<'a, REG, Bge>;
impl<'a, REG> BgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No block gap event"]
    #[inline(always)]
    pub fn bge_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bge::Bge0)
    }
    #[doc = "Transaction stopped at block gap"]
    #[inline(always)]
    pub fn bge_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bge::Bge1)
    }
}
#[doc = "DMA Interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dint {
    #[doc = "0: No DMA Interrupt"]
    Dint0 = 0,
    #[doc = "1: DMA Interrupt is generated"]
    Dint1 = 1,
}
impl From<Dint> for bool {
    #[inline(always)]
    fn from(variant: Dint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINT` reader - DMA Interrupt"]
pub type DintR = crate::BitReader<Dint>;
impl DintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dint {
        match self.bits {
            false => Dint::Dint0,
            true => Dint::Dint1,
        }
    }
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn is_dint_0(&self) -> bool {
        *self == Dint::Dint0
    }
    #[doc = "DMA Interrupt is generated"]
    #[inline(always)]
    pub fn is_dint_1(&self) -> bool {
        *self == Dint::Dint1
    }
}
#[doc = "Field `DINT` writer - DMA Interrupt"]
pub type DintW<'a, REG> = crate::BitWriter1C<'a, REG, Dint>;
impl<'a, REG> DintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn dint_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dint::Dint0)
    }
    #[doc = "DMA Interrupt is generated"]
    #[inline(always)]
    pub fn dint_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dint::Dint1)
    }
}
#[doc = "Buffer Write Ready\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwr {
    #[doc = "0: Not ready to write buffer"]
    Bwr0 = 0,
    #[doc = "1: Ready to write buffer:"]
    Bwr1 = 1,
}
impl From<Bwr> for bool {
    #[inline(always)]
    fn from(variant: Bwr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWR` reader - Buffer Write Ready"]
pub type BwrR = crate::BitReader<Bwr>;
impl BwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwr {
        match self.bits {
            false => Bwr::Bwr0,
            true => Bwr::Bwr1,
        }
    }
    #[doc = "Not ready to write buffer"]
    #[inline(always)]
    pub fn is_bwr_0(&self) -> bool {
        *self == Bwr::Bwr0
    }
    #[doc = "Ready to write buffer:"]
    #[inline(always)]
    pub fn is_bwr_1(&self) -> bool {
        *self == Bwr::Bwr1
    }
}
#[doc = "Field `BWR` writer - Buffer Write Ready"]
pub type BwrW<'a, REG> = crate::BitWriter1C<'a, REG, Bwr>;
impl<'a, REG> BwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not ready to write buffer"]
    #[inline(always)]
    pub fn bwr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwr::Bwr0)
    }
    #[doc = "Ready to write buffer:"]
    #[inline(always)]
    pub fn bwr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwr::Bwr1)
    }
}
#[doc = "Buffer Read Ready\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brr {
    #[doc = "0: Not ready to read buffer"]
    Brr0 = 0,
    #[doc = "1: Ready to read buffer"]
    Brr1 = 1,
}
impl From<Brr> for bool {
    #[inline(always)]
    fn from(variant: Brr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRR` reader - Buffer Read Ready"]
pub type BrrR = crate::BitReader<Brr>;
impl BrrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brr {
        match self.bits {
            false => Brr::Brr0,
            true => Brr::Brr1,
        }
    }
    #[doc = "Not ready to read buffer"]
    #[inline(always)]
    pub fn is_brr_0(&self) -> bool {
        *self == Brr::Brr0
    }
    #[doc = "Ready to read buffer"]
    #[inline(always)]
    pub fn is_brr_1(&self) -> bool {
        *self == Brr::Brr1
    }
}
#[doc = "Field `BRR` writer - Buffer Read Ready"]
pub type BrrW<'a, REG> = crate::BitWriter1C<'a, REG, Brr>;
impl<'a, REG> BrrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not ready to read buffer"]
    #[inline(always)]
    pub fn brr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Brr::Brr0)
    }
    #[doc = "Ready to read buffer"]
    #[inline(always)]
    pub fn brr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Brr::Brr1)
    }
}
#[doc = "Card Insertion\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cins {
    #[doc = "0: Card state unstable or removed"]
    Cins0 = 0,
    #[doc = "1: Card inserted"]
    Cins1 = 1,
}
impl From<Cins> for bool {
    #[inline(always)]
    fn from(variant: Cins) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINS` reader - Card Insertion"]
pub type CinsR = crate::BitReader<Cins>;
impl CinsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cins {
        match self.bits {
            false => Cins::Cins0,
            true => Cins::Cins1,
        }
    }
    #[doc = "Card state unstable or removed"]
    #[inline(always)]
    pub fn is_cins_0(&self) -> bool {
        *self == Cins::Cins0
    }
    #[doc = "Card inserted"]
    #[inline(always)]
    pub fn is_cins_1(&self) -> bool {
        *self == Cins::Cins1
    }
}
#[doc = "Field `CINS` writer - Card Insertion"]
pub type CinsW<'a, REG> = crate::BitWriter1C<'a, REG, Cins>;
impl<'a, REG> CinsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card state unstable or removed"]
    #[inline(always)]
    pub fn cins_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cins::Cins0)
    }
    #[doc = "Card inserted"]
    #[inline(always)]
    pub fn cins_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cins::Cins1)
    }
}
#[doc = "Card Removal\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crm {
    #[doc = "0: Card state unstable or inserted"]
    Crm0 = 0,
    #[doc = "1: Card removed"]
    Crm1 = 1,
}
impl From<Crm> for bool {
    #[inline(always)]
    fn from(variant: Crm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRM` reader - Card Removal"]
pub type CrmR = crate::BitReader<Crm>;
impl CrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crm {
        match self.bits {
            false => Crm::Crm0,
            true => Crm::Crm1,
        }
    }
    #[doc = "Card state unstable or inserted"]
    #[inline(always)]
    pub fn is_crm_0(&self) -> bool {
        *self == Crm::Crm0
    }
    #[doc = "Card removed"]
    #[inline(always)]
    pub fn is_crm_1(&self) -> bool {
        *self == Crm::Crm1
    }
}
#[doc = "Field `CRM` writer - Card Removal"]
pub type CrmW<'a, REG> = crate::BitWriter1C<'a, REG, Crm>;
impl<'a, REG> CrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card state unstable or inserted"]
    #[inline(always)]
    pub fn crm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Crm::Crm0)
    }
    #[doc = "Card removed"]
    #[inline(always)]
    pub fn crm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Crm::Crm1)
    }
}
#[doc = "Card Interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cint {
    #[doc = "0: No Card Interrupt"]
    Cint0 = 0,
    #[doc = "1: Generate Card Interrupt"]
    Cint1 = 1,
}
impl From<Cint> for bool {
    #[inline(always)]
    fn from(variant: Cint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINT` reader - Card Interrupt"]
pub type CintR = crate::BitReader<Cint>;
impl CintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cint {
        match self.bits {
            false => Cint::Cint0,
            true => Cint::Cint1,
        }
    }
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn is_cint_0(&self) -> bool {
        *self == Cint::Cint0
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn is_cint_1(&self) -> bool {
        *self == Cint::Cint1
    }
}
#[doc = "Field `CINT` writer - Card Interrupt"]
pub type CintW<'a, REG> = crate::BitWriter1C<'a, REG, Cint>;
impl<'a, REG> CintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn cint_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cint::Cint0)
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn cint_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cint::Cint1)
    }
}
#[doc = "Re-Tuning Event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rte {
    #[doc = "0: Re-Tuning is not required"]
    Rte0 = 0,
    #[doc = "1: Re-Tuning should be performed"]
    Rte1 = 1,
}
impl From<Rte> for bool {
    #[inline(always)]
    fn from(variant: Rte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTE` reader - Re-Tuning Event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type RteR = crate::BitReader<Rte>;
impl RteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rte {
        match self.bits {
            false => Rte::Rte0,
            true => Rte::Rte1,
        }
    }
    #[doc = "Re-Tuning is not required"]
    #[inline(always)]
    pub fn is_rte_0(&self) -> bool {
        *self == Rte::Rte0
    }
    #[doc = "Re-Tuning should be performed"]
    #[inline(always)]
    pub fn is_rte_1(&self) -> bool {
        *self == Rte::Rte1
    }
}
#[doc = "Field `RTE` writer - Re-Tuning Event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type RteW<'a, REG> = crate::BitWriter1C<'a, REG, Rte>;
impl<'a, REG> RteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Re-Tuning is not required"]
    #[inline(always)]
    pub fn rte_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rte::Rte0)
    }
    #[doc = "Re-Tuning should be performed"]
    #[inline(always)]
    pub fn rte_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rte::Rte1)
    }
}
#[doc = "Field `TP` reader - Tuning Pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type TpR = crate::BitReader;
#[doc = "Field `TP` writer - Tuning Pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type TpW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Command Timeout Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctoe {
    #[doc = "0: No Error"]
    Ctoe0 = 0,
    #[doc = "1: Time out"]
    Ctoe1 = 1,
}
impl From<Ctoe> for bool {
    #[inline(always)]
    fn from(variant: Ctoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTOE` reader - Command Timeout Error"]
pub type CtoeR = crate::BitReader<Ctoe>;
impl CtoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctoe {
        match self.bits {
            false => Ctoe::Ctoe0,
            true => Ctoe::Ctoe1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_ctoe_0(&self) -> bool {
        *self == Ctoe::Ctoe0
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub fn is_ctoe_1(&self) -> bool {
        *self == Ctoe::Ctoe1
    }
}
#[doc = "Field `CTOE` writer - Command Timeout Error"]
pub type CtoeW<'a, REG> = crate::BitWriter1C<'a, REG, Ctoe>;
impl<'a, REG> CtoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn ctoe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctoe::Ctoe0)
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub fn ctoe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctoe::Ctoe1)
    }
}
#[doc = "Command CRC Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cce {
    #[doc = "0: No Error"]
    Cce0 = 0,
    #[doc = "1: CRC Error Generated."]
    Cce1 = 1,
}
impl From<Cce> for bool {
    #[inline(always)]
    fn from(variant: Cce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` reader - Command CRC Error"]
pub type CceR = crate::BitReader<Cce>;
impl CceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cce {
        match self.bits {
            false => Cce::Cce0,
            true => Cce::Cce1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_cce_0(&self) -> bool {
        *self == Cce::Cce0
    }
    #[doc = "CRC Error Generated."]
    #[inline(always)]
    pub fn is_cce_1(&self) -> bool {
        *self == Cce::Cce1
    }
}
#[doc = "Field `CCE` writer - Command CRC Error"]
pub type CceW<'a, REG> = crate::BitWriter1C<'a, REG, Cce>;
impl<'a, REG> CceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn cce_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cce::Cce0)
    }
    #[doc = "CRC Error Generated."]
    #[inline(always)]
    pub fn cce_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cce::Cce1)
    }
}
#[doc = "Command End Bit Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cebe {
    #[doc = "0: No Error"]
    Cebe0 = 0,
    #[doc = "1: End Bit Error Generated"]
    Cebe1 = 1,
}
impl From<Cebe> for bool {
    #[inline(always)]
    fn from(variant: Cebe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEBE` reader - Command End Bit Error"]
pub type CebeR = crate::BitReader<Cebe>;
impl CebeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cebe {
        match self.bits {
            false => Cebe::Cebe0,
            true => Cebe::Cebe1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_cebe_0(&self) -> bool {
        *self == Cebe::Cebe0
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn is_cebe_1(&self) -> bool {
        *self == Cebe::Cebe1
    }
}
#[doc = "Field `CEBE` writer - Command End Bit Error"]
pub type CebeW<'a, REG> = crate::BitWriter1C<'a, REG, Cebe>;
impl<'a, REG> CebeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn cebe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cebe::Cebe0)
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn cebe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cebe::Cebe1)
    }
}
#[doc = "Command Index Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cie {
    #[doc = "0: No Error"]
    Cie0 = 0,
    #[doc = "1: Error"]
    Cie1 = 1,
}
impl From<Cie> for bool {
    #[inline(always)]
    fn from(variant: Cie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIE` reader - Command Index Error"]
pub type CieR = crate::BitReader<Cie>;
impl CieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cie {
        match self.bits {
            false => Cie::Cie0,
            true => Cie::Cie1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_cie_0(&self) -> bool {
        *self == Cie::Cie0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_cie_1(&self) -> bool {
        *self == Cie::Cie1
    }
}
#[doc = "Field `CIE` writer - Command Index Error"]
pub type CieW<'a, REG> = crate::BitWriter1C<'a, REG, Cie>;
impl<'a, REG> CieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn cie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cie::Cie0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn cie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cie::Cie1)
    }
}
#[doc = "Data Timeout Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtoe {
    #[doc = "0: No Error"]
    Dtoe0 = 0,
    #[doc = "1: Time out"]
    Dtoe1 = 1,
}
impl From<Dtoe> for bool {
    #[inline(always)]
    fn from(variant: Dtoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTOE` reader - Data Timeout Error"]
pub type DtoeR = crate::BitReader<Dtoe>;
impl DtoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtoe {
        match self.bits {
            false => Dtoe::Dtoe0,
            true => Dtoe::Dtoe1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_dtoe_0(&self) -> bool {
        *self == Dtoe::Dtoe0
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub fn is_dtoe_1(&self) -> bool {
        *self == Dtoe::Dtoe1
    }
}
#[doc = "Field `DTOE` writer - Data Timeout Error"]
pub type DtoeW<'a, REG> = crate::BitWriter1C<'a, REG, Dtoe>;
impl<'a, REG> DtoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn dtoe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtoe::Dtoe0)
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub fn dtoe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtoe::Dtoe1)
    }
}
#[doc = "Data CRC Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dce {
    #[doc = "0: No Error"]
    Dce0 = 0,
    #[doc = "1: Error"]
    Dce1 = 1,
}
impl From<Dce> for bool {
    #[inline(always)]
    fn from(variant: Dce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCE` reader - Data CRC Error"]
pub type DceR = crate::BitReader<Dce>;
impl DceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dce {
        match self.bits {
            false => Dce::Dce0,
            true => Dce::Dce1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_dce_0(&self) -> bool {
        *self == Dce::Dce0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_dce_1(&self) -> bool {
        *self == Dce::Dce1
    }
}
#[doc = "Field `DCE` writer - Data CRC Error"]
pub type DceW<'a, REG> = crate::BitWriter1C<'a, REG, Dce>;
impl<'a, REG> DceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn dce_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dce::Dce0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn dce_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dce::Dce1)
    }
}
#[doc = "Data End Bit Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debe {
    #[doc = "0: No Error"]
    Debe0 = 0,
    #[doc = "1: Error"]
    Debe1 = 1,
}
impl From<Debe> for bool {
    #[inline(always)]
    fn from(variant: Debe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBE` reader - Data End Bit Error"]
pub type DebeR = crate::BitReader<Debe>;
impl DebeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debe {
        match self.bits {
            false => Debe::Debe0,
            true => Debe::Debe1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_debe_0(&self) -> bool {
        *self == Debe::Debe0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_debe_1(&self) -> bool {
        *self == Debe::Debe1
    }
}
#[doc = "Field `DEBE` writer - Data End Bit Error"]
pub type DebeW<'a, REG> = crate::BitWriter1C<'a, REG, Debe>;
impl<'a, REG> DebeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn debe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Debe::Debe0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn debe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Debe::Debe1)
    }
}
#[doc = "Auto CMD12 Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ac12e {
    #[doc = "0: No Error"]
    Ac12e0 = 0,
    #[doc = "1: Error"]
    Ac12e1 = 1,
}
impl From<Ac12e> for bool {
    #[inline(always)]
    fn from(variant: Ac12e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AC12E` reader - Auto CMD12 Error"]
pub type Ac12eR = crate::BitReader<Ac12e>;
impl Ac12eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ac12e {
        match self.bits {
            false => Ac12e::Ac12e0,
            true => Ac12e::Ac12e1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_ac12e_0(&self) -> bool {
        *self == Ac12e::Ac12e0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_ac12e_1(&self) -> bool {
        *self == Ac12e::Ac12e1
    }
}
#[doc = "Field `AC12E` writer - Auto CMD12 Error"]
pub type Ac12eW<'a, REG> = crate::BitWriter1C<'a, REG, Ac12e>;
impl<'a, REG> Ac12eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn ac12e_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ac12e::Ac12e0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn ac12e_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ac12e::Ac12e1)
    }
}
#[doc = "Field `TNE` reader - Tuning Error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type TneR = crate::BitReader;
#[doc = "Field `TNE` writer - Tuning Error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type TneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "DMA Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmae {
    #[doc = "0: No Error"]
    Dmae0 = 0,
    #[doc = "1: Error"]
    Dmae1 = 1,
}
impl From<Dmae> for bool {
    #[inline(always)]
    fn from(variant: Dmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAE` reader - DMA Error"]
pub type DmaeR = crate::BitReader<Dmae>;
impl DmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmae {
        match self.bits {
            false => Dmae::Dmae0,
            true => Dmae::Dmae1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_dmae_0(&self) -> bool {
        *self == Dmae::Dmae0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_dmae_1(&self) -> bool {
        *self == Dmae::Dmae1
    }
}
#[doc = "Field `DMAE` writer - DMA Error"]
pub type DmaeW<'a, REG> = crate::BitWriter1C<'a, REG, Dmae>;
impl<'a, REG> DmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn dmae_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmae::Dmae0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn dmae_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmae::Dmae1)
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn bge(&self) -> BgeR {
        BgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dint(&self) -> DintR {
        DintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bwr(&self) -> BwrR {
        BwrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn brr(&self) -> BrrR {
        BrrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cins(&self) -> CinsR {
        CinsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn crm(&self) -> CrmR {
        CrmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cint(&self) -> CintR {
        CintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn rte(&self) -> RteR {
        RteR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Tuning Pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn tp(&self) -> TpR {
        TpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    pub fn ctoe(&self) -> CtoeR {
        CtoeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    pub fn cebe(&self) -> CebeR {
        CebeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    pub fn cie(&self) -> CieR {
        CieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline(always)]
    pub fn dtoe(&self) -> DtoeR {
        DtoeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn dce(&self) -> DceR {
        DceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    pub fn debe(&self) -> DebeR {
        DebeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error"]
    #[inline(always)]
    pub fn ac12e(&self) -> Ac12eR {
        Ac12eR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning Error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn tne(&self) -> TneR {
        TneR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Error"]
    #[inline(always)]
    pub fn dmae(&self) -> DmaeR {
        DmaeR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS")
            .field("cc", &self.cc())
            .field("tc", &self.tc())
            .field("bge", &self.bge())
            .field("dint", &self.dint())
            .field("bwr", &self.bwr())
            .field("brr", &self.brr())
            .field("cins", &self.cins())
            .field("crm", &self.crm())
            .field("cint", &self.cint())
            .field("rte", &self.rte())
            .field("tp", &self.tp())
            .field("ctoe", &self.ctoe())
            .field("cce", &self.cce())
            .field("cebe", &self.cebe())
            .field("cie", &self.cie())
            .field("dtoe", &self.dtoe())
            .field("dce", &self.dce())
            .field("debe", &self.debe())
            .field("ac12e", &self.ac12e())
            .field("tne", &self.tne())
            .field("dmae", &self.dmae())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<IntStatusSpec> {
        CcW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<IntStatusSpec> {
        TcW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn bge(&mut self) -> BgeW<IntStatusSpec> {
        BgeW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dint(&mut self) -> DintW<IntStatusSpec> {
        DintW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bwr(&mut self) -> BwrW<IntStatusSpec> {
        BwrW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn brr(&mut self) -> BrrW<IntStatusSpec> {
        BrrW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cins(&mut self) -> CinsW<IntStatusSpec> {
        CinsW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn crm(&mut self) -> CrmW<IntStatusSpec> {
        CrmW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cint(&mut self) -> CintW<IntStatusSpec> {
        CintW::new(self, 8)
    }
    #[doc = "Bit 12 - Re-Tuning Event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn rte(&mut self) -> RteW<IntStatusSpec> {
        RteW::new(self, 12)
    }
    #[doc = "Bit 14 - Tuning Pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn tp(&mut self) -> TpW<IntStatusSpec> {
        TpW::new(self, 14)
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    pub fn ctoe(&mut self) -> CtoeW<IntStatusSpec> {
        CtoeW::new(self, 16)
    }
    #[doc = "Bit 17 - Command CRC Error"]
    #[inline(always)]
    pub fn cce(&mut self) -> CceW<IntStatusSpec> {
        CceW::new(self, 17)
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    pub fn cebe(&mut self) -> CebeW<IntStatusSpec> {
        CebeW::new(self, 18)
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    pub fn cie(&mut self) -> CieW<IntStatusSpec> {
        CieW::new(self, 19)
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline(always)]
    pub fn dtoe(&mut self) -> DtoeW<IntStatusSpec> {
        DtoeW::new(self, 20)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn dce(&mut self) -> DceW<IntStatusSpec> {
        DceW::new(self, 21)
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    pub fn debe(&mut self) -> DebeW<IntStatusSpec> {
        DebeW::new(self, 22)
    }
    #[doc = "Bit 24 - Auto CMD12 Error"]
    #[inline(always)]
    pub fn ac12e(&mut self) -> Ac12eW<IntStatusSpec> {
        Ac12eW::new(self, 24)
    }
    #[doc = "Bit 26 - Tuning Error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn tne(&mut self) -> TneW<IntStatusSpec> {
        TneW::new(self, 26)
    }
    #[doc = "Bit 28 - DMA Error"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DmaeW<IntStatusSpec> {
        DmaeW::new(self, 28)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`int_status::W`](W) writer structure"]
impl crate::Writable for IntStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x157f_51ff;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for IntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
