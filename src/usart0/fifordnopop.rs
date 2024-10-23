#[doc = "Register `FIFORDNOPOP` reader"]
pub type R = crate::R<FifordnopopSpec>;
#[doc = "Field `RXDATA` reader - Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
pub type RxdataR = crate::FieldReader<u16>;
#[doc = "Field `FRAMERR` reader - Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
pub type FramerrR = crate::BitReader;
#[doc = "Field `PARITYERR` reader - Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
pub type ParityerrR = crate::BitReader;
#[doc = "Field `RXNOISE` reader - Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
pub type RxnoiseR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub fn framerr(&self) -> FramerrR {
        FramerrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
    #[inline(always)]
    pub fn parityerr(&self) -> ParityerrR {
        ParityerrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
    #[inline(always)]
    pub fn rxnoise(&self) -> RxnoiseR {
        RxnoiseR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFORDNOPOP")
            .field("rxdata", &self.rxdata())
            .field("framerr", &self.framerr())
            .field("parityerr", &self.parityerr())
            .field("rxnoise", &self.rxnoise())
            .finish()
    }
}
#[doc = "FIFO data read with no FIFO pop.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifordnopop::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifordnopopSpec;
impl crate::RegisterSpec for FifordnopopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifordnopop::R`](R) reader structure"]
impl crate::Readable for FifordnopopSpec {}
#[doc = "`reset()` method sets FIFORDNOPOP to value 0"]
impl crate::Resettable for FifordnopopSpec {
    const RESET_VALUE: u32 = 0;
}
