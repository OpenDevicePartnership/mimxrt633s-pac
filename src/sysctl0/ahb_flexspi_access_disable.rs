#[doc = "Register `AHB_FLEXSPI_ACCESS_DISABLE` reader"]
pub type R = crate::R<AhbFlexspiAccessDisableSpec>;
#[doc = "Register `AHB_FLEXSPI_ACCESS_DISABLE` writer"]
pub type W = crate::W<AhbFlexspiAccessDisableSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AhbFlexspiAccessDisable {
    #[doc = "0: Enable AHB access to FLEXSPI"]
    Enable = 0,
    #[doc = "1: Disable AHB access to FLEXSPI"]
    Disable = 1,
}
impl From<AhbFlexspiAccessDisable> for bool {
    #[inline(always)]
    fn from(variant: AhbFlexspiAccessDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_FLEXSPI_ACCESS_DISABLE` reader - no description available"]
pub type AhbFlexspiAccessDisableR = crate::BitReader<AhbFlexspiAccessDisable>;
impl AhbFlexspiAccessDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbFlexspiAccessDisable {
        match self.bits {
            false => AhbFlexspiAccessDisable::Enable,
            true => AhbFlexspiAccessDisable::Disable,
        }
    }
    #[doc = "Enable AHB access to FLEXSPI"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AhbFlexspiAccessDisable::Enable
    }
    #[doc = "Disable AHB access to FLEXSPI"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AhbFlexspiAccessDisable::Disable
    }
}
#[doc = "Field `AHB_FLEXSPI_ACCESS_DISABLE` writer - no description available"]
pub type AhbFlexspiAccessDisableW<'a, REG> = crate::BitWriter<'a, REG, AhbFlexspiAccessDisable>;
impl<'a, REG> AhbFlexspiAccessDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable AHB access to FLEXSPI"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AhbFlexspiAccessDisable::Enable)
    }
    #[doc = "Disable AHB access to FLEXSPI"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AhbFlexspiAccessDisable::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn ahb_flexspi_access_disable(&self) -> AhbFlexspiAccessDisableR {
        AhbFlexspiAccessDisableR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_FLEXSPI_ACCESS_DISABLE")
            .field(
                "ahb_flexspi_access_disable",
                &self.ahb_flexspi_access_disable(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_flexspi_access_disable(
        &mut self,
    ) -> AhbFlexspiAccessDisableW<AhbFlexspiAccessDisableSpec> {
        AhbFlexspiAccessDisableW::new(self, 0)
    }
}
#[doc = "AHB Flexspi access control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_flexspi_access_disable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_flexspi_access_disable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbFlexspiAccessDisableSpec;
impl crate::RegisterSpec for AhbFlexspiAccessDisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_flexspi_access_disable::R`](R) reader structure"]
impl crate::Readable for AhbFlexspiAccessDisableSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb_flexspi_access_disable::W`](W) writer structure"]
impl crate::Writable for AhbFlexspiAccessDisableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB_FLEXSPI_ACCESS_DISABLE to value 0"]
impl crate::Resettable for AhbFlexspiAccessDisableSpec {
    const RESET_VALUE: u32 = 0;
}
