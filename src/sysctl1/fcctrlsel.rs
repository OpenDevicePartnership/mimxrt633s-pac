#[doc = "Register `FCCTRLSEL%s` reader"]
pub type R = crate::R<FcctrlselSpec>;
#[doc = "Register `FCCTRLSEL%s` writer"]
pub type W = crate::W<FcctrlselSpec>;
#[doc = "SCK IN Select. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sckinsel {
    #[doc = "0: Original FLEXCOMM I2S signals"]
    OrigFlexI2sSignals = 0,
    #[doc = "1: Shared Set0 I2S signals."]
    SharedSet0I2sSignals = 1,
    #[doc = "2: Shared Set1 I2S signals."]
    SharedSet1I2sSignals = 2,
}
impl From<Sckinsel> for u8 {
    #[inline(always)]
    fn from(variant: Sckinsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sckinsel {
    type Ux = u8;
}
impl crate::IsEnum for Sckinsel {}
#[doc = "Field `SCKINSEL` reader - SCK IN Select. . ."]
pub type SckinselR = crate::FieldReader<Sckinsel>;
impl SckinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sckinsel> {
        match self.bits {
            0 => Some(Sckinsel::OrigFlexI2sSignals),
            1 => Some(Sckinsel::SharedSet0I2sSignals),
            2 => Some(Sckinsel::SharedSet1I2sSignals),
            _ => None,
        }
    }
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == Sckinsel::OrigFlexI2sSignals
    }
    #[doc = "Shared Set0 I2S signals."]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == Sckinsel::SharedSet0I2sSignals
    }
    #[doc = "Shared Set1 I2S signals."]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == Sckinsel::SharedSet1I2sSignals
    }
}
#[doc = "Field `SCKINSEL` writer - SCK IN Select. . ."]
pub type SckinselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sckinsel>;
impl<'a, REG> SckinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Sckinsel::OrigFlexI2sSignals)
    }
    #[doc = "Shared Set0 I2S signals."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Sckinsel::SharedSet0I2sSignals)
    }
    #[doc = "Shared Set1 I2S signals."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Sckinsel::SharedSet1I2sSignals)
    }
}
#[doc = "WS IN Select. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wsinsel {
    #[doc = "0: Original FLEXCOMM I2S signals"]
    OrigFlexI2sSignals = 0,
    #[doc = "1: Shared Set0 I2S signals."]
    SharedSet0I2sSignals = 1,
    #[doc = "2: Shared Set1 I2S signals."]
    SharedSet1I2sSignals = 2,
}
impl From<Wsinsel> for u8 {
    #[inline(always)]
    fn from(variant: Wsinsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wsinsel {
    type Ux = u8;
}
impl crate::IsEnum for Wsinsel {}
#[doc = "Field `WSINSEL` reader - WS IN Select. . ."]
pub type WsinselR = crate::FieldReader<Wsinsel>;
impl WsinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wsinsel> {
        match self.bits {
            0 => Some(Wsinsel::OrigFlexI2sSignals),
            1 => Some(Wsinsel::SharedSet0I2sSignals),
            2 => Some(Wsinsel::SharedSet1I2sSignals),
            _ => None,
        }
    }
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == Wsinsel::OrigFlexI2sSignals
    }
    #[doc = "Shared Set0 I2S signals."]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == Wsinsel::SharedSet0I2sSignals
    }
    #[doc = "Shared Set1 I2S signals."]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == Wsinsel::SharedSet1I2sSignals
    }
}
#[doc = "Field `WSINSEL` writer - WS IN Select. . ."]
pub type WsinselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wsinsel>;
impl<'a, REG> WsinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Wsinsel::OrigFlexI2sSignals)
    }
    #[doc = "Shared Set0 I2S signals."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Wsinsel::SharedSet0I2sSignals)
    }
    #[doc = "Shared Set1 I2S signals."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Wsinsel::SharedSet1I2sSignals)
    }
}
#[doc = "DATA IN Select. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datainsel {
    #[doc = "0: Original FLEXCOMM I2S signals"]
    OrigFlexI2sSignals = 0,
    #[doc = "1: Shared Set0 I2S signals."]
    SharedSet0I2sSignals = 1,
    #[doc = "2: Shared Set1 I2S signals."]
    SharedSet1I2sSignals = 2,
}
impl From<Datainsel> for u8 {
    #[inline(always)]
    fn from(variant: Datainsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datainsel {
    type Ux = u8;
}
impl crate::IsEnum for Datainsel {}
#[doc = "Field `DATAINSEL` reader - DATA IN Select. . ."]
pub type DatainselR = crate::FieldReader<Datainsel>;
impl DatainselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datainsel> {
        match self.bits {
            0 => Some(Datainsel::OrigFlexI2sSignals),
            1 => Some(Datainsel::SharedSet0I2sSignals),
            2 => Some(Datainsel::SharedSet1I2sSignals),
            _ => None,
        }
    }
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == Datainsel::OrigFlexI2sSignals
    }
    #[doc = "Shared Set0 I2S signals."]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == Datainsel::SharedSet0I2sSignals
    }
    #[doc = "Shared Set1 I2S signals."]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == Datainsel::SharedSet1I2sSignals
    }
}
#[doc = "Field `DATAINSEL` writer - DATA IN Select. . ."]
pub type DatainselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datainsel>;
impl<'a, REG> DatainselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Datainsel::OrigFlexI2sSignals)
    }
    #[doc = "Shared Set0 I2S signals."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Datainsel::SharedSet0I2sSignals)
    }
    #[doc = "Shared Set1 I2S signals."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Datainsel::SharedSet1I2sSignals)
    }
}
#[doc = "DATA OUT Select. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dataoutsel {
    #[doc = "0: Original FLEXCOMM I2S signals"]
    OrigFlexI2sSignals = 0,
    #[doc = "1: Shared Set0 I2S signals."]
    SharedSet0I2sSignals = 1,
    #[doc = "2: Shared Set1 I2S signals."]
    SharedSet1I2sSignals = 2,
}
impl From<Dataoutsel> for u8 {
    #[inline(always)]
    fn from(variant: Dataoutsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dataoutsel {
    type Ux = u8;
}
impl crate::IsEnum for Dataoutsel {}
#[doc = "Field `DATAOUTSEL` reader - DATA OUT Select. . ."]
pub type DataoutselR = crate::FieldReader<Dataoutsel>;
impl DataoutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dataoutsel> {
        match self.bits {
            0 => Some(Dataoutsel::OrigFlexI2sSignals),
            1 => Some(Dataoutsel::SharedSet0I2sSignals),
            2 => Some(Dataoutsel::SharedSet1I2sSignals),
            _ => None,
        }
    }
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == Dataoutsel::OrigFlexI2sSignals
    }
    #[doc = "Shared Set0 I2S signals."]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == Dataoutsel::SharedSet0I2sSignals
    }
    #[doc = "Shared Set1 I2S signals."]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == Dataoutsel::SharedSet1I2sSignals
    }
}
#[doc = "Field `DATAOUTSEL` writer - DATA OUT Select. . ."]
pub type DataoutselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dataoutsel>;
impl<'a, REG> DataoutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Dataoutsel::OrigFlexI2sSignals)
    }
    #[doc = "Shared Set0 I2S signals."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Dataoutsel::SharedSet0I2sSignals)
    }
    #[doc = "Shared Set1 I2S signals."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut crate::W<REG> {
        self.variant(Dataoutsel::SharedSet1I2sSignals)
    }
}
impl R {
    #[doc = "Bits 0:1 - SCK IN Select. . ."]
    #[inline(always)]
    pub fn sckinsel(&self) -> SckinselR {
        SckinselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - WS IN Select. . ."]
    #[inline(always)]
    pub fn wsinsel(&self) -> WsinselR {
        WsinselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DATA IN Select. . ."]
    #[inline(always)]
    pub fn datainsel(&self) -> DatainselR {
        DatainselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DATA OUT Select. . ."]
    #[inline(always)]
    pub fn dataoutsel(&self) -> DataoutselR {
        DataoutselR::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCTRLSEL")
            .field("sckinsel", &self.sckinsel())
            .field("wsinsel", &self.wsinsel())
            .field("datainsel", &self.datainsel())
            .field("dataoutsel", &self.dataoutsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SCK IN Select. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sckinsel(&mut self) -> SckinselW<FcctrlselSpec> {
        SckinselW::new(self, 0)
    }
    #[doc = "Bits 8:9 - WS IN Select. . ."]
    #[inline(always)]
    #[must_use]
    pub fn wsinsel(&mut self) -> WsinselW<FcctrlselSpec> {
        WsinselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - DATA IN Select. . ."]
    #[inline(always)]
    #[must_use]
    pub fn datainsel(&mut self) -> DatainselW<FcctrlselSpec> {
        DatainselW::new(self, 16)
    }
    #[doc = "Bits 24:25 - DATA OUT Select. . ."]
    #[inline(always)]
    #[must_use]
    pub fn dataoutsel(&mut self) -> DataoutselW<FcctrlselSpec> {
        DataoutselW::new(self, 24)
    }
}
#[doc = "flexcomm control selection N\n\nYou can [`read`](crate::Reg::read) this register and get [`fcctrlsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcctrlsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcctrlselSpec;
impl crate::RegisterSpec for FcctrlselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcctrlsel::R`](R) reader structure"]
impl crate::Readable for FcctrlselSpec {}
#[doc = "`write(|w| ..)` method takes [`fcctrlsel::W`](W) writer structure"]
impl crate::Writable for FcctrlselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCCTRLSEL%s to value 0"]
impl crate::Resettable for FcctrlselSpec {
    const RESET_VALUE: u32 = 0;
}
