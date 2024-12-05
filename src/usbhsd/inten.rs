#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `EP_INT_EN` reader - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub type EpIntEnR = crate::FieldReader<u16>;
#[doc = "Field `EP_INT_EN` writer - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub type EpIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FRAME_INT_EN` reader - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub type FrameIntEnR = crate::BitReader;
#[doc = "Field `FRAME_INT_EN` writer - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub type FrameIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_INT_EN` reader - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub type DevIntEnR = crate::BitReader;
#[doc = "Field `DEV_INT_EN` writer - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
pub type DevIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn ep_int_en(&self) -> EpIntEnR {
        EpIntEnR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn frame_int_en(&self) -> FrameIntEnR {
        FrameIntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn dev_int_en(&self) -> DevIntEnR {
        DevIntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("ep_int_en", &self.ep_int_en())
            .field("frame_int_en", &self.frame_int_en())
            .field("dev_int_en", &self.dev_int_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn ep_int_en(&mut self) -> EpIntEnW<IntenSpec> {
        EpIntEnW::new(self, 0)
    }
    #[doc = "Bit 30 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn frame_int_en(&mut self) -> FrameIntEnW<IntenSpec> {
        FrameIntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn dev_int_en(&mut self) -> DevIntEnW<IntenSpec> {
        DevIntEnW::new(self, 31)
    }
}
#[doc = "USB interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
