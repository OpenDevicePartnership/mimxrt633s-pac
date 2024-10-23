#[doc = "Register `SYSOSCBYPASS` reader"]
pub type R = crate::R<SysoscbypassSpec>;
#[doc = "Register `SYSOSCBYPASS` writer"]
pub type W = crate::W<SysoscbypassSpec>;
#[doc = "Extenal Clock Source Selection.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: External XTAL Clock."]
    ExtXtalClk = 0,
    #[doc = "1: Clock IN Clock."]
    ClockInClk = 1,
    #[doc = "7: NONE.this may be selected in order to reduce power when no output is needed."]
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
#[doc = "Field `SEL` reader - Extenal Clock Source Selection."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::ExtXtalClk),
            1 => Some(Sel::ClockInClk),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "External XTAL Clock."]
    #[inline(always)]
    pub fn is_ext_xtal_clk(&self) -> bool {
        *self == Sel::ExtXtalClk
    }
    #[doc = "Clock IN Clock."]
    #[inline(always)]
    pub fn is_clock_in_clk(&self) -> bool {
        *self == Sel::ClockInClk
    }
    #[doc = "NONE.this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - Extenal Clock Source Selection."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External XTAL Clock."]
    #[inline(always)]
    pub fn ext_xtal_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::ExtXtalClk)
    }
    #[doc = "Clock IN Clock."]
    #[inline(always)]
    pub fn clock_in_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::ClockInClk)
    }
    #[doc = "NONE.this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - Extenal Clock Source Selection."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSOSCBYPASS")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Extenal Clock Source Selection."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<SysoscbypassSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "system oscillator bypass\n\nYou can [`read`](crate::Reg::read) this register and get [`sysoscbypass::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysoscbypass::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysoscbypassSpec;
impl crate::RegisterSpec for SysoscbypassSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysoscbypass::R`](R) reader structure"]
impl crate::Readable for SysoscbypassSpec {}
#[doc = "`write(|w| ..)` method takes [`sysoscbypass::W`](W) writer structure"]
impl crate::Writable for SysoscbypassSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSOSCBYPASS to value 0"]
impl crate::Resettable for SysoscbypassSpec {
    const RESET_VALUE: u32 = 0;
}
