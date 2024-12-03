#[doc = "Register `PAUSE` reader"]
pub type R = crate::R<PauseSpec>;
#[doc = "Register `PAUSE` writer"]
pub type W = crate::W<PauseSpec>;
#[doc = "Field `PAUSEDLY` reader - Pause Delay"]
pub type PausedlyR = crate::FieldReader<u16>;
#[doc = "Field `PAUSEDLY` writer - Pause Delay"]
pub type PausedlyW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "PAUSE Option Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pauseen {
    #[doc = "0: Pause operation disabled"]
    Pauseen0 = 0,
    #[doc = "1: Pause operation enabled"]
    Pauseen1 = 1,
}
impl From<Pauseen> for bool {
    #[inline(always)]
    fn from(variant: Pauseen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSEEN` reader - PAUSE Option Enable"]
pub type PauseenR = crate::BitReader<Pauseen>;
impl PauseenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pauseen {
        match self.bits {
            false => Pauseen::Pauseen0,
            true => Pauseen::Pauseen1,
        }
    }
    #[doc = "Pause operation disabled"]
    #[inline(always)]
    pub fn is_pauseen_0(&self) -> bool {
        *self == Pauseen::Pauseen0
    }
    #[doc = "Pause operation enabled"]
    #[inline(always)]
    pub fn is_pauseen_1(&self) -> bool {
        *self == Pauseen::Pauseen1
    }
}
#[doc = "Field `PAUSEEN` writer - PAUSE Option Enable"]
pub type PauseenW<'a, REG> = crate::BitWriter<'a, REG, Pauseen>;
impl<'a, REG> PauseenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pause operation disabled"]
    #[inline(always)]
    pub fn pauseen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pauseen::Pauseen0)
    }
    #[doc = "Pause operation enabled"]
    #[inline(always)]
    pub fn pauseen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pauseen::Pauseen1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Pause Delay"]
    #[inline(always)]
    pub fn pausedly(&self) -> PausedlyR {
        PausedlyR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - PAUSE Option Enable"]
    #[inline(always)]
    pub fn pauseen(&self) -> PauseenR {
        PauseenR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAUSE")
            .field("pausedly", &self.pausedly())
            .field("pauseen", &self.pauseen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Pause Delay"]
    #[inline(always)]
    pub fn pausedly(&mut self) -> PausedlyW<PauseSpec> {
        PausedlyW::new(self, 0)
    }
    #[doc = "Bit 31 - PAUSE Option Enable"]
    #[inline(always)]
    pub fn pauseen(&mut self) -> PauseenW<PauseSpec> {
        PauseenW::new(self, 31)
    }
}
#[doc = "ADC Pause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PauseSpec;
impl crate::RegisterSpec for PauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pause::R`](R) reader structure"]
impl crate::Readable for PauseSpec {}
#[doc = "`write(|w| ..)` method takes [`pause::W`](W) writer structure"]
impl crate::Writable for PauseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAUSE to value 0"]
impl crate::Resettable for PauseSpec {
    const RESET_VALUE: u32 = 0;
}
