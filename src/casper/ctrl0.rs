#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<Ctrl0Spec>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<Ctrl0Spec>;
#[doc = "Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abbpair {
    #[doc = "0: Bank-pair 0 (1st)"]
    Pair0 = 0,
    #[doc = "1: Bank-pair 1 (2nd)"]
    Pair1 = 1,
}
impl From<Abbpair> for bool {
    #[inline(always)]
    fn from(variant: Abbpair) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABBPAIR` reader - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
pub type AbbpairR = crate::BitReader<Abbpair>;
impl AbbpairR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abbpair {
        match self.bits {
            false => Abbpair::Pair0,
            true => Abbpair::Pair1,
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == Abbpair::Pair0
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == Abbpair::Pair1
    }
}
#[doc = "Field `ABBPAIR` writer - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
pub type AbbpairW<'a, REG> = crate::BitWriter<'a, REG, Abbpair>;
impl<'a, REG> AbbpairW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut crate::W<REG> {
        self.variant(Abbpair::Pair0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut crate::W<REG> {
        self.variant(Abbpair::Pair1)
    }
}
#[doc = "Field `ABOFF` reader - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
pub type AboffR = crate::BitReader;
#[doc = "Field `ABOFF` writer - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
pub type AboffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdbpair {
    #[doc = "0: Bank-pair 0 (1st)"]
    Pair0 = 0,
    #[doc = "1: Bank-pair 1 (2nd)"]
    Pair1 = 1,
}
impl From<Cdbpair> for bool {
    #[inline(always)]
    fn from(variant: Cdbpair) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDBPAIR` reader - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
pub type CdbpairR = crate::BitReader<Cdbpair>;
impl CdbpairR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdbpair {
        match self.bits {
            false => Cdbpair::Pair0,
            true => Cdbpair::Pair1,
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == Cdbpair::Pair0
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == Cdbpair::Pair1
    }
}
#[doc = "Field `CDBPAIR` writer - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
pub type CdbpairW<'a, REG> = crate::BitWriter<'a, REG, Cdbpair>;
impl<'a, REG> CdbpairW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdbpair::Pair0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdbpair::Pair1)
    }
}
#[doc = "Field `CDOFF` reader - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
pub type CdoffR = crate::FieldReader<u16>;
#[doc = "Field `CDOFF` writer - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
pub type CdoffW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn abbpair(&self) -> AbbpairR {
        AbbpairR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline(always)]
    pub fn aboff(&self) -> AboffR {
        AboffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn cdbpair(&self) -> CdbpairR {
        CdbpairR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline(always)]
    pub fn cdoff(&self) -> CdoffR {
        CdoffR::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL0")
            .field("abbpair", &self.abbpair())
            .field("aboff", &self.aboff())
            .field("cdbpair", &self.cdbpair())
            .field("cdoff", &self.cdoff())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    #[must_use]
    pub fn abbpair(&mut self) -> AbbpairW<Ctrl0Spec> {
        AbbpairW::new(self, 0)
    }
    #[doc = "Bit 2 - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline(always)]
    #[must_use]
    pub fn aboff(&mut self) -> AboffW<Ctrl0Spec> {
        AboffW::new(self, 2)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    #[must_use]
    pub fn cdbpair(&mut self) -> CdbpairW<Ctrl0Spec> {
        CdbpairW::new(self, 16)
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline(always)]
    #[must_use]
    pub fn cdoff(&mut self) -> CdoffW<Ctrl0Spec> {
        CdoffW::new(self, 18)
    }
}
#[doc = "Contains the offsets of AB and CD in the RAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for Ctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
