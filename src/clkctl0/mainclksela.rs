#[doc = "Register `MAINCLKSELA` reader"]
pub type R = crate::R<MainclkselaSpec>;
#[doc = "Register `MAINCLKSELA` writer"]
pub type W = crate::W<MainclkselaSpec>;
#[doc = "Control Main 1st Stage Control Clock Source. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: FFRO Clock Divided by 4."]
    FfroDiv4 = 0,
    #[doc = "1: SYSXTALIN Clock."]
    SysxtalClk = 1,
    #[doc = "2: Low Power Oscillator Clock (LPOSC)."]
    Lposc = 2,
    #[doc = "3: FFRO Clock."]
    FfroClk = 3,
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
#[doc = "Field `SEL` reader - Control Main 1st Stage Control Clock Source. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::FfroDiv4,
            1 => Sel::SysxtalClk,
            2 => Sel::Lposc,
            3 => Sel::FfroClk,
            _ => unreachable!(),
        }
    }
    #[doc = "FFRO Clock Divided by 4."]
    #[inline(always)]
    pub fn is_ffro_div_4(&self) -> bool {
        *self == Sel::FfroDiv4
    }
    #[doc = "SYSXTALIN Clock."]
    #[inline(always)]
    pub fn is_sysxtal_clk(&self) -> bool {
        *self == Sel::SysxtalClk
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == Sel::Lposc
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn is_ffro_clk(&self) -> bool {
        *self == Sel::FfroClk
    }
}
#[doc = "Field `SEL` writer - Control Main 1st Stage Control Clock Source. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sel, crate::Safe>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FFRO Clock Divided by 4."]
    #[inline(always)]
    pub fn ffro_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FfroDiv4)
    }
    #[doc = "SYSXTALIN Clock."]
    #[inline(always)]
    pub fn sysxtal_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SysxtalClk)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Lposc)
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn ffro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FfroClk)
    }
}
impl R {
    #[doc = "Bits 0:1 - Control Main 1st Stage Control Clock Source. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAINCLKSELA")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Control Main 1st Stage Control Clock Source. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<MainclkselaSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "main clock selection A\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclksela::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclksela::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MainclkselaSpec;
impl crate::RegisterSpec for MainclkselaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mainclksela::R`](R) reader structure"]
impl crate::Readable for MainclkselaSpec {}
#[doc = "`write(|w| ..)` method takes [`mainclksela::W`](W) writer structure"]
impl crate::Writable for MainclkselaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAINCLKSELA to value 0"]
impl crate::Resettable for MainclkselaSpec {
    const RESET_VALUE: u32 = 0;
}
