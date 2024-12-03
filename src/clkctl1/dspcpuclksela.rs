#[doc = "Register `DSPCPUCLKSELA` reader"]
pub type R = crate::R<DspcpuclkselaSpec>;
#[doc = "Register `DSPCPUCLKSELA` writer"]
pub type W = crate::W<DspcpuclkselaSpec>;
#[doc = "Control Main 1st Stage Control Clock Source. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: FFRO Clock."]
    FfroClk = 0,
    #[doc = "1: XTALIN Clock."]
    XtalClk = 1,
    #[doc = "2: Low Power Oscillator Clock (LPOSC)."]
    Lposc = 2,
    #[doc = "3: SFRO Clock."]
    SfroClk = 3,
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
            0 => Sel::FfroClk,
            1 => Sel::XtalClk,
            2 => Sel::Lposc,
            3 => Sel::SfroClk,
            _ => unreachable!(),
        }
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn is_ffro_clk(&self) -> bool {
        *self == Sel::FfroClk
    }
    #[doc = "XTALIN Clock."]
    #[inline(always)]
    pub fn is_xtal_clk(&self) -> bool {
        *self == Sel::XtalClk
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == Sel::Lposc
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn is_sfro_clk(&self) -> bool {
        *self == Sel::SfroClk
    }
}
#[doc = "Field `SEL` writer - Control Main 1st Stage Control Clock Source. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sel, crate::Safe>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn ffro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FfroClk)
    }
    #[doc = "XTALIN Clock."]
    #[inline(always)]
    pub fn xtal_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::XtalClk)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Lposc)
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn sfro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SfroClk)
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
        f.debug_struct("DSPCPUCLKSELA")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Control Main 1st Stage Control Clock Source. . ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<DspcpuclkselaSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "DSP clock selection A\n\nYou can [`read`](crate::Reg::read) this register and get [`dspcpuclksela::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspcpuclksela::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspcpuclkselaSpec;
impl crate::RegisterSpec for DspcpuclkselaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dspcpuclksela::R`](R) reader structure"]
impl crate::Readable for DspcpuclkselaSpec {}
#[doc = "`write(|w| ..)` method takes [`dspcpuclksela::W`](W) writer structure"]
impl crate::Writable for DspcpuclkselaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSPCPUCLKSELA to value 0"]
impl crate::Resettable for DspcpuclkselaSpec {
    const RESET_VALUE: u32 = 0;
}
