#[doc = "Register `STATADDR` reader"]
pub type R = crate::R<StataddrSpec>;
#[doc = "Register `STATADDR` writer"]
pub type W = crate::W<StataddrSpec>;
#[doc = "Field `OFF` reader - The offset in Host space for the status block. It must be double-word aligned."]
pub type OffR = crate::FieldReader<u16>;
#[doc = "Field `OFF` writer - The offset in Host space for the status block. It must be double-word aligned."]
pub type OffW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "The meaning is dependent on type of port:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Base {
    #[doc = "0: Is offset from 0 in host memory or I/O space"]
    OffsetFrom0 = 0,
    #[doc = "1: Uses BASE1 offset in host memory (see MAPBASE reg)"]
    Base1 = 1,
    #[doc = "2: Uses BASE2 offset in host memory (see MAPBASE reg)"]
    Base2 = 2,
}
impl From<Base> for u8 {
    #[inline(always)]
    fn from(variant: Base) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Base {
    type Ux = u8;
}
impl crate::IsEnum for Base {}
#[doc = "Field `BASE` reader - The meaning is dependent on type of port:"]
pub type BaseR = crate::FieldReader<Base>;
impl BaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Base> {
        match self.bits {
            0 => Some(Base::OffsetFrom0),
            1 => Some(Base::Base1),
            2 => Some(Base::Base2),
            _ => None,
        }
    }
    #[doc = "Is offset from 0 in host memory or I/O space"]
    #[inline(always)]
    pub fn is_offset_from_0(&self) -> bool {
        *self == Base::OffsetFrom0
    }
    #[doc = "Uses BASE1 offset in host memory (see MAPBASE reg)"]
    #[inline(always)]
    pub fn is_base1(&self) -> bool {
        *self == Base::Base1
    }
    #[doc = "Uses BASE2 offset in host memory (see MAPBASE reg)"]
    #[inline(always)]
    pub fn is_base2(&self) -> bool {
        *self == Base::Base2
    }
}
#[doc = "Field `BASE` writer - The meaning is dependent on type of port:"]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 2, Base>;
impl<'a, REG> BaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Is offset from 0 in host memory or I/O space"]
    #[inline(always)]
    pub fn offset_from_0(self) -> &'a mut crate::W<REG> {
        self.variant(Base::OffsetFrom0)
    }
    #[doc = "Uses BASE1 offset in host memory (see MAPBASE reg)"]
    #[inline(always)]
    pub fn base1(self) -> &'a mut crate::W<REG> {
        self.variant(Base::Base1)
    }
    #[doc = "Uses BASE2 offset in host memory (see MAPBASE reg)"]
    #[inline(always)]
    pub fn base2(self) -> &'a mut crate::W<REG> {
        self.variant(Base::Base2)
    }
}
impl R {
    #[doc = "Bits 3:15 - The offset in Host space for the status block. It must be double-word aligned."]
    #[inline(always)]
    pub fn off(&self) -> OffR {
        OffR::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:17 - The meaning is dependent on type of port:"]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATADDR")
            .field("off", &self.off())
            .field("base", &self.base())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:15 - The offset in Host space for the status block. It must be double-word aligned."]
    #[inline(always)]
    pub fn off(&mut self) -> OffW<StataddrSpec> {
        OffW::new(self, 3)
    }
    #[doc = "Bits 16:17 - The meaning is dependent on type of port:"]
    #[inline(always)]
    pub fn base(&mut self) -> BaseW<StataddrSpec> {
        BaseW::new(self, 16)
    }
}
#[doc = "Location of Status block in memory space, if enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`stataddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stataddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StataddrSpec;
impl crate::RegisterSpec for StataddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stataddr::R`](R) reader structure"]
impl crate::Readable for StataddrSpec {}
#[doc = "`write(|w| ..)` method takes [`stataddr::W`](W) writer structure"]
impl crate::Writable for StataddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATADDR to value 0"]
impl crate::Resettable for StataddrSpec {
    const RESET_VALUE: u32 = 0;
}
