#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Enable the TIMERn interrupt.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten {
    #[doc = "0: Disabled. TIMERn interrupt is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. TIMERn interrupt is enabled."]
    Enabled = 1,
}
impl From<Inten> for bool {
    #[inline(always)]
    fn from(variant: Inten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - Enable the TIMERn interrupt."]
pub type IntenR = crate::BitReader<Inten>;
impl IntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inten {
        match self.bits {
            false => Inten::Disabled,
            true => Inten::Enabled,
        }
    }
    #[doc = "Disabled. TIMERn interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inten::Disabled
    }
    #[doc = "Enabled. TIMERn interrupt is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inten::Enabled
    }
}
#[doc = "Field `INTEN` writer - Enable the TIMERn interrupt."]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG, Inten>;
impl<'a, REG> IntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. TIMERn interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Disabled)
    }
    #[doc = "Enabled. TIMERn interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Enabled)
    }
}
#[doc = "Selects timer mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Repeat interrupt mode."]
    RepeatInterruptMode = 0,
    #[doc = "1: One-shot interrupt mode."]
    OneShotInterruptMode = 1,
    #[doc = "2: One-shot stall mode."]
    OneShotStallMode = 2,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Selects timer mode."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::RepeatInterruptMode),
            1 => Some(Mode::OneShotInterruptMode),
            2 => Some(Mode::OneShotStallMode),
            _ => None,
        }
    }
    #[doc = "Repeat interrupt mode."]
    #[inline(always)]
    pub fn is_repeat_interrupt_mode(&self) -> bool {
        *self == Mode::RepeatInterruptMode
    }
    #[doc = "One-shot interrupt mode."]
    #[inline(always)]
    pub fn is_one_shot_interrupt_mode(&self) -> bool {
        *self == Mode::OneShotInterruptMode
    }
    #[doc = "One-shot stall mode."]
    #[inline(always)]
    pub fn is_one_shot_stall_mode(&self) -> bool {
        *self == Mode::OneShotStallMode
    }
}
#[doc = "Field `MODE` writer - Selects timer mode."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Repeat interrupt mode."]
    #[inline(always)]
    pub fn repeat_interrupt_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::RepeatInterruptMode)
    }
    #[doc = "One-shot interrupt mode."]
    #[inline(always)]
    pub fn one_shot_interrupt_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::OneShotInterruptMode)
    }
    #[doc = "One-shot stall mode."]
    #[inline(always)]
    pub fn one_shot_stall_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::OneShotStallMode)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the TIMERn interrupt."]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Selects timer mode."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("inten", &self.inten())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable the TIMERn interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> IntenW<CtrlSpec> {
        IntenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Selects timer mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtrlSpec> {
        ModeW::new(self, 1)
    }
}
#[doc = "MRT Control register. This register controls the MRT modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
