#[doc = "Register `CMD_XFR_TYP` reader"]
pub type R = crate::R<CmdXfrTypSpec>;
#[doc = "Register `CMD_XFR_TYP` writer"]
pub type W = crate::W<CmdXfrTypSpec>;
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rsptyp {
    #[doc = "0: No Response"]
    Rsptyp0 = 0,
    #[doc = "1: Response Length 136"]
    Rsptyp1 = 1,
    #[doc = "2: Response Length 48"]
    Rsptyp2 = 2,
    #[doc = "3: Response Length 48, check Busy after response"]
    Rsptyp3 = 3,
}
impl From<Rsptyp> for u8 {
    #[inline(always)]
    fn from(variant: Rsptyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rsptyp {
    type Ux = u8;
}
impl crate::IsEnum for Rsptyp {}
#[doc = "Field `RSPTYP` reader - Response Type Select"]
pub type RsptypR = crate::FieldReader<Rsptyp>;
impl RsptypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsptyp {
        match self.bits {
            0 => Rsptyp::Rsptyp0,
            1 => Rsptyp::Rsptyp1,
            2 => Rsptyp::Rsptyp2,
            3 => Rsptyp::Rsptyp3,
            _ => unreachable!(),
        }
    }
    #[doc = "No Response"]
    #[inline(always)]
    pub fn is_rsptyp_0(&self) -> bool {
        *self == Rsptyp::Rsptyp0
    }
    #[doc = "Response Length 136"]
    #[inline(always)]
    pub fn is_rsptyp_1(&self) -> bool {
        *self == Rsptyp::Rsptyp1
    }
    #[doc = "Response Length 48"]
    #[inline(always)]
    pub fn is_rsptyp_2(&self) -> bool {
        *self == Rsptyp::Rsptyp2
    }
    #[doc = "Response Length 48, check Busy after response"]
    #[inline(always)]
    pub fn is_rsptyp_3(&self) -> bool {
        *self == Rsptyp::Rsptyp3
    }
}
#[doc = "Field `RSPTYP` writer - Response Type Select"]
pub type RsptypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rsptyp, crate::Safe>;
impl<'a, REG> RsptypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Response"]
    #[inline(always)]
    pub fn rsptyp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptyp::Rsptyp0)
    }
    #[doc = "Response Length 136"]
    #[inline(always)]
    pub fn rsptyp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptyp::Rsptyp1)
    }
    #[doc = "Response Length 48"]
    #[inline(always)]
    pub fn rsptyp_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptyp::Rsptyp2)
    }
    #[doc = "Response Length 48, check Busy after response"]
    #[inline(always)]
    pub fn rsptyp_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptyp::Rsptyp3)
    }
}
#[doc = "Command CRC Check Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cccen {
    #[doc = "0: Disable"]
    Cccen0 = 0,
    #[doc = "1: Enable"]
    Cccen1 = 1,
}
impl From<Cccen> for bool {
    #[inline(always)]
    fn from(variant: Cccen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCCEN` reader - Command CRC Check Enable"]
pub type CccenR = crate::BitReader<Cccen>;
impl CccenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cccen {
        match self.bits {
            false => Cccen::Cccen0,
            true => Cccen::Cccen1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_cccen_0(&self) -> bool {
        *self == Cccen::Cccen0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_cccen_1(&self) -> bool {
        *self == Cccen::Cccen1
    }
}
#[doc = "Field `CCCEN` writer - Command CRC Check Enable"]
pub type CccenW<'a, REG> = crate::BitWriter<'a, REG, Cccen>;
impl<'a, REG> CccenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn cccen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cccen::Cccen0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn cccen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cccen::Cccen1)
    }
}
#[doc = "Command Index Check Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cicen {
    #[doc = "0: Disable"]
    Cicen0 = 0,
    #[doc = "1: Enable"]
    Cicen1 = 1,
}
impl From<Cicen> for bool {
    #[inline(always)]
    fn from(variant: Cicen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CICEN` reader - Command Index Check Enable"]
pub type CicenR = crate::BitReader<Cicen>;
impl CicenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cicen {
        match self.bits {
            false => Cicen::Cicen0,
            true => Cicen::Cicen1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_cicen_0(&self) -> bool {
        *self == Cicen::Cicen0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_cicen_1(&self) -> bool {
        *self == Cicen::Cicen1
    }
}
#[doc = "Field `CICEN` writer - Command Index Check Enable"]
pub type CicenW<'a, REG> = crate::BitWriter<'a, REG, Cicen>;
impl<'a, REG> CicenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn cicen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cicen::Cicen0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn cicen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cicen::Cicen1)
    }
}
#[doc = "Data Present Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpsel {
    #[doc = "0: No Data Present"]
    Dpsel0 = 0,
    #[doc = "1: Data Present"]
    Dpsel1 = 1,
}
impl From<Dpsel> for bool {
    #[inline(always)]
    fn from(variant: Dpsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPSEL` reader - Data Present Select"]
pub type DpselR = crate::BitReader<Dpsel>;
impl DpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpsel {
        match self.bits {
            false => Dpsel::Dpsel0,
            true => Dpsel::Dpsel1,
        }
    }
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn is_dpsel_0(&self) -> bool {
        *self == Dpsel::Dpsel0
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn is_dpsel_1(&self) -> bool {
        *self == Dpsel::Dpsel1
    }
}
#[doc = "Field `DPSEL` writer - Data Present Select"]
pub type DpselW<'a, REG> = crate::BitWriter<'a, REG, Dpsel>;
impl<'a, REG> DpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn dpsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpsel::Dpsel0)
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn dpsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpsel::Dpsel1)
    }
}
#[doc = "Command Type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdtyp {
    #[doc = "0: Normal Other commands"]
    Cmdtyp0 = 0,
    #[doc = "1: Suspend CMD52 for writing Bus Suspend in CCCR"]
    Cmdtyp1 = 1,
    #[doc = "2: Resume CMD52 for writing Function Select in CCCR"]
    Cmdtyp2 = 2,
    #[doc = "3: Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    Cmdtyp3 = 3,
}
impl From<Cmdtyp> for u8 {
    #[inline(always)]
    fn from(variant: Cmdtyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdtyp {
    type Ux = u8;
}
impl crate::IsEnum for Cmdtyp {}
#[doc = "Field `CMDTYP` reader - Command Type"]
pub type CmdtypR = crate::FieldReader<Cmdtyp>;
impl CmdtypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdtyp {
        match self.bits {
            0 => Cmdtyp::Cmdtyp0,
            1 => Cmdtyp::Cmdtyp1,
            2 => Cmdtyp::Cmdtyp2,
            3 => Cmdtyp::Cmdtyp3,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal Other commands"]
    #[inline(always)]
    pub fn is_cmdtyp_0(&self) -> bool {
        *self == Cmdtyp::Cmdtyp0
    }
    #[doc = "Suspend CMD52 for writing Bus Suspend in CCCR"]
    #[inline(always)]
    pub fn is_cmdtyp_1(&self) -> bool {
        *self == Cmdtyp::Cmdtyp1
    }
    #[doc = "Resume CMD52 for writing Function Select in CCCR"]
    #[inline(always)]
    pub fn is_cmdtyp_2(&self) -> bool {
        *self == Cmdtyp::Cmdtyp2
    }
    #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline(always)]
    pub fn is_cmdtyp_3(&self) -> bool {
        *self == Cmdtyp::Cmdtyp3
    }
}
#[doc = "Field `CMDTYP` writer - Command Type"]
pub type CmdtypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmdtyp, crate::Safe>;
impl<'a, REG> CmdtypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Other commands"]
    #[inline(always)]
    pub fn cmdtyp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::Cmdtyp0)
    }
    #[doc = "Suspend CMD52 for writing Bus Suspend in CCCR"]
    #[inline(always)]
    pub fn cmdtyp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::Cmdtyp1)
    }
    #[doc = "Resume CMD52 for writing Function Select in CCCR"]
    #[inline(always)]
    pub fn cmdtyp_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::Cmdtyp2)
    }
    #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline(always)]
    pub fn cmdtyp_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtyp::Cmdtyp3)
    }
}
#[doc = "Field `CMDINX` reader - Command Index"]
pub type CmdinxR = crate::FieldReader;
#[doc = "Field `CMDINX` writer - Command Index"]
pub type CmdinxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn rsptyp(&self) -> RsptypR {
        RsptypR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cccen(&self) -> CccenR {
        CccenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cicen(&self) -> CicenR {
        CicenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DpselR {
        DpselR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CmdtypR {
        CmdtypR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdinx(&self) -> CmdinxR {
        CmdinxR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_XFR_TYP")
            .field("rsptyp", &self.rsptyp())
            .field("cccen", &self.cccen())
            .field("cicen", &self.cicen())
            .field("dpsel", &self.dpsel())
            .field("cmdtyp", &self.cmdtyp())
            .field("cmdinx", &self.cmdinx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn rsptyp(&mut self) -> RsptypW<CmdXfrTypSpec> {
        RsptypW::new(self, 16)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cccen(&mut self) -> CccenW<CmdXfrTypSpec> {
        CccenW::new(self, 19)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cicen(&mut self) -> CicenW<CmdXfrTypSpec> {
        CicenW::new(self, 20)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&mut self) -> DpselW<CmdXfrTypSpec> {
        DpselW::new(self, 21)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&mut self) -> CmdtypW<CmdXfrTypSpec> {
        CmdtypW::new(self, 22)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdinx(&mut self) -> CmdinxW<CmdXfrTypSpec> {
        CmdinxW::new(self, 24)
    }
}
#[doc = "Command Transfer Type\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_xfr_typ::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_xfr_typ::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdXfrTypSpec;
impl crate::RegisterSpec for CmdXfrTypSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_xfr_typ::R`](R) reader structure"]
impl crate::Readable for CmdXfrTypSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_xfr_typ::W`](W) writer structure"]
impl crate::Writable for CmdXfrTypSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_XFR_TYP to value 0"]
impl crate::Resettable for CmdXfrTypSpec {
    const RESET_VALUE: u32 = 0;
}
