#[doc = "Register `SDIO1FCLKDIV` reader"]
pub type R = crate::R<Sdio1fclkdivSpec>;
#[doc = "Register `SDIO1FCLKDIV` writer"]
pub type W = crate::W<Sdio1fclkdivSpec>;
#[doc = "Field `DIV` reader - Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESET` reader - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT` reader - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
pub type HaltR = crate::BitReader;
#[doc = "Field `HALT` writer - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 29 - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 30) & 1) != 0)
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
        f.debug_struct("SDIO1FCLKDIV")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<Sdio1fclkdivSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 29 - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<Sdio1fclkdivSpec> {
        ResetW::new(self, 29)
    }
    #[doc = "Bit 30 - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn halt(&mut self) -> HaltW<Sdio1fclkdivSpec> {
        HaltW::new(self, 30)
    }
    #[doc = "Bit 31 - Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn reqflag(&mut self) -> ReqflagW<Sdio1fclkdivSpec> {
        ReqflagW::new(self, 31)
    }
}
#[doc = "SDIO1 FCLK divider\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio1fclkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio1fclkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdio1fclkdivSpec;
impl crate::RegisterSpec for Sdio1fclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio1fclkdiv::R`](R) reader structure"]
impl crate::Readable for Sdio1fclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`sdio1fclkdiv::W`](W) writer structure"]
impl crate::Writable for Sdio1fclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIO1FCLKDIV to value 0x4000_0000"]
impl crate::Resettable for Sdio1fclkdivSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
