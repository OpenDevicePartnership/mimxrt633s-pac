#[doc = "Register `PCFG1` reader"]
pub type R = crate::R<Pcfg1Spec>;
#[doc = "Register `PCFG1` writer"]
pub type W = crate::W<Pcfg1Spec>;
#[doc = "Field `PAIRENABLE` reader - Enable for this channel pair.."]
pub type PairenableR = crate::BitReader;
#[doc = "Field `PAIRENABLE` writer - Enable for this channel pair.."]
pub type PairenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONECHANNEL` reader - Single channel mode."]
pub type OnechannelR = crate::BitReader;
#[doc = "Field `ONECHANNEL` writer - Single channel mode."]
pub type OnechannelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable for this channel pair.."]
    #[inline(always)]
    pub fn pairenable(&self) -> PairenableR {
        PairenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - Single channel mode."]
    #[inline(always)]
    pub fn onechannel(&self) -> OnechannelR {
        OnechannelR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCFG1")
            .field("pairenable", &self.pairenable())
            .field("onechannel", &self.onechannel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable for this channel pair.."]
    #[inline(always)]
    #[must_use]
    pub fn pairenable(&mut self) -> PairenableW<Pcfg1Spec> {
        PairenableW::new(self, 0)
    }
    #[doc = "Bit 10 - Single channel mode."]
    #[inline(always)]
    #[must_use]
    pub fn onechannel(&mut self) -> OnechannelW<Pcfg1Spec> {
        OnechannelW::new(self, 10)
    }
}
#[doc = "Configuration register 1 for channel pair\n\nYou can [`read`](crate::Reg::read) this register and get [`pcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcfg1Spec;
impl crate::RegisterSpec for Pcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcfg1::R`](R) reader structure"]
impl crate::Readable for Pcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcfg1::W`](W) writer structure"]
impl crate::Writable for Pcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCFG1 to value 0"]
impl crate::Resettable for Pcfg1Spec {
    const RESET_VALUE: u32 = 0;
}
