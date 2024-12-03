#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: SHA1 is enabled"]
    Sha1 = 1,
    #[doc = "2: SHA2-256 is enabled"]
    Sha2_256 = 2,
    #[doc = "4: AES if available (see also CRYPTCFG register for more controls)"]
    Aes = 4,
    #[doc = "5: ICB-AES if available (see also CRYPTCFG register for more controls)"]
    IcbAes = 5,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `Mode` reader - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Disabled),
            1 => Some(Mode::Sha1),
            2 => Some(Mode::Sha2_256),
            4 => Some(Mode::Aes),
            5 => Some(Mode::IcbAes),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode::Disabled
    }
    #[doc = "SHA1 is enabled"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == Mode::Sha1
    }
    #[doc = "SHA2-256 is enabled"]
    #[inline(always)]
    pub fn is_sha2_256(&self) -> bool {
        *self == Mode::Sha2_256
    }
    #[doc = "AES if available (see also CRYPTCFG register for more controls)"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == Mode::Aes
    }
    #[doc = "ICB-AES if available (see also CRYPTCFG register for more controls)"]
    #[inline(always)]
    pub fn is_icb_aes(&self) -> bool {
        *self == Mode::IcbAes
    }
}
#[doc = "Field `Mode` writer - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Disabled)
    }
    #[doc = "SHA1 is enabled"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Sha1)
    }
    #[doc = "SHA2-256 is enabled"]
    #[inline(always)]
    pub fn sha2_256(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Sha2_256)
    }
    #[doc = "AES if available (see also CRYPTCFG register for more controls)"]
    #[inline(always)]
    pub fn aes(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Aes)
    }
    #[doc = "ICB-AES if available (see also CRYPTCFG register for more controls)"]
    #[inline(always)]
    pub fn icb_aes(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::IcbAes)
    }
}
#[doc = "Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NewHash {
    #[doc = "1: Starts a new Hash/Crypto and initializes the Digest/Result."]
    Start = 1,
}
impl From<NewHash> for bool {
    #[inline(always)]
    fn from(variant: NewHash) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `New_Hash` writer - Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
pub type NewHashW<'a, REG> = crate::BitWriter<'a, REG, NewHash>;
impl<'a, REG> NewHashW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Starts a new Hash/Crypto and initializes the Digest/Result."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(NewHash::Start)
    }
}
#[doc = "If 1, allows the SHA RELOAD registers to be used. This is used to save a partial Hash Digest (e.g. when need to run AES) and then reload it later for continuation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reload {
    #[doc = "1: Allow RELOAD registers to be used."]
    Reload = 1,
}
impl From<Reload> for bool {
    #[inline(always)]
    fn from(variant: Reload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELOAD` reader - If 1, allows the SHA RELOAD registers to be used. This is used to save a partial Hash Digest (e.g. when need to run AES) and then reload it later for continuation."]
pub type ReloadR = crate::BitReader<Reload>;
impl ReloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Reload> {
        match self.bits {
            true => Some(Reload::Reload),
            _ => None,
        }
    }
    #[doc = "Allow RELOAD registers to be used."]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == Reload::Reload
    }
}
#[doc = "Field `RELOAD` writer - If 1, allows the SHA RELOAD registers to be used. This is used to save a partial Hash Digest (e.g. when need to run AES) and then reload it later for continuation."]
pub type ReloadW<'a, REG> = crate::BitWriter<'a, REG, Reload>;
impl<'a, REG> ReloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow RELOAD registers to be used."]
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::Reload)
    }
}
#[doc = "Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaI {
    #[doc = "0: DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    NotUsed = 0,
    #[doc = "1: DMA will push in the data."]
    Push = 1,
}
impl From<DmaI> for bool {
    #[inline(always)]
    fn from(variant: DmaI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_I` reader - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
pub type DmaIR = crate::BitReader<DmaI>;
impl DmaIR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaI {
        match self.bits {
            false => DmaI::NotUsed,
            true => DmaI::Push,
        }
    }
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == DmaI::NotUsed
    }
    #[doc = "DMA will push in the data."]
    #[inline(always)]
    pub fn is_push(&self) -> bool {
        *self == DmaI::Push
    }
}
#[doc = "Field `DMA_I` writer - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
pub type DmaIW<'a, REG> = crate::BitWriter<'a, REG, DmaI>;
impl<'a, REG> DmaIW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut crate::W<REG> {
        self.variant(DmaI::NotUsed)
    }
    #[doc = "DMA will push in the data."]
    #[inline(always)]
    pub fn push(self) -> &'a mut crate::W<REG> {
        self.variant(DmaI::Push)
    }
}
#[doc = "Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaO {
    #[doc = "0: DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    Notused = 0,
}
impl From<DmaO> for bool {
    #[inline(always)]
    fn from(variant: DmaO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_O` reader - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
pub type DmaOR = crate::BitReader<DmaO>;
impl DmaOR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmaO> {
        match self.bits {
            false => Some(DmaO::Notused),
            _ => None,
        }
    }
    #[doc = "DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    #[inline(always)]
    pub fn is_notused(&self) -> bool {
        *self == DmaO::Notused
    }
}
#[doc = "Field `DMA_O` writer - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
pub type DmaOW<'a, REG> = crate::BitWriter<'a, REG, DmaO>;
impl<'a, REG> DmaOW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    #[inline(always)]
    pub fn notused(self) -> &'a mut crate::W<REG> {
        self.variant(DmaO::Notused)
    }
}
#[doc = "Field `HASHSWPB` reader - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
pub type HashswpbR = crate::BitReader;
#[doc = "Field `HASHSWPB` writer - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
pub type HashswpbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - If 1, allows the SHA RELOAD registers to be used. This is used to save a partial Hash Digest (e.g. when need to run AES) and then reload it later for continuation."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[inline(always)]
    pub fn dma_i(&self) -> DmaIR {
        DmaIR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[inline(always)]
    pub fn dma_o(&self) -> DmaOR {
        DmaOR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[inline(always)]
    pub fn hashswpb(&self) -> HashswpbR {
        HashswpbR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("mode", &self.mode())
            .field("reload", &self.reload())
            .field("dma_i", &self.dma_i())
            .field("dma_o", &self.dma_o())
            .field("hashswpb", &self.hashswpb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CtrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 4 - Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
    #[inline(always)]
    pub fn new_hash(&mut self) -> NewHashW<CtrlSpec> {
        NewHashW::new(self, 4)
    }
    #[doc = "Bit 5 - If 1, allows the SHA RELOAD registers to be used. This is used to save a partial Hash Digest (e.g. when need to run AES) and then reload it later for continuation."]
    #[inline(always)]
    pub fn reload(&mut self) -> ReloadW<CtrlSpec> {
        ReloadW::new(self, 5)
    }
    #[doc = "Bit 8 - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[inline(always)]
    pub fn dma_i(&mut self) -> DmaIW<CtrlSpec> {
        DmaIW::new(self, 8)
    }
    #[doc = "Bit 9 - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[inline(always)]
    pub fn dma_o(&mut self) -> DmaOW<CtrlSpec> {
        DmaOW::new(self, 9)
    }
    #[doc = "Bit 12 - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[inline(always)]
    pub fn hashswpb(&mut self) -> HashswpbW<CtrlSpec> {
        HashswpbW::new(self, 12)
    }
}
#[doc = "Control register to enable and operate Hash and Crypto\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
