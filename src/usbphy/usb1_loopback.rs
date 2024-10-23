#[doc = "Register `USB1_LOOPBACK` reader"]
pub type R = crate::R<Usb1LoopbackSpec>;
#[doc = "Register `USB1_LOOPBACK` writer"]
pub type W = crate::W<Usb1LoopbackSpec>;
#[doc = "Field `UTMI_TESTSTART` reader - This bit enables the USB loopback test."]
pub type UtmiTeststartR = crate::BitReader;
#[doc = "Field `UTMI_TESTSTART` writer - This bit enables the USB loopback test."]
pub type UtmiTeststartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTMI_DIG_TST0` reader - Mode control for USB loopback test"]
pub type UtmiDigTst0R = crate::BitReader;
#[doc = "Field `UTMI_DIG_TST0` writer - Mode control for USB loopback test"]
pub type UtmiDigTst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTMI_DIG_TST1` reader - Mode control for USB loopback test"]
pub type UtmiDigTst1R = crate::BitReader;
#[doc = "Field `UTMI_DIG_TST1` writer - Mode control for USB loopback test"]
pub type UtmiDigTst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTI_TX_HS_MODE` reader - Select HS or FS mode for USB loopback testing"]
pub type TstiTxHsModeR = crate::BitReader;
#[doc = "Field `TSTI_TX_HS_MODE` writer - Select HS or FS mode for USB loopback testing"]
pub type TstiTxHsModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTI_TX_LS_MODE` reader - Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
pub type TstiTxLsModeR = crate::BitReader;
#[doc = "Field `TSTI_TX_LS_MODE` writer - Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
pub type TstiTxLsModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTI_TX_EN` reader - Enable TX for USB loopback test."]
pub type TstiTxEnR = crate::BitReader;
#[doc = "Field `TSTI_TX_EN` writer - Enable TX for USB loopback test."]
pub type TstiTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTI_TX_HIZ` reader - Sets TX Hi-Z for USB loopback test."]
pub type TstiTxHizR = crate::BitReader;
#[doc = "Field `TSTI_TX_HIZ` writer - Sets TX Hi-Z for USB loopback test."]
pub type TstiTxHizW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTMO_DIG_TST0` reader - This read-only bit is a status bit for USB loopback test results"]
pub type UtmoDigTst0R = crate::BitReader;
#[doc = "Field `UTMO_DIG_TST0` writer - This read-only bit is a status bit for USB loopback test results"]
pub type UtmoDigTst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTMO_DIG_TST1` reader - This read-only bit is a status bit for USB loopback test"]
pub type UtmoDigTst1R = crate::BitReader;
#[doc = "Field `UTMO_DIG_TST1` writer - This read-only bit is a status bit for USB loopback test"]
pub type UtmoDigTst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTI_HSFS_MODE_EN` reader - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
pub type TstiHsfsModeEnR = crate::BitReader;
#[doc = "Field `TSTI_HSFS_MODE_EN` writer - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
pub type TstiHsfsModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTPKT` reader - Selects the packet data byte used for USB loopback testing in Pulse mode"]
pub type TstpktR = crate::FieldReader;
#[doc = "Field `TSTPKT` writer - Selects the packet data byte used for USB loopback testing in Pulse mode"]
pub type TstpktW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - This bit enables the USB loopback test."]
    #[inline(always)]
    pub fn utmi_teststart(&self) -> UtmiTeststartR {
        UtmiTeststartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode control for USB loopback test"]
    #[inline(always)]
    pub fn utmi_dig_tst0(&self) -> UtmiDigTst0R {
        UtmiDigTst0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode control for USB loopback test"]
    #[inline(always)]
    pub fn utmi_dig_tst1(&self) -> UtmiDigTst1R {
        UtmiDigTst1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select HS or FS mode for USB loopback testing"]
    #[inline(always)]
    pub fn tsti_tx_hs_mode(&self) -> TstiTxHsModeR {
        TstiTxHsModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[inline(always)]
    pub fn tsti_tx_ls_mode(&self) -> TstiTxLsModeR {
        TstiTxLsModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable TX for USB loopback test."]
    #[inline(always)]
    pub fn tsti_tx_en(&self) -> TstiTxEnR {
        TstiTxEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    pub fn tsti_tx_hiz(&self) -> TstiTxHizR {
        TstiTxHizR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This read-only bit is a status bit for USB loopback test results"]
    #[inline(always)]
    pub fn utmo_dig_tst0(&self) -> UtmoDigTst0R {
        UtmoDigTst0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This read-only bit is a status bit for USB loopback test"]
    #[inline(always)]
    pub fn utmo_dig_tst1(&self) -> UtmoDigTst1R {
        UtmoDigTst1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline(always)]
    pub fn tsti_hsfs_mode_en(&self) -> TstiHsfsModeEnR {
        TstiHsfsModeEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[inline(always)]
    pub fn tstpkt(&self) -> TstpktR {
        TstpktR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_LOOPBACK")
            .field("utmi_teststart", &self.utmi_teststart())
            .field("utmi_dig_tst0", &self.utmi_dig_tst0())
            .field("utmi_dig_tst1", &self.utmi_dig_tst1())
            .field("tsti_tx_hs_mode", &self.tsti_tx_hs_mode())
            .field("tsti_tx_ls_mode", &self.tsti_tx_ls_mode())
            .field("tsti_tx_en", &self.tsti_tx_en())
            .field("tsti_tx_hiz", &self.tsti_tx_hiz())
            .field("utmo_dig_tst0", &self.utmo_dig_tst0())
            .field("utmo_dig_tst1", &self.utmo_dig_tst1())
            .field("tsti_hsfs_mode_en", &self.tsti_hsfs_mode_en())
            .field("tstpkt", &self.tstpkt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn utmi_teststart(&mut self) -> UtmiTeststartW<Usb1LoopbackSpec> {
        UtmiTeststartW::new(self, 0)
    }
    #[doc = "Bit 1 - Mode control for USB loopback test"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_dig_tst0(&mut self) -> UtmiDigTst0W<Usb1LoopbackSpec> {
        UtmiDigTst0W::new(self, 1)
    }
    #[doc = "Bit 2 - Mode control for USB loopback test"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_dig_tst1(&mut self) -> UtmiDigTst1W<Usb1LoopbackSpec> {
        UtmiDigTst1W::new(self, 2)
    }
    #[doc = "Bit 3 - Select HS or FS mode for USB loopback testing"]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_hs_mode(&mut self) -> TstiTxHsModeW<Usb1LoopbackSpec> {
        TstiTxHsModeW::new(self, 3)
    }
    #[doc = "Bit 4 - Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_ls_mode(&mut self) -> TstiTxLsModeW<Usb1LoopbackSpec> {
        TstiTxLsModeW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable TX for USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_en(&mut self) -> TstiTxEnW<Usb1LoopbackSpec> {
        TstiTxEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_hiz(&mut self) -> TstiTxHizW<Usb1LoopbackSpec> {
        TstiTxHizW::new(self, 6)
    }
    #[doc = "Bit 7 - This read-only bit is a status bit for USB loopback test results"]
    #[inline(always)]
    #[must_use]
    pub fn utmo_dig_tst0(&mut self) -> UtmoDigTst0W<Usb1LoopbackSpec> {
        UtmoDigTst0W::new(self, 7)
    }
    #[doc = "Bit 8 - This read-only bit is a status bit for USB loopback test"]
    #[inline(always)]
    #[must_use]
    pub fn utmo_dig_tst1(&mut self) -> UtmoDigTst1W<Usb1LoopbackSpec> {
        UtmoDigTst1W::new(self, 8)
    }
    #[doc = "Bit 15 - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline(always)]
    #[must_use]
    pub fn tsti_hsfs_mode_en(&mut self) -> TstiHsfsModeEnW<Usb1LoopbackSpec> {
        TstiHsfsModeEnW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[inline(always)]
    #[must_use]
    pub fn tstpkt(&mut self) -> TstpktW<Usb1LoopbackSpec> {
        TstpktW::new(self, 16)
    }
}
#[doc = "USB PHY Loopback Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_loopback::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_loopback::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb1LoopbackSpec;
impl crate::RegisterSpec for Usb1LoopbackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb1_loopback::R`](R) reader structure"]
impl crate::Readable for Usb1LoopbackSpec {}
#[doc = "`write(|w| ..)` method takes [`usb1_loopback::W`](W) writer structure"]
impl crate::Writable for Usb1LoopbackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB1_LOOPBACK to value 0x0055_0000"]
impl crate::Resettable for Usb1LoopbackSpec {
    const RESET_VALUE: u32 = 0x0055_0000;
}
