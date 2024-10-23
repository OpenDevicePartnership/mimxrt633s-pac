#[doc = "Register `TIMER2_BC11` reader"]
pub type R = crate::R<Timer2Bc11Spec>;
#[doc = "Register `TIMER2_BC11` writer"]
pub type W = crate::W<Timer2Bc11Spec>;
#[doc = "Field `CHECK_DM` reader - Time Before Check of D- Line"]
pub type CheckDmR = crate::FieldReader;
#[doc = "Field `CHECK_DM` writer - Time Before Check of D- Line"]
pub type CheckDmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TVDPSRC_CON` reader - Time Period Before Enabling D+ Pullup"]
pub type TvdpsrcConR = crate::FieldReader<u16>;
#[doc = "Field `TVDPSRC_CON` writer - Time Period Before Enabling D+ Pullup"]
pub type TvdpsrcConW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - Time Before Check of D- Line"]
    #[inline(always)]
    pub fn check_dm(&self) -> CheckDmR {
        CheckDmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:25 - Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    pub fn tvdpsrc_con(&self) -> TvdpsrcConR {
        TvdpsrcConR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER2_BC11")
            .field("check_dm", &self.check_dm())
            .field("tvdpsrc_con", &self.tvdpsrc_con())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Time Before Check of D- Line"]
    #[inline(always)]
    #[must_use]
    pub fn check_dm(&mut self) -> CheckDmW<Timer2Bc11Spec> {
        CheckDmW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    #[must_use]
    pub fn tvdpsrc_con(&mut self) -> TvdpsrcConW<Timer2Bc11Spec> {
        TvdpsrcConW::new(self, 16)
    }
}
#[doc = "TIMER2_BC11 register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_bc11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_bc11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2Bc11Spec;
impl crate::RegisterSpec for Timer2Bc11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2_bc11::R`](R) reader structure"]
impl crate::Readable for Timer2Bc11Spec {}
#[doc = "`write(|w| ..)` method takes [`timer2_bc11::W`](W) writer structure"]
impl crate::Writable for Timer2Bc11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2_BC11 to value 0x0028_0001"]
impl crate::Resettable for Timer2Bc11Spec {
    const RESET_VALUE: u32 = 0x0028_0001;
}
