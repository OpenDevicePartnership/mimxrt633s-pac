#[doc = "Register `I3C0FCLKSTCSEL` reader"]
pub type R = crate::R<I3c0fclkstcselSpec>;
#[doc = "Register `I3C0FCLKSTCSEL` writer"]
pub type W = crate::W<I3c0fclkstcselSpec>;
#[doc = "I3C0 Clock Source Selection. . .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: I3C0 FCLK Selection."]
    I3c0FclkSelection = 0,
    #[doc = "1: Low Power Oscillator Clock (LPOSC)."]
    Lposc = 1,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    None = 7,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - I3C0 Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::I3c0FclkSelection),
            1 => Some(Sel::Lposc),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "I3C0 FCLK Selection."]
    #[inline(always)]
    pub fn is_i3c0_fclk_selection(&self) -> bool {
        *self == Sel::I3c0FclkSelection
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == Sel::Lposc
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - I3C0 Clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I3C0 FCLK Selection."]
    #[inline(always)]
    pub fn i3c0_fclk_selection(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::I3c0FclkSelection)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Lposc)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - I3C0 Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C0FCLKSTCSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - I3C0 Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<I3c0fclkstcselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "I3C0 fclk STC selection\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c0fclkstcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c0fclkstcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3c0fclkstcselSpec;
impl crate::RegisterSpec for I3c0fclkstcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c0fclkstcsel::R`](R) reader structure"]
impl crate::Readable for I3c0fclkstcselSpec {}
#[doc = "`write(|w| ..)` method takes [`i3c0fclkstcsel::W`](W) writer structure"]
impl crate::Writable for I3c0fclkstcselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I3C0FCLKSTCSEL to value 0x07"]
impl crate::Resettable for I3c0fclkstcselSpec {
    const RESET_VALUE: u32 = 0x07;
}
