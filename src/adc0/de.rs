#[doc = "Register `DE` reader"]
pub type R = crate::R<DeSpec>;
#[doc = "Register `DE` writer"]
pub type W = crate::W<DeSpec>;
#[doc = "FIFO Watermark DMA Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwmde {
    #[doc = "0: DMA request disabled."]
    Fwmde0 = 0,
    #[doc = "1: DMA request enabled."]
    Fwmde1 = 1,
}
impl From<Fwmde> for bool {
    #[inline(always)]
    fn from(variant: Fwmde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWMDE` reader - FIFO Watermark DMA Enable"]
pub type FwmdeR = crate::BitReader<Fwmde>;
impl FwmdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwmde {
        match self.bits {
            false => Fwmde::Fwmde0,
            true => Fwmde::Fwmde1,
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn is_fwmde_0(&self) -> bool {
        *self == Fwmde::Fwmde0
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn is_fwmde_1(&self) -> bool {
        *self == Fwmde::Fwmde1
    }
}
#[doc = "Field `FWMDE` writer - FIFO Watermark DMA Enable"]
pub type FwmdeW<'a, REG> = crate::BitWriter<'a, REG, Fwmde>;
impl<'a, REG> FwmdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn fwmde_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fwmde::Fwmde0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn fwmde_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fwmde::Fwmde1)
    }
}
impl R {
    #[doc = "Bit 0 - FIFO Watermark DMA Enable"]
    #[inline(always)]
    pub fn fwmde(&self) -> FwmdeR {
        FwmdeR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DE").field("fwmde", &self.fwmde()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Watermark DMA Enable"]
    #[inline(always)]
    pub fn fwmde(&mut self) -> FwmdeW<DeSpec> {
        FwmdeW::new(self, 0)
    }
}
#[doc = "DMA Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`de::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`de::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeSpec;
impl crate::RegisterSpec for DeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`de::R`](R) reader structure"]
impl crate::Readable for DeSpec {}
#[doc = "`write(|w| ..)` method takes [`de::W`](W) writer structure"]
impl crate::Writable for DeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DE to value 0"]
impl crate::Resettable for DeSpec {
    const RESET_VALUE: u32 = 0;
}
