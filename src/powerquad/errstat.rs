#[doc = "Register `ERRSTAT` reader"]
pub type R = crate::R<ErrstatSpec>;
#[doc = "Register `ERRSTAT` writer"]
pub type W = crate::W<ErrstatSpec>;
#[doc = "Field `OVERFLOW` reader - overflow"]
pub type OverflowR = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - overflow"]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAN` reader - nan"]
pub type NanR = crate::BitReader;
#[doc = "Field `NAN` writer - nan"]
pub type NanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIXEDOVERFLOW` reader - fixed_pt_overflow"]
pub type FixedoverflowR = crate::BitReader;
#[doc = "Field `FIXEDOVERFLOW` writer - fixed_pt_overflow"]
pub type FixedoverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - underflow"]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - underflow"]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERROR` reader - bus_error"]
pub type BuserrorR = crate::BitReader;
#[doc = "Field `BUSERROR` writer - bus_error"]
pub type BuserrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - overflow"]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - nan"]
    #[inline(always)]
    pub fn nan(&self) -> NanR {
        NanR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - fixed_pt_overflow"]
    #[inline(always)]
    pub fn fixedoverflow(&self) -> FixedoverflowR {
        FixedoverflowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - underflow"]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - bus_error"]
    #[inline(always)]
    pub fn buserror(&self) -> BuserrorR {
        BuserrorR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERRSTAT")
            .field("overflow", &self.overflow())
            .field("nan", &self.nan())
            .field("fixedoverflow", &self.fixedoverflow())
            .field("underflow", &self.underflow())
            .field("buserror", &self.buserror())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - overflow"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<ErrstatSpec> {
        OverflowW::new(self, 0)
    }
    #[doc = "Bit 1 - nan"]
    #[inline(always)]
    pub fn nan(&mut self) -> NanW<ErrstatSpec> {
        NanW::new(self, 1)
    }
    #[doc = "Bit 2 - fixed_pt_overflow"]
    #[inline(always)]
    pub fn fixedoverflow(&mut self) -> FixedoverflowW<ErrstatSpec> {
        FixedoverflowW::new(self, 2)
    }
    #[doc = "Bit 3 - underflow"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UnderflowW<ErrstatSpec> {
        UnderflowW::new(self, 3)
    }
    #[doc = "Bit 4 - bus_error"]
    #[inline(always)]
    pub fn buserror(&mut self) -> BuserrorW<ErrstatSpec> {
        BuserrorW::new(self, 4)
    }
}
#[doc = "Read/Write register where error statuses are captured (sticky)\n\nYou can [`read`](crate::Reg::read) this register and get [`errstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrstatSpec;
impl crate::RegisterSpec for ErrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errstat::R`](R) reader structure"]
impl crate::Readable for ErrstatSpec {}
#[doc = "`write(|w| ..)` method takes [`errstat::W`](W) writer structure"]
impl crate::Writable for ErrstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRSTAT to value 0"]
impl crate::Resettable for ErrstatSpec {
    const RESET_VALUE: u32 = 0;
}
