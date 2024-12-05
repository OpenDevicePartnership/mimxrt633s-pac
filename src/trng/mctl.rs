#[doc = "Register `MCTL` reader"]
pub type R = crate::R<MctlSpec>;
#[doc = "Register `MCTL` writer"]
pub type W = crate::W<MctlSpec>;
#[doc = "Sample Mode\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SampMode {
    #[doc = "0: use Von Neumann data into both Entropy shifter and Statistical Checker"]
    SampMode0 = 0,
    #[doc = "1: use raw data into both Entropy shifter and Statistical Checker"]
    SampMode1 = 1,
    #[doc = "2: use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    SampMode2 = 2,
    #[doc = "3: undefined/reserved."]
    SampMode3 = 3,
}
impl From<SampMode> for u8 {
    #[inline(always)]
    fn from(variant: SampMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SampMode {
    type Ux = u8;
}
impl crate::IsEnum for SampMode {}
#[doc = "Field `SAMP_MODE` reader - Sample Mode"]
pub type SampModeR = crate::FieldReader<SampMode>;
impl SampModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SampMode {
        match self.bits {
            0 => SampMode::SampMode0,
            1 => SampMode::SampMode1,
            2 => SampMode::SampMode2,
            3 => SampMode::SampMode3,
            _ => unreachable!(),
        }
    }
    #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
    #[inline(always)]
    pub fn is_samp_mode_0(&self) -> bool {
        *self == SampMode::SampMode0
    }
    #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
    #[inline(always)]
    pub fn is_samp_mode_1(&self) -> bool {
        *self == SampMode::SampMode1
    }
    #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    #[inline(always)]
    pub fn is_samp_mode_2(&self) -> bool {
        *self == SampMode::SampMode2
    }
    #[doc = "undefined/reserved."]
    #[inline(always)]
    pub fn is_samp_mode_3(&self) -> bool {
        *self == SampMode::SampMode3
    }
}
#[doc = "Field `SAMP_MODE` writer - Sample Mode"]
pub type SampModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, SampMode, crate::Safe>;
impl<'a, REG> SampModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
    #[inline(always)]
    pub fn samp_mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(SampMode::SampMode0)
    }
    #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
    #[inline(always)]
    pub fn samp_mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(SampMode::SampMode1)
    }
    #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    #[inline(always)]
    pub fn samp_mode_2(self) -> &'a mut crate::W<REG> {
        self.variant(SampMode::SampMode2)
    }
    #[doc = "undefined/reserved."]
    #[inline(always)]
    pub fn samp_mode_3(self) -> &'a mut crate::W<REG> {
        self.variant(SampMode::SampMode3)
    }
}
#[doc = "Oscillator Divide\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OscDiv {
    #[doc = "0: use ring oscillator with no divide"]
    OscDiv0 = 0,
    #[doc = "1: use ring oscillator divided-by-2"]
    OscDiv1 = 1,
    #[doc = "2: use ring oscillator divided-by-4"]
    OscDiv2 = 2,
    #[doc = "3: use ring oscillator divided-by-8"]
    OscDiv3 = 3,
}
impl From<OscDiv> for u8 {
    #[inline(always)]
    fn from(variant: OscDiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OscDiv {
    type Ux = u8;
}
impl crate::IsEnum for OscDiv {}
#[doc = "Field `OSC_DIV` reader - Oscillator Divide"]
pub type OscDivR = crate::FieldReader<OscDiv>;
impl OscDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OscDiv {
        match self.bits {
            0 => OscDiv::OscDiv0,
            1 => OscDiv::OscDiv1,
            2 => OscDiv::OscDiv2,
            3 => OscDiv::OscDiv3,
            _ => unreachable!(),
        }
    }
    #[doc = "use ring oscillator with no divide"]
    #[inline(always)]
    pub fn is_osc_div_0(&self) -> bool {
        *self == OscDiv::OscDiv0
    }
    #[doc = "use ring oscillator divided-by-2"]
    #[inline(always)]
    pub fn is_osc_div_1(&self) -> bool {
        *self == OscDiv::OscDiv1
    }
    #[doc = "use ring oscillator divided-by-4"]
    #[inline(always)]
    pub fn is_osc_div_2(&self) -> bool {
        *self == OscDiv::OscDiv2
    }
    #[doc = "use ring oscillator divided-by-8"]
    #[inline(always)]
    pub fn is_osc_div_3(&self) -> bool {
        *self == OscDiv::OscDiv3
    }
}
#[doc = "Field `OSC_DIV` writer - Oscillator Divide"]
pub type OscDivW<'a, REG> = crate::FieldWriter<'a, REG, 2, OscDiv, crate::Safe>;
impl<'a, REG> OscDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "use ring oscillator with no divide"]
    #[inline(always)]
    pub fn osc_div_0(self) -> &'a mut crate::W<REG> {
        self.variant(OscDiv::OscDiv0)
    }
    #[doc = "use ring oscillator divided-by-2"]
    #[inline(always)]
    pub fn osc_div_1(self) -> &'a mut crate::W<REG> {
        self.variant(OscDiv::OscDiv1)
    }
    #[doc = "use ring oscillator divided-by-4"]
    #[inline(always)]
    pub fn osc_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(OscDiv::OscDiv2)
    }
    #[doc = "use ring oscillator divided-by-8"]
    #[inline(always)]
    pub fn osc_div_3(self) -> &'a mut crate::W<REG> {
        self.variant(OscDiv::OscDiv3)
    }
}
#[doc = "Field `TRNG_ACC` reader - TRNG Access Mode"]
pub type TrngAccR = crate::BitReader;
#[doc = "Field `TRNG_ACC` writer - TRNG Access Mode"]
pub type TrngAccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_DEF` writer - Reset Defaults"]
pub type RstDefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOR_SCLK` reader - Force System Clock"]
pub type ForSclkR = crate::BitReader;
#[doc = "Field `FOR_SCLK` writer - Force System Clock"]
pub type ForSclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCT_FAIL` reader - Read only: Frequency Count Fail"]
pub type FctFailR = crate::BitReader;
#[doc = "Field `FCT_VAL` reader - Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
pub type FctValR = crate::BitReader;
#[doc = "Field `ENT_VAL` reader - Read only: Entropy Valid"]
pub type EntValR = crate::BitReader;
#[doc = "Field `TST_OUT` reader - Read only: Test point inside ring oscillator."]
pub type TstOutR = crate::BitReader;
#[doc = "Field `ERR` reader - Read: Error status"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - Read: Error status"]
pub type ErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TSTOP_OK` reader - TRNG_OK_TO_STOP"]
pub type TstopOkR = crate::BitReader;
#[doc = "Field `PRGM` reader - Programming Mode Select"]
pub type PrgmR = crate::BitReader;
#[doc = "Field `PRGM` writer - Programming Mode Select"]
pub type PrgmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Sample Mode"]
    #[inline(always)]
    pub fn samp_mode(&self) -> SampModeR {
        SampModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Oscillator Divide"]
    #[inline(always)]
    pub fn osc_div(&self) -> OscDivR {
        OscDivR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - TRNG Access Mode"]
    #[inline(always)]
    pub fn trng_acc(&self) -> TrngAccR {
        TrngAccR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Force System Clock"]
    #[inline(always)]
    pub fn for_sclk(&self) -> ForSclkR {
        ForSclkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read only: Frequency Count Fail"]
    #[inline(always)]
    pub fn fct_fail(&self) -> FctFailR {
        FctFailR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
    #[inline(always)]
    pub fn fct_val(&self) -> FctValR {
        FctValR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read only: Entropy Valid"]
    #[inline(always)]
    pub fn ent_val(&self) -> EntValR {
        EntValR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Read only: Test point inside ring oscillator."]
    #[inline(always)]
    pub fn tst_out(&self) -> TstOutR {
        TstOutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Read: Error status"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TRNG_OK_TO_STOP"]
    #[inline(always)]
    pub fn tstop_ok(&self) -> TstopOkR {
        TstopOkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Programming Mode Select"]
    #[inline(always)]
    pub fn prgm(&self) -> PrgmR {
        PrgmR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCTL")
            .field("samp_mode", &self.samp_mode())
            .field("osc_div", &self.osc_div())
            .field("trng_acc", &self.trng_acc())
            .field("for_sclk", &self.for_sclk())
            .field("fct_fail", &self.fct_fail())
            .field("fct_val", &self.fct_val())
            .field("ent_val", &self.ent_val())
            .field("tst_out", &self.tst_out())
            .field("err", &self.err())
            .field("tstop_ok", &self.tstop_ok())
            .field("prgm", &self.prgm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Sample Mode"]
    #[inline(always)]
    pub fn samp_mode(&mut self) -> SampModeW<MctlSpec> {
        SampModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Oscillator Divide"]
    #[inline(always)]
    pub fn osc_div(&mut self) -> OscDivW<MctlSpec> {
        OscDivW::new(self, 2)
    }
    #[doc = "Bit 5 - TRNG Access Mode"]
    #[inline(always)]
    pub fn trng_acc(&mut self) -> TrngAccW<MctlSpec> {
        TrngAccW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset Defaults"]
    #[inline(always)]
    pub fn rst_def(&mut self) -> RstDefW<MctlSpec> {
        RstDefW::new(self, 6)
    }
    #[doc = "Bit 7 - Force System Clock"]
    #[inline(always)]
    pub fn for_sclk(&mut self) -> ForSclkW<MctlSpec> {
        ForSclkW::new(self, 7)
    }
    #[doc = "Bit 12 - Read: Error status"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<MctlSpec> {
        ErrW::new(self, 12)
    }
    #[doc = "Bit 16 - Programming Mode Select"]
    #[inline(always)]
    pub fn prgm(&mut self) -> PrgmW<MctlSpec> {
        PrgmW::new(self, 16)
    }
}
#[doc = "Miscellaneous Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctlSpec;
impl crate::RegisterSpec for MctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctl::R`](R) reader structure"]
impl crate::Readable for MctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mctl::W`](W) writer structure"]
impl crate::Writable for MctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1000;
}
#[doc = "`reset()` method sets MCTL to value 0x0001_2001"]
impl crate::Resettable for MctlSpec {
    const RESET_VALUE: u32 = 0x0001_2001;
}
