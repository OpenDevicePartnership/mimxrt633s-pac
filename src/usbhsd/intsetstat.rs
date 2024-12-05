#[doc = "Register `INTSETSTAT` reader"]
pub type R = crate::R<IntsetstatSpec>;
#[doc = "Register `INTSETSTAT` writer"]
pub type W = crate::W<IntsetstatSpec>;
#[doc = "Field `EP_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub type EpSetIntR = crate::FieldReader<u16>;
#[doc = "Field `EP_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub type EpSetIntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FRAME_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub type FrameSetIntR = crate::BitReader;
#[doc = "Field `FRAME_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub type FrameSetIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_SET_INT` reader - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub type DevSetIntR = crate::BitReader;
#[doc = "Field `DEV_SET_INT` writer - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
pub type DevSetIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn ep_set_int(&self) -> EpSetIntR {
        EpSetIntR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn frame_set_int(&self) -> FrameSetIntR {
        FrameSetIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn dev_set_int(&self) -> DevSetIntR {
        DevSetIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSETSTAT")
            .field("ep_set_int", &self.ep_set_int())
            .field("frame_set_int", &self.frame_set_int())
            .field("dev_set_int", &self.dev_set_int())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn ep_set_int(&mut self) -> EpSetIntW<IntsetstatSpec> {
        EpSetIntW::new(self, 0)
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn frame_set_int(&mut self) -> FrameSetIntW<IntsetstatSpec> {
        FrameSetIntW::new(self, 30)
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn dev_set_int(&mut self) -> DevSetIntW<IntsetstatSpec> {
        DevSetIntW::new(self, 31)
    }
}
#[doc = "USB set interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsetstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsetstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsetstatSpec;
impl crate::RegisterSpec for IntsetstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsetstat::R`](R) reader structure"]
impl crate::Readable for IntsetstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intsetstat::W`](W) writer structure"]
impl crate::Writable for IntsetstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSETSTAT to value 0"]
impl crate::Resettable for IntsetstatSpec {
    const RESET_VALUE: u32 = 0;
}
