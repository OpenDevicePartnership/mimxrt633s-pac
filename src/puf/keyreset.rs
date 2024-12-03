#[doc = "Register `KEYRESET` writer"]
pub type W = crate::W<KeyresetSpec>;
#[doc = "Key 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key0 {
    #[doc = "2: Reset KEY0 Hold register and SHIFT_STATUS\\[KEY0\\]."]
    Reset = 2,
}
impl From<Key0> for u8 {
    #[inline(always)]
    fn from(variant: Key0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key0 {
    type Ux = u8;
}
impl crate::IsEnum for Key0 {}
#[doc = "Field `KEY0` writer - Key 0"]
pub type Key0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key0>;
impl<'a, REG> Key0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset KEY0 Hold register and SHIFT_STATUS\\[KEY0\\]."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Key0::Reset)
    }
}
#[doc = "Key 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key1 {
    #[doc = "2: Reset KEY1 Hold register and SHIFT_STATUS\\[KEY1\\]."]
    Reset = 2,
}
impl From<Key1> for u8 {
    #[inline(always)]
    fn from(variant: Key1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key1 {
    type Ux = u8;
}
impl crate::IsEnum for Key1 {}
#[doc = "Field `KEY1` writer - Key 1"]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key1>;
impl<'a, REG> Key1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset KEY1 Hold register and SHIFT_STATUS\\[KEY1\\]."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Key1::Reset)
    }
}
#[doc = "Key 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key2 {
    #[doc = "2: Reset KEY2 Hold register and SHIFT_STATUS\\[KEY2\\]."]
    Reset = 2,
}
impl From<Key2> for u8 {
    #[inline(always)]
    fn from(variant: Key2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key2 {
    type Ux = u8;
}
impl crate::IsEnum for Key2 {}
#[doc = "Field `KEY2` writer - Key 2"]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key2>;
impl<'a, REG> Key2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset KEY2 Hold register and SHIFT_STATUS\\[KEY2\\]."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Key2::Reset)
    }
}
#[doc = "Key 3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key3 {
    #[doc = "2: Reset KEY3 Hold register and SHIFT_STATUS\\[KEY3\\]."]
    Reset = 2,
}
impl From<Key3> for u8 {
    #[inline(always)]
    fn from(variant: Key3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key3 {
    type Ux = u8;
}
impl crate::IsEnum for Key3 {}
#[doc = "Field `KEY3` writer - Key 3"]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key3>;
impl<'a, REG> Key3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset KEY3 Hold register and SHIFT_STATUS\\[KEY3\\]."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Key3::Reset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<KeyresetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:1 - Key 0"]
    #[inline(always)]
    pub fn key0(&mut self) -> Key0W<KeyresetSpec> {
        Key0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Key 1"]
    #[inline(always)]
    pub fn key1(&mut self) -> Key1W<KeyresetSpec> {
        Key1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Key 2"]
    #[inline(always)]
    pub fn key2(&mut self) -> Key2W<KeyresetSpec> {
        Key2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Key 3"]
    #[inline(always)]
    pub fn key3(&mut self) -> Key3W<KeyresetSpec> {
        Key3W::new(self, 6)
    }
}
#[doc = "Key Reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyreset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyresetSpec;
impl crate::RegisterSpec for KeyresetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyreset::W`](W) writer structure"]
impl crate::Writable for KeyresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYRESET to value 0"]
impl crate::Resettable for KeyresetSpec {
    const RESET_VALUE: u32 = 0;
}
