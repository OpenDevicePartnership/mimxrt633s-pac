#[doc = "Register `MCONFIG` reader"]
pub type R = crate::R<MconfigSpec>;
#[doc = "Register `MCONFIG` writer"]
pub type W = crate::W<MconfigSpec>;
#[doc = "Master enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mstena {
    #[doc = "0: MASTER_OFF: Master is off (is not enabled). If MASTER_OFF is enabled, then the I3C module can only use slave mode."]
    MasterOff = 0,
    #[doc = "1: MASTER_ON: Master is on (is enabled). When used from start-up, this I3C module is master by default (the main master). The module will control the bus unless the master is handed off. If the master is handed off, then MSTENA must move to 2 after that happens. The handoff means emitting GETACCMST and if accepted, the module will emit a STOP and set the MSTENA bit to 2 (or 0)."]
    MasterOn = 1,
    #[doc = "2: MASTER_CAPABLE: The I3C module is master-capable; however the module is operating as a slave now. When used from the start, the I3C module will start as a slave, but will be prepared to switch to master mode. To switch to master mode, the slave emits an Master Request (MR), or gets a GETACCMST CCC command and accepts it (to switch on the STOP)."]
    MasterCapable = 2,
}
impl From<Mstena> for u8 {
    #[inline(always)]
    fn from(variant: Mstena) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mstena {
    type Ux = u8;
}
impl crate::IsEnum for Mstena {}
#[doc = "Field `MSTENA` reader - Master enable"]
pub type MstenaR = crate::FieldReader<Mstena>;
impl MstenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mstena> {
        match self.bits {
            0 => Some(Mstena::MasterOff),
            1 => Some(Mstena::MasterOn),
            2 => Some(Mstena::MasterCapable),
            _ => None,
        }
    }
    #[doc = "MASTER_OFF: Master is off (is not enabled). If MASTER_OFF is enabled, then the I3C module can only use slave mode."]
    #[inline(always)]
    pub fn is_master_off(&self) -> bool {
        *self == Mstena::MasterOff
    }
    #[doc = "MASTER_ON: Master is on (is enabled). When used from start-up, this I3C module is master by default (the main master). The module will control the bus unless the master is handed off. If the master is handed off, then MSTENA must move to 2 after that happens. The handoff means emitting GETACCMST and if accepted, the module will emit a STOP and set the MSTENA bit to 2 (or 0)."]
    #[inline(always)]
    pub fn is_master_on(&self) -> bool {
        *self == Mstena::MasterOn
    }
    #[doc = "MASTER_CAPABLE: The I3C module is master-capable; however the module is operating as a slave now. When used from the start, the I3C module will start as a slave, but will be prepared to switch to master mode. To switch to master mode, the slave emits an Master Request (MR), or gets a GETACCMST CCC command and accepts it (to switch on the STOP)."]
    #[inline(always)]
    pub fn is_master_capable(&self) -> bool {
        *self == Mstena::MasterCapable
    }
}
#[doc = "Field `MSTENA` writer - Master enable"]
pub type MstenaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mstena>;
impl<'a, REG> MstenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MASTER_OFF: Master is off (is not enabled). If MASTER_OFF is enabled, then the I3C module can only use slave mode."]
    #[inline(always)]
    pub fn master_off(self) -> &'a mut crate::W<REG> {
        self.variant(Mstena::MasterOff)
    }
    #[doc = "MASTER_ON: Master is on (is enabled). When used from start-up, this I3C module is master by default (the main master). The module will control the bus unless the master is handed off. If the master is handed off, then MSTENA must move to 2 after that happens. The handoff means emitting GETACCMST and if accepted, the module will emit a STOP and set the MSTENA bit to 2 (or 0)."]
    #[inline(always)]
    pub fn master_on(self) -> &'a mut crate::W<REG> {
        self.variant(Mstena::MasterOn)
    }
    #[doc = "MASTER_CAPABLE: The I3C module is master-capable; however the module is operating as a slave now. When used from the start, the I3C module will start as a slave, but will be prepared to switch to master mode. To switch to master mode, the slave emits an Master Request (MR), or gets a GETACCMST CCC command and accepts it (to switch on the STOP)."]
    #[inline(always)]
    pub fn master_capable(self) -> &'a mut crate::W<REG> {
        self.variant(Mstena::MasterCapable)
    }
}
#[doc = "Field `DISTO` reader - Disable Timeout"]
pub type DistoR = crate::BitReader;
#[doc = "Field `DISTO` writer - Disable Timeout"]
pub type DistoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "High-Keeper\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hkeep {
    #[doc = "0: NONE: Use PUR (Pull-Up Resistor). Hold SCL High."]
    None = 0,
    #[doc = "1: WIRED_IN: Wired-in High Keeper controls; use pin_HK (High Keeper) controls."]
    WiredIn = 1,
    #[doc = "2: PASSIVE_SDA: Passive on SDA; can Hi-Z (high impedance) for Bus Free (IDLE) and hold."]
    PassiveSda = 2,
    #[doc = "3: PASSIVE_ON_SDA_SCL: Passive on SDA and SCL; can Hi-Z (high impedance) both for Bus Free (IDLE), and can Hi-Z SDA for hold."]
    PassiveOnSdaScl = 3,
}
impl From<Hkeep> for u8 {
    #[inline(always)]
    fn from(variant: Hkeep) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hkeep {
    type Ux = u8;
}
impl crate::IsEnum for Hkeep {}
#[doc = "Field `HKEEP` reader - High-Keeper"]
pub type HkeepR = crate::FieldReader<Hkeep>;
impl HkeepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hkeep {
        match self.bits {
            0 => Hkeep::None,
            1 => Hkeep::WiredIn,
            2 => Hkeep::PassiveSda,
            3 => Hkeep::PassiveOnSdaScl,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE: Use PUR (Pull-Up Resistor). Hold SCL High."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Hkeep::None
    }
    #[doc = "WIRED_IN: Wired-in High Keeper controls; use pin_HK (High Keeper) controls."]
    #[inline(always)]
    pub fn is_wired_in(&self) -> bool {
        *self == Hkeep::WiredIn
    }
    #[doc = "PASSIVE_SDA: Passive on SDA; can Hi-Z (high impedance) for Bus Free (IDLE) and hold."]
    #[inline(always)]
    pub fn is_passive_sda(&self) -> bool {
        *self == Hkeep::PassiveSda
    }
    #[doc = "PASSIVE_ON_SDA_SCL: Passive on SDA and SCL; can Hi-Z (high impedance) both for Bus Free (IDLE), and can Hi-Z SDA for hold."]
    #[inline(always)]
    pub fn is_passive_on_sda_scl(&self) -> bool {
        *self == Hkeep::PassiveOnSdaScl
    }
}
#[doc = "Field `HKEEP` writer - High-Keeper"]
pub type HkeepW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hkeep, crate::Safe>;
impl<'a, REG> HkeepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE: Use PUR (Pull-Up Resistor). Hold SCL High."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Hkeep::None)
    }
    #[doc = "WIRED_IN: Wired-in High Keeper controls; use pin_HK (High Keeper) controls."]
    #[inline(always)]
    pub fn wired_in(self) -> &'a mut crate::W<REG> {
        self.variant(Hkeep::WiredIn)
    }
    #[doc = "PASSIVE_SDA: Passive on SDA; can Hi-Z (high impedance) for Bus Free (IDLE) and hold."]
    #[inline(always)]
    pub fn passive_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Hkeep::PassiveSda)
    }
    #[doc = "PASSIVE_ON_SDA_SCL: Passive on SDA and SCL; can Hi-Z (high impedance) both for Bus Free (IDLE), and can Hi-Z SDA for hold."]
    #[inline(always)]
    pub fn passive_on_sda_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Hkeep::PassiveOnSdaScl)
    }
}
#[doc = "Field `ODSTOP` reader - Open drain stop"]
pub type OdstopR = crate::BitReader;
#[doc = "Field `ODSTOP` writer - Open drain stop"]
pub type OdstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPBAUD` reader - Push-pull baud rate"]
pub type PpbaudR = crate::FieldReader;
#[doc = "Field `PPBAUD` writer - Push-pull baud rate"]
pub type PpbaudW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPLOW` reader - Push-Pull low"]
pub type PplowR = crate::FieldReader;
#[doc = "Field `PPLOW` writer - Push-Pull low"]
pub type PplowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ODBAUD` reader - Open drain baud rate"]
pub type OdbaudR = crate::FieldReader;
#[doc = "Field `ODBAUD` writer - Open drain baud rate"]
pub type OdbaudW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ODHPP` reader - Open drain high push-pull"]
pub type OdhppR = crate::BitReader;
#[doc = "Field `ODHPP` writer - Open drain high push-pull"]
pub type OdhppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKEW` reader - Skew"]
pub type SkewR = crate::FieldReader;
#[doc = "Field `SKEW` writer - Skew"]
pub type SkewW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `I2CBAUD` reader - I2C baud rate"]
pub type I2cbaudR = crate::FieldReader;
#[doc = "Field `I2CBAUD` writer - I2C baud rate"]
pub type I2cbaudW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Master enable"]
    #[inline(always)]
    pub fn mstena(&self) -> MstenaR {
        MstenaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Disable Timeout"]
    #[inline(always)]
    pub fn disto(&self) -> DistoR {
        DistoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - High-Keeper"]
    #[inline(always)]
    pub fn hkeep(&self) -> HkeepR {
        HkeepR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Open drain stop"]
    #[inline(always)]
    pub fn odstop(&self) -> OdstopR {
        OdstopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Push-pull baud rate"]
    #[inline(always)]
    pub fn ppbaud(&self) -> PpbaudR {
        PpbaudR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Push-Pull low"]
    #[inline(always)]
    pub fn pplow(&self) -> PplowR {
        PplowR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Open drain baud rate"]
    #[inline(always)]
    pub fn odbaud(&self) -> OdbaudR {
        OdbaudR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Open drain high push-pull"]
    #[inline(always)]
    pub fn odhpp(&self) -> OdhppR {
        OdhppR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Skew"]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - I2C baud rate"]
    #[inline(always)]
    pub fn i2cbaud(&self) -> I2cbaudR {
        I2cbaudR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCONFIG")
            .field("mstena", &self.mstena())
            .field("disto", &self.disto())
            .field("hkeep", &self.hkeep())
            .field("odstop", &self.odstop())
            .field("ppbaud", &self.ppbaud())
            .field("pplow", &self.pplow())
            .field("odbaud", &self.odbaud())
            .field("odhpp", &self.odhpp())
            .field("skew", &self.skew())
            .field("i2cbaud", &self.i2cbaud())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Master enable"]
    #[inline(always)]
    pub fn mstena(&mut self) -> MstenaW<MconfigSpec> {
        MstenaW::new(self, 0)
    }
    #[doc = "Bit 3 - Disable Timeout"]
    #[inline(always)]
    pub fn disto(&mut self) -> DistoW<MconfigSpec> {
        DistoW::new(self, 3)
    }
    #[doc = "Bits 4:5 - High-Keeper"]
    #[inline(always)]
    pub fn hkeep(&mut self) -> HkeepW<MconfigSpec> {
        HkeepW::new(self, 4)
    }
    #[doc = "Bit 6 - Open drain stop"]
    #[inline(always)]
    pub fn odstop(&mut self) -> OdstopW<MconfigSpec> {
        OdstopW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Push-pull baud rate"]
    #[inline(always)]
    pub fn ppbaud(&mut self) -> PpbaudW<MconfigSpec> {
        PpbaudW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Push-Pull low"]
    #[inline(always)]
    pub fn pplow(&mut self) -> PplowW<MconfigSpec> {
        PplowW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Open drain baud rate"]
    #[inline(always)]
    pub fn odbaud(&mut self) -> OdbaudW<MconfigSpec> {
        OdbaudW::new(self, 16)
    }
    #[doc = "Bit 24 - Open drain high push-pull"]
    #[inline(always)]
    pub fn odhpp(&mut self) -> OdhppW<MconfigSpec> {
        OdhppW::new(self, 24)
    }
    #[doc = "Bits 25:27 - Skew"]
    #[inline(always)]
    pub fn skew(&mut self) -> SkewW<MconfigSpec> {
        SkewW::new(self, 25)
    }
    #[doc = "Bits 28:31 - I2C baud rate"]
    #[inline(always)]
    pub fn i2cbaud(&mut self) -> I2cbaudW<MconfigSpec> {
        I2cbaudW::new(self, 28)
    }
}
#[doc = "Master Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MconfigSpec;
impl crate::RegisterSpec for MconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mconfig::R`](R) reader structure"]
impl crate::Readable for MconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mconfig::W`](W) writer structure"]
impl crate::Writable for MconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCONFIG to value 0"]
impl crate::Resettable for MconfigSpec {
    const RESET_VALUE: u32 = 0;
}
