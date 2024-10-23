#[doc = "Register `MODCFG` reader"]
pub type R = crate::R<ModcfgSpec>;
#[doc = "Register `MODCFG` writer"]
pub type W = crate::W<ModcfgSpec>;
#[doc = "Field `NOC` reader - Identifies the number of channels in this MRT.(4 channels on this device.)"]
pub type NocR = crate::FieldReader;
#[doc = "Field `NOC` writer - Identifies the number of channels in this MRT.(4 channels on this device.)"]
pub type NocW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NOB` reader - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
pub type NobR = crate::FieldReader;
#[doc = "Field `NOB` writer - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
pub type NobW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Selects the operating mode for the INUSE flags and the IDLE_CH register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Multitask {
    #[doc = "0: Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    HardwareStatusMode = 0,
    #[doc = "1: Multi-task mode."]
    MultiTaskMode = 1,
}
impl From<Multitask> for bool {
    #[inline(always)]
    fn from(variant: Multitask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULTITASK` reader - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
pub type MultitaskR = crate::BitReader<Multitask>;
impl MultitaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Multitask {
        match self.bits {
            false => Multitask::HardwareStatusMode,
            true => Multitask::MultiTaskMode,
        }
    }
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    #[inline(always)]
    pub fn is_hardware_status_mode(&self) -> bool {
        *self == Multitask::HardwareStatusMode
    }
    #[doc = "Multi-task mode."]
    #[inline(always)]
    pub fn is_multi_task_mode(&self) -> bool {
        *self == Multitask::MultiTaskMode
    }
}
#[doc = "Field `MULTITASK` writer - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
pub type MultitaskW<'a, REG> = crate::BitWriter<'a, REG, Multitask>;
impl<'a, REG> MultitaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    #[inline(always)]
    pub fn hardware_status_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Multitask::HardwareStatusMode)
    }
    #[doc = "Multi-task mode."]
    #[inline(always)]
    pub fn multi_task_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Multitask::MultiTaskMode)
    }
}
impl R {
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    pub fn noc(&self) -> NocR {
        NocR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline(always)]
    pub fn nob(&self) -> NobR {
        NobR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    pub fn multitask(&self) -> MultitaskR {
        MultitaskR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODCFG")
            .field("noc", &self.noc())
            .field("nob", &self.nob())
            .field("multitask", &self.multitask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    #[must_use]
    pub fn noc(&mut self) -> NocW<ModcfgSpec> {
        NocW::new(self, 0)
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline(always)]
    #[must_use]
    pub fn nob(&mut self) -> NobW<ModcfgSpec> {
        NobW::new(self, 4)
    }
    #[doc = "Bit 31 - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    #[must_use]
    pub fn multitask(&mut self) -> MultitaskW<ModcfgSpec> {
        MultitaskW::new(self, 31)
    }
}
#[doc = "Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature.\n\nYou can [`read`](crate::Reg::read) this register and get [`modcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModcfgSpec;
impl crate::RegisterSpec for ModcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modcfg::R`](R) reader structure"]
impl crate::Readable for ModcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`modcfg::W`](W) writer structure"]
impl crate::Writable for ModcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODCFG to value 0x0173"]
impl crate::Resettable for ModcfgSpec {
    const RESET_VALUE: u32 = 0x0173;
}
