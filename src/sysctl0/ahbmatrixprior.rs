#[doc = "Register `AHBMATRIXPRIOR` reader"]
pub type R = crate::R<AhbmatrixpriorSpec>;
#[doc = "Register `AHBMATRIXPRIOR` writer"]
pub type W = crate::W<AhbmatrixpriorSpec>;
#[doc = "Field `M0` reader - Master 0 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3. (0 High)"]
pub type M0R = crate::FieldReader;
#[doc = "Field `M0` writer - Master 0 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3. (0 High)"]
pub type M0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M1` reader - Master 1 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M1R = crate::FieldReader;
#[doc = "Field `M1` writer - Master 1 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M2` reader - Master 2 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M2R = crate::FieldReader;
#[doc = "Field `M2` writer - Master 2 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M3` reader - Master 3 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M3R = crate::FieldReader;
#[doc = "Field `M3` writer - Master 3 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M4` reader - Master 4 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M4R = crate::FieldReader;
#[doc = "Field `M4` writer - Master 4 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M5` reader - Master 5 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M5R = crate::FieldReader;
#[doc = "Field `M5` writer - Master 5 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M6` reader - Master 6 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M6R = crate::FieldReader;
#[doc = "Field `M6` writer - Master 6 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M7` reader - Master 7 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M7R = crate::FieldReader;
#[doc = "Field `M7` writer - Master 7 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M8` reader - Master 8 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M8R = crate::FieldReader;
#[doc = "Field `M8` writer - Master 8 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
pub type M8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Master 0 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3. (0 High)"]
    #[inline(always)]
    pub fn m0(&self) -> M0R {
        M0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Master 1 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub fn m1(&self) -> M1R {
        M1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master 2 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub fn m2(&self) -> M2R {
        M2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Master 3 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub fn m3(&self) -> M3R {
        M3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master 4 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub fn m4(&self) -> M4R {
        M4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Master 5 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub fn m5(&self) -> M5R {
        M5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Master 6 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub fn m6(&self) -> M6R {
        M6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Master 7 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub fn m7(&self) -> M7R {
        M7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Master 8 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub fn m8(&self) -> M8R {
        M8R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBMATRIXPRIOR")
            .field("m0", &self.m0())
            .field("m1", &self.m1())
            .field("m2", &self.m2())
            .field("m3", &self.m3())
            .field("m4", &self.m4())
            .field("m5", &self.m5())
            .field("m6", &self.m6())
            .field("m7", &self.m7())
            .field("m8", &self.m8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 0 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3. (0 High)"]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0W<AhbmatrixpriorSpec> {
        M0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Master 1 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1W<AhbmatrixpriorSpec> {
        M1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Master 2 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    #[must_use]
    pub fn m2(&mut self) -> M2W<AhbmatrixpriorSpec> {
        M2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Master 3 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    #[must_use]
    pub fn m3(&mut self) -> M3W<AhbmatrixpriorSpec> {
        M3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Master 4 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    #[must_use]
    pub fn m4(&mut self) -> M4W<AhbmatrixpriorSpec> {
        M4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Master 5 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    #[must_use]
    pub fn m5(&mut self) -> M5W<AhbmatrixpriorSpec> {
        M5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Master 6 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    #[must_use]
    pub fn m6(&mut self) -> M6W<AhbmatrixpriorSpec> {
        M6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Master 7 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    #[must_use]
    pub fn m7(&mut self) -> M7W<AhbmatrixpriorSpec> {
        M7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Master 8 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    #[must_use]
    pub fn m8(&mut self) -> M8W<AhbmatrixpriorSpec> {
        M8W::new(self, 16)
    }
}
#[doc = "AHB matrix priority\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmatrixprior::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmatrixprior::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmatrixpriorSpec;
impl crate::RegisterSpec for AhbmatrixpriorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmatrixprior::R`](R) reader structure"]
impl crate::Readable for AhbmatrixpriorSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmatrixprior::W`](W) writer structure"]
impl crate::Writable for AhbmatrixpriorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBMATRIXPRIOR to value 0"]
impl crate::Resettable for AhbmatrixpriorSpec {
    const RESET_VALUE: u32 = 0;
}
