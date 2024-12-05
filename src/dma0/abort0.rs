#[doc = "Register `ABORT0` writer"]
pub type W = crate::W<Abort0Spec>;
#[doc = "Field `CHANNEL(0-31)` writer - "]
pub type ChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Abort0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CHANNEL0` field.</div>"]
    #[inline(always)]
    pub fn channel(&mut self, n: u8) -> ChannelW<Abort0Spec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ChannelW::new(self, n)
    }
    #[doc = "Bit 0 - CHANNEL0"]
    #[inline(always)]
    pub fn channel0(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 0)
    }
    #[doc = "Bit 1 - CHANNEL1"]
    #[inline(always)]
    pub fn channel1(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 1)
    }
    #[doc = "Bit 2 - CHANNEL2"]
    #[inline(always)]
    pub fn channel2(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 2)
    }
    #[doc = "Bit 3 - CHANNEL3"]
    #[inline(always)]
    pub fn channel3(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 3)
    }
    #[doc = "Bit 4 - CHANNEL4"]
    #[inline(always)]
    pub fn channel4(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 4)
    }
    #[doc = "Bit 5 - CHANNEL5"]
    #[inline(always)]
    pub fn channel5(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 5)
    }
    #[doc = "Bit 6 - CHANNEL6"]
    #[inline(always)]
    pub fn channel6(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 6)
    }
    #[doc = "Bit 7 - CHANNEL7"]
    #[inline(always)]
    pub fn channel7(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 7)
    }
    #[doc = "Bit 8 - CHANNEL8"]
    #[inline(always)]
    pub fn channel8(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 8)
    }
    #[doc = "Bit 9 - CHANNEL9"]
    #[inline(always)]
    pub fn channel9(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 9)
    }
    #[doc = "Bit 10 - CHANNEL10"]
    #[inline(always)]
    pub fn channel10(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 10)
    }
    #[doc = "Bit 11 - CHANNEL11"]
    #[inline(always)]
    pub fn channel11(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 11)
    }
    #[doc = "Bit 12 - CHANNEL12"]
    #[inline(always)]
    pub fn channel12(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 12)
    }
    #[doc = "Bit 13 - CHANNEL13"]
    #[inline(always)]
    pub fn channel13(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 13)
    }
    #[doc = "Bit 14 - CHANNEL14"]
    #[inline(always)]
    pub fn channel14(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 14)
    }
    #[doc = "Bit 15 - CHANNEL15"]
    #[inline(always)]
    pub fn channel15(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 15)
    }
    #[doc = "Bit 16 - CHANNEL16"]
    #[inline(always)]
    pub fn channel16(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 16)
    }
    #[doc = "Bit 17 - CHANNEL17"]
    #[inline(always)]
    pub fn channel17(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 17)
    }
    #[doc = "Bit 18 - CHANNEL18"]
    #[inline(always)]
    pub fn channel18(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 18)
    }
    #[doc = "Bit 19 - CHANNEL19"]
    #[inline(always)]
    pub fn channel19(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 19)
    }
    #[doc = "Bit 20 - CHANNEL20"]
    #[inline(always)]
    pub fn channel20(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 20)
    }
    #[doc = "Bit 21 - CHANNEL21"]
    #[inline(always)]
    pub fn channel21(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 21)
    }
    #[doc = "Bit 22 - CHANNEL22"]
    #[inline(always)]
    pub fn channel22(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 22)
    }
    #[doc = "Bit 23 - CHANNEL23"]
    #[inline(always)]
    pub fn channel23(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 23)
    }
    #[doc = "Bit 24 - CHANNEL24"]
    #[inline(always)]
    pub fn channel24(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 24)
    }
    #[doc = "Bit 25 - CHANNEL25"]
    #[inline(always)]
    pub fn channel25(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 25)
    }
    #[doc = "Bit 26 - CHANNEL26"]
    #[inline(always)]
    pub fn channel26(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 26)
    }
    #[doc = "Bit 27 - CHANNEL27"]
    #[inline(always)]
    pub fn channel27(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 27)
    }
    #[doc = "Bit 28 - CHANNEL28"]
    #[inline(always)]
    pub fn channel28(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 28)
    }
    #[doc = "Bit 29 - CHANNEL29"]
    #[inline(always)]
    pub fn channel29(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 29)
    }
    #[doc = "Bit 30 - CHANNEL30"]
    #[inline(always)]
    pub fn channel30(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 30)
    }
    #[doc = "Bit 31 - CHANNEL31"]
    #[inline(always)]
    pub fn channel31(&mut self) -> ChannelW<Abort0Spec> {
        ChannelW::new(self, 31)
    }
}
#[doc = "Channel Abort control for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abort0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Abort0Spec;
impl crate::RegisterSpec for Abort0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`abort0::W`](W) writer structure"]
impl crate::Writable for Abort0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABORT0 to value 0"]
impl crate::Resettable for Abort0Spec {
    const RESET_VALUE: u32 = 0;
}
