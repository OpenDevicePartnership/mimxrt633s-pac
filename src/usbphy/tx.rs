#[doc = "Register `TX` reader"]
pub type R = crate::R<TxSpec>;
#[doc = "Register `TX` writer"]
pub type W = crate::W<TxSpec>;
#[doc = "Decode to trim the nominal 17\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCal {
    #[doc = "0: Maximum current, approximately 19% above nominal."]
    DCal0 = 0,
    #[doc = "7: Nominal"]
    DCal7 = 7,
    #[doc = "15: Minimum current, approximately 19% below nominal."]
    DCal15 = 15,
}
impl From<DCal> for u8 {
    #[inline(always)]
    fn from(variant: DCal) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DCal {
    type Ux = u8;
}
impl crate::IsEnum for DCal {}
#[doc = "Field `D_CAL` reader - Decode to trim the nominal 17"]
pub type DCalR = crate::FieldReader<DCal>;
impl DCalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DCal> {
        match self.bits {
            0 => Some(DCal::DCal0),
            7 => Some(DCal::DCal7),
            15 => Some(DCal::DCal15),
            _ => None,
        }
    }
    #[doc = "Maximum current, approximately 19% above nominal."]
    #[inline(always)]
    pub fn is_d_cal_0(&self) -> bool {
        *self == DCal::DCal0
    }
    #[doc = "Nominal"]
    #[inline(always)]
    pub fn is_d_cal_7(&self) -> bool {
        *self == DCal::DCal7
    }
    #[doc = "Minimum current, approximately 19% below nominal."]
    #[inline(always)]
    pub fn is_d_cal_15(&self) -> bool {
        *self == DCal::DCal15
    }
}
#[doc = "Field `D_CAL` writer - Decode to trim the nominal 17"]
pub type DCalW<'a, REG> = crate::FieldWriter<'a, REG, 4, DCal>;
impl<'a, REG> DCalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum current, approximately 19% above nominal."]
    #[inline(always)]
    pub fn d_cal_0(self) -> &'a mut crate::W<REG> {
        self.variant(DCal::DCal0)
    }
    #[doc = "Nominal"]
    #[inline(always)]
    pub fn d_cal_7(self) -> &'a mut crate::W<REG> {
        self.variant(DCal::DCal7)
    }
    #[doc = "Minimum current, approximately 19% below nominal."]
    #[inline(always)]
    pub fn d_cal_15(self) -> &'a mut crate::W<REG> {
        self.variant(DCal::DCal15)
    }
}
#[doc = "Field `TXCAL45DM` reader - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
pub type Txcal45dmR = crate::FieldReader;
#[doc = "Field `TXCAL45DM` writer - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
pub type Txcal45dmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXENCAL45DN` reader - Enable resistance calibration on DN."]
pub type Txencal45dnR = crate::BitReader;
#[doc = "Field `TXENCAL45DN` writer - Enable resistance calibration on DN."]
pub type Txencal45dnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCAL45DP` reader - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
pub type Txcal45dpR = crate::FieldReader;
#[doc = "Field `TXCAL45DP` writer - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
pub type Txcal45dpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXENCAL45DP` reader - Enable resistance calibration on DP."]
pub type Txencal45dpR = crate::BitReader;
#[doc = "Field `TXENCAL45DP` writer - Enable resistance calibration on DP."]
pub type Txencal45dpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline(always)]
    pub fn d_cal(&self) -> DCalR {
        DCalR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub fn txcal45dm(&self) -> Txcal45dmR {
        Txcal45dmR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline(always)]
    pub fn txencal45dn(&self) -> Txencal45dnR {
        Txencal45dnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&self) -> Txcal45dpR {
        Txcal45dpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP."]
    #[inline(always)]
    pub fn txencal45dp(&self) -> Txencal45dpR {
        Txencal45dpR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX")
            .field("d_cal", &self.d_cal())
            .field("txcal45dm", &self.txcal45dm())
            .field("txencal45dn", &self.txencal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .field("txencal45dp", &self.txencal45dp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline(always)]
    #[must_use]
    pub fn d_cal(&mut self) -> DCalW<TxSpec> {
        DCalW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    #[must_use]
    pub fn txcal45dm(&mut self) -> Txcal45dmW<TxSpec> {
        Txcal45dmW::new(self, 8)
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline(always)]
    #[must_use]
    pub fn txencal45dn(&mut self) -> Txencal45dnW<TxSpec> {
        Txencal45dnW::new(self, 13)
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    #[must_use]
    pub fn txcal45dp(&mut self) -> Txcal45dpW<TxSpec> {
        Txcal45dpW::new(self, 16)
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP."]
    #[inline(always)]
    #[must_use]
    pub fn txencal45dp(&mut self) -> Txencal45dpW<TxSpec> {
        Txencal45dpW::new(self, 21)
    }
}
#[doc = "USB PHY Transmitter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxSpec;
impl crate::RegisterSpec for TxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx::R`](R) reader structure"]
impl crate::Readable for TxSpec {}
#[doc = "`write(|w| ..)` method takes [`tx::W`](W) writer structure"]
impl crate::Writable for TxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX to value 0x0a00_0402"]
impl crate::Resettable for TxSpec {
    const RESET_VALUE: u32 = 0x0a00_0402;
}
