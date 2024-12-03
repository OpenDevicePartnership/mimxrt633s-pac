#[doc = "Register `MAPBASE` reader"]
pub type R = crate::R<MapbaseSpec>;
#[doc = "Register `MAPBASE` writer"]
pub type W = crate::W<MapbaseSpec>;
#[doc = "Field `BASE0` reader - no description available"]
pub type Base0R = crate::FieldReader<u16>;
#[doc = "Field `BASE0` writer - no description available"]
pub type Base0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BASE1` reader - no description available"]
pub type Base1R = crate::FieldReader<u16>;
#[doc = "Field `BASE1` writer - no description available"]
pub type Base1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn base0(&self) -> Base0R {
        Base0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn base1(&self) -> Base1R {
        Base1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAPBASE")
            .field("base0", &self.base0())
            .field("base1", &self.base1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn base0(&mut self) -> Base0W<MapbaseSpec> {
        Base0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn base1(&mut self) -> Base1W<MapbaseSpec> {
        Base1W::new(self, 16)
    }
}
#[doc = "Base0 and Base1 mapped offsets for ports\n\nYou can [`read`](crate::Reg::read) this register and get [`mapbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mapbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MapbaseSpec;
impl crate::RegisterSpec for MapbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mapbase::R`](R) reader structure"]
impl crate::Readable for MapbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`mapbase::W`](W) writer structure"]
impl crate::Writable for MapbaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAPBASE to value 0"]
impl crate::Resettable for MapbaseSpec {
    const RESET_VALUE: u32 = 0;
}
