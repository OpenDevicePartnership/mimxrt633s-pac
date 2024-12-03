#[doc = "Register `OUT_CLR` reader"]
pub type R = crate::R<OutClrSpec>;
#[doc = "Register `OUT_CLR` writer"]
pub type W = crate::W<OutClrSpec>;
#[doc = "Field `CLR` reader - A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
pub type ClrR = crate::FieldReader<u16>;
#[doc = "Field `CLR` writer - A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CLR").field("clr", &self.clr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "SCT output 0 clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutClrSpec;
impl crate::RegisterSpec for OutClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_clr::R`](R) reader structure"]
impl crate::Readable for OutClrSpec {}
#[doc = "`write(|w| ..)` method takes [`out_clr::W`](W) writer structure"]
impl crate::Writable for OutClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_CLR to value 0"]
impl crate::Resettable for OutClrSpec {
    const RESET_VALUE: u32 = 0;
}
