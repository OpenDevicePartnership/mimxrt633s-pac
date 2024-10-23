#[doc = "Register `STS0` reader"]
pub type R = crate::R<Sts0Spec>;
#[doc = "Field `SEQIDLE` reader - This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
pub type SeqidleR = crate::BitReader;
#[doc = "Field `ARBIDLE` reader - This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
pub type ArbidleR = crate::BitReader;
#[doc = "This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Arbcmdsrc {
    #[doc = "0: Triggered by AHB read command (triggered by AHB read)."]
    Arbcmdsrc0 = 0,
    #[doc = "1: Triggered by AHB write command (triggered by AHB Write)."]
    Arbcmdsrc1 = 1,
    #[doc = "2: Triggered by IP command (triggered by setting register bit IPCMD.TRG)."]
    Arbcmdsrc2 = 2,
    #[doc = "3: Triggered by suspended command (resumed)."]
    Arbcmdsrc3 = 3,
}
impl From<Arbcmdsrc> for u8 {
    #[inline(always)]
    fn from(variant: Arbcmdsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Arbcmdsrc {
    type Ux = u8;
}
impl crate::IsEnum for Arbcmdsrc {}
#[doc = "Field `ARBCMDSRC` reader - This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1)."]
pub type ArbcmdsrcR = crate::FieldReader<Arbcmdsrc>;
impl ArbcmdsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbcmdsrc {
        match self.bits {
            0 => Arbcmdsrc::Arbcmdsrc0,
            1 => Arbcmdsrc::Arbcmdsrc1,
            2 => Arbcmdsrc::Arbcmdsrc2,
            3 => Arbcmdsrc::Arbcmdsrc3,
            _ => unreachable!(),
        }
    }
    #[doc = "Triggered by AHB read command (triggered by AHB read)."]
    #[inline(always)]
    pub fn is_arbcmdsrc_0(&self) -> bool {
        *self == Arbcmdsrc::Arbcmdsrc0
    }
    #[doc = "Triggered by AHB write command (triggered by AHB Write)."]
    #[inline(always)]
    pub fn is_arbcmdsrc_1(&self) -> bool {
        *self == Arbcmdsrc::Arbcmdsrc1
    }
    #[doc = "Triggered by IP command (triggered by setting register bit IPCMD.TRG)."]
    #[inline(always)]
    pub fn is_arbcmdsrc_2(&self) -> bool {
        *self == Arbcmdsrc::Arbcmdsrc2
    }
    #[doc = "Triggered by suspended command (resumed)."]
    #[inline(always)]
    pub fn is_arbcmdsrc_3(&self) -> bool {
        *self == Arbcmdsrc::Arbcmdsrc3
    }
}
#[doc = "Field `DATALEARNPHASEA` reader - Indicate the sampling clock phase selection on Port A after Data Learning."]
pub type DatalearnphaseaR = crate::FieldReader;
#[doc = "Field `DATALEARNPHASEB` reader - Indicate the sampling clock phase selection on Port B after Data Learning."]
pub type DatalearnphasebR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
    #[inline(always)]
    pub fn seqidle(&self) -> SeqidleR {
        SeqidleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
    #[inline(always)]
    pub fn arbidle(&self) -> ArbidleR {
        ArbidleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1)."]
    #[inline(always)]
    pub fn arbcmdsrc(&self) -> ArbcmdsrcR {
        ArbcmdsrcR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Indicate the sampling clock phase selection on Port A after Data Learning."]
    #[inline(always)]
    pub fn datalearnphasea(&self) -> DatalearnphaseaR {
        DatalearnphaseaR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicate the sampling clock phase selection on Port B after Data Learning."]
    #[inline(always)]
    pub fn datalearnphaseb(&self) -> DatalearnphasebR {
        DatalearnphasebR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS0")
            .field("seqidle", &self.seqidle())
            .field("arbidle", &self.arbidle())
            .field("arbcmdsrc", &self.arbcmdsrc())
            .field("datalearnphasea", &self.datalearnphasea())
            .field("datalearnphaseb", &self.datalearnphaseb())
            .finish()
    }
}
#[doc = "Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sts0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts0Spec;
impl crate::RegisterSpec for Sts0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts0::R`](R) reader structure"]
impl crate::Readable for Sts0Spec {}
#[doc = "`reset()` method sets STS0 to value 0x02"]
impl crate::Resettable for Sts0Spec {
    const RESET_VALUE: u32 = 0x02;
}
