#[doc = "Register `PWD` reader"]
pub type R = crate::R<PwdSpec>;
#[doc = "Register `PWD` writer"]
pub type W = crate::W<PwdSpec>;
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txpwdfs {
    #[doc = "0: Normal operation."]
    Txpwdfs0 = 0,
    #[doc = "1: Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    Txpwdfs1 = 1,
}
impl From<Txpwdfs> for bool {
    #[inline(always)]
    fn from(variant: Txpwdfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPWDFS` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TxpwdfsR = crate::BitReader<Txpwdfs>;
impl TxpwdfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txpwdfs {
        match self.bits {
            false => Txpwdfs::Txpwdfs0,
            true => Txpwdfs::Txpwdfs1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_txpwdfs_0(&self) -> bool {
        *self == Txpwdfs::Txpwdfs0
    }
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    #[inline(always)]
    pub fn is_txpwdfs_1(&self) -> bool {
        *self == Txpwdfs::Txpwdfs1
    }
}
#[doc = "Field `TXPWDFS` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TxpwdfsW<'a, REG> = crate::BitWriter<'a, REG, Txpwdfs>;
impl<'a, REG> TxpwdfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn txpwdfs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdfs::Txpwdfs0)
    }
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    #[inline(always)]
    pub fn txpwdfs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdfs::Txpwdfs1)
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txpwdibias {
    #[doc = "0: Normal operation."]
    Txpwdibias0 = 0,
    #[doc = "1: Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    Txpwdibias1 = 1,
}
impl From<Txpwdibias> for bool {
    #[inline(always)]
    fn from(variant: Txpwdibias) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPWDIBIAS` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TxpwdibiasR = crate::BitReader<Txpwdibias>;
impl TxpwdibiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txpwdibias {
        match self.bits {
            false => Txpwdibias::Txpwdibias0,
            true => Txpwdibias::Txpwdibias1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_txpwdibias_0(&self) -> bool {
        *self == Txpwdibias::Txpwdibias0
    }
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    #[inline(always)]
    pub fn is_txpwdibias_1(&self) -> bool {
        *self == Txpwdibias::Txpwdibias1
    }
}
#[doc = "Field `TXPWDIBIAS` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TxpwdibiasW<'a, REG> = crate::BitWriter<'a, REG, Txpwdibias>;
impl<'a, REG> TxpwdibiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn txpwdibias_0(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdibias::Txpwdibias0)
    }
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    #[inline(always)]
    pub fn txpwdibias_1(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdibias::Txpwdibias1)
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txpwdv2i {
    #[doc = "0: Normal operation."]
    Txpwdv2i0 = 0,
    #[doc = "1: Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    Txpwdv2i1 = 1,
}
impl From<Txpwdv2i> for bool {
    #[inline(always)]
    fn from(variant: Txpwdv2i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPWDV2I` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type Txpwdv2iR = crate::BitReader<Txpwdv2i>;
impl Txpwdv2iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txpwdv2i {
        match self.bits {
            false => Txpwdv2i::Txpwdv2i0,
            true => Txpwdv2i::Txpwdv2i1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_txpwdv2i_0(&self) -> bool {
        *self == Txpwdv2i::Txpwdv2i0
    }
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    #[inline(always)]
    pub fn is_txpwdv2i_1(&self) -> bool {
        *self == Txpwdv2i::Txpwdv2i1
    }
}
#[doc = "Field `TXPWDV2I` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type Txpwdv2iW<'a, REG> = crate::BitWriter<'a, REG, Txpwdv2i>;
impl<'a, REG> Txpwdv2iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn txpwdv2i_0(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdv2i::Txpwdv2i0)
    }
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    #[inline(always)]
    pub fn txpwdv2i_1(self) -> &'a mut crate::W<REG> {
        self.variant(Txpwdv2i::Txpwdv2i1)
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpwdenv {
    #[doc = "0: Normal operation."]
    Rxpwdenv0 = 0,
    #[doc = "1: Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    Rxpwdenv1 = 1,
}
impl From<Rxpwdenv> for bool {
    #[inline(always)]
    fn from(variant: Rxpwdenv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPWDENV` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwdenvR = crate::BitReader<Rxpwdenv>;
impl RxpwdenvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpwdenv {
        match self.bits {
            false => Rxpwdenv::Rxpwdenv0,
            true => Rxpwdenv::Rxpwdenv1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_rxpwdenv_0(&self) -> bool {
        *self == Rxpwdenv::Rxpwdenv0
    }
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    #[inline(always)]
    pub fn is_rxpwdenv_1(&self) -> bool {
        *self == Rxpwdenv::Rxpwdenv1
    }
}
#[doc = "Field `RXPWDENV` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwdenvW<'a, REG> = crate::BitWriter<'a, REG, Rxpwdenv>;
impl<'a, REG> RxpwdenvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn rxpwdenv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwdenv::Rxpwdenv0)
    }
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    #[inline(always)]
    pub fn rxpwdenv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwdenv::Rxpwdenv1)
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpwd1pt1 {
    #[doc = "0: Normal operation."]
    Rxpwd1pt1_0 = 0,
    #[doc = "1: Power-down the USB full-speed differential receiver."]
    Rxpwd1pt1_1 = 1,
}
impl From<Rxpwd1pt1> for bool {
    #[inline(always)]
    fn from(variant: Rxpwd1pt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPWD1PT1` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type Rxpwd1pt1R = crate::BitReader<Rxpwd1pt1>;
impl Rxpwd1pt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpwd1pt1 {
        match self.bits {
            false => Rxpwd1pt1::Rxpwd1pt1_0,
            true => Rxpwd1pt1::Rxpwd1pt1_1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_rxpwd1pt1_0(&self) -> bool {
        *self == Rxpwd1pt1::Rxpwd1pt1_0
    }
    #[doc = "Power-down the USB full-speed differential receiver."]
    #[inline(always)]
    pub fn is_rxpwd1pt1_1(&self) -> bool {
        *self == Rxpwd1pt1::Rxpwd1pt1_1
    }
}
#[doc = "Field `RXPWD1PT1` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type Rxpwd1pt1W<'a, REG> = crate::BitWriter<'a, REG, Rxpwd1pt1>;
impl<'a, REG> Rxpwd1pt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn rxpwd1pt1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwd1pt1::Rxpwd1pt1_0)
    }
    #[doc = "Power-down the USB full-speed differential receiver."]
    #[inline(always)]
    pub fn rxpwd1pt1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwd1pt1::Rxpwd1pt1_1)
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpwddiff {
    #[doc = "0: Normal operation."]
    Rxpwddiff0 = 0,
    #[doc = "1: Power-down the USB high-speed differential receive"]
    Rxpwddiff1 = 1,
}
impl From<Rxpwddiff> for bool {
    #[inline(always)]
    fn from(variant: Rxpwddiff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPWDDIFF` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwddiffR = crate::BitReader<Rxpwddiff>;
impl RxpwddiffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpwddiff {
        match self.bits {
            false => Rxpwddiff::Rxpwddiff0,
            true => Rxpwddiff::Rxpwddiff1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_rxpwddiff_0(&self) -> bool {
        *self == Rxpwddiff::Rxpwddiff0
    }
    #[doc = "Power-down the USB high-speed differential receive"]
    #[inline(always)]
    pub fn is_rxpwddiff_1(&self) -> bool {
        *self == Rxpwddiff::Rxpwddiff1
    }
}
#[doc = "Field `RXPWDDIFF` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwddiffW<'a, REG> = crate::BitWriter<'a, REG, Rxpwddiff>;
impl<'a, REG> RxpwddiffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn rxpwddiff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwddiff::Rxpwddiff0)
    }
    #[doc = "Power-down the USB high-speed differential receive"]
    #[inline(always)]
    pub fn rxpwddiff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwddiff::Rxpwddiff1)
    }
}
#[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpwdrx {
    #[doc = "0: Normal operation."]
    Rxpwdrx0 = 0,
    #[doc = "1: Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    Rxpwdrx1 = 1,
}
impl From<Rxpwdrx> for bool {
    #[inline(always)]
    fn from(variant: Rxpwdrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPWDRX` reader - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwdrxR = crate::BitReader<Rxpwdrx>;
impl RxpwdrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpwdrx {
        match self.bits {
            false => Rxpwdrx::Rxpwdrx0,
            true => Rxpwdrx::Rxpwdrx1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_rxpwdrx_0(&self) -> bool {
        *self == Rxpwdrx::Rxpwdrx0
    }
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    #[inline(always)]
    pub fn is_rxpwdrx_1(&self) -> bool {
        *self == Rxpwdrx::Rxpwdrx1
    }
}
#[doc = "Field `RXPWDRX` writer - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RxpwdrxW<'a, REG> = crate::BitWriter<'a, REG, Rxpwdrx>;
impl<'a, REG> RxpwdrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn rxpwdrx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwdrx::Rxpwdrx0)
    }
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    #[inline(always)]
    pub fn rxpwdrx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpwdrx::Rxpwdrx1)
    }
}
impl R {
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdfs(&self) -> TxpwdfsR {
        TxpwdfsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdibias(&self) -> TxpwdibiasR {
        TxpwdibiasR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdv2i(&self) -> Txpwdv2iR {
        Txpwdv2iR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdenv(&self) -> RxpwdenvR {
        RxpwdenvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwd1pt1(&self) -> Rxpwd1pt1R {
        Rxpwd1pt1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwddiff(&self) -> RxpwddiffR {
        RxpwddiffR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdrx(&self) -> RxpwdrxR {
        RxpwdrxR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD")
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn txpwdfs(&mut self) -> TxpwdfsW<PwdSpec> {
        TxpwdfsW::new(self, 10)
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn txpwdibias(&mut self) -> TxpwdibiasW<PwdSpec> {
        TxpwdibiasW::new(self, 11)
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn txpwdv2i(&mut self) -> Txpwdv2iW<PwdSpec> {
        Txpwdv2iW::new(self, 12)
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwdenv(&mut self) -> RxpwdenvW<PwdSpec> {
        RxpwdenvW::new(self, 17)
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwd1pt1(&mut self) -> Rxpwd1pt1W<PwdSpec> {
        Rxpwd1pt1W::new(self, 18)
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwddiff(&mut self) -> RxpwddiffW<PwdSpec> {
        RxpwddiffW::new(self, 19)
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwdrx(&mut self) -> RxpwdrxW<PwdSpec> {
        RxpwdrxW::new(self, 20)
    }
}
#[doc = "USB PHY Power-Down Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdSpec;
impl crate::RegisterSpec for PwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwd::R`](R) reader structure"]
impl crate::Readable for PwdSpec {}
#[doc = "`write(|w| ..)` method takes [`pwd::W`](W) writer structure"]
impl crate::Writable for PwdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWD to value 0x001e_1c00"]
impl crate::Resettable for PwdSpec {
    const RESET_VALUE: u32 = 0x001e_1c00;
}
