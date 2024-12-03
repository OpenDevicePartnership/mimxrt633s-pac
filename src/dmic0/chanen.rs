#[doc = "Register `CHANEN` reader"]
pub type R = crate::R<ChanenSpec>;
#[doc = "Register `CHANEN` writer"]
pub type W = crate::W<ChanenSpec>;
#[doc = "Field `EN_CH0` reader - Enable channel 0. When 1, PDM channel 0 is enabled."]
pub type EnCh0R = crate::BitReader;
#[doc = "Field `EN_CH0` writer - Enable channel 0. When 1, PDM channel 0 is enabled."]
pub type EnCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CH1` reader - Enable channel 1. When 1, PDM channel 1 is enabled."]
pub type EnCh1R = crate::BitReader;
#[doc = "Field `EN_CH1` writer - Enable channel 1. When 1, PDM channel 1 is enabled."]
pub type EnCh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CH2` reader - Enable channel 2. When 1, PDM channel 2 is enabled."]
pub type EnCh2R = crate::BitReader;
#[doc = "Field `EN_CH2` writer - Enable channel 2. When 1, PDM channel 2 is enabled."]
pub type EnCh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CH3` reader - Enable channel 3. When 1, PDM channel 3 is enabled."]
pub type EnCh3R = crate::BitReader;
#[doc = "Field `EN_CH3` writer - Enable channel 3. When 1, PDM channel 3 is enabled."]
pub type EnCh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CH4` reader - Enable channel 4. When 1, PDM channel 4 is enabled."]
pub type EnCh4R = crate::BitReader;
#[doc = "Field `EN_CH4` writer - Enable channel 4. When 1, PDM channel 4 is enabled."]
pub type EnCh4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CH5` reader - Enable channel 5. When 1, PDM channel 5 is enabled."]
pub type EnCh5R = crate::BitReader;
#[doc = "Field `EN_CH5` writer - Enable channel 5. When 1, PDM channel 5 is enabled."]
pub type EnCh5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CH6` reader - Enable channel 6. When 1, PDM channel 6 is enabled."]
pub type EnCh6R = crate::BitReader;
#[doc = "Field `EN_CH6` writer - Enable channel 6. When 1, PDM channel 6 is enabled."]
pub type EnCh6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CH7` reader - Enable channel 7. When 1, PDM channel 7 is enabled."]
pub type EnCh7R = crate::BitReader;
#[doc = "Field `EN_CH7` writer - Enable channel 7. When 1, PDM channel 7 is enabled."]
pub type EnCh7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable channel 0. When 1, PDM channel 0 is enabled."]
    #[inline(always)]
    pub fn en_ch0(&self) -> EnCh0R {
        EnCh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable channel 1. When 1, PDM channel 1 is enabled."]
    #[inline(always)]
    pub fn en_ch1(&self) -> EnCh1R {
        EnCh1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable channel 2. When 1, PDM channel 2 is enabled."]
    #[inline(always)]
    pub fn en_ch2(&self) -> EnCh2R {
        EnCh2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable channel 3. When 1, PDM channel 3 is enabled."]
    #[inline(always)]
    pub fn en_ch3(&self) -> EnCh3R {
        EnCh3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable channel 4. When 1, PDM channel 4 is enabled."]
    #[inline(always)]
    pub fn en_ch4(&self) -> EnCh4R {
        EnCh4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable channel 5. When 1, PDM channel 5 is enabled."]
    #[inline(always)]
    pub fn en_ch5(&self) -> EnCh5R {
        EnCh5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable channel 6. When 1, PDM channel 6 is enabled."]
    #[inline(always)]
    pub fn en_ch6(&self) -> EnCh6R {
        EnCh6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable channel 7. When 1, PDM channel 7 is enabled."]
    #[inline(always)]
    pub fn en_ch7(&self) -> EnCh7R {
        EnCh7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHANEN")
            .field("en_ch0", &self.en_ch0())
            .field("en_ch1", &self.en_ch1())
            .field("en_ch2", &self.en_ch2())
            .field("en_ch3", &self.en_ch3())
            .field("en_ch4", &self.en_ch4())
            .field("en_ch5", &self.en_ch5())
            .field("en_ch6", &self.en_ch6())
            .field("en_ch7", &self.en_ch7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable channel 0. When 1, PDM channel 0 is enabled."]
    #[inline(always)]
    pub fn en_ch0(&mut self) -> EnCh0W<ChanenSpec> {
        EnCh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable channel 1. When 1, PDM channel 1 is enabled."]
    #[inline(always)]
    pub fn en_ch1(&mut self) -> EnCh1W<ChanenSpec> {
        EnCh1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable channel 2. When 1, PDM channel 2 is enabled."]
    #[inline(always)]
    pub fn en_ch2(&mut self) -> EnCh2W<ChanenSpec> {
        EnCh2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable channel 3. When 1, PDM channel 3 is enabled."]
    #[inline(always)]
    pub fn en_ch3(&mut self) -> EnCh3W<ChanenSpec> {
        EnCh3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable channel 4. When 1, PDM channel 4 is enabled."]
    #[inline(always)]
    pub fn en_ch4(&mut self) -> EnCh4W<ChanenSpec> {
        EnCh4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable channel 5. When 1, PDM channel 5 is enabled."]
    #[inline(always)]
    pub fn en_ch5(&mut self) -> EnCh5W<ChanenSpec> {
        EnCh5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable channel 6. When 1, PDM channel 6 is enabled."]
    #[inline(always)]
    pub fn en_ch6(&mut self) -> EnCh6W<ChanenSpec> {
        EnCh6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable channel 7. When 1, PDM channel 7 is enabled."]
    #[inline(always)]
    pub fn en_ch7(&mut self) -> EnCh7W<ChanenSpec> {
        EnCh7W::new(self, 7)
    }
}
#[doc = "Channel Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`chanen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chanen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChanenSpec;
impl crate::RegisterSpec for ChanenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chanen::R`](R) reader structure"]
impl crate::Readable for ChanenSpec {}
#[doc = "`write(|w| ..)` method takes [`chanen::W`](W) writer structure"]
impl crate::Writable for ChanenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHANEN to value 0"]
impl crate::Resettable for ChanenSpec {
    const RESET_VALUE: u32 = 0;
}
