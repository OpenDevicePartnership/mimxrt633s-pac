#[doc = "Register `MCR2` reader"]
pub type R = crate::R<Mcr2Spec>;
#[doc = "Register `MCR2` writer"]
pub type W = crate::W<Mcr2Spec>;
#[doc = "This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clrahbbufopt {
    #[doc = "0: AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    Clrahbbufopt0 = 0,
    #[doc = "1: AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    Clrahbbufopt1 = 1,
}
impl From<Clrahbbufopt> for bool {
    #[inline(always)]
    fn from(variant: Clrahbbufopt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRAHBBUFOPT` reader - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
pub type ClrahbbufoptR = crate::BitReader<Clrahbbufopt>;
impl ClrahbbufoptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clrahbbufopt {
        match self.bits {
            false => Clrahbbufopt::Clrahbbufopt0,
            true => Clrahbbufopt::Clrahbbufopt1,
        }
    }
    #[doc = "AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    #[inline(always)]
    pub fn is_clrahbbufopt_0(&self) -> bool {
        *self == Clrahbbufopt::Clrahbbufopt0
    }
    #[doc = "AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    #[inline(always)]
    pub fn is_clrahbbufopt_1(&self) -> bool {
        *self == Clrahbbufopt::Clrahbbufopt1
    }
}
#[doc = "Field `CLRAHBBUFOPT` writer - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
pub type ClrahbbufoptW<'a, REG> = crate::BitWriter<'a, REG, Clrahbbufopt>;
impl<'a, REG> ClrahbbufoptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    #[inline(always)]
    pub fn clrahbbufopt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clrahbbufopt::Clrahbbufopt0)
    }
    #[doc = "AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    #[inline(always)]
    pub fn clrahbbufopt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clrahbbufopt::Clrahbbufopt1)
    }
}
#[doc = "Field `CLRLEARNPHASE` reader - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
pub type ClrlearnphaseR = crate::BitReader;
#[doc = "Field `CLRLEARNPHASE` writer - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
pub type ClrlearnphaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "All external devices are same devices (both in types and size) for A1/A2/B1/B2.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Samedeviceen {
    #[doc = "0: In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 separately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    Samedeviceen0 = 0,
    #[doc = "1: FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    Samedeviceen1 = 1,
}
impl From<Samedeviceen> for bool {
    #[inline(always)]
    fn from(variant: Samedeviceen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMEDEVICEEN` reader - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
pub type SamedeviceenR = crate::BitReader<Samedeviceen>;
impl SamedeviceenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Samedeviceen {
        match self.bits {
            false => Samedeviceen::Samedeviceen0,
            true => Samedeviceen::Samedeviceen1,
        }
    }
    #[doc = "In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 separately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    #[inline(always)]
    pub fn is_samedeviceen_0(&self) -> bool {
        *self == Samedeviceen::Samedeviceen0
    }
    #[doc = "FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    #[inline(always)]
    pub fn is_samedeviceen_1(&self) -> bool {
        *self == Samedeviceen::Samedeviceen1
    }
}
#[doc = "Field `SAMEDEVICEEN` writer - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
pub type SamedeviceenW<'a, REG> = crate::BitWriter<'a, REG, Samedeviceen>;
impl<'a, REG> SamedeviceenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 separately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    #[inline(always)]
    pub fn samedeviceen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Samedeviceen::Samedeviceen0)
    }
    #[doc = "FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    #[inline(always)]
    pub fn samedeviceen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Samedeviceen::Samedeviceen1)
    }
}
#[doc = "B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\]
should be set.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sckbdiffopt {
    #[doc = "0: B_SCLK pad is used as port B SCLK clock output. Port B flash access is available."]
    Sckbdiffopt0 = 0,
    #[doc = "1: B_SCLK pad is used as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash access is not available."]
    Sckbdiffopt1 = 1,
}
impl From<Sckbdiffopt> for bool {
    #[inline(always)]
    fn from(variant: Sckbdiffopt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCKBDIFFOPT` reader - B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\]
should be set."]
pub type SckbdiffoptR = crate::BitReader<Sckbdiffopt>;
impl SckbdiffoptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sckbdiffopt {
        match self.bits {
            false => Sckbdiffopt::Sckbdiffopt0,
            true => Sckbdiffopt::Sckbdiffopt1,
        }
    }
    #[doc = "B_SCLK pad is used as port B SCLK clock output. Port B flash access is available."]
    #[inline(always)]
    pub fn is_sckbdiffopt_0(&self) -> bool {
        *self == Sckbdiffopt::Sckbdiffopt0
    }
    #[doc = "B_SCLK pad is used as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash access is not available."]
    #[inline(always)]
    pub fn is_sckbdiffopt_1(&self) -> bool {
        *self == Sckbdiffopt::Sckbdiffopt1
    }
}
#[doc = "Field `SCKBDIFFOPT` writer - B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\]
should be set."]
pub type SckbdiffoptW<'a, REG> = crate::BitWriter<'a, REG, Sckbdiffopt>;
impl<'a, REG> SckbdiffoptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "B_SCLK pad is used as port B SCLK clock output. Port B flash access is available."]
    #[inline(always)]
    pub fn sckbdiffopt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sckbdiffopt::Sckbdiffopt0)
    }
    #[doc = "B_SCLK pad is used as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash access is not available."]
    #[inline(always)]
    pub fn sckbdiffopt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sckbdiffopt::Sckbdiffopt1)
    }
}
#[doc = "Field `RESUMEWAIT` reader - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
pub type ResumewaitR = crate::FieldReader;
#[doc = "Field `RESUMEWAIT` writer - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
pub type ResumewaitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 11 - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline(always)]
    pub fn clrahbbufopt(&self) -> ClrahbbufoptR {
        ClrahbbufoptR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline(always)]
    pub fn clrlearnphase(&self) -> ClrlearnphaseR {
        ClrlearnphaseR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline(always)]
    pub fn samedeviceen(&self) -> SamedeviceenR {
        SamedeviceenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\]
should be set."]
    #[inline(always)]
    pub fn sckbdiffopt(&self) -> SckbdiffoptR {
        SckbdiffoptR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    pub fn resumewait(&self) -> ResumewaitR {
        ResumewaitR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR2")
            .field("clrahbbufopt", &self.clrahbbufopt())
            .field("clrlearnphase", &self.clrlearnphase())
            .field("samedeviceen", &self.samedeviceen())
            .field("sckbdiffopt", &self.sckbdiffopt())
            .field("resumewait", &self.resumewait())
            .finish()
    }
}
impl W {
    #[doc = "Bit 11 - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline(always)]
    pub fn clrahbbufopt(&mut self) -> ClrahbbufoptW<Mcr2Spec> {
        ClrahbbufoptW::new(self, 11)
    }
    #[doc = "Bit 14 - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline(always)]
    pub fn clrlearnphase(&mut self) -> ClrlearnphaseW<Mcr2Spec> {
        ClrlearnphaseW::new(self, 14)
    }
    #[doc = "Bit 15 - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline(always)]
    pub fn samedeviceen(&mut self) -> SamedeviceenW<Mcr2Spec> {
        SamedeviceenW::new(self, 15)
    }
    #[doc = "Bit 19 - B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\]
should be set."]
    #[inline(always)]
    pub fn sckbdiffopt(&mut self) -> SckbdiffoptW<Mcr2Spec> {
        SckbdiffoptW::new(self, 19)
    }
    #[doc = "Bits 24:31 - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    pub fn resumewait(&mut self) -> ResumewaitW<Mcr2Spec> {
        ResumewaitW::new(self, 24)
    }
}
#[doc = "Module Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcr2Spec;
impl crate::RegisterSpec for Mcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr2::R`](R) reader structure"]
impl crate::Readable for Mcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`mcr2::W`](W) writer structure"]
impl crate::Writable for Mcr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR2 to value 0x2000_81f7"]
impl crate::Resettable for Mcr2Spec {
    const RESET_VALUE: u32 = 0x2000_81f7;
}
