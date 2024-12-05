#[doc = "Register `TCTRL[%s]` reader"]
pub type R = crate::R<TctrlSpec>;
#[doc = "Register `TCTRL[%s]` writer"]
pub type W = crate::W<TctrlSpec>;
#[doc = "Trigger enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hten {
    #[doc = "0: Hardware trigger source disabled"]
    Hten0 = 0,
    #[doc = "1: Hardware trigger source enabled"]
    Hten1 = 1,
}
impl From<Hten> for bool {
    #[inline(always)]
    fn from(variant: Hten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTEN` reader - Trigger enable"]
pub type HtenR = crate::BitReader<Hten>;
impl HtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hten {
        match self.bits {
            false => Hten::Hten0,
            true => Hten::Hten1,
        }
    }
    #[doc = "Hardware trigger source disabled"]
    #[inline(always)]
    pub fn is_hten_0(&self) -> bool {
        *self == Hten::Hten0
    }
    #[doc = "Hardware trigger source enabled"]
    #[inline(always)]
    pub fn is_hten_1(&self) -> bool {
        *self == Hten::Hten1
    }
}
#[doc = "Field `HTEN` writer - Trigger enable"]
pub type HtenW<'a, REG> = crate::BitWriter<'a, REG, Hten>;
impl<'a, REG> HtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware trigger source disabled"]
    #[inline(always)]
    pub fn hten_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hten::Hten0)
    }
    #[doc = "Hardware trigger source enabled"]
    #[inline(always)]
    pub fn hten_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hten::Hten1)
    }
}
#[doc = "Trigger priority setting\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tpri {
    #[doc = "0: Set to highest priority, Level 1"]
    Tpri0 = 0,
    #[doc = "1: Set to corresponding priority level"]
    Tpri1 = 1,
    #[doc = "2: Set to corresponding priority level"]
    Tpri2 = 2,
    #[doc = "3: Set to corresponding priority level"]
    Tpri3 = 3,
    #[doc = "4: Set to corresponding priority level"]
    Tpri4 = 4,
    #[doc = "5: Set to corresponding priority level"]
    Tpri5 = 5,
    #[doc = "6: Set to corresponding priority level"]
    Tpri6 = 6,
    #[doc = "7: Set to corresponding priority level"]
    Tpri7 = 7,
    #[doc = "8: Set to corresponding priority level"]
    Tpri8 = 8,
    #[doc = "9: Set to corresponding priority level"]
    Tpri9 = 9,
    #[doc = "15: Set to lowest priority, Level 16"]
    Tpri15 = 15,
}
impl From<Tpri> for u8 {
    #[inline(always)]
    fn from(variant: Tpri) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tpri {
    type Ux = u8;
}
impl crate::IsEnum for Tpri {}
#[doc = "Field `TPRI` reader - Trigger priority setting"]
pub type TpriR = crate::FieldReader<Tpri>;
impl TpriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tpri> {
        match self.bits {
            0 => Some(Tpri::Tpri0),
            1 => Some(Tpri::Tpri1),
            2 => Some(Tpri::Tpri2),
            3 => Some(Tpri::Tpri3),
            4 => Some(Tpri::Tpri4),
            5 => Some(Tpri::Tpri5),
            6 => Some(Tpri::Tpri6),
            7 => Some(Tpri::Tpri7),
            8 => Some(Tpri::Tpri8),
            9 => Some(Tpri::Tpri9),
            15 => Some(Tpri::Tpri15),
            _ => None,
        }
    }
    #[doc = "Set to highest priority, Level 1"]
    #[inline(always)]
    pub fn is_tpri_0(&self) -> bool {
        *self == Tpri::Tpri0
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn is_tpri_1(&self) -> bool {
        *self == Tpri::Tpri1
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn is_tpri_2(&self) -> bool {
        *self == Tpri::Tpri2
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn is_tpri_3(&self) -> bool {
        *self == Tpri::Tpri3
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn is_tpri_4(&self) -> bool {
        *self == Tpri::Tpri4
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn is_tpri_5(&self) -> bool {
        *self == Tpri::Tpri5
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn is_tpri_6(&self) -> bool {
        *self == Tpri::Tpri6
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn is_tpri_7(&self) -> bool {
        *self == Tpri::Tpri7
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn is_tpri_8(&self) -> bool {
        *self == Tpri::Tpri8
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn is_tpri_9(&self) -> bool {
        *self == Tpri::Tpri9
    }
    #[doc = "Set to lowest priority, Level 16"]
    #[inline(always)]
    pub fn is_tpri_15(&self) -> bool {
        *self == Tpri::Tpri15
    }
}
#[doc = "Field `TPRI` writer - Trigger priority setting"]
pub type TpriW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tpri>;
impl<'a, REG> TpriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set to highest priority, Level 1"]
    #[inline(always)]
    pub fn tpri_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri0)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri1)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_2(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri2)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_3(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri3)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_4(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri4)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_5(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri5)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_6(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri6)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_7(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri7)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_8(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri8)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_9(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri9)
    }
    #[doc = "Set to lowest priority, Level 16"]
    #[inline(always)]
    pub fn tpri_15(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::Tpri15)
    }
}
#[doc = "Field `TDLY` reader - Trigger delay select"]
pub type TdlyR = crate::FieldReader;
#[doc = "Field `TDLY` writer - Trigger delay select"]
pub type TdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Trigger command select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcmd {
    #[doc = "0: Not a valid selection from the command buffer. Trigger event is ignored."]
    Tcmd0 = 0,
    #[doc = "1: CMD1 is executed"]
    Tcmd1 = 1,
    #[doc = "2: Corresponding CMD is executed"]
    Tcmd2 = 2,
    #[doc = "3: Corresponding CMD is executed"]
    Tcmd3 = 3,
    #[doc = "4: Corresponding CMD is executed"]
    Tcmd4 = 4,
    #[doc = "5: Corresponding CMD is executed"]
    Tcmd5 = 5,
    #[doc = "6: Corresponding CMD is executed"]
    Tcmd6 = 6,
    #[doc = "7: Corresponding CMD is executed"]
    Tcmd7 = 7,
    #[doc = "8: Corresponding CMD is executed"]
    Tcmd8 = 8,
    #[doc = "9: Corresponding CMD is executed"]
    Tcmd9 = 9,
    #[doc = "15: CMD15 is executed"]
    Tcmd15 = 15,
}
impl From<Tcmd> for u8 {
    #[inline(always)]
    fn from(variant: Tcmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcmd {
    type Ux = u8;
}
impl crate::IsEnum for Tcmd {}
#[doc = "Field `TCMD` reader - Trigger command select"]
pub type TcmdR = crate::FieldReader<Tcmd>;
impl TcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tcmd> {
        match self.bits {
            0 => Some(Tcmd::Tcmd0),
            1 => Some(Tcmd::Tcmd1),
            2 => Some(Tcmd::Tcmd2),
            3 => Some(Tcmd::Tcmd3),
            4 => Some(Tcmd::Tcmd4),
            5 => Some(Tcmd::Tcmd5),
            6 => Some(Tcmd::Tcmd6),
            7 => Some(Tcmd::Tcmd7),
            8 => Some(Tcmd::Tcmd8),
            9 => Some(Tcmd::Tcmd9),
            15 => Some(Tcmd::Tcmd15),
            _ => None,
        }
    }
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    #[inline(always)]
    pub fn is_tcmd_0(&self) -> bool {
        *self == Tcmd::Tcmd0
    }
    #[doc = "CMD1 is executed"]
    #[inline(always)]
    pub fn is_tcmd_1(&self) -> bool {
        *self == Tcmd::Tcmd1
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn is_tcmd_2(&self) -> bool {
        *self == Tcmd::Tcmd2
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn is_tcmd_3(&self) -> bool {
        *self == Tcmd::Tcmd3
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn is_tcmd_4(&self) -> bool {
        *self == Tcmd::Tcmd4
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn is_tcmd_5(&self) -> bool {
        *self == Tcmd::Tcmd5
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn is_tcmd_6(&self) -> bool {
        *self == Tcmd::Tcmd6
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn is_tcmd_7(&self) -> bool {
        *self == Tcmd::Tcmd7
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn is_tcmd_8(&self) -> bool {
        *self == Tcmd::Tcmd8
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn is_tcmd_9(&self) -> bool {
        *self == Tcmd::Tcmd9
    }
    #[doc = "CMD15 is executed"]
    #[inline(always)]
    pub fn is_tcmd_15(&self) -> bool {
        *self == Tcmd::Tcmd15
    }
}
#[doc = "Field `TCMD` writer - Trigger command select"]
pub type TcmdW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tcmd>;
impl<'a, REG> TcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    #[inline(always)]
    pub fn tcmd_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd0)
    }
    #[doc = "CMD1 is executed"]
    #[inline(always)]
    pub fn tcmd_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd1)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_2(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd2)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_3(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd3)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_4(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd4)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_5(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd5)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_6(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd6)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_7(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd7)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_8(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd8)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_9(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd9)
    }
    #[doc = "CMD15 is executed"]
    #[inline(always)]
    pub fn tcmd_15(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmd::Tcmd15)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger enable"]
    #[inline(always)]
    pub fn hten(&self) -> HtenR {
        HtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Trigger priority setting"]
    #[inline(always)]
    pub fn tpri(&self) -> TpriR {
        TpriR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Trigger delay select"]
    #[inline(always)]
    pub fn tdly(&self) -> TdlyR {
        TdlyR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Trigger command select"]
    #[inline(always)]
    pub fn tcmd(&self) -> TcmdR {
        TcmdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCTRL")
            .field("hten", &self.hten())
            .field("tpri", &self.tpri())
            .field("tdly", &self.tdly())
            .field("tcmd", &self.tcmd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Trigger enable"]
    #[inline(always)]
    pub fn hten(&mut self) -> HtenW<TctrlSpec> {
        HtenW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Trigger priority setting"]
    #[inline(always)]
    pub fn tpri(&mut self) -> TpriW<TctrlSpec> {
        TpriW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Trigger delay select"]
    #[inline(always)]
    pub fn tdly(&mut self) -> TdlyW<TctrlSpec> {
        TdlyW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Trigger command select"]
    #[inline(always)]
    pub fn tcmd(&mut self) -> TcmdW<TctrlSpec> {
        TcmdW::new(self, 24)
    }
}
#[doc = "Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TctrlSpec;
impl crate::RegisterSpec for TctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tctrl::R`](R) reader structure"]
impl crate::Readable for TctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tctrl::W`](W) writer structure"]
impl crate::Writable for TctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCTRL[%s]
to value 0"]
impl crate::Resettable for TctrlSpec {
    const RESET_VALUE: u32 = 0;
}
