#[doc = "Register `OUT_SET` reader"]
pub type R = crate::R<OutSetSpec>;
#[doc = "Register `OUT_SET` writer"]
pub type W = crate::W<OutSetSpec>;
#[doc = "Field `SET` reader - A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
pub type SetR = crate::FieldReader<u16>;
#[doc = "Field `SET` writer - A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_SET")
            .field("set_", &self.set_())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<OutSetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "SCT output 0 set register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutSetSpec;
impl crate::RegisterSpec for OutSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_set::R`](R) reader structure"]
impl crate::Readable for OutSetSpec {}
#[doc = "`write(|w| ..)` method takes [`out_set::W`](W) writer structure"]
impl crate::Writable for OutSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_SET to value 0"]
impl crate::Resettable for OutSetSpec {
    const RESET_VALUE: u32 = 0;
}
