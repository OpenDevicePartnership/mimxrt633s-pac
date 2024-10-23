#[doc = "Register `RSTGT_R` reader"]
pub type R = crate::R<RstgtRSpec>;
#[doc = "Field `RSTGTN` reader - RSTGTN"]
pub type RstgtnR = crate::FieldReader;
#[doc = "Field `RSTGMS` reader - RSTGMS"]
pub type RstgmsR = crate::FieldReader;
#[doc = "RSTGSM\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rstgsm {
    #[doc = "0: Idle, waiting for the first data pattern write."]
    Rstgsm0 = 0,
    #[doc = "1: Waiting for the second data pattern write."]
    Rstgsm1 = 1,
    #[doc = "2: The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state. The \"01\" state persists for only one clock cycle. Software cannot observe this state."]
    Rstgsm2 = 2,
    #[doc = "3: This state encoding is never used and therefore reserved."]
    Rstgsm3 = 3,
}
impl From<Rstgsm> for u8 {
    #[inline(always)]
    fn from(variant: Rstgsm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rstgsm {
    type Ux = u8;
}
impl crate::IsEnum for Rstgsm {}
#[doc = "Field `RSTGSM` reader - RSTGSM"]
pub type RstgsmR = crate::FieldReader<Rstgsm>;
impl RstgsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstgsm {
        match self.bits {
            0 => Rstgsm::Rstgsm0,
            1 => Rstgsm::Rstgsm1,
            2 => Rstgsm::Rstgsm2,
            3 => Rstgsm::Rstgsm3,
            _ => unreachable!(),
        }
    }
    #[doc = "Idle, waiting for the first data pattern write."]
    #[inline(always)]
    pub fn is_rstgsm_0(&self) -> bool {
        *self == Rstgsm::Rstgsm0
    }
    #[doc = "Waiting for the second data pattern write."]
    #[inline(always)]
    pub fn is_rstgsm_1(&self) -> bool {
        *self == Rstgsm::Rstgsm1
    }
    #[doc = "The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state. The \"01\" state persists for only one clock cycle. Software cannot observe this state."]
    #[inline(always)]
    pub fn is_rstgsm_2(&self) -> bool {
        *self == Rstgsm::Rstgsm2
    }
    #[doc = "This state encoding is never used and therefore reserved."]
    #[inline(always)]
    pub fn is_rstgsm_3(&self) -> bool {
        *self == Rstgsm::Rstgsm3
    }
}
#[doc = "Field `ROZ` reader - ROZ"]
pub type RozR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RSTGTN"]
    #[inline(always)]
    pub fn rstgtn(&self) -> RstgtnR {
        RstgtnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - RSTGMS"]
    #[inline(always)]
    pub fn rstgms(&self) -> RstgmsR {
        RstgmsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - RSTGSM"]
    #[inline(always)]
    pub fn rstgsm(&self) -> RstgsmR {
        RstgsmR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - ROZ"]
    #[inline(always)]
    pub fn roz(&self) -> RozR {
        RozR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTGT_R")
            .field("rstgtn", &self.rstgtn())
            .field("rstgms", &self.rstgms())
            .field("rstgsm", &self.rstgsm())
            .field("roz", &self.roz())
            .finish()
    }
}
#[doc = "Reset Gate Read\n\nYou can [`read`](crate::Reg::read) this register and get [`rstgt_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstgtRSpec;
impl crate::RegisterSpec for RstgtRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rstgt_r::R`](R) reader structure"]
impl crate::Readable for RstgtRSpec {}
#[doc = "`reset()` method sets RSTGT_R to value 0"]
impl crate::Resettable for RstgtRSpec {
    const RESET_VALUE: u16 = 0;
}
