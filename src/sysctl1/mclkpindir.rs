#[doc = "Register `MCLKPINDIR` reader"]
pub type R = crate::R<MclkpindirSpec>;
#[doc = "Register `MCLKPINDIR` writer"]
pub type W = crate::W<MclkpindirSpec>;
#[doc = "Selects one of the M33 interrupt sources\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclkpindir {
    #[doc = "0: MCLK is in input direction."]
    InputDirection = 0,
    #[doc = "1: MCLK is in the output direction."]
    OutputDirection = 1,
}
impl From<Mclkpindir> for bool {
    #[inline(always)]
    fn from(variant: Mclkpindir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKPINDIR` reader - Selects one of the M33 interrupt sources"]
pub type MclkpindirR = crate::BitReader<Mclkpindir>;
impl MclkpindirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclkpindir {
        match self.bits {
            false => Mclkpindir::InputDirection,
            true => Mclkpindir::OutputDirection,
        }
    }
    #[doc = "MCLK is in input direction."]
    #[inline(always)]
    pub fn is_input_direction(&self) -> bool {
        *self == Mclkpindir::InputDirection
    }
    #[doc = "MCLK is in the output direction."]
    #[inline(always)]
    pub fn is_output_direction(&self) -> bool {
        *self == Mclkpindir::OutputDirection
    }
}
#[doc = "Field `MCLKPINDIR` writer - Selects one of the M33 interrupt sources"]
pub type MclkpindirW<'a, REG> = crate::BitWriter<'a, REG, Mclkpindir>;
impl<'a, REG> MclkpindirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCLK is in input direction."]
    #[inline(always)]
    pub fn input_direction(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkpindir::InputDirection)
    }
    #[doc = "MCLK is in the output direction."]
    #[inline(always)]
    pub fn output_direction(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkpindir::OutputDirection)
    }
}
impl R {
    #[doc = "Bit 0 - Selects one of the M33 interrupt sources"]
    #[inline(always)]
    pub fn mclkpindir(&self) -> MclkpindirR {
        MclkpindirR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCLKPINDIR")
            .field("mclkpindir", &self.mclkpindir())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Selects one of the M33 interrupt sources"]
    #[inline(always)]
    pub fn mclkpindir(&mut self) -> MclkpindirW<MclkpindirSpec> {
        MclkpindirW::new(self, 0)
    }
}
#[doc = "mclk direction control\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkpindir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkpindir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MclkpindirSpec;
impl crate::RegisterSpec for MclkpindirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mclkpindir::R`](R) reader structure"]
impl crate::Readable for MclkpindirSpec {}
#[doc = "`write(|w| ..)` method takes [`mclkpindir::W`](W) writer structure"]
impl crate::Writable for MclkpindirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCLKPINDIR to value 0"]
impl crate::Resettable for MclkpindirSpec {
    const RESET_VALUE: u32 = 0;
}
