#[doc = "Register `IE` reader"]
pub type R = crate::R<IeSpec>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IeSpec>;
#[doc = "FIFO Watermark Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwmie {
    #[doc = "0: FIFO watermark interrupts are not enabled."]
    Fwmie0 = 0,
    #[doc = "1: FIFO watermark interrupts are enabled."]
    Fwmie1 = 1,
}
impl From<Fwmie> for bool {
    #[inline(always)]
    fn from(variant: Fwmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWMIE` reader - FIFO Watermark Interrupt Enable"]
pub type FwmieR = crate::BitReader<Fwmie>;
impl FwmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwmie {
        match self.bits {
            false => Fwmie::Fwmie0,
            true => Fwmie::Fwmie1,
        }
    }
    #[doc = "FIFO watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn is_fwmie_0(&self) -> bool {
        *self == Fwmie::Fwmie0
    }
    #[doc = "FIFO watermark interrupts are enabled."]
    #[inline(always)]
    pub fn is_fwmie_1(&self) -> bool {
        *self == Fwmie::Fwmie1
    }
}
#[doc = "Field `FWMIE` writer - FIFO Watermark Interrupt Enable"]
pub type FwmieW<'a, REG> = crate::BitWriter<'a, REG, Fwmie>;
impl<'a, REG> FwmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn fwmie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fwmie::Fwmie0)
    }
    #[doc = "FIFO watermark interrupts are enabled."]
    #[inline(always)]
    pub fn fwmie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fwmie::Fwmie1)
    }
}
#[doc = "Result FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fofie {
    #[doc = "0: FIFO overflow interrupts are not enabled."]
    Fofie0 = 0,
    #[doc = "1: FIFO overflow interrupts are enabled."]
    Fofie1 = 1,
}
impl From<Fofie> for bool {
    #[inline(always)]
    fn from(variant: Fofie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOFIE` reader - Result FIFO Overflow Interrupt Enable"]
pub type FofieR = crate::BitReader<Fofie>;
impl FofieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fofie {
        match self.bits {
            false => Fofie::Fofie0,
            true => Fofie::Fofie1,
        }
    }
    #[doc = "FIFO overflow interrupts are not enabled."]
    #[inline(always)]
    pub fn is_fofie_0(&self) -> bool {
        *self == Fofie::Fofie0
    }
    #[doc = "FIFO overflow interrupts are enabled."]
    #[inline(always)]
    pub fn is_fofie_1(&self) -> bool {
        *self == Fofie::Fofie1
    }
}
#[doc = "Field `FOFIE` writer - Result FIFO Overflow Interrupt Enable"]
pub type FofieW<'a, REG> = crate::BitWriter<'a, REG, Fofie>;
impl<'a, REG> FofieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO overflow interrupts are not enabled."]
    #[inline(always)]
    pub fn fofie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fofie::Fofie0)
    }
    #[doc = "FIFO overflow interrupts are enabled."]
    #[inline(always)]
    pub fn fofie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fofie::Fofie1)
    }
}
impl R {
    #[doc = "Bit 0 - FIFO Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie(&self) -> FwmieR {
        FwmieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie(&self) -> FofieR {
        FofieR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IE")
            .field("fwmie", &self.fwmie())
            .field("fofie", &self.fofie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Watermark Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwmie(&mut self) -> FwmieW<IeSpec> {
        FwmieW::new(self, 0)
    }
    #[doc = "Bit 1 - Result FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fofie(&mut self) -> FofieW<IeSpec> {
        FofieW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeSpec;
impl crate::RegisterSpec for IeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IeSpec {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IeSpec {
    const RESET_VALUE: u32 = 0;
}
