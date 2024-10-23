#[doc = "Register `FIFOWR` writer"]
pub type W = crate::W<FifowrSpec>;
#[doc = "Field `TXDATA` writer - Transmit data to the FIFO."]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txssel0N {
    #[doc = "0: SSEL0 asserted."]
    Asserted = 0,
    #[doc = "1: SSEL0 not asserted."]
    NotAsserted = 1,
}
impl From<Txssel0N> for bool {
    #[inline(always)]
    fn from(variant: Txssel0N) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL0_N` writer - Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
pub type Txssel0NW<'a, REG> = crate::BitWriter<'a, REG, Txssel0N>;
impl<'a, REG> Txssel0NW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSEL0 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Txssel0N::Asserted)
    }
    #[doc = "SSEL0 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Txssel0N::NotAsserted)
    }
}
#[doc = "Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txssel1N {
    #[doc = "0: SSEL1 asserted."]
    Asserted = 0,
    #[doc = "1: SSEL1 not asserted."]
    NotAsserted = 1,
}
impl From<Txssel1N> for bool {
    #[inline(always)]
    fn from(variant: Txssel1N) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL1_N` writer - Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
pub type Txssel1NW<'a, REG> = crate::BitWriter<'a, REG, Txssel1N>;
impl<'a, REG> Txssel1NW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSEL1 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Txssel1N::Asserted)
    }
    #[doc = "SSEL1 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Txssel1N::NotAsserted)
    }
}
#[doc = "Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txssel2N {
    #[doc = "0: SSEL2 asserted."]
    Asserted = 0,
    #[doc = "1: SSEL2 not asserted."]
    NotAsserted = 1,
}
impl From<Txssel2N> for bool {
    #[inline(always)]
    fn from(variant: Txssel2N) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL2_N` writer - Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
pub type Txssel2NW<'a, REG> = crate::BitWriter<'a, REG, Txssel2N>;
impl<'a, REG> Txssel2NW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSEL2 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Txssel2N::Asserted)
    }
    #[doc = "SSEL2 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Txssel2N::NotAsserted)
    }
}
#[doc = "Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txssel3N {
    #[doc = "0: SSEL3 asserted."]
    Asserted = 0,
    #[doc = "1: SSEL3 not asserted."]
    NotAsserted = 1,
}
impl From<Txssel3N> for bool {
    #[inline(always)]
    fn from(variant: Txssel3N) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL3_N` writer - Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
pub type Txssel3NW<'a, REG> = crate::BitWriter<'a, REG, Txssel3N>;
impl<'a, REG> Txssel3NW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSEL3 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Txssel3N::Asserted)
    }
    #[doc = "SSEL3 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Txssel3N::NotAsserted)
    }
}
#[doc = "End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eot {
    #[doc = "0: SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    NotDeasserted = 0,
    #[doc = "1: SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    Deasserted = 1,
}
impl From<Eot> for bool {
    #[inline(always)]
    fn from(variant: Eot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` writer - End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG, Eot>;
impl<'a, REG> EotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    #[inline(always)]
    pub fn not_deasserted(self) -> &'a mut crate::W<REG> {
        self.variant(Eot::NotDeasserted)
    }
    #[doc = "SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut crate::W<REG> {
        self.variant(Eot::Deasserted)
    }
}
#[doc = "End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eof {
    #[doc = "0: Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    NotEof = 0,
    #[doc = "1: Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    Eof = 1,
}
impl From<Eof> for bool {
    #[inline(always)]
    fn from(variant: Eof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOF` writer - End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
pub type EofW<'a, REG> = crate::BitWriter<'a, REG, Eof>;
impl<'a, REG> EofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    #[inline(always)]
    pub fn not_eof(self) -> &'a mut crate::W<REG> {
        self.variant(Eof::NotEof)
    }
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    #[inline(always)]
    pub fn eof(self) -> &'a mut crate::W<REG> {
        self.variant(Eof::Eof)
    }
}
#[doc = "Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxignore {
    #[doc = "0: Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    Read = 0,
    #[doc = "1: Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    Ignore = 1,
}
impl From<Rxignore> for bool {
    #[inline(always)]
    fn from(variant: Rxignore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIGNORE` writer - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
pub type RxignoreW<'a, REG> = crate::BitWriter<'a, REG, Rxignore>;
impl<'a, REG> RxignoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Rxignore::Read)
    }
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(Rxignore::Ignore)
    }
}
#[doc = "Transmit Ignore\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txignore {
    #[doc = "0: Write transmit data. Transmit data must be written for each data exchange between master and slave. In slave mode, an underrun error occurs if transmit data is not provided before needed in a data frame."]
    Writetxdata = 0,
    #[doc = "1: Ignore transmit data. Data can be received without transmitting data (after FIFOWR has been initialized to set TXIGNORE). No transmitter flags are generated. When configured with TXIGNORE = 1, the slave sets the data to always 0."]
    Ignoretxdata = 1,
}
impl From<Txignore> for bool {
    #[inline(always)]
    fn from(variant: Txignore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIGNORE` writer - Transmit Ignore"]
pub type TxignoreW<'a, REG> = crate::BitWriter<'a, REG, Txignore>;
impl<'a, REG> TxignoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write transmit data. Transmit data must be written for each data exchange between master and slave. In slave mode, an underrun error occurs if transmit data is not provided before needed in a data frame."]
    #[inline(always)]
    pub fn writetxdata(self) -> &'a mut crate::W<REG> {
        self.variant(Txignore::Writetxdata)
    }
    #[doc = "Ignore transmit data. Data can be received without transmitting data (after FIFOWR has been initialized to set TXIGNORE). No transmitter flags are generated. When configured with TXIGNORE = 1, the slave sets the data to always 0."]
    #[inline(always)]
    pub fn ignoretxdata(self) -> &'a mut crate::W<REG> {
        self.variant(Txignore::Ignoretxdata)
    }
}
#[doc = "Field `LEN` writer - Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FifowrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit data to the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<FifowrSpec> {
        TxdataW::new(self, 0)
    }
    #[doc = "Bit 16 - Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    #[must_use]
    pub fn txssel0_n(&mut self) -> Txssel0NW<FifowrSpec> {
        Txssel0NW::new(self, 16)
    }
    #[doc = "Bit 17 - Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    #[must_use]
    pub fn txssel1_n(&mut self) -> Txssel1NW<FifowrSpec> {
        Txssel1NW::new(self, 17)
    }
    #[doc = "Bit 18 - Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    #[must_use]
    pub fn txssel2_n(&mut self) -> Txssel2NW<FifowrSpec> {
        Txssel2NW::new(self, 18)
    }
    #[doc = "Bit 19 - Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    #[must_use]
    pub fn txssel3_n(&mut self) -> Txssel3NW<FifowrSpec> {
        Txssel3NW::new(self, 19)
    }
    #[doc = "Bit 20 - End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline(always)]
    #[must_use]
    pub fn eot(&mut self) -> EotW<FifowrSpec> {
        EotW::new(self, 20)
    }
    #[doc = "Bit 21 - End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline(always)]
    #[must_use]
    pub fn eof(&mut self) -> EofW<FifowrSpec> {
        EofW::new(self, 21)
    }
    #[doc = "Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline(always)]
    #[must_use]
    pub fn rxignore(&mut self) -> RxignoreW<FifowrSpec> {
        RxignoreW::new(self, 22)
    }
    #[doc = "Bit 23 - Transmit Ignore"]
    #[inline(always)]
    #[must_use]
    pub fn txignore(&mut self) -> TxignoreW<FifowrSpec> {
        TxignoreW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<FifowrSpec> {
        LenW::new(self, 24)
    }
}
#[doc = "FIFO write data.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifowr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifowrSpec;
impl crate::RegisterSpec for FifowrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fifowr::W`](W) writer structure"]
impl crate::Writable for FifowrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOWR to value 0"]
impl crate::Resettable for FifowrSpec {
    const RESET_VALUE: u32 = 0;
}
