#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `CAPEN0` reader - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
pub type Capen0R = crate::BitReader;
#[doc = "Field `CAPEN0` writer - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
pub type Capen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPEN1` reader - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
pub type Capen1R = crate::BitReader;
#[doc = "Field `CAPEN1` writer - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
pub type Capen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPEN2` reader - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
pub type Capen2R = crate::BitReader;
#[doc = "Field `CAPEN2` writer - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
pub type Capen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPEN3` reader - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
pub type Capen3R = crate::BitReader;
#[doc = "Field `CAPEN3` writer - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
pub type Capen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPPOL0` reader - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type Cappol0R = crate::BitReader;
#[doc = "Field `CAPPOL0` writer - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type Cappol0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPPOL1` reader - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type Cappol1R = crate::BitReader;
#[doc = "Field `CAPPOL1` writer - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type Cappol1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPPOL2` reader - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type Cappol2R = crate::BitReader;
#[doc = "Field `CAPPOL2` writer - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type Cappol2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPPOL3` reader - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type Cappol3R = crate::BitReader;
#[doc = "Field `CAPPOL3` writer - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type Cappol3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen0(&self) -> Capen0R {
        Capen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen1(&self) -> Capen1R {
        Capen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen2(&self) -> Capen2R {
        Capen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen3(&self) -> Capen3R {
        Capen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol0(&self) -> Cappol0R {
        Cappol0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol1(&self) -> Cappol1R {
        Cappol1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol2(&self) -> Cappol2R {
        Cappol2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol3(&self) -> Cappol3R {
        Cappol3R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("capen0", &self.capen0())
            .field("capen1", &self.capen1())
            .field("capen2", &self.capen2())
            .field("capen3", &self.capen3())
            .field("cappol0", &self.cappol0())
            .field("cappol1", &self.cappol1())
            .field("cappol2", &self.cappol2())
            .field("cappol3", &self.cappol3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen0(&mut self) -> Capen0W<CfgSpec> {
        Capen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen1(&mut self) -> Capen1W<CfgSpec> {
        Capen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen2(&mut self) -> Capen2W<CfgSpec> {
        Capen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen3(&mut self) -> Capen3W<CfgSpec> {
        Capen3W::new(self, 3)
    }
    #[doc = "Bit 8 - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol0(&mut self) -> Cappol0W<CfgSpec> {
        Cappol0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol1(&mut self) -> Cappol1W<CfgSpec> {
        Cappol1W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol2(&mut self) -> Cappol2W<CfgSpec> {
        Cappol2W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol3(&mut self) -> Cappol3W<CfgSpec> {
        Cappol3W::new(self, 11)
    }
}
#[doc = "Capture configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
