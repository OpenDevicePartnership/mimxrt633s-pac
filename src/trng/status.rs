#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `TF1BR0` reader - Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed."]
pub type Tf1br0R = crate::BitReader;
#[doc = "Field `TF1BR1` reader - Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed."]
pub type Tf1br1R = crate::BitReader;
#[doc = "Field `TF2BR0` reader - Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed."]
pub type Tf2br0R = crate::BitReader;
#[doc = "Field `TF2BR1` reader - Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed."]
pub type Tf2br1R = crate::BitReader;
#[doc = "Field `TF3BR0` reader - Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed."]
pub type Tf3br0R = crate::BitReader;
#[doc = "Field `TF3BR1` reader - Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed."]
pub type Tf3br1R = crate::BitReader;
#[doc = "Field `TF4BR0` reader - Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed."]
pub type Tf4br0R = crate::BitReader;
#[doc = "Field `TF4BR1` reader - Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed."]
pub type Tf4br1R = crate::BitReader;
#[doc = "Field `TF5BR0` reader - Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed."]
pub type Tf5br0R = crate::BitReader;
#[doc = "Field `TF5BR1` reader - Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed."]
pub type Tf5br1R = crate::BitReader;
#[doc = "Field `TF6PBR0` reader - Test Fail, 6 Plus Bit Run, Sampling 0s"]
pub type Tf6pbr0R = crate::BitReader;
#[doc = "Field `TF6PBR1` reader - Test Fail, 6 Plus Bit Run, Sampling 1s"]
pub type Tf6pbr1R = crate::BitReader;
#[doc = "Field `TFSB` reader - Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed."]
pub type TfsbR = crate::BitReader;
#[doc = "Field `TFLR` reader - Test Fail, Long Run. If TFLR=1, the Long Run Test has failed."]
pub type TflrR = crate::BitReader;
#[doc = "Field `TFP` reader - Test Fail, Poker. If TFP=1, the Poker Test has failed."]
pub type TfpR = crate::BitReader;
#[doc = "Field `TFMB` reader - Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed."]
pub type TfmbR = crate::BitReader;
#[doc = "Field `RETRY_CT` reader - RETRY COUNT"]
pub type RetryCtR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf1br0(&self) -> Tf1br0R {
        Tf1br0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf1br1(&self) -> Tf1br1R {
        Tf1br1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf2br0(&self) -> Tf2br0R {
        Tf2br0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf2br1(&self) -> Tf2br1R {
        Tf2br1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf3br0(&self) -> Tf3br0R {
        Tf3br0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf3br1(&self) -> Tf3br1R {
        Tf3br1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf4br0(&self) -> Tf4br0R {
        Tf4br0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf4br1(&self) -> Tf4br1R {
        Tf4br1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf5br0(&self) -> Tf5br0R {
        Tf5br0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf5br1(&self) -> Tf5br1R {
        Tf5br1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Test Fail, 6 Plus Bit Run, Sampling 0s"]
    #[inline(always)]
    pub fn tf6pbr0(&self) -> Tf6pbr0R {
        Tf6pbr0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Test Fail, 6 Plus Bit Run, Sampling 1s"]
    #[inline(always)]
    pub fn tf6pbr1(&self) -> Tf6pbr1R {
        Tf6pbr1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed."]
    #[inline(always)]
    pub fn tfsb(&self) -> TfsbR {
        TfsbR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Test Fail, Long Run. If TFLR=1, the Long Run Test has failed."]
    #[inline(always)]
    pub fn tflr(&self) -> TflrR {
        TflrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Test Fail, Poker. If TFP=1, the Poker Test has failed."]
    #[inline(always)]
    pub fn tfp(&self) -> TfpR {
        TfpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed."]
    #[inline(always)]
    pub fn tfmb(&self) -> TfmbR {
        TfmbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn retry_ct(&self) -> RetryCtR {
        RetryCtR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("tf1br0", &self.tf1br0())
            .field("tf1br1", &self.tf1br1())
            .field("tf2br0", &self.tf2br0())
            .field("tf2br1", &self.tf2br1())
            .field("tf3br0", &self.tf3br0())
            .field("tf3br1", &self.tf3br1())
            .field("tf4br0", &self.tf4br0())
            .field("tf4br1", &self.tf4br1())
            .field("tf5br0", &self.tf5br0())
            .field("tf5br1", &self.tf5br1())
            .field("tf6pbr0", &self.tf6pbr0())
            .field("tf6pbr1", &self.tf6pbr1())
            .field("tfsb", &self.tfsb())
            .field("tflr", &self.tflr())
            .field("tfp", &self.tfp())
            .field("tfmb", &self.tfmb())
            .field("retry_ct", &self.retry_ct())
            .finish()
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
