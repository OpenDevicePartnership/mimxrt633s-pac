#[doc = "Register `RAMUSE` reader"]
pub type R = crate::R<RamuseSpec>;
#[doc = "Register `RAMUSE` writer"]
pub type W = crate::W<RamuseSpec>;
#[doc = "Field `OFF` reader - This is the word offset into the RAM"]
pub type OffR = crate::FieldReader<u16>;
#[doc = "Field `OFF` writer - This is the word offset into the RAM"]
pub type OffW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Len {
    #[doc = "0: 4 bytes"]
    Len4 = 0,
    #[doc = "1: 8 bytes"]
    Len8 = 1,
    #[doc = "2: 16 bytes"]
    Len16 = 2,
    #[doc = "3: 32 bytes"]
    Len32 = 3,
    #[doc = "4: 64 bytes"]
    Len64 = 4,
    #[doc = "5: 128 bytes"]
    Len128 = 5,
    #[doc = "6: 256 bytes"]
    Len256 = 6,
    #[doc = "7: 512 bytes"]
    Len512 = 7,
}
impl From<Len> for u8 {
    #[inline(always)]
    fn from(variant: Len) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Len {
    type Ux = u8;
}
impl crate::IsEnum for Len {}
#[doc = "Field `LEN` reader - This is the length of the mailbox or mastering area as 4<<LEN per direction"]
pub type LenR = crate::FieldReader<Len>;
impl LenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Len {
        match self.bits {
            0 => Len::Len4,
            1 => Len::Len8,
            2 => Len::Len16,
            3 => Len::Len32,
            4 => Len::Len64,
            5 => Len::Len128,
            6 => Len::Len256,
            7 => Len::Len512,
            _ => unreachable!(),
        }
    }
    #[doc = "4 bytes"]
    #[inline(always)]
    pub fn is_len_4(&self) -> bool {
        *self == Len::Len4
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_len_8(&self) -> bool {
        *self == Len::Len8
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_len_16(&self) -> bool {
        *self == Len::Len16
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_len_32(&self) -> bool {
        *self == Len::Len32
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_len_64(&self) -> bool {
        *self == Len::Len64
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_len_128(&self) -> bool {
        *self == Len::Len128
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_len_256(&self) -> bool {
        *self == Len::Len256
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_len_512(&self) -> bool {
        *self == Len::Len512
    }
}
#[doc = "Field `LEN` writer - This is the length of the mailbox or mastering area as 4<<LEN per direction"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Len, crate::Safe>;
impl<'a, REG> LenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 bytes"]
    #[inline(always)]
    pub fn len_4(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len4)
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn len_8(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len8)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn len_16(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len16)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn len_32(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len32)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn len_64(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn len_128(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len128)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn len_256(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len256)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn len_512(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len512)
    }
}
impl R {
    #[doc = "Bits 0:11 - This is the word offset into the RAM"]
    #[inline(always)]
    pub fn off(&self) -> OffR {
        OffR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMUSE")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - This is the word offset into the RAM"]
    #[inline(always)]
    pub fn off(&mut self) -> OffW<RamuseSpec> {
        OffW::new(self, 0)
    }
    #[doc = "Bits 16:18 - This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<RamuseSpec> {
        LenW::new(self, 16)
    }
}
#[doc = "Port RAM base and size (for Mailbox and Bus Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`ramuse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramuse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamuseSpec;
impl crate::RegisterSpec for RamuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramuse::R`](R) reader structure"]
impl crate::Readable for RamuseSpec {}
#[doc = "`write(|w| ..)` method takes [`ramuse::W`](W) writer structure"]
impl crate::Writable for RamuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAMUSE to value 0"]
impl crate::Resettable for RamuseSpec {
    const RESET_VALUE: u32 = 0;
}
