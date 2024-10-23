#[doc = "Register `CAPCLR` writer"]
pub type W = crate::W<CapclrSpec>;
#[doc = "Field `CAPCLR0` writer - Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
pub type Capclr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCLR1` writer - Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
pub type Capclr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCLR2` writer - Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
pub type Capclr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCLR3` writer - Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
pub type Capclr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CapclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
    #[inline(always)]
    #[must_use]
    pub fn capclr0(&mut self) -> Capclr0W<CapclrSpec> {
        Capclr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
    #[inline(always)]
    #[must_use]
    pub fn capclr1(&mut self) -> Capclr1W<CapclrSpec> {
        Capclr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
    #[inline(always)]
    #[must_use]
    pub fn capclr2(&mut self) -> Capclr2W<CapclrSpec> {
        Capclr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
    #[inline(always)]
    #[must_use]
    pub fn capclr3(&mut self) -> Capclr3W<CapclrSpec> {
        Capclr3W::new(self, 3)
    }
}
#[doc = "Capture clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapclrSpec;
impl crate::RegisterSpec for CapclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`capclr::W`](W) writer structure"]
impl crate::Writable for CapclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAPCLR to value 0"]
impl crate::Resettable for CapclrSpec {
    const RESET_VALUE: u32 = 0;
}
