#[doc = "Register `FLSHCR1%s` reader"]
pub type R = crate::R<Flshcr1Spec>;
#[doc = "Register `FLSHCR1%s` writer"]
pub type W = crate::W<Flshcr1Spec>;
#[doc = "Field `TCSS` reader - Serial Flash CS setup time."]
pub type TcssR = crate::FieldReader;
#[doc = "Field `TCSS` writer - Serial Flash CS setup time."]
pub type TcssW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCSH` reader - Serial Flash CS Hold time."]
pub type TcshR = crate::FieldReader;
#[doc = "Field `TCSH` writer - Serial Flash CS Hold time."]
pub type TcshW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WA` reader - Word Addressable."]
pub type WaR = crate::BitReader;
#[doc = "Field `WA` writer - Word Addressable."]
pub type WaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAS` reader - Column Address Size."]
pub type CasR = crate::FieldReader;
#[doc = "Field `CAS` writer - Column Address Size."]
pub type CasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "CS interval unit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csintervalunit {
    #[doc = "0: The CS interval unit is 1 serial clock cycle"]
    Csintervalunit0 = 0,
    #[doc = "1: The CS interval unit is 256 serial clock cycle"]
    Csintervalunit1 = 1,
}
impl From<Csintervalunit> for bool {
    #[inline(always)]
    fn from(variant: Csintervalunit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSINTERVALUNIT` reader - CS interval unit"]
pub type CsintervalunitR = crate::BitReader<Csintervalunit>;
impl CsintervalunitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csintervalunit {
        match self.bits {
            false => Csintervalunit::Csintervalunit0,
            true => Csintervalunit::Csintervalunit1,
        }
    }
    #[doc = "The CS interval unit is 1 serial clock cycle"]
    #[inline(always)]
    pub fn is_csintervalunit_0(&self) -> bool {
        *self == Csintervalunit::Csintervalunit0
    }
    #[doc = "The CS interval unit is 256 serial clock cycle"]
    #[inline(always)]
    pub fn is_csintervalunit_1(&self) -> bool {
        *self == Csintervalunit::Csintervalunit1
    }
}
#[doc = "Field `CSINTERVALUNIT` writer - CS interval unit"]
pub type CsintervalunitW<'a, REG> = crate::BitWriter<'a, REG, Csintervalunit>;
impl<'a, REG> CsintervalunitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CS interval unit is 1 serial clock cycle"]
    #[inline(always)]
    pub fn csintervalunit_0(self) -> &'a mut crate::W<REG> {
        self.variant(Csintervalunit::Csintervalunit0)
    }
    #[doc = "The CS interval unit is 256 serial clock cycle"]
    #[inline(always)]
    pub fn csintervalunit_1(self) -> &'a mut crate::W<REG> {
        self.variant(Csintervalunit::Csintervalunit1)
    }
}
#[doc = "Field `CSINTERVAL` reader - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
pub type CsintervalR = crate::FieldReader<u16>;
#[doc = "Field `CSINTERVAL` writer - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
pub type CsintervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - Serial Flash CS setup time."]
    #[inline(always)]
    pub fn tcss(&self) -> TcssR {
        TcssR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Serial Flash CS Hold time."]
    #[inline(always)]
    pub fn tcsh(&self) -> TcshR {
        TcshR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Word Addressable."]
    #[inline(always)]
    pub fn wa(&self) -> WaR {
        WaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - Column Address Size."]
    #[inline(always)]
    pub fn cas(&self) -> CasR {
        CasR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - CS interval unit"]
    #[inline(always)]
    pub fn csintervalunit(&self) -> CsintervalunitR {
        CsintervalunitR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline(always)]
    pub fn csinterval(&self) -> CsintervalR {
        CsintervalR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLSHCR1")
            .field("tcss", &self.tcss())
            .field("tcsh", &self.tcsh())
            .field("wa", &self.wa())
            .field("cas", &self.cas())
            .field("csintervalunit", &self.csintervalunit())
            .field("csinterval", &self.csinterval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Serial Flash CS setup time."]
    #[inline(always)]
    #[must_use]
    pub fn tcss(&mut self) -> TcssW<Flshcr1Spec> {
        TcssW::new(self, 0)
    }
    #[doc = "Bits 5:9 - Serial Flash CS Hold time."]
    #[inline(always)]
    #[must_use]
    pub fn tcsh(&mut self) -> TcshW<Flshcr1Spec> {
        TcshW::new(self, 5)
    }
    #[doc = "Bit 10 - Word Addressable."]
    #[inline(always)]
    #[must_use]
    pub fn wa(&mut self) -> WaW<Flshcr1Spec> {
        WaW::new(self, 10)
    }
    #[doc = "Bits 11:14 - Column Address Size."]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CasW<Flshcr1Spec> {
        CasW::new(self, 11)
    }
    #[doc = "Bit 15 - CS interval unit"]
    #[inline(always)]
    #[must_use]
    pub fn csintervalunit(&mut self) -> CsintervalunitW<Flshcr1Spec> {
        CsintervalunitW::new(self, 15)
    }
    #[doc = "Bits 16:31 - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn csinterval(&mut self) -> CsintervalW<Flshcr1Spec> {
        CsintervalW::new(self, 16)
    }
}
#[doc = "Flash Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flshcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flshcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flshcr1Spec;
impl crate::RegisterSpec for Flshcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flshcr1::R`](R) reader structure"]
impl crate::Readable for Flshcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`flshcr1::W`](W) writer structure"]
impl crate::Writable for Flshcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLSHCR1%s to value 0x63"]
impl crate::Resettable for Flshcr1Spec {
    const RESET_VALUE: u32 = 0x63;
}
