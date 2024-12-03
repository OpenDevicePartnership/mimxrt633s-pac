#[doc = "Register `ABORT1` writer"]
pub type W = crate::W<Abort1Spec>;
#[doc = "Field `CHANNEL(32-63)` writer - "]
pub type ChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Abort1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CHANNEL32` field.</div>"]
    #[inline(always)]
    pub fn channel(&mut self, n: u8) -> ChannelW<Abort1Spec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ChannelW::new(self, n)
    }
    #[doc = "Bit 0 - CHANNEL32"]
    #[inline(always)]
    pub fn channel32(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 0)
    }
    #[doc = "Bit 1 - CHANNEL33"]
    #[inline(always)]
    pub fn channel33(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 1)
    }
    #[doc = "Bit 2 - CHANNEL34"]
    #[inline(always)]
    pub fn channel34(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 2)
    }
    #[doc = "Bit 3 - CHANNEL35"]
    #[inline(always)]
    pub fn channel35(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 3)
    }
    #[doc = "Bit 4 - CHANNEL36"]
    #[inline(always)]
    pub fn channel36(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 4)
    }
    #[doc = "Bit 5 - CHANNEL37"]
    #[inline(always)]
    pub fn channel37(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 5)
    }
    #[doc = "Bit 6 - CHANNEL38"]
    #[inline(always)]
    pub fn channel38(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 6)
    }
    #[doc = "Bit 7 - CHANNEL39"]
    #[inline(always)]
    pub fn channel39(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 7)
    }
    #[doc = "Bit 8 - CHANNEL40"]
    #[inline(always)]
    pub fn channel40(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 8)
    }
    #[doc = "Bit 9 - CHANNEL41"]
    #[inline(always)]
    pub fn channel41(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 9)
    }
    #[doc = "Bit 10 - CHANNEL42"]
    #[inline(always)]
    pub fn channel42(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 10)
    }
    #[doc = "Bit 11 - CHANNEL43"]
    #[inline(always)]
    pub fn channel43(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 11)
    }
    #[doc = "Bit 12 - CHANNEL44"]
    #[inline(always)]
    pub fn channel44(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 12)
    }
    #[doc = "Bit 13 - CHANNEL45"]
    #[inline(always)]
    pub fn channel45(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 13)
    }
    #[doc = "Bit 14 - CHANNEL46"]
    #[inline(always)]
    pub fn channel46(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 14)
    }
    #[doc = "Bit 15 - CHANNEL47"]
    #[inline(always)]
    pub fn channel47(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 15)
    }
    #[doc = "Bit 16 - CHANNEL48"]
    #[inline(always)]
    pub fn channel48(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 16)
    }
    #[doc = "Bit 17 - CHANNEL49"]
    #[inline(always)]
    pub fn channel49(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 17)
    }
    #[doc = "Bit 18 - CHANNEL50"]
    #[inline(always)]
    pub fn channel50(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 18)
    }
    #[doc = "Bit 19 - CHANNEL51"]
    #[inline(always)]
    pub fn channel51(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 19)
    }
    #[doc = "Bit 20 - CHANNEL52"]
    #[inline(always)]
    pub fn channel52(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 20)
    }
    #[doc = "Bit 21 - CHANNEL53"]
    #[inline(always)]
    pub fn channel53(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 21)
    }
    #[doc = "Bit 22 - CHANNEL54"]
    #[inline(always)]
    pub fn channel54(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 22)
    }
    #[doc = "Bit 23 - CHANNEL55"]
    #[inline(always)]
    pub fn channel55(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 23)
    }
    #[doc = "Bit 24 - CHANNEL56"]
    #[inline(always)]
    pub fn channel56(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 24)
    }
    #[doc = "Bit 25 - CHANNEL57"]
    #[inline(always)]
    pub fn channel57(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 25)
    }
    #[doc = "Bit 26 - CHANNEL58"]
    #[inline(always)]
    pub fn channel58(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 26)
    }
    #[doc = "Bit 27 - CHANNEL59"]
    #[inline(always)]
    pub fn channel59(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 27)
    }
    #[doc = "Bit 28 - CHANNEL60"]
    #[inline(always)]
    pub fn channel60(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 28)
    }
    #[doc = "Bit 29 - CHANNEL61"]
    #[inline(always)]
    pub fn channel61(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 29)
    }
    #[doc = "Bit 30 - CHANNEL62"]
    #[inline(always)]
    pub fn channel62(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 30)
    }
    #[doc = "Bit 31 - CHANNEL63"]
    #[inline(always)]
    pub fn channel63(&mut self) -> ChannelW<Abort1Spec> {
        ChannelW::new(self, 31)
    }
}
#[doc = "Channel Abort control for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abort1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Abort1Spec;
impl crate::RegisterSpec for Abort1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`abort1::W`](W) writer structure"]
impl crate::Writable for Abort1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABORT1 to value 0"]
impl crate::Resettable for Abort1Spec {
    const RESET_VALUE: u32 = 0;
}
