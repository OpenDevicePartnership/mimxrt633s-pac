#[doc = "Register `INTA1` reader"]
pub type R = crate::R<Inta1Spec>;
#[doc = "Register `INTA1` writer"]
pub type W = crate::W<Inta1Spec>;
#[doc = "Interrupt A status for DMA channel 32.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inta32 {
    #[doc = "0: The DMAchannel 32 interrupt A is not active."]
    NotActive = 0,
    #[doc = "1: The DMAchannel 0 interrupt A is active."]
    Active = 1,
}
impl From<Inta32> for bool {
    #[inline(always)]
    fn from(variant: Inta32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTA32` reader - Interrupt A status for DMA channel 32."]
pub type Inta32R = crate::BitReader<Inta32>;
impl Inta32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inta32 {
        match self.bits {
            false => Inta32::NotActive,
            true => Inta32::Active,
        }
    }
    #[doc = "The DMAchannel 32 interrupt A is not active."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Inta32::NotActive
    }
    #[doc = "The DMAchannel 0 interrupt A is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Inta32::Active
    }
}
#[doc = "Field `INTA32` writer - Interrupt A status for DMA channel 32."]
pub type Inta32W<'a, REG> = crate::BitWriter<'a, REG, Inta32>;
impl<'a, REG> Inta32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DMAchannel 32 interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Inta32::NotActive)
    }
    #[doc = "The DMAchannel 0 interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Inta32::Active)
    }
}
#[doc = "Additional Interrupt A status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Inta63_33 {
    #[doc = "0: Interrupt A is not active for the relevant DMA channel."]
    NotActive = 0,
    #[doc = "1: Interrupt A is active for the relevant DMA channel."]
    Active = 1,
}
impl From<Inta63_33> for u32 {
    #[inline(always)]
    fn from(variant: Inta63_33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inta63_33 {
    type Ux = u32;
}
impl crate::IsEnum for Inta63_33 {}
#[doc = "Field `INTA63_33` reader - Additional Interrupt A status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Inta63_33R = crate::FieldReader<Inta63_33>;
impl Inta63_33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Inta63_33> {
        match self.bits {
            0 => Some(Inta63_33::NotActive),
            1 => Some(Inta63_33::Active),
            _ => None,
        }
    }
    #[doc = "Interrupt A is not active for the relevant DMA channel."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Inta63_33::NotActive
    }
    #[doc = "Interrupt A is active for the relevant DMA channel."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Inta63_33::Active
    }
}
#[doc = "Field `INTA63_33` writer - Additional Interrupt A status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Inta63_33W<'a, REG> = crate::FieldWriter<'a, REG, 31, Inta63_33>;
impl<'a, REG> Inta63_33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Interrupt A is not active for the relevant DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Inta63_33::NotActive)
    }
    #[doc = "Interrupt A is active for the relevant DMA channel."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Inta63_33::Active)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt A status for DMA channel 32."]
    #[inline(always)]
    pub fn inta32(&self) -> Inta32R {
        Inta32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Additional Interrupt A status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn inta63_33(&self) -> Inta63_33R {
        Inta63_33R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTA1")
            .field("inta32", &self.inta32())
            .field("inta63_33", &self.inta63_33())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt A status for DMA channel 32."]
    #[inline(always)]
    #[must_use]
    pub fn inta32(&mut self) -> Inta32W<Inta1Spec> {
        Inta32W::new(self, 0)
    }
    #[doc = "Bits 1:31 - Additional Interrupt A status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn inta63_33(&mut self) -> Inta63_33W<Inta1Spec> {
        Inta63_33W::new(self, 1)
    }
}
#[doc = "Interrupt A status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`inta1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inta1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inta1Spec;
impl crate::RegisterSpec for Inta1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inta1::R`](R) reader structure"]
impl crate::Readable for Inta1Spec {}
#[doc = "`write(|w| ..)` method takes [`inta1::W`](W) writer structure"]
impl crate::Writable for Inta1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTA1 to value 0"]
impl crate::Resettable for Inta1Spec {
    const RESET_VALUE: u32 = 0;
}
