#[doc = "Register `EVEN` reader"]
pub type R = crate::R<EvenSpec>;
#[doc = "Register `EVEN` writer"]
pub type W = crate::W<EvenSpec>;
#[doc = "Field `IEN` reader - The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type IenR = crate::FieldReader<u16>;
#[doc = "Field `IEN` writer - The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type IenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn ien(&self) -> IenR {
        IenR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVEN").field("ien", &self.ien()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn ien(&mut self) -> IenW<EvenSpec> {
        IenW::new(self, 0)
    }
}
#[doc = "SCT event interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`even::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`even::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvenSpec;
impl crate::RegisterSpec for EvenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`even::R`](R) reader structure"]
impl crate::Readable for EvenSpec {}
#[doc = "`write(|w| ..)` method takes [`even::W`](W) writer structure"]
impl crate::Writable for EvenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVEN to value 0"]
impl crate::Resettable for EvenSpec {
    const RESET_VALUE: u32 = 0;
}
