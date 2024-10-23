#[doc = "Register `PACKERENABLE` reader"]
pub type R = crate::R<PackerenableSpec>;
#[doc = "Register `PACKERENABLE` writer"]
pub type W = crate::W<PackerenableSpec>;
#[doc = "Write Packer Enable.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrpenable {
    #[doc = "0: disabled"]
    Disabled = 0,
    #[doc = "1: enabled"]
    Enabled = 1,
}
impl From<Wrpenable> for bool {
    #[inline(always)]
    fn from(variant: Wrpenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPENABLE` reader - Write Packer Enable."]
pub type WrpenableR = crate::BitReader<Wrpenable>;
impl WrpenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrpenable {
        match self.bits {
            false => Wrpenable::Disabled,
            true => Wrpenable::Enabled,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wrpenable::Disabled
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wrpenable::Enabled
    }
}
#[doc = "Field `WRPENABLE` writer - Write Packer Enable."]
pub type WrpenableW<'a, REG> = crate::BitWriter<'a, REG, Wrpenable>;
impl<'a, REG> WrpenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wrpenable::Disabled)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wrpenable::Enabled)
    }
}
#[doc = "Read Packer Enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdpenable {
    #[doc = "0: disabled"]
    Disabled = 0,
    #[doc = "1: enabled"]
    Enabled = 1,
}
impl From<Rdpenable> for bool {
    #[inline(always)]
    fn from(variant: Rdpenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDPENABLE` reader - Read Packer Enable"]
pub type RdpenableR = crate::BitReader<Rdpenable>;
impl RdpenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdpenable {
        match self.bits {
            false => Rdpenable::Disabled,
            true => Rdpenable::Enabled,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rdpenable::Disabled
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rdpenable::Enabled
    }
}
#[doc = "Field `RDPENABLE` writer - Read Packer Enable"]
pub type RdpenableW<'a, REG> = crate::BitWriter<'a, REG, Rdpenable>;
impl<'a, REG> RdpenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rdpenable::Disabled)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rdpenable::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Write Packer Enable."]
    #[inline(always)]
    pub fn wrpenable(&self) -> WrpenableR {
        WrpenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Packer Enable"]
    #[inline(always)]
    pub fn rdpenable(&self) -> RdpenableR {
        RdpenableR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PACKERENABLE")
            .field("wrpenable", &self.wrpenable())
            .field("rdpenable", &self.rdpenable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write Packer Enable."]
    #[inline(always)]
    #[must_use]
    pub fn wrpenable(&mut self) -> WrpenableW<PackerenableSpec> {
        WrpenableW::new(self, 0)
    }
    #[doc = "Bit 1 - Read Packer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdpenable(&mut self) -> RdpenableW<PackerenableSpec> {
        RdpenableW::new(self, 1)
    }
}
#[doc = "Packer enable for DSP RAM packer\n\nYou can [`read`](crate::Reg::read) this register and get [`packerenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packerenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PackerenableSpec;
impl crate::RegisterSpec for PackerenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`packerenable::R`](R) reader structure"]
impl crate::Readable for PackerenableSpec {}
#[doc = "`write(|w| ..)` method takes [`packerenable::W`](W) writer structure"]
impl crate::Writable for PackerenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PACKERENABLE to value 0x07"]
impl crate::Resettable for PackerenableSpec {
    const RESET_VALUE: u32 = 0x07;
}
