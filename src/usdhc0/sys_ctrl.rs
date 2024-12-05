#[doc = "Register `SYS_CTRL` reader"]
pub type R = crate::R<SysCtrlSpec>;
#[doc = "Register `SYS_CTRL` writer"]
pub type W = crate::W<SysCtrlSpec>;
#[doc = "Divisor\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvs {
    #[doc = "0: Divide-by-1"]
    Dvs0 = 0,
    #[doc = "1: Divide-by-2"]
    Dvs1 = 1,
    #[doc = "14: Divide-by-15"]
    Dvs14 = 14,
    #[doc = "15: Divide-by-16"]
    Dvs15 = 15,
}
impl From<Dvs> for u8 {
    #[inline(always)]
    fn from(variant: Dvs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvs {
    type Ux = u8;
}
impl crate::IsEnum for Dvs {}
#[doc = "Field `DVS` reader - Divisor"]
pub type DvsR = crate::FieldReader<Dvs>;
impl DvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dvs> {
        match self.bits {
            0 => Some(Dvs::Dvs0),
            1 => Some(Dvs::Dvs1),
            14 => Some(Dvs::Dvs14),
            15 => Some(Dvs::Dvs15),
            _ => None,
        }
    }
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn is_dvs_0(&self) -> bool {
        *self == Dvs::Dvs0
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn is_dvs_1(&self) -> bool {
        *self == Dvs::Dvs1
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn is_dvs_14(&self) -> bool {
        *self == Dvs::Dvs14
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn is_dvs_15(&self) -> bool {
        *self == Dvs::Dvs15
    }
}
#[doc = "Field `DVS` writer - Divisor"]
pub type DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dvs>;
impl<'a, REG> DvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn dvs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dvs::Dvs0)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn dvs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dvs::Dvs1)
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn dvs_14(self) -> &'a mut crate::W<REG> {
        self.variant(Dvs::Dvs14)
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn dvs_15(self) -> &'a mut crate::W<REG> {
        self.variant(Dvs::Dvs15)
    }
}
#[doc = "Field `SDCLKFS` reader - SDCLK Frequency Select"]
pub type SdclkfsR = crate::FieldReader;
#[doc = "Field `SDCLKFS` writer - SDCLK Frequency Select"]
pub type SdclkfsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Data Timeout Counter Value\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtocv {
    #[doc = "0: SDCLK x 2 14"]
    Dtocv0 = 0,
    #[doc = "1: SDCLK x 2 15"]
    Dtocv1 = 1,
    #[doc = "13: SDCLK x 2 27"]
    Dtocv13 = 13,
    #[doc = "14: SDCLK x 2 28"]
    Dtocv14 = 14,
    #[doc = "15: SDCLK x 2 29"]
    Dtocv15 = 15,
}
impl From<Dtocv> for u8 {
    #[inline(always)]
    fn from(variant: Dtocv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtocv {
    type Ux = u8;
}
impl crate::IsEnum for Dtocv {}
#[doc = "Field `DTOCV` reader - Data Timeout Counter Value"]
pub type DtocvR = crate::FieldReader<Dtocv>;
impl DtocvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtocv> {
        match self.bits {
            0 => Some(Dtocv::Dtocv0),
            1 => Some(Dtocv::Dtocv1),
            13 => Some(Dtocv::Dtocv13),
            14 => Some(Dtocv::Dtocv14),
            15 => Some(Dtocv::Dtocv15),
            _ => None,
        }
    }
    #[doc = "SDCLK x 2 14"]
    #[inline(always)]
    pub fn is_dtocv_0(&self) -> bool {
        *self == Dtocv::Dtocv0
    }
    #[doc = "SDCLK x 2 15"]
    #[inline(always)]
    pub fn is_dtocv_1(&self) -> bool {
        *self == Dtocv::Dtocv1
    }
    #[doc = "SDCLK x 2 27"]
    #[inline(always)]
    pub fn is_dtocv_13(&self) -> bool {
        *self == Dtocv::Dtocv13
    }
    #[doc = "SDCLK x 2 28"]
    #[inline(always)]
    pub fn is_dtocv_14(&self) -> bool {
        *self == Dtocv::Dtocv14
    }
    #[doc = "SDCLK x 2 29"]
    #[inline(always)]
    pub fn is_dtocv_15(&self) -> bool {
        *self == Dtocv::Dtocv15
    }
}
#[doc = "Field `DTOCV` writer - Data Timeout Counter Value"]
pub type DtocvW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dtocv>;
impl<'a, REG> DtocvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDCLK x 2 14"]
    #[inline(always)]
    pub fn dtocv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtocv::Dtocv0)
    }
    #[doc = "SDCLK x 2 15"]
    #[inline(always)]
    pub fn dtocv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtocv::Dtocv1)
    }
    #[doc = "SDCLK x 2 27"]
    #[inline(always)]
    pub fn dtocv_13(self) -> &'a mut crate::W<REG> {
        self.variant(Dtocv::Dtocv13)
    }
    #[doc = "SDCLK x 2 28"]
    #[inline(always)]
    pub fn dtocv_14(self) -> &'a mut crate::W<REG> {
        self.variant(Dtocv::Dtocv14)
    }
    #[doc = "SDCLK x 2 29"]
    #[inline(always)]
    pub fn dtocv_15(self) -> &'a mut crate::W<REG> {
        self.variant(Dtocv::Dtocv15)
    }
}
#[doc = "Field `IPP_RST_N` reader - IPP_RST_N"]
pub type IppRstNR = crate::BitReader;
#[doc = "Field `IPP_RST_N` writer - IPP_RST_N"]
pub type IppRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Software Reset For ALL\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsta {
    #[doc = "0: No Reset"]
    Rsta0 = 0,
    #[doc = "1: Reset"]
    Rsta1 = 1,
}
impl From<Rsta> for bool {
    #[inline(always)]
    fn from(variant: Rsta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTA` reader - Software Reset For ALL"]
pub type RstaR = crate::BitReader<Rsta>;
impl RstaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsta {
        match self.bits {
            false => Rsta::Rsta0,
            true => Rsta::Rsta1,
        }
    }
    #[doc = "No Reset"]
    #[inline(always)]
    pub fn is_rsta_0(&self) -> bool {
        *self == Rsta::Rsta0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_rsta_1(&self) -> bool {
        *self == Rsta::Rsta1
    }
}
#[doc = "Field `RSTA` writer - Software Reset For ALL"]
pub type RstaW<'a, REG> = crate::BitWriter<'a, REG, Rsta>;
impl<'a, REG> RstaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Reset"]
    #[inline(always)]
    pub fn rsta_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsta::Rsta0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rsta_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsta::Rsta1)
    }
}
#[doc = "Software Reset For CMD Line\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstc {
    #[doc = "0: No Reset"]
    Rstc0 = 0,
    #[doc = "1: Reset"]
    Rstc1 = 1,
}
impl From<Rstc> for bool {
    #[inline(always)]
    fn from(variant: Rstc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTC` reader - Software Reset For CMD Line"]
pub type RstcR = crate::BitReader<Rstc>;
impl RstcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstc {
        match self.bits {
            false => Rstc::Rstc0,
            true => Rstc::Rstc1,
        }
    }
    #[doc = "No Reset"]
    #[inline(always)]
    pub fn is_rstc_0(&self) -> bool {
        *self == Rstc::Rstc0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_rstc_1(&self) -> bool {
        *self == Rstc::Rstc1
    }
}
#[doc = "Field `RSTC` writer - Software Reset For CMD Line"]
pub type RstcW<'a, REG> = crate::BitWriter<'a, REG, Rstc>;
impl<'a, REG> RstcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Reset"]
    #[inline(always)]
    pub fn rstc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rstc::Rstc0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rstc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rstc::Rstc1)
    }
}
#[doc = "Software Reset For DATA Line\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstd {
    #[doc = "0: No Reset"]
    Rstd0 = 0,
    #[doc = "1: Reset"]
    Rstd1 = 1,
}
impl From<Rstd> for bool {
    #[inline(always)]
    fn from(variant: Rstd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTD` reader - Software Reset For DATA Line"]
pub type RstdR = crate::BitReader<Rstd>;
impl RstdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstd {
        match self.bits {
            false => Rstd::Rstd0,
            true => Rstd::Rstd1,
        }
    }
    #[doc = "No Reset"]
    #[inline(always)]
    pub fn is_rstd_0(&self) -> bool {
        *self == Rstd::Rstd0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_rstd_1(&self) -> bool {
        *self == Rstd::Rstd1
    }
}
#[doc = "Field `RSTD` writer - Software Reset For DATA Line"]
pub type RstdW<'a, REG> = crate::BitWriter<'a, REG, Rstd>;
impl<'a, REG> RstdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Reset"]
    #[inline(always)]
    pub fn rstd_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rstd::Rstd0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rstd_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rstd::Rstd1)
    }
}
#[doc = "Field `INITA` reader - Initialization Active"]
pub type InitaR = crate::BitReader;
#[doc = "Field `INITA` writer - Initialization Active"]
pub type InitaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTT` reader - Reset Tuning"]
pub type RsttR = crate::BitReader;
#[doc = "Field `RSTT` writer - Reset Tuning"]
pub type RsttW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:7 - Divisor"]
    #[inline(always)]
    pub fn dvs(&self) -> DvsR {
        DvsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfs(&self) -> SdclkfsR {
        SdclkfsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dtocv(&self) -> DtocvR {
        DtocvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - IPP_RST_N"]
    #[inline(always)]
    pub fn ipp_rst_n(&self) -> IppRstNR {
        IppRstNR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Software Reset For ALL"]
    #[inline(always)]
    pub fn rsta(&self) -> RstaR {
        RstaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn rstc(&self) -> RstcR {
        RstcR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software Reset For DATA Line"]
    #[inline(always)]
    pub fn rstd(&self) -> RstdR {
        RstdR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline(always)]
    pub fn inita(&self) -> InitaR {
        InitaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset Tuning"]
    #[inline(always)]
    pub fn rstt(&self) -> RsttR {
        RsttR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_CTRL")
            .field("dvs", &self.dvs())
            .field("sdclkfs", &self.sdclkfs())
            .field("dtocv", &self.dtocv())
            .field("ipp_rst_n", &self.ipp_rst_n())
            .field("rsta", &self.rsta())
            .field("rstc", &self.rstc())
            .field("rstd", &self.rstd())
            .field("inita", &self.inita())
            .field("rstt", &self.rstt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:7 - Divisor"]
    #[inline(always)]
    pub fn dvs(&mut self) -> DvsW<SysCtrlSpec> {
        DvsW::new(self, 4)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfs(&mut self) -> SdclkfsW<SysCtrlSpec> {
        SdclkfsW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dtocv(&mut self) -> DtocvW<SysCtrlSpec> {
        DtocvW::new(self, 16)
    }
    #[doc = "Bit 23 - IPP_RST_N"]
    #[inline(always)]
    pub fn ipp_rst_n(&mut self) -> IppRstNW<SysCtrlSpec> {
        IppRstNW::new(self, 23)
    }
    #[doc = "Bit 24 - Software Reset For ALL"]
    #[inline(always)]
    pub fn rsta(&mut self) -> RstaW<SysCtrlSpec> {
        RstaW::new(self, 24)
    }
    #[doc = "Bit 25 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn rstc(&mut self) -> RstcW<SysCtrlSpec> {
        RstcW::new(self, 25)
    }
    #[doc = "Bit 26 - Software Reset For DATA Line"]
    #[inline(always)]
    pub fn rstd(&mut self) -> RstdW<SysCtrlSpec> {
        RstdW::new(self, 26)
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline(always)]
    pub fn inita(&mut self) -> InitaW<SysCtrlSpec> {
        InitaW::new(self, 27)
    }
    #[doc = "Bit 28 - Reset Tuning"]
    #[inline(always)]
    pub fn rstt(&mut self) -> RsttW<SysCtrlSpec> {
        RsttW::new(self, 28)
    }
}
#[doc = "System Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCtrlSpec;
impl crate::RegisterSpec for SysCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctrl::R`](R) reader structure"]
impl crate::Readable for SysCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_ctrl::W`](W) writer structure"]
impl crate::Writable for SysCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_CTRL to value 0x0080_800f"]
impl crate::Resettable for SysCtrlSpec {
    const RESET_VALUE: u32 = 0x0080_800f;
}
