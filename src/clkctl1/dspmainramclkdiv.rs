#[doc = "Register `DSPMAINRAMCLKDIV` reader"]
pub type R = crate::R<DspmainramclkdivSpec>;
#[doc = "Register `DSPMAINRAMCLKDIV` writer"]
pub type W = crate::W<DspmainramclkdivSpec>;
#[doc = "DSP MAINRAM Clock Ratio Control:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dspmramclkdiv {
    #[doc = "0: DSP MAINRAM Clk = DSP Core CLK / 1."]
    DspClkDivBy1 = 0,
    #[doc = "1: DSP MAINRAM Clk = DSP Core CLK / 2."]
    DspClkDivBy2 = 1,
    #[doc = "2: DSP MAINRAM Clk = DSP Core CLK / 3."]
    DspClkDivBy3 = 2,
    #[doc = "3: DSP MAINRAM Clk = DSP Core CLK / 4."]
    DspClkDivBy4 = 3,
}
impl From<Dspmramclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Dspmramclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dspmramclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Dspmramclkdiv {}
#[doc = "Field `DSPMRAMCLKDIV` reader - DSP MAINRAM Clock Ratio Control:"]
pub type DspmramclkdivR = crate::FieldReader<Dspmramclkdiv>;
impl DspmramclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dspmramclkdiv {
        match self.bits {
            0 => Dspmramclkdiv::DspClkDivBy1,
            1 => Dspmramclkdiv::DspClkDivBy2,
            2 => Dspmramclkdiv::DspClkDivBy3,
            3 => Dspmramclkdiv::DspClkDivBy4,
            _ => unreachable!(),
        }
    }
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 1."]
    #[inline(always)]
    pub fn is_dsp_clk_div_by_1(&self) -> bool {
        *self == Dspmramclkdiv::DspClkDivBy1
    }
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 2."]
    #[inline(always)]
    pub fn is_dsp_clk_div_by_2(&self) -> bool {
        *self == Dspmramclkdiv::DspClkDivBy2
    }
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 3."]
    #[inline(always)]
    pub fn is_dsp_clk_div_by_3(&self) -> bool {
        *self == Dspmramclkdiv::DspClkDivBy3
    }
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 4."]
    #[inline(always)]
    pub fn is_dsp_clk_div_by_4(&self) -> bool {
        *self == Dspmramclkdiv::DspClkDivBy4
    }
}
#[doc = "Field `DSPMRAMCLKDIV` writer - DSP MAINRAM Clock Ratio Control:"]
pub type DspmramclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dspmramclkdiv, crate::Safe>;
impl<'a, REG> DspmramclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 1."]
    #[inline(always)]
    pub fn dsp_clk_div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dspmramclkdiv::DspClkDivBy1)
    }
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 2."]
    #[inline(always)]
    pub fn dsp_clk_div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dspmramclkdiv::DspClkDivBy2)
    }
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 3."]
    #[inline(always)]
    pub fn dsp_clk_div_by_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dspmramclkdiv::DspClkDivBy3)
    }
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 4."]
    #[inline(always)]
    pub fn dsp_clk_div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dspmramclkdiv::DspClkDivBy4)
    }
}
impl R {
    #[doc = "Bits 0:1 - DSP MAINRAM Clock Ratio Control:"]
    #[inline(always)]
    pub fn dspmramclkdiv(&self) -> DspmramclkdivR {
        DspmramclkdivR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSPMAINRAMCLKDIV")
            .field("dspmramclkdiv", &self.dspmramclkdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - DSP MAINRAM Clock Ratio Control:"]
    #[inline(always)]
    pub fn dspmramclkdiv(&mut self) -> DspmramclkdivW<DspmainramclkdivSpec> {
        DspmramclkdivW::new(self, 0)
    }
}
#[doc = "DSP main ram clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`dspmainramclkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspmainramclkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspmainramclkdivSpec;
impl crate::RegisterSpec for DspmainramclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dspmainramclkdiv::R`](R) reader structure"]
impl crate::Readable for DspmainramclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`dspmainramclkdiv::W`](W) writer structure"]
impl crate::Writable for DspmainramclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSPMAINRAMCLKDIV to value 0x01"]
impl crate::Resettable for DspmainramclkdivSpec {
    const RESET_VALUE: u32 = 0x01;
}
