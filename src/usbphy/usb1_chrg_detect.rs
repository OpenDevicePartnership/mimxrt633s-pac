#[doc = "Register `USB1_CHRG_DETECT` reader"]
pub type R = crate::R<Usb1ChrgDetectSpec>;
#[doc = "Register `USB1_CHRG_DETECT` writer"]
pub type W = crate::W<Usb1ChrgDetectSpec>;
#[doc = "Field `PULLUP_DP` reader - This bit is used to pull up DP, for digital charge detect."]
pub type PullupDpR = crate::BitReader;
#[doc = "Field `PULLUP_DP` writer - This bit is used to pull up DP, for digital charge detect."]
pub type PullupDpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BgrIbias {
    #[doc = "0: Bias current is derived from the USB PHY internal current generator."]
    BgrIbias0 = 0,
    #[doc = "1: Bias current is derived from the reference generator of the bandgap."]
    BgrIbias1 = 1,
}
impl From<BgrIbias> for bool {
    #[inline(always)]
    fn from(variant: BgrIbias) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGR_IBIAS` reader - USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
pub type BgrIbiasR = crate::BitReader<BgrIbias>;
impl BgrIbiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BgrIbias {
        match self.bits {
            false => BgrIbias::BgrIbias0,
            true => BgrIbias::BgrIbias1,
        }
    }
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    #[inline(always)]
    pub fn is_bgr_ibias_0(&self) -> bool {
        *self == BgrIbias::BgrIbias0
    }
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    #[inline(always)]
    pub fn is_bgr_ibias_1(&self) -> bool {
        *self == BgrIbias::BgrIbias1
    }
}
#[doc = "Field `BGR_IBIAS` writer - USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
pub type BgrIbiasW<'a, REG> = crate::BitWriter<'a, REG, BgrIbias>;
impl<'a, REG> BgrIbiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    #[inline(always)]
    pub fn bgr_ibias_0(self) -> &'a mut crate::W<REG> {
        self.variant(BgrIbias::BgrIbias0)
    }
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    #[inline(always)]
    pub fn bgr_ibias_1(self) -> &'a mut crate::W<REG> {
        self.variant(BgrIbias::BgrIbias1)
    }
}
impl R {
    #[doc = "Bit 2 - This bit is used to pull up DP, for digital charge detect."]
    #[inline(always)]
    pub fn pullup_dp(&self) -> PullupDpR {
        PullupDpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 23 - USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[inline(always)]
    pub fn bgr_ibias(&self) -> BgrIbiasR {
        BgrIbiasR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_CHRG_DETECT")
            .field("pullup_dp", &self.pullup_dp())
            .field("bgr_ibias", &self.bgr_ibias())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - This bit is used to pull up DP, for digital charge detect."]
    #[inline(always)]
    #[must_use]
    pub fn pullup_dp(&mut self) -> PullupDpW<Usb1ChrgDetectSpec> {
        PullupDpW::new(self, 2)
    }
    #[doc = "Bit 23 - USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[inline(always)]
    #[must_use]
    pub fn bgr_ibias(&mut self) -> BgrIbiasW<Usb1ChrgDetectSpec> {
        BgrIbiasW::new(self, 23)
    }
}
#[doc = "USB PHY Charger Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_chrg_detect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_chrg_detect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb1ChrgDetectSpec;
impl crate::RegisterSpec for Usb1ChrgDetectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb1_chrg_detect::R`](R) reader structure"]
impl crate::Readable for Usb1ChrgDetectSpec {}
#[doc = "`write(|w| ..)` method takes [`usb1_chrg_detect::W`](W) writer structure"]
impl crate::Writable for Usb1ChrgDetectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB1_CHRG_DETECT to value 0x8018_0000"]
impl crate::Resettable for Usb1ChrgDetectSpec {
    const RESET_VALUE: u32 = 0x8018_0000;
}
