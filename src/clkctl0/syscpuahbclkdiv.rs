#[doc = "Register `SYSCPUAHBCLKDIV` reader"]
pub type R = crate::R<SyscpuahbclkdivSpec>;
#[doc = "Register `SYSCPUAHBCLKDIV` writer"]
pub type W = crate::W<SyscpuahbclkdivSpec>;
#[doc = "Field `DIV` reader - Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REQFLAG` reader - Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
pub type ReqflagR = crate::BitReader;
#[doc = "Field `REQFLAG` writer - Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
pub type ReqflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn reqflag(&self) -> ReqflagR {
        ReqflagR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCPUAHBCLKDIV")
            .field("div", &self.div())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<SyscpuahbclkdivSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 31 - Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn reqflag(&mut self) -> ReqflagW<SyscpuahbclkdivSpec> {
        ReqflagW::new(self, 31)
    }
}
#[doc = "system cpu AHB clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`syscpuahbclkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscpuahbclkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscpuahbclkdivSpec;
impl crate::RegisterSpec for SyscpuahbclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscpuahbclkdiv::R`](R) reader structure"]
impl crate::Readable for SyscpuahbclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`syscpuahbclkdiv::W`](W) writer structure"]
impl crate::Writable for SyscpuahbclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCPUAHBCLKDIV to value 0"]
impl crate::Resettable for SyscpuahbclkdivSpec {
    const RESET_VALUE: u32 = 0;
}
