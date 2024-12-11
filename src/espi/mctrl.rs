#[doc = "Register `MCTRL` reader"]
pub type R = crate::R<MctrlSpec>;
#[doc = "Register `MCTRL` writer"]
pub type W = crate::W<MctrlSpec>;
#[doc = "The main enable for the whole block\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enable {
    #[doc = "0: Disabled. Block is not operational"]
    Disabled = 0,
    #[doc = "1: eSPI"]
    Espi = 1,
    #[doc = "2: LPC"]
    Lpc = 2,
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enable {
    type Ux = u8;
}
impl crate::IsEnum for Enable {}
#[doc = "Field `ENABLE` reader - The main enable for the whole block"]
pub type EnableR = crate::FieldReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Enable> {
        match self.bits {
            0 => Some(Enable::Disabled),
            1 => Some(Enable::Espi),
            2 => Some(Enable::Lpc),
            _ => None,
        }
    }
    #[doc = "Disabled. Block is not operational"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "eSPI"]
    #[inline(always)]
    pub fn is_espi(&self) -> bool {
        *self == Enable::Espi
    }
    #[doc = "LPC"]
    #[inline(always)]
    pub fn is_lpc(&self) -> bool {
        *self == Enable::Lpc
    }
}
#[doc = "Field `ENABLE` writer - The main enable for the whole block"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 2, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled. Block is not operational"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "eSPI"]
    #[inline(always)]
    pub fn espi(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Espi)
    }
    #[doc = "LPC"]
    #[inline(always)]
    pub fn lpc(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Lpc)
    }
}
#[doc = "Enable for port %s\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pena {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Pena> for bool {
    #[inline(always)]
    fn from(variant: Pena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENA(0-4)` reader - Enable for port %s"]
pub type PenaR = crate::BitReader<Pena>;
impl PenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pena {
        match self.bits {
            false => Pena::Disabled,
            true => Pena::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pena::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pena::Enabled
    }
}
#[doc = "Field `PENA(0-4)` writer - Enable for port %s"]
pub type PenaW<'a, REG> = crate::BitWriter<'a, REG, Pena>;
impl<'a, REG> PenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pena::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pena::Enabled)
    }
}
#[doc = "Field `P80ENA` reader - Port 80 enable."]
pub type P80enaR = crate::BitReader;
#[doc = "Field `P80ENA` writer - Port 80 enable."]
pub type P80enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBLKENA` reader - Status block is enabled and mapped according to the STATADDR register."]
pub type SblkenaR = crate::BitReader;
#[doc = "Field `SBLKENA` writer - Status block is enabled and mapped according to the STATADDR register."]
pub type SblkenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE60MHZ` reader - If 1, then the functional clock provided to the block is 60MHz vs"]
pub type Use60mhzR = crate::BitReader;
#[doc = "Field `USE60MHZ` writer - If 1, then the functional clock provided to the block is 60MHz vs"]
pub type Use60mhzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - The main enable for the whole block"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 3) as u8)
    }
    #[doc = "Enable for port (0-4)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PENA0` field.</div>"]
    #[inline(always)]
    pub fn pena(&self, n: u8) -> PenaR {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        PenaR::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable for port (0-4)"]
    #[inline(always)]
    pub fn pena_iter(&self) -> impl Iterator<Item = PenaR> + '_ {
        (0..5).map(move |n| PenaR::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - Enable for port 0"]
    #[inline(always)]
    pub fn pena0(&self) -> PenaR {
        PenaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable for port 1"]
    #[inline(always)]
    pub fn pena1(&self) -> PenaR {
        PenaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable for port 2"]
    #[inline(always)]
    pub fn pena2(&self) -> PenaR {
        PenaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable for port 3"]
    #[inline(always)]
    pub fn pena3(&self) -> PenaR {
        PenaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable for port 4"]
    #[inline(always)]
    pub fn pena4(&self) -> PenaR {
        PenaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Port 80 enable."]
    #[inline(always)]
    pub fn p80ena(&self) -> P80enaR {
        P80enaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Status block is enabled and mapped according to the STATADDR register."]
    #[inline(always)]
    pub fn sblkena(&self) -> SblkenaR {
        SblkenaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - If 1, then the functional clock provided to the block is 60MHz vs"]
    #[inline(always)]
    pub fn use60mhz(&self) -> Use60mhzR {
        Use60mhzR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCTRL")
            .field("enable", &self.enable())
            .field("p80ena", &self.p80ena())
            .field("sblkena", &self.sblkena())
            .field("use60mhz", &self.use60mhz())
            .field("pena0", &self.pena0())
            .field("pena1", &self.pena1())
            .field("pena2", &self.pena2())
            .field("pena3", &self.pena3())
            .field("pena4", &self.pena4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - The main enable for the whole block"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<MctrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Enable for port (0-4)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PENA0` field.</div>"]
    #[inline(always)]
    pub fn pena(&mut self, n: u8) -> PenaW<MctrlSpec> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        PenaW::new(self, n + 8)
    }
    #[doc = "Bit 8 - Enable for port 0"]
    #[inline(always)]
    pub fn pena0(&mut self) -> PenaW<MctrlSpec> {
        PenaW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable for port 1"]
    #[inline(always)]
    pub fn pena1(&mut self) -> PenaW<MctrlSpec> {
        PenaW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable for port 2"]
    #[inline(always)]
    pub fn pena2(&mut self) -> PenaW<MctrlSpec> {
        PenaW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable for port 3"]
    #[inline(always)]
    pub fn pena3(&mut self) -> PenaW<MctrlSpec> {
        PenaW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable for port 4"]
    #[inline(always)]
    pub fn pena4(&mut self) -> PenaW<MctrlSpec> {
        PenaW::new(self, 12)
    }
    #[doc = "Bit 16 - Port 80 enable."]
    #[inline(always)]
    pub fn p80ena(&mut self) -> P80enaW<MctrlSpec> {
        P80enaW::new(self, 16)
    }
    #[doc = "Bit 17 - Status block is enabled and mapped according to the STATADDR register."]
    #[inline(always)]
    pub fn sblkena(&mut self) -> SblkenaW<MctrlSpec> {
        SblkenaW::new(self, 17)
    }
    #[doc = "Bit 20 - If 1, then the functional clock provided to the block is 60MHz vs"]
    #[inline(always)]
    pub fn use60mhz(&mut self) -> Use60mhzW<MctrlSpec> {
        Use60mhzW::new(self, 20)
    }
}
#[doc = "Master Control for whole peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`mctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctrlSpec;
impl crate::RegisterSpec for MctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctrl::R`](R) reader structure"]
impl crate::Readable for MctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mctrl::W`](W) writer structure"]
impl crate::Writable for MctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MctrlSpec {
    const RESET_VALUE: u32 = 0;
}
