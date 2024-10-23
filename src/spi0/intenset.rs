#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssaen {
    #[doc = "0: Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    Disabled = 0,
    #[doc = "1: Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    Enabled = 1,
}
impl From<Ssaen> for bool {
    #[inline(always)]
    fn from(variant: Ssaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSAEN` reader - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
pub type SsaenR = crate::BitReader<Ssaen>;
impl SsaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssaen {
        match self.bits {
            false => Ssaen::Disabled,
            true => Ssaen::Enabled,
        }
    }
    #[doc = "Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ssaen::Disabled
    }
    #[doc = "Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ssaen::Enabled
    }
}
#[doc = "Field `SSAEN` writer - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
pub type SsaenW<'a, REG> = crate::BitWriter<'a, REG, Ssaen>;
impl<'a, REG> SsaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ssaen::Disabled)
    }
    #[doc = "Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ssaen::Enabled)
    }
}
#[doc = "Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssden {
    #[doc = "0: Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    Disabled = 0,
    #[doc = "1: Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    Enabled = 1,
}
impl From<Ssden> for bool {
    #[inline(always)]
    fn from(variant: Ssden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSDEN` reader - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
pub type SsdenR = crate::BitReader<Ssden>;
impl SsdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssden {
        match self.bits {
            false => Ssden::Disabled,
            true => Ssden::Enabled,
        }
    }
    #[doc = "Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ssden::Disabled
    }
    #[doc = "Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ssden::Enabled
    }
}
#[doc = "Field `SSDEN` writer - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
pub type SsdenW<'a, REG> = crate::BitWriter<'a, REG, Ssden>;
impl<'a, REG> SsdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ssden::Disabled)
    }
    #[doc = "Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ssden::Enabled)
    }
}
#[doc = "Master idle interrupt enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstidleen {
    #[doc = "0: No interrupt will be generated when the SPI master function is idle."]
    Disabled = 0,
    #[doc = "1: An interrupt will be generated when the SPI master function is fully idle."]
    Enabled = 1,
}
impl From<Mstidleen> for bool {
    #[inline(always)]
    fn from(variant: Mstidleen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTIDLEEN` reader - Master idle interrupt enable."]
pub type MstidleenR = crate::BitReader<Mstidleen>;
impl MstidleenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstidleen {
        match self.bits {
            false => Mstidleen::Disabled,
            true => Mstidleen::Enabled,
        }
    }
    #[doc = "No interrupt will be generated when the SPI master function is idle."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mstidleen::Disabled
    }
    #[doc = "An interrupt will be generated when the SPI master function is fully idle."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mstidleen::Enabled
    }
}
#[doc = "Field `MSTIDLEEN` writer - Master idle interrupt enable."]
pub type MstidleenW<'a, REG> = crate::BitWriter<'a, REG, Mstidleen>;
impl<'a, REG> MstidleenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt will be generated when the SPI master function is idle."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mstidleen::Disabled)
    }
    #[doc = "An interrupt will be generated when the SPI master function is fully idle."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mstidleen::Enabled)
    }
}
impl R {
    #[doc = "Bit 4 - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub fn ssaen(&self) -> SsaenR {
        SsaenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub fn ssden(&self) -> SsdenR {
        SsdenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Master idle interrupt enable."]
    #[inline(always)]
    pub fn mstidleen(&self) -> MstidleenR {
        MstidleenR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET")
            .field("ssaen", &self.ssaen())
            .field("ssden", &self.ssden())
            .field("mstidleen", &self.mstidleen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn ssaen(&mut self) -> SsaenW<IntensetSpec> {
        SsaenW::new(self, 4)
    }
    #[doc = "Bit 5 - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    #[must_use]
    pub fn ssden(&mut self) -> SsdenW<IntensetSpec> {
        SsdenW::new(self, 5)
    }
    #[doc = "Bit 8 - Master idle interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn mstidleen(&mut self) -> MstidleenW<IntensetSpec> {
        MstidleenW::new(self, 8)
    }
}
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
