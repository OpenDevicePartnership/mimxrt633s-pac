#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Fn\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fn {
    #[doc = "0: Clears the Fn bit in the SR register."]
    Fn0 = 0,
    #[doc = "1: Sets the Fn bit in the SR register."]
    Fn1 = 1,
}
impl From<Fn> for u8 {
    #[inline(always)]
    fn from(variant: Fn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fn {
    type Ux = u8;
}
impl crate::IsEnum for Fn {}
#[doc = "Field `Fn` reader - Fn"]
pub type FnR = crate::FieldReader<Fn>;
impl FnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fn> {
        match self.bits {
            0 => Some(Fn::Fn0),
            1 => Some(Fn::Fn1),
            _ => None,
        }
    }
    #[doc = "Clears the Fn bit in the SR register."]
    #[inline(always)]
    pub fn is_fn_0(&self) -> bool {
        *self == Fn::Fn0
    }
    #[doc = "Sets the Fn bit in the SR register."]
    #[inline(always)]
    pub fn is_fn_1(&self) -> bool {
        *self == Fn::Fn1
    }
}
#[doc = "Field `Fn` writer - Fn"]
pub type FnW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fn>;
impl<'a, REG> FnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clears the Fn bit in the SR register."]
    #[inline(always)]
    pub fn fn_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fn::Fn0)
    }
    #[doc = "Sets the Fn bit in the SR register."]
    #[inline(always)]
    pub fn fn_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fn::Fn1)
    }
}
#[doc = "MUR\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mur {
    #[doc = "0: N/A. Self clearing bit (default)."]
    Mur0 = 0,
    #[doc = "1: Asserts the MU reset."]
    Mur1 = 1,
}
impl From<Mur> for bool {
    #[inline(always)]
    fn from(variant: Mur) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUR` reader - MUR"]
pub type MurR = crate::BitReader<Mur>;
impl MurR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mur {
        match self.bits {
            false => Mur::Mur0,
            true => Mur::Mur1,
        }
    }
    #[doc = "N/A. Self clearing bit (default)."]
    #[inline(always)]
    pub fn is_mur_0(&self) -> bool {
        *self == Mur::Mur0
    }
    #[doc = "Asserts the MU reset."]
    #[inline(always)]
    pub fn is_mur_1(&self) -> bool {
        *self == Mur::Mur1
    }
}
#[doc = "Field `MUR` writer - MUR"]
pub type MurW<'a, REG> = crate::BitWriter<'a, REG, Mur>;
impl<'a, REG> MurW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "N/A. Self clearing bit (default)."]
    #[inline(always)]
    pub fn mur_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mur::Mur0)
    }
    #[doc = "Asserts the MU reset."]
    #[inline(always)]
    pub fn mur_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mur::Mur1)
    }
}
#[doc = "BRDIE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdie {
    #[doc = "0: Disables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    Rdie0 = 0,
    #[doc = "1: Enables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    Rdie1 = 1,
}
impl From<Rdie> for bool {
    #[inline(always)]
    fn from(variant: Rdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIE` reader - BRDIE"]
pub type RdieR = crate::BitReader<Rdie>;
impl RdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdie {
        match self.bits {
            false => Rdie::Rdie0,
            true => Rdie::Rdie1,
        }
    }
    #[doc = "Disables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    #[inline(always)]
    pub fn is_rdie_0(&self) -> bool {
        *self == Rdie::Rdie0
    }
    #[doc = "Enables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    #[inline(always)]
    pub fn is_rdie_1(&self) -> bool {
        *self == Rdie::Rdie1
    }
}
#[doc = "Field `RDIE` writer - BRDIE"]
pub type RdieW<'a, REG> = crate::BitWriter<'a, REG, Rdie>;
impl<'a, REG> RdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    #[inline(always)]
    pub fn rdie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdie::Rdie0)
    }
    #[doc = "Enables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    #[inline(always)]
    pub fn rdie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdie::Rdie1)
    }
}
#[doc = "RAIE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Raie {
    #[doc = "0: Disables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    Raie0 = 0,
    #[doc = "1: Enables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    Raie1 = 1,
}
impl From<Raie> for bool {
    #[inline(always)]
    fn from(variant: Raie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAIE` reader - RAIE"]
pub type RaieR = crate::BitReader<Raie>;
impl RaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Raie {
        match self.bits {
            false => Raie::Raie0,
            true => Raie::Raie1,
        }
    }
    #[doc = "Disables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    #[inline(always)]
    pub fn is_raie_0(&self) -> bool {
        *self == Raie::Raie0
    }
    #[doc = "Enables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    #[inline(always)]
    pub fn is_raie_1(&self) -> bool {
        *self == Raie::Raie1
    }
}
#[doc = "Field `RAIE` writer - RAIE"]
pub type RaieW<'a, REG> = crate::BitWriter<'a, REG, Raie>;
impl<'a, REG> RaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    #[inline(always)]
    pub fn raie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Raie::Raie0)
    }
    #[doc = "Enables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    #[inline(always)]
    pub fn raie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Raie::Raie1)
    }
}
#[doc = "GIRn\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Girn {
    #[doc = "0: MUA General Interrupt n is not requested to the MUB (default)."]
    Girn0 = 0,
    #[doc = "1: MUA General Interrupt n is requested to the MUB."]
    Girn1 = 1,
}
impl From<Girn> for u8 {
    #[inline(always)]
    fn from(variant: Girn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Girn {
    type Ux = u8;
}
impl crate::IsEnum for Girn {}
#[doc = "Field `GIRn` reader - GIRn"]
pub type GirnR = crate::FieldReader<Girn>;
impl GirnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Girn> {
        match self.bits {
            0 => Some(Girn::Girn0),
            1 => Some(Girn::Girn1),
            _ => None,
        }
    }
    #[doc = "MUA General Interrupt n is not requested to the MUB (default)."]
    #[inline(always)]
    pub fn is_girn_0(&self) -> bool {
        *self == Girn::Girn0
    }
    #[doc = "MUA General Interrupt n is requested to the MUB."]
    #[inline(always)]
    pub fn is_girn_1(&self) -> bool {
        *self == Girn::Girn1
    }
}
#[doc = "Field `GIRn` writer - GIRn"]
pub type GirnW<'a, REG> = crate::FieldWriter<'a, REG, 4, Girn>;
impl<'a, REG> GirnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MUA General Interrupt n is not requested to the MUB (default)."]
    #[inline(always)]
    pub fn girn_0(self) -> &'a mut crate::W<REG> {
        self.variant(Girn::Girn0)
    }
    #[doc = "MUA General Interrupt n is requested to the MUB."]
    #[inline(always)]
    pub fn girn_1(self) -> &'a mut crate::W<REG> {
        self.variant(Girn::Girn1)
    }
}
#[doc = "TIEn\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tien {
    #[doc = "0: Disables MUA Transmit Interrupt n. (default)"]
    Tien0 = 0,
    #[doc = "1: Enables MUA Transmit Interrupt n."]
    Tien1 = 1,
}
impl From<Tien> for u8 {
    #[inline(always)]
    fn from(variant: Tien) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tien {
    type Ux = u8;
}
impl crate::IsEnum for Tien {}
#[doc = "Field `TIEn` reader - TIEn"]
pub type TienR = crate::FieldReader<Tien>;
impl TienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tien> {
        match self.bits {
            0 => Some(Tien::Tien0),
            1 => Some(Tien::Tien1),
            _ => None,
        }
    }
    #[doc = "Disables MUA Transmit Interrupt n. (default)"]
    #[inline(always)]
    pub fn is_tien_0(&self) -> bool {
        *self == Tien::Tien0
    }
    #[doc = "Enables MUA Transmit Interrupt n."]
    #[inline(always)]
    pub fn is_tien_1(&self) -> bool {
        *self == Tien::Tien1
    }
}
#[doc = "Field `TIEn` writer - TIEn"]
pub type TienW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tien>;
impl<'a, REG> TienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disables MUA Transmit Interrupt n. (default)"]
    #[inline(always)]
    pub fn tien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tien::Tien0)
    }
    #[doc = "Enables MUA Transmit Interrupt n."]
    #[inline(always)]
    pub fn tien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tien::Tien1)
    }
}
#[doc = "RIEn\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rien {
    #[doc = "0: Disables MUA Receive Interrupt n. (default)"]
    Rien0 = 0,
    #[doc = "1: Enables MUA Receive Interrupt n."]
    Rien1 = 1,
}
impl From<Rien> for u8 {
    #[inline(always)]
    fn from(variant: Rien) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rien {
    type Ux = u8;
}
impl crate::IsEnum for Rien {}
#[doc = "Field `RIEn` reader - RIEn"]
pub type RienR = crate::FieldReader<Rien>;
impl RienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rien> {
        match self.bits {
            0 => Some(Rien::Rien0),
            1 => Some(Rien::Rien1),
            _ => None,
        }
    }
    #[doc = "Disables MUA Receive Interrupt n. (default)"]
    #[inline(always)]
    pub fn is_rien_0(&self) -> bool {
        *self == Rien::Rien0
    }
    #[doc = "Enables MUA Receive Interrupt n."]
    #[inline(always)]
    pub fn is_rien_1(&self) -> bool {
        *self == Rien::Rien1
    }
}
#[doc = "Field `RIEn` writer - RIEn"]
pub type RienW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rien>;
impl<'a, REG> RienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disables MUA Receive Interrupt n. (default)"]
    #[inline(always)]
    pub fn rien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rien::Rien0)
    }
    #[doc = "Enables MUA Receive Interrupt n."]
    #[inline(always)]
    pub fn rien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rien::Rien1)
    }
}
#[doc = "GIEn\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gien {
    #[doc = "0: Disables MUA General Interrupt n. (default)"]
    Gien0 = 0,
    #[doc = "1: Enables MUA General Interrupt n."]
    Gien1 = 1,
}
impl From<Gien> for u8 {
    #[inline(always)]
    fn from(variant: Gien) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gien {
    type Ux = u8;
}
impl crate::IsEnum for Gien {}
#[doc = "Field `GIEn` reader - GIEn"]
pub type GienR = crate::FieldReader<Gien>;
impl GienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gien> {
        match self.bits {
            0 => Some(Gien::Gien0),
            1 => Some(Gien::Gien1),
            _ => None,
        }
    }
    #[doc = "Disables MUA General Interrupt n. (default)"]
    #[inline(always)]
    pub fn is_gien_0(&self) -> bool {
        *self == Gien::Gien0
    }
    #[doc = "Enables MUA General Interrupt n."]
    #[inline(always)]
    pub fn is_gien_1(&self) -> bool {
        *self == Gien::Gien1
    }
}
#[doc = "Field `GIEn` writer - GIEn"]
pub type GienW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gien>;
impl<'a, REG> GienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disables MUA General Interrupt n. (default)"]
    #[inline(always)]
    pub fn gien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gien::Gien0)
    }
    #[doc = "Enables MUA General Interrupt n."]
    #[inline(always)]
    pub fn gien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gien::Gien1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Fn"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - MUR"]
    #[inline(always)]
    pub fn mur(&self) -> MurR {
        MurR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRDIE"]
    #[inline(always)]
    pub fn rdie(&self) -> RdieR {
        RdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - RAIE"]
    #[inline(always)]
    pub fn raie(&self) -> RaieR {
        RaieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - GIRn"]
    #[inline(always)]
    pub fn girn(&self) -> GirnR {
        GirnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - TIEn"]
    #[inline(always)]
    pub fn tien(&self) -> TienR {
        TienR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RIEn"]
    #[inline(always)]
    pub fn rien(&self) -> RienR {
        RienR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GIEn"]
    #[inline(always)]
    pub fn gien(&self) -> GienR {
        GienR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("fn_", &self.fn_())
            .field("mur", &self.mur())
            .field("rdie", &self.rdie())
            .field("raie", &self.raie())
            .field("girn", &self.girn())
            .field("tien", &self.tien())
            .field("rien", &self.rien())
            .field("gien", &self.gien())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Fn"]
    #[inline(always)]
    #[must_use]
    pub fn fn_(&mut self) -> FnW<CrSpec> {
        FnW::new(self, 0)
    }
    #[doc = "Bit 5 - MUR"]
    #[inline(always)]
    #[must_use]
    pub fn mur(&mut self) -> MurW<CrSpec> {
        MurW::new(self, 5)
    }
    #[doc = "Bit 6 - BRDIE"]
    #[inline(always)]
    #[must_use]
    pub fn rdie(&mut self) -> RdieW<CrSpec> {
        RdieW::new(self, 6)
    }
    #[doc = "Bit 12 - RAIE"]
    #[inline(always)]
    #[must_use]
    pub fn raie(&mut self) -> RaieW<CrSpec> {
        RaieW::new(self, 12)
    }
    #[doc = "Bits 16:19 - GIRn"]
    #[inline(always)]
    #[must_use]
    pub fn girn(&mut self) -> GirnW<CrSpec> {
        GirnW::new(self, 16)
    }
    #[doc = "Bits 20:23 - TIEn"]
    #[inline(always)]
    #[must_use]
    pub fn tien(&mut self) -> TienW<CrSpec> {
        TienW::new(self, 20)
    }
    #[doc = "Bits 24:27 - RIEn"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RienW<CrSpec> {
        RienW::new(self, 24)
    }
    #[doc = "Bits 28:31 - GIEn"]
    #[inline(always)]
    #[must_use]
    pub fn gien(&mut self) -> GienW<CrSpec> {
        GienW::new(self, 28)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x0100"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
