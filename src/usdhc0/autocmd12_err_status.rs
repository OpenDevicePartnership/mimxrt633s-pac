#[doc = "Register `AUTOCMD12_ERR_STATUS` reader"]
pub type R = crate::R<Autocmd12ErrStatusSpec>;
#[doc = "Register `AUTOCMD12_ERR_STATUS` writer"]
pub type W = crate::W<Autocmd12ErrStatusSpec>;
#[doc = "Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ac12ne {
    #[doc = "0: Executed"]
    Ac12ne0 = 0,
    #[doc = "1: Not executed"]
    Ac12ne1 = 1,
}
impl From<Ac12ne> for bool {
    #[inline(always)]
    fn from(variant: Ac12ne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AC12NE` reader - Auto CMD12 Not Executed"]
pub type Ac12neR = crate::BitReader<Ac12ne>;
impl Ac12neR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ac12ne {
        match self.bits {
            false => Ac12ne::Ac12ne0,
            true => Ac12ne::Ac12ne1,
        }
    }
    #[doc = "Executed"]
    #[inline(always)]
    pub fn is_ac12ne_0(&self) -> bool {
        *self == Ac12ne::Ac12ne0
    }
    #[doc = "Not executed"]
    #[inline(always)]
    pub fn is_ac12ne_1(&self) -> bool {
        *self == Ac12ne::Ac12ne1
    }
}
#[doc = "Auto CMD12 / 23 Timeout Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ac12toe {
    #[doc = "0: No error"]
    Ac12toe0 = 0,
    #[doc = "1: Time out"]
    Ac12toe1 = 1,
}
impl From<Ac12toe> for bool {
    #[inline(always)]
    fn from(variant: Ac12toe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AC12TOE` reader - Auto CMD12 / 23 Timeout Error"]
pub type Ac12toeR = crate::BitReader<Ac12toe>;
impl Ac12toeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ac12toe {
        match self.bits {
            false => Ac12toe::Ac12toe0,
            true => Ac12toe::Ac12toe1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_ac12toe_0(&self) -> bool {
        *self == Ac12toe::Ac12toe0
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub fn is_ac12toe_1(&self) -> bool {
        *self == Ac12toe::Ac12toe1
    }
}
#[doc = "Auto CMD12 / 23 End Bit Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ac12ebe {
    #[doc = "0: No error"]
    Ac12ebe0 = 0,
    #[doc = "1: End Bit Error Generated"]
    Ac12ebe1 = 1,
}
impl From<Ac12ebe> for bool {
    #[inline(always)]
    fn from(variant: Ac12ebe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AC12EBE` reader - Auto CMD12 / 23 End Bit Error"]
pub type Ac12ebeR = crate::BitReader<Ac12ebe>;
impl Ac12ebeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ac12ebe {
        match self.bits {
            false => Ac12ebe::Ac12ebe0,
            true => Ac12ebe::Ac12ebe1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_ac12ebe_0(&self) -> bool {
        *self == Ac12ebe::Ac12ebe0
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn is_ac12ebe_1(&self) -> bool {
        *self == Ac12ebe::Ac12ebe1
    }
}
#[doc = "Auto CMD12 / 23 CRC Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ac12ce {
    #[doc = "0: No CRC error"]
    Ac12ce0 = 0,
    #[doc = "1: CRC Error Met in Auto CMD12/23 Response"]
    Ac12ce1 = 1,
}
impl From<Ac12ce> for bool {
    #[inline(always)]
    fn from(variant: Ac12ce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AC12CE` reader - Auto CMD12 / 23 CRC Error"]
pub type Ac12ceR = crate::BitReader<Ac12ce>;
impl Ac12ceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ac12ce {
        match self.bits {
            false => Ac12ce::Ac12ce0,
            true => Ac12ce::Ac12ce1,
        }
    }
    #[doc = "No CRC error"]
    #[inline(always)]
    pub fn is_ac12ce_0(&self) -> bool {
        *self == Ac12ce::Ac12ce0
    }
    #[doc = "CRC Error Met in Auto CMD12/23 Response"]
    #[inline(always)]
    pub fn is_ac12ce_1(&self) -> bool {
        *self == Ac12ce::Ac12ce1
    }
}
#[doc = "Auto CMD12 / 23 Index Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ac12ie {
    #[doc = "0: No error"]
    Ac12ie0 = 0,
    #[doc = "1: Error, the CMD index in response is not CMD12/23"]
    Ac12ie1 = 1,
}
impl From<Ac12ie> for bool {
    #[inline(always)]
    fn from(variant: Ac12ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AC12IE` reader - Auto CMD12 / 23 Index Error"]
pub type Ac12ieR = crate::BitReader<Ac12ie>;
impl Ac12ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ac12ie {
        match self.bits {
            false => Ac12ie::Ac12ie0,
            true => Ac12ie::Ac12ie1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_ac12ie_0(&self) -> bool {
        *self == Ac12ie::Ac12ie0
    }
    #[doc = "Error, the CMD index in response is not CMD12/23"]
    #[inline(always)]
    pub fn is_ac12ie_1(&self) -> bool {
        *self == Ac12ie::Ac12ie1
    }
}
#[doc = "Command Not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnibac12e {
    #[doc = "0: No error"]
    Cnibac12e0 = 0,
    #[doc = "1: Not Issued"]
    Cnibac12e1 = 1,
}
impl From<Cnibac12e> for bool {
    #[inline(always)]
    fn from(variant: Cnibac12e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNIBAC12E` reader - Command Not Issued By Auto CMD12 Error"]
pub type Cnibac12eR = crate::BitReader<Cnibac12e>;
impl Cnibac12eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cnibac12e {
        match self.bits {
            false => Cnibac12e::Cnibac12e0,
            true => Cnibac12e::Cnibac12e1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_cnibac12e_0(&self) -> bool {
        *self == Cnibac12e::Cnibac12e0
    }
    #[doc = "Not Issued"]
    #[inline(always)]
    pub fn is_cnibac12e_1(&self) -> bool {
        *self == Cnibac12e::Cnibac12e1
    }
}
#[doc = "Field `EXECUTE_TUNING` reader - Execute Tuning"]
pub type ExecuteTuningR = crate::BitReader;
#[doc = "Field `EXECUTE_TUNING` writer - Execute Tuning"]
pub type ExecuteTuningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sample Clock Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmpClkSel {
    #[doc = "0: Fixed clock is used to sample data"]
    SmpClkSel0 = 0,
    #[doc = "1: Tuned clock is used to sample data"]
    SmpClkSel1 = 1,
}
impl From<SmpClkSel> for bool {
    #[inline(always)]
    fn from(variant: SmpClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMP_CLK_SEL` reader - Sample Clock Select"]
pub type SmpClkSelR = crate::BitReader<SmpClkSel>;
impl SmpClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmpClkSel {
        match self.bits {
            false => SmpClkSel::SmpClkSel0,
            true => SmpClkSel::SmpClkSel1,
        }
    }
    #[doc = "Fixed clock is used to sample data"]
    #[inline(always)]
    pub fn is_smp_clk_sel_0(&self) -> bool {
        *self == SmpClkSel::SmpClkSel0
    }
    #[doc = "Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn is_smp_clk_sel_1(&self) -> bool {
        *self == SmpClkSel::SmpClkSel1
    }
}
#[doc = "Field `SMP_CLK_SEL` writer - Sample Clock Select"]
pub type SmpClkSelW<'a, REG> = crate::BitWriter<'a, REG, SmpClkSel>;
impl<'a, REG> SmpClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed clock is used to sample data"]
    #[inline(always)]
    pub fn smp_clk_sel_0(self) -> &'a mut crate::W<REG> {
        self.variant(SmpClkSel::SmpClkSel0)
    }
    #[doc = "Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn smp_clk_sel_1(self) -> &'a mut crate::W<REG> {
        self.variant(SmpClkSel::SmpClkSel1)
    }
}
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn ac12ne(&self) -> Ac12neR {
        Ac12neR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD12 / 23 Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&self) -> Ac12toeR {
        Ac12toeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD12 / 23 End Bit Error"]
    #[inline(always)]
    pub fn ac12ebe(&self) -> Ac12ebeR {
        Ac12ebeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD12 / 23 CRC Error"]
    #[inline(always)]
    pub fn ac12ce(&self) -> Ac12ceR {
        Ac12ceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD12 / 23 Index Error"]
    #[inline(always)]
    pub fn ac12ie(&self) -> Ac12ieR {
        Ac12ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12e(&self) -> Cnibac12eR {
        Cnibac12eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline(always)]
    pub fn execute_tuning(&self) -> ExecuteTuningR {
        ExecuteTuningR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sample Clock Select"]
    #[inline(always)]
    pub fn smp_clk_sel(&self) -> SmpClkSelR {
        SmpClkSelR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCMD12_ERR_STATUS")
            .field("ac12ne", &self.ac12ne())
            .field("ac12toe", &self.ac12toe())
            .field("ac12ebe", &self.ac12ebe())
            .field("ac12ce", &self.ac12ce())
            .field("ac12ie", &self.ac12ie())
            .field("cnibac12e", &self.cnibac12e())
            .field("execute_tuning", &self.execute_tuning())
            .field("smp_clk_sel", &self.smp_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline(always)]
    pub fn execute_tuning(&mut self) -> ExecuteTuningW<Autocmd12ErrStatusSpec> {
        ExecuteTuningW::new(self, 22)
    }
    #[doc = "Bit 23 - Sample Clock Select"]
    #[inline(always)]
    pub fn smp_clk_sel(&mut self) -> SmpClkSelW<Autocmd12ErrStatusSpec> {
        SmpClkSelW::new(self, 23)
    }
}
#[doc = "Auto CMD12 Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`autocmd12_err_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocmd12_err_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Autocmd12ErrStatusSpec;
impl crate::RegisterSpec for Autocmd12ErrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocmd12_err_status::R`](R) reader structure"]
impl crate::Readable for Autocmd12ErrStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`autocmd12_err_status::W`](W) writer structure"]
impl crate::Writable for Autocmd12ErrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOCMD12_ERR_STATUS to value 0"]
impl crate::Resettable for Autocmd12ErrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
