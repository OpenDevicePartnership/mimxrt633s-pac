#[doc = "Register `INTB1` reader"]
pub type R = crate::R<Intb1Spec>;
#[doc = "Register `INTB1` writer"]
pub type W = crate::W<Intb1Spec>;
#[doc = "Interrupt B status for DMA channel 32.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intb32 {
    #[doc = "0: The DMAchannel 32 interrupt B is not active."]
    NotActive = 0,
    #[doc = "1: The DMAchannel 32 interrupt B is active."]
    Active = 1,
}
impl From<Intb32> for bool {
    #[inline(always)]
    fn from(variant: Intb32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTB32` reader - Interrupt B status for DMA channel 32."]
pub type Intb32R = crate::BitReader<Intb32>;
impl Intb32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intb32 {
        match self.bits {
            false => Intb32::NotActive,
            true => Intb32::Active,
        }
    }
    #[doc = "The DMAchannel 32 interrupt B is not active."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Intb32::NotActive
    }
    #[doc = "The DMAchannel 32 interrupt B is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Intb32::Active
    }
}
#[doc = "Field `INTB32` writer - Interrupt B status for DMA channel 32."]
pub type Intb32W<'a, REG> = crate::BitWriter<'a, REG, Intb32>;
impl<'a, REG> Intb32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DMAchannel 32 interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Intb32::NotActive)
    }
    #[doc = "The DMAchannel 32 interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Intb32::Active)
    }
}
#[doc = "Additional Interrupt B status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Intb63_33 {
    #[doc = "0: Interrupt B is not active for the relevant DMA channel."]
    NotActive = 0,
    #[doc = "1: Interrupt B is active for the relevant DMA channel."]
    Active = 1,
}
impl From<Intb63_33> for u32 {
    #[inline(always)]
    fn from(variant: Intb63_33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Intb63_33 {
    type Ux = u32;
}
impl crate::IsEnum for Intb63_33 {}
#[doc = "Field `INTB63_33` reader - Additional Interrupt B status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Intb63_33R = crate::FieldReader<Intb63_33>;
impl Intb63_33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Intb63_33> {
        match self.bits {
            0 => Some(Intb63_33::NotActive),
            1 => Some(Intb63_33::Active),
            _ => None,
        }
    }
    #[doc = "Interrupt B is not active for the relevant DMA channel."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Intb63_33::NotActive
    }
    #[doc = "Interrupt B is active for the relevant DMA channel."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Intb63_33::Active
    }
}
#[doc = "Field `INTB63_33` writer - Additional Interrupt B status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Intb63_33W<'a, REG> = crate::FieldWriter<'a, REG, 31, Intb63_33>;
impl<'a, REG> Intb63_33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Interrupt B is not active for the relevant DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Intb63_33::NotActive)
    }
    #[doc = "Interrupt B is active for the relevant DMA channel."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Intb63_33::Active)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt B status for DMA channel 32."]
    #[inline(always)]
    pub fn intb32(&self) -> Intb32R {
        Intb32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Additional Interrupt B status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn intb63_33(&self) -> Intb63_33R {
        Intb63_33R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTB1")
            .field("intb32", &self.intb32())
            .field("intb63_33", &self.intb63_33())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt B status for DMA channel 32."]
    #[inline(always)]
    pub fn intb32(&mut self) -> Intb32W<Intb1Spec> {
        Intb32W::new(self, 0)
    }
    #[doc = "Bits 1:31 - Additional Interrupt B status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn intb63_33(&mut self) -> Intb63_33W<Intb1Spec> {
        Intb63_33W::new(self, 1)
    }
}
#[doc = "Interrupt B status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`intb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intb1Spec;
impl crate::RegisterSpec for Intb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intb1::R`](R) reader structure"]
impl crate::Readable for Intb1Spec {}
#[doc = "`write(|w| ..)` method takes [`intb1::W`](W) writer structure"]
impl crate::Writable for Intb1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTB1 to value 0"]
impl crate::Resettable for Intb1Spec {
    const RESET_VALUE: u32 = 0;
}
