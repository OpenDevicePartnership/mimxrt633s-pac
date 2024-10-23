#[doc = "Register `PMCTRL` reader"]
pub type R = crate::R<PmctrlSpec>;
#[doc = "Register `PMCTRL` writer"]
pub type W = crate::W<PmctrlSpec>;
#[doc = "Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelPmatch {
    #[doc = "0: Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    PinInterrupt = 0,
    #[doc = "1: Pattern match. Interrupts are driven in response to pattern matches."]
    PatternMatch = 1,
}
impl From<SelPmatch> for bool {
    #[inline(always)]
    fn from(variant: SelPmatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL_PMATCH` reader - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
pub type SelPmatchR = crate::BitReader<SelPmatch>;
impl SelPmatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SelPmatch {
        match self.bits {
            false => SelPmatch::PinInterrupt,
            true => SelPmatch::PatternMatch,
        }
    }
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    #[inline(always)]
    pub fn is_pin_interrupt(&self) -> bool {
        *self == SelPmatch::PinInterrupt
    }
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    #[inline(always)]
    pub fn is_pattern_match(&self) -> bool {
        *self == SelPmatch::PatternMatch
    }
}
#[doc = "Field `SEL_PMATCH` writer - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
pub type SelPmatchW<'a, REG> = crate::BitWriter<'a, REG, SelPmatch>;
impl<'a, REG> SelPmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    #[inline(always)]
    pub fn pin_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(SelPmatch::PinInterrupt)
    }
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    #[inline(always)]
    pub fn pattern_match(self) -> &'a mut crate::W<REG> {
        self.variant(SelPmatch::PatternMatch)
    }
}
#[doc = "Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnaRxev {
    #[doc = "0: Disabled. RXEV output to the CPU is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. RXEV output to the CPU is enabled."]
    Enabled = 1,
}
impl From<EnaRxev> for bool {
    #[inline(always)]
    fn from(variant: EnaRxev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA_RXEV` reader - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
pub type EnaRxevR = crate::BitReader<EnaRxev>;
impl EnaRxevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnaRxev {
        match self.bits {
            false => EnaRxev::Disabled,
            true => EnaRxev::Enabled,
        }
    }
    #[doc = "Disabled. RXEV output to the CPU is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EnaRxev::Disabled
    }
    #[doc = "Enabled. RXEV output to the CPU is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EnaRxev::Enabled
    }
}
#[doc = "Field `ENA_RXEV` writer - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
pub type EnaRxevW<'a, REG> = crate::BitWriter<'a, REG, EnaRxev>;
impl<'a, REG> EnaRxevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. RXEV output to the CPU is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EnaRxev::Disabled)
    }
    #[doc = "Enabled. RXEV output to the CPU is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EnaRxev::Enabled)
    }
}
#[doc = "Field `PMAT` reader - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
pub type PmatR = crate::FieldReader;
#[doc = "Field `PMAT` writer - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
pub type PmatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline(always)]
    pub fn sel_pmatch(&self) -> SelPmatchR {
        SelPmatchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline(always)]
    pub fn ena_rxev(&self) -> EnaRxevR {
        EnaRxevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 24:31 - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline(always)]
    pub fn pmat(&self) -> PmatR {
        PmatR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCTRL")
            .field("sel_pmatch", &self.sel_pmatch())
            .field("ena_rxev", &self.ena_rxev())
            .field("pmat", &self.pmat())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline(always)]
    #[must_use]
    pub fn sel_pmatch(&mut self) -> SelPmatchW<PmctrlSpec> {
        SelPmatchW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline(always)]
    #[must_use]
    pub fn ena_rxev(&mut self) -> EnaRxevW<PmctrlSpec> {
        EnaRxevW::new(self, 1)
    }
    #[doc = "Bits 24:31 - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline(always)]
    #[must_use]
    pub fn pmat(&mut self) -> PmatW<PmctrlSpec> {
        PmatW::new(self, 24)
    }
}
#[doc = "Pattern match interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmctrlSpec;
impl crate::RegisterSpec for PmctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmctrl::R`](R) reader structure"]
impl crate::Readable for PmctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pmctrl::W`](W) writer structure"]
impl crate::Writable for PmctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMCTRL to value 0"]
impl crate::Resettable for PmctrlSpec {
    const RESET_VALUE: u32 = 0;
}
