#[doc = "Register `WTMK_LVL` reader"]
pub type R = crate::R<WtmkLvlSpec>;
#[doc = "Register `WTMK_LVL` writer"]
pub type W = crate::W<WtmkLvlSpec>;
#[doc = "Field `RD_WML` reader - Read Watermark Level"]
pub type RdWmlR = crate::FieldReader;
#[doc = "Field `RD_WML` writer - Read Watermark Level"]
pub type RdWmlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD_BRST_LEN` reader - Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
pub type RdBrstLenR = crate::FieldReader;
#[doc = "Field `RD_BRST_LEN` writer - Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
pub type RdBrstLenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WR_WML` reader - Write Watermark Level"]
pub type WrWmlR = crate::FieldReader;
#[doc = "Field `WR_WML` writer - Write Watermark Level"]
pub type WrWmlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WR_BRST_LEN` reader - Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
pub type WrBrstLenR = crate::FieldReader;
#[doc = "Field `WR_BRST_LEN` writer - Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
pub type WrBrstLenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Read Watermark Level"]
    #[inline(always)]
    pub fn rd_wml(&self) -> RdWmlR {
        RdWmlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub fn rd_brst_len(&self) -> RdBrstLenR {
        RdBrstLenR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Write Watermark Level"]
    #[inline(always)]
    pub fn wr_wml(&self) -> WrWmlR {
        WrWmlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub fn wr_brst_len(&self) -> WrBrstLenR {
        WrBrstLenR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WTMK_LVL")
            .field("rd_wml", &self.rd_wml())
            .field("rd_brst_len", &self.rd_brst_len())
            .field("wr_wml", &self.wr_wml())
            .field("wr_brst_len", &self.wr_brst_len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Watermark Level"]
    #[inline(always)]
    pub fn rd_wml(&mut self) -> RdWmlW<WtmkLvlSpec> {
        RdWmlW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub fn rd_brst_len(&mut self) -> RdBrstLenW<WtmkLvlSpec> {
        RdBrstLenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Write Watermark Level"]
    #[inline(always)]
    pub fn wr_wml(&mut self) -> WrWmlW<WtmkLvlSpec> {
        WrWmlW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub fn wr_brst_len(&mut self) -> WrBrstLenW<WtmkLvlSpec> {
        WrBrstLenW::new(self, 24)
    }
}
#[doc = "Watermark Level\n\nYou can [`read`](crate::Reg::read) this register and get [`wtmk_lvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtmk_lvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WtmkLvlSpec;
impl crate::RegisterSpec for WtmkLvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtmk_lvl::R`](R) reader structure"]
impl crate::Readable for WtmkLvlSpec {}
#[doc = "`write(|w| ..)` method takes [`wtmk_lvl::W`](W) writer structure"]
impl crate::Writable for WtmkLvlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTMK_LVL to value 0x0810_0810"]
impl crate::Resettable for WtmkLvlSpec {
    const RESET_VALUE: u32 = 0x0810_0810;
}
