#[doc = "Register `DSPNMISRCSEL` reader"]
pub type R = crate::R<DspnmisrcselSpec>;
#[doc = "Register `DSPNMISRCSEL` writer"]
pub type W = crate::W<DspnmisrcselSpec>;
#[doc = "Field `NMISRCSEL` reader - Selects one of the DSP interrupt sources as the NMI source. See DSP Interrupt Slot Table for Interrupt Slot Numers."]
pub type NmisrcselR = crate::FieldReader;
#[doc = "Field `NMISRCSEL` writer - Selects one of the DSP interrupt sources as the NMI source. See DSP Interrupt Slot Table for Interrupt Slot Numers."]
pub type NmisrcselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "NMI interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmien {
    #[doc = "0: Disable NMI Interrupt"]
    Disabled = 0,
    #[doc = "1: Enable NMI Interrupt."]
    Enabled = 1,
}
impl From<Nmien> for bool {
    #[inline(always)]
    fn from(variant: Nmien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIEN` reader - NMI interrupt enable"]
pub type NmienR = crate::BitReader<Nmien>;
impl NmienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmien {
        match self.bits {
            false => Nmien::Disabled,
            true => Nmien::Enabled,
        }
    }
    #[doc = "Disable NMI Interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Nmien::Disabled
    }
    #[doc = "Enable NMI Interrupt."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Nmien::Enabled
    }
}
#[doc = "Field `NMIEN` writer - NMI interrupt enable"]
pub type NmienW<'a, REG> = crate::BitWriter<'a, REG, Nmien>;
impl<'a, REG> NmienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable NMI Interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nmien::Disabled)
    }
    #[doc = "Enable NMI Interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nmien::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects one of the DSP interrupt sources as the NMI source. See DSP Interrupt Slot Table for Interrupt Slot Numers."]
    #[inline(always)]
    pub fn nmisrcsel(&self) -> NmisrcselR {
        NmisrcselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - NMI interrupt enable"]
    #[inline(always)]
    pub fn nmien(&self) -> NmienR {
        NmienR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSPNMISRCSEL")
            .field("nmisrcsel", &self.nmisrcsel())
            .field("nmien", &self.nmien())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects one of the DSP interrupt sources as the NMI source. See DSP Interrupt Slot Table for Interrupt Slot Numers."]
    #[inline(always)]
    #[must_use]
    pub fn nmisrcsel(&mut self) -> NmisrcselW<DspnmisrcselSpec> {
        NmisrcselW::new(self, 0)
    }
    #[doc = "Bit 31 - NMI interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmien(&mut self) -> NmienW<DspnmisrcselSpec> {
        NmienW::new(self, 31)
    }
}
#[doc = "DSP NMI source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dspnmisrcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspnmisrcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspnmisrcselSpec;
impl crate::RegisterSpec for DspnmisrcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dspnmisrcsel::R`](R) reader structure"]
impl crate::Readable for DspnmisrcselSpec {}
#[doc = "`write(|w| ..)` method takes [`dspnmisrcsel::W`](W) writer structure"]
impl crate::Writable for DspnmisrcselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSPNMISRCSEL to value 0x05"]
impl crate::Resettable for DspnmisrcselSpec {
    const RESET_VALUE: u32 = 0x05;
}
