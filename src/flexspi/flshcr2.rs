#[doc = "Register `FLSHCR2%s` reader"]
pub type R = crate::R<Flshcr2Spec>;
#[doc = "Register `FLSHCR2%s` writer"]
pub type W = crate::W<Flshcr2Spec>;
#[doc = "Field `ARDSEQID` reader - Sequence Index for AHB Read triggered Command in LUT."]
pub type ArdseqidR = crate::FieldReader;
#[doc = "Field `ARDSEQID` writer - Sequence Index for AHB Read triggered Command in LUT."]
pub type ArdseqidW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ARDSEQNUM` reader - Sequence Number for AHB Read triggered Command in LUT."]
pub type ArdseqnumR = crate::FieldReader;
#[doc = "Field `ARDSEQNUM` writer - Sequence Number for AHB Read triggered Command in LUT."]
pub type ArdseqnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AWRSEQID` reader - Sequence Index for AHB Write triggered Command."]
pub type AwrseqidR = crate::FieldReader;
#[doc = "Field `AWRSEQID` writer - Sequence Index for AHB Write triggered Command."]
pub type AwrseqidW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AWRSEQNUM` reader - Sequence Number for AHB Write triggered Command."]
pub type AwrseqnumR = crate::FieldReader;
#[doc = "Field `AWRSEQNUM` writer - Sequence Number for AHB Write triggered Command."]
pub type AwrseqnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AWRWAIT` reader - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
pub type AwrwaitR = crate::FieldReader<u16>;
#[doc = "Field `AWRWAIT` writer - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
pub type AwrwaitW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "AWRWAIT unit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Awrwaitunit {
    #[doc = "0: The AWRWAIT unit is 2 ahb clock cycle"]
    Awrwaitunit0 = 0,
    #[doc = "1: The AWRWAIT unit is 8 ahb clock cycle"]
    Awrwaitunit1 = 1,
    #[doc = "2: The AWRWAIT unit is 32 ahb clock cycle"]
    Awrwaitunit2 = 2,
    #[doc = "3: The AWRWAIT unit is 128 ahb clock cycle"]
    Awrwaitunit3 = 3,
    #[doc = "4: The AWRWAIT unit is 512 ahb clock cycle"]
    Awrwaitunit4 = 4,
    #[doc = "5: The AWRWAIT unit is 2048 ahb clock cycle"]
    Awrwaitunit5 = 5,
    #[doc = "6: The AWRWAIT unit is 8192 ahb clock cycle"]
    Awrwaitunit6 = 6,
    #[doc = "7: The AWRWAIT unit is 32768 ahb clock cycle"]
    Awrwaitunit7 = 7,
}
impl From<Awrwaitunit> for u8 {
    #[inline(always)]
    fn from(variant: Awrwaitunit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Awrwaitunit {
    type Ux = u8;
}
impl crate::IsEnum for Awrwaitunit {}
#[doc = "Field `AWRWAITUNIT` reader - AWRWAIT unit"]
pub type AwrwaitunitR = crate::FieldReader<Awrwaitunit>;
impl AwrwaitunitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awrwaitunit {
        match self.bits {
            0 => Awrwaitunit::Awrwaitunit0,
            1 => Awrwaitunit::Awrwaitunit1,
            2 => Awrwaitunit::Awrwaitunit2,
            3 => Awrwaitunit::Awrwaitunit3,
            4 => Awrwaitunit::Awrwaitunit4,
            5 => Awrwaitunit::Awrwaitunit5,
            6 => Awrwaitunit::Awrwaitunit6,
            7 => Awrwaitunit::Awrwaitunit7,
            _ => unreachable!(),
        }
    }
    #[doc = "The AWRWAIT unit is 2 ahb clock cycle"]
    #[inline(always)]
    pub fn is_awrwaitunit_0(&self) -> bool {
        *self == Awrwaitunit::Awrwaitunit0
    }
    #[doc = "The AWRWAIT unit is 8 ahb clock cycle"]
    #[inline(always)]
    pub fn is_awrwaitunit_1(&self) -> bool {
        *self == Awrwaitunit::Awrwaitunit1
    }
    #[doc = "The AWRWAIT unit is 32 ahb clock cycle"]
    #[inline(always)]
    pub fn is_awrwaitunit_2(&self) -> bool {
        *self == Awrwaitunit::Awrwaitunit2
    }
    #[doc = "The AWRWAIT unit is 128 ahb clock cycle"]
    #[inline(always)]
    pub fn is_awrwaitunit_3(&self) -> bool {
        *self == Awrwaitunit::Awrwaitunit3
    }
    #[doc = "The AWRWAIT unit is 512 ahb clock cycle"]
    #[inline(always)]
    pub fn is_awrwaitunit_4(&self) -> bool {
        *self == Awrwaitunit::Awrwaitunit4
    }
    #[doc = "The AWRWAIT unit is 2048 ahb clock cycle"]
    #[inline(always)]
    pub fn is_awrwaitunit_5(&self) -> bool {
        *self == Awrwaitunit::Awrwaitunit5
    }
    #[doc = "The AWRWAIT unit is 8192 ahb clock cycle"]
    #[inline(always)]
    pub fn is_awrwaitunit_6(&self) -> bool {
        *self == Awrwaitunit::Awrwaitunit6
    }
    #[doc = "The AWRWAIT unit is 32768 ahb clock cycle"]
    #[inline(always)]
    pub fn is_awrwaitunit_7(&self) -> bool {
        *self == Awrwaitunit::Awrwaitunit7
    }
}
#[doc = "Field `AWRWAITUNIT` writer - AWRWAIT unit"]
pub type AwrwaitunitW<'a, REG> = crate::FieldWriter<'a, REG, 3, Awrwaitunit, crate::Safe>;
impl<'a, REG> AwrwaitunitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The AWRWAIT unit is 2 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_0(self) -> &'a mut crate::W<REG> {
        self.variant(Awrwaitunit::Awrwaitunit0)
    }
    #[doc = "The AWRWAIT unit is 8 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_1(self) -> &'a mut crate::W<REG> {
        self.variant(Awrwaitunit::Awrwaitunit1)
    }
    #[doc = "The AWRWAIT unit is 32 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_2(self) -> &'a mut crate::W<REG> {
        self.variant(Awrwaitunit::Awrwaitunit2)
    }
    #[doc = "The AWRWAIT unit is 128 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_3(self) -> &'a mut crate::W<REG> {
        self.variant(Awrwaitunit::Awrwaitunit3)
    }
    #[doc = "The AWRWAIT unit is 512 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_4(self) -> &'a mut crate::W<REG> {
        self.variant(Awrwaitunit::Awrwaitunit4)
    }
    #[doc = "The AWRWAIT unit is 2048 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_5(self) -> &'a mut crate::W<REG> {
        self.variant(Awrwaitunit::Awrwaitunit5)
    }
    #[doc = "The AWRWAIT unit is 8192 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_6(self) -> &'a mut crate::W<REG> {
        self.variant(Awrwaitunit::Awrwaitunit6)
    }
    #[doc = "The AWRWAIT unit is 32768 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_7(self) -> &'a mut crate::W<REG> {
        self.variant(Awrwaitunit::Awrwaitunit7)
    }
}
#[doc = "Field `CLRINSTRPTR` reader - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
pub type ClrinstrptrR = crate::BitReader;
#[doc = "Field `CLRINSTRPTR` writer - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
pub type ClrinstrptrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Sequence Index for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub fn ardseqid(&self) -> ArdseqidR {
        ArdseqidR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Sequence Number for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub fn ardseqnum(&self) -> ArdseqnumR {
        ArdseqnumR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - Sequence Index for AHB Write triggered Command."]
    #[inline(always)]
    pub fn awrseqid(&self) -> AwrseqidR {
        AwrseqidR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Sequence Number for AHB Write triggered Command."]
    #[inline(always)]
    pub fn awrseqnum(&self) -> AwrseqnumR {
        AwrseqnumR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:27 - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline(always)]
    pub fn awrwait(&self) -> AwrwaitR {
        AwrwaitR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - AWRWAIT unit"]
    #[inline(always)]
    pub fn awrwaitunit(&self) -> AwrwaitunitR {
        AwrwaitunitR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline(always)]
    pub fn clrinstrptr(&self) -> ClrinstrptrR {
        ClrinstrptrR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLSHCR2")
            .field("ardseqid", &self.ardseqid())
            .field("ardseqnum", &self.ardseqnum())
            .field("awrseqid", &self.awrseqid())
            .field("awrseqnum", &self.awrseqnum())
            .field("awrwait", &self.awrwait())
            .field("awrwaitunit", &self.awrwaitunit())
            .field("clrinstrptr", &self.clrinstrptr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Sequence Index for AHB Read triggered Command in LUT."]
    #[inline(always)]
    #[must_use]
    pub fn ardseqid(&mut self) -> ArdseqidW<Flshcr2Spec> {
        ArdseqidW::new(self, 0)
    }
    #[doc = "Bits 5:7 - Sequence Number for AHB Read triggered Command in LUT."]
    #[inline(always)]
    #[must_use]
    pub fn ardseqnum(&mut self) -> ArdseqnumW<Flshcr2Spec> {
        ArdseqnumW::new(self, 5)
    }
    #[doc = "Bits 8:12 - Sequence Index for AHB Write triggered Command."]
    #[inline(always)]
    #[must_use]
    pub fn awrseqid(&mut self) -> AwrseqidW<Flshcr2Spec> {
        AwrseqidW::new(self, 8)
    }
    #[doc = "Bits 13:15 - Sequence Number for AHB Write triggered Command."]
    #[inline(always)]
    #[must_use]
    pub fn awrseqnum(&mut self) -> AwrseqnumW<Flshcr2Spec> {
        AwrseqnumW::new(self, 13)
    }
    #[doc = "Bits 16:27 - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline(always)]
    #[must_use]
    pub fn awrwait(&mut self) -> AwrwaitW<Flshcr2Spec> {
        AwrwaitW::new(self, 16)
    }
    #[doc = "Bits 28:30 - AWRWAIT unit"]
    #[inline(always)]
    #[must_use]
    pub fn awrwaitunit(&mut self) -> AwrwaitunitW<Flshcr2Spec> {
        AwrwaitunitW::new(self, 28)
    }
    #[doc = "Bit 31 - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline(always)]
    #[must_use]
    pub fn clrinstrptr(&mut self) -> ClrinstrptrW<Flshcr2Spec> {
        ClrinstrptrW::new(self, 31)
    }
}
#[doc = "Flash Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`flshcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flshcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flshcr2Spec;
impl crate::RegisterSpec for Flshcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flshcr2::R`](R) reader structure"]
impl crate::Readable for Flshcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`flshcr2::W`](W) writer structure"]
impl crate::Writable for Flshcr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLSHCR2%s to value 0"]
impl crate::Resettable for Flshcr2Spec {
    const RESET_VALUE: u32 = 0;
}
