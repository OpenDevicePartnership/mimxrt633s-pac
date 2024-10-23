#[doc = "Register `DSP_FLEXSPI_ACCESS_DISABLE` reader"]
pub type R = crate::R<DspFlexspiAccessDisableSpec>;
#[doc = "Register `DSP_FLEXSPI_ACCESS_DISABLE` writer"]
pub type W = crate::W<DspFlexspiAccessDisableSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspFlexspiAccessDisable {
    #[doc = "0: Enable DSP access to FLEXSPI"]
    Enable = 0,
    #[doc = "1: Disable DSP access to FLEXSPI"]
    Disable = 1,
}
impl From<DspFlexspiAccessDisable> for bool {
    #[inline(always)]
    fn from(variant: DspFlexspiAccessDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_FLEXSPI_ACCESS_DISABLE` reader - no description available"]
pub type DspFlexspiAccessDisableR = crate::BitReader<DspFlexspiAccessDisable>;
impl DspFlexspiAccessDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspFlexspiAccessDisable {
        match self.bits {
            false => DspFlexspiAccessDisable::Enable,
            true => DspFlexspiAccessDisable::Disable,
        }
    }
    #[doc = "Enable DSP access to FLEXSPI"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DspFlexspiAccessDisable::Enable
    }
    #[doc = "Disable DSP access to FLEXSPI"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DspFlexspiAccessDisable::Disable
    }
}
#[doc = "Field `DSP_FLEXSPI_ACCESS_DISABLE` writer - no description available"]
pub type DspFlexspiAccessDisableW<'a, REG> = crate::BitWriter<'a, REG, DspFlexspiAccessDisable>;
impl<'a, REG> DspFlexspiAccessDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable DSP access to FLEXSPI"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DspFlexspiAccessDisable::Enable)
    }
    #[doc = "Disable DSP access to FLEXSPI"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DspFlexspiAccessDisable::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn dsp_flexspi_access_disable(&self) -> DspFlexspiAccessDisableR {
        DspFlexspiAccessDisableR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSP_FLEXSPI_ACCESS_DISABLE")
            .field(
                "dsp_flexspi_access_disable",
                &self.dsp_flexspi_access_disable(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_flexspi_access_disable(
        &mut self,
    ) -> DspFlexspiAccessDisableW<DspFlexspiAccessDisableSpec> {
        DspFlexspiAccessDisableW::new(self, 0)
    }
}
#[doc = "DSP Flexspi access control\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_flexspi_access_disable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_flexspi_access_disable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspFlexspiAccessDisableSpec;
impl crate::RegisterSpec for DspFlexspiAccessDisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_flexspi_access_disable::R`](R) reader structure"]
impl crate::Readable for DspFlexspiAccessDisableSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_flexspi_access_disable::W`](W) writer structure"]
impl crate::Writable for DspFlexspiAccessDisableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_FLEXSPI_ACCESS_DISABLE to value 0"]
impl crate::Resettable for DspFlexspiAccessDisableSpec {
    const RESET_VALUE: u32 = 0;
}
