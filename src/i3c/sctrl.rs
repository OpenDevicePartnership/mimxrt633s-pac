#[doc = "Register `SCTRL` reader"]
pub type R = crate::R<SctrlSpec>;
#[doc = "Register `SCTRL` writer"]
pub type W = crate::W<SctrlSpec>;
#[doc = "EVENT\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Event {
    #[doc = "0: NORMAL_MODE: If EVENT is set to 0 after was a non-0 value, event processing will cancel if the event processing has not yet started; if event processing has already been started, then event processing will not be be cancelled."]
    NormalMode = 0,
    #[doc = "1: IBI: Start an In-Band Interrupt. This will try to push an IBI interrupt onto the I3C bus. If data is associated with the IBI, then the data will be read from the SCTRL.IBIDATA field. If time control is enabled, then this data will also include any time control-related bytes; additionally, the IBIDATA byte will have bit 7 set to 1 automatically (as is required for time control). The IBI interrupt will occur after the 1st (mandatory) IBIDATA, if any."]
    Ibi = 1,
    #[doc = "2: MASTER_REQUEST: Start a Master-Request."]
    MasterRequest = 2,
    #[doc = "3: HOT_JOIN_REQUEST: Start a Hot-Join request. A Hot-Join Request should only be used when the device is powered on after the I3C bus is already powered up, or when the device is connected using hot insertion methods (the device is powered up when it is physically inserted onto the powered-up I3C bus). The hot join will wait for Bus Idle, and SCTRL.EVENT=HOT_JOIN_REQUEST must be set before the slave enable (SCONFIG.SLVENA)."]
    HotJoinRequest = 3,
}
impl From<Event> for u8 {
    #[inline(always)]
    fn from(variant: Event) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Event {
    type Ux = u8;
}
impl crate::IsEnum for Event {}
#[doc = "Field `EVENT` reader - EVENT"]
pub type EventR = crate::FieldReader<Event>;
impl EventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Event {
        match self.bits {
            0 => Event::NormalMode,
            1 => Event::Ibi,
            2 => Event::MasterRequest,
            3 => Event::HotJoinRequest,
            _ => unreachable!(),
        }
    }
    #[doc = "NORMAL_MODE: If EVENT is set to 0 after was a non-0 value, event processing will cancel if the event processing has not yet started; if event processing has already been started, then event processing will not be be cancelled."]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == Event::NormalMode
    }
    #[doc = "IBI: Start an In-Band Interrupt. This will try to push an IBI interrupt onto the I3C bus. If data is associated with the IBI, then the data will be read from the SCTRL.IBIDATA field. If time control is enabled, then this data will also include any time control-related bytes; additionally, the IBIDATA byte will have bit 7 set to 1 automatically (as is required for time control). The IBI interrupt will occur after the 1st (mandatory) IBIDATA, if any."]
    #[inline(always)]
    pub fn is_ibi(&self) -> bool {
        *self == Event::Ibi
    }
    #[doc = "MASTER_REQUEST: Start a Master-Request."]
    #[inline(always)]
    pub fn is_master_request(&self) -> bool {
        *self == Event::MasterRequest
    }
    #[doc = "HOT_JOIN_REQUEST: Start a Hot-Join request. A Hot-Join Request should only be used when the device is powered on after the I3C bus is already powered up, or when the device is connected using hot insertion methods (the device is powered up when it is physically inserted onto the powered-up I3C bus). The hot join will wait for Bus Idle, and SCTRL.EVENT=HOT_JOIN_REQUEST must be set before the slave enable (SCONFIG.SLVENA)."]
    #[inline(always)]
    pub fn is_hot_join_request(&self) -> bool {
        *self == Event::HotJoinRequest
    }
}
#[doc = "Field `EVENT` writer - EVENT"]
pub type EventW<'a, REG> = crate::FieldWriter<'a, REG, 2, Event, crate::Safe>;
impl<'a, REG> EventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NORMAL_MODE: If EVENT is set to 0 after was a non-0 value, event processing will cancel if the event processing has not yet started; if event processing has already been started, then event processing will not be be cancelled."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Event::NormalMode)
    }
    #[doc = "IBI: Start an In-Band Interrupt. This will try to push an IBI interrupt onto the I3C bus. If data is associated with the IBI, then the data will be read from the SCTRL.IBIDATA field. If time control is enabled, then this data will also include any time control-related bytes; additionally, the IBIDATA byte will have bit 7 set to 1 automatically (as is required for time control). The IBI interrupt will occur after the 1st (mandatory) IBIDATA, if any."]
    #[inline(always)]
    pub fn ibi(self) -> &'a mut crate::W<REG> {
        self.variant(Event::Ibi)
    }
    #[doc = "MASTER_REQUEST: Start a Master-Request."]
    #[inline(always)]
    pub fn master_request(self) -> &'a mut crate::W<REG> {
        self.variant(Event::MasterRequest)
    }
    #[doc = "HOT_JOIN_REQUEST: Start a Hot-Join request. A Hot-Join Request should only be used when the device is powered on after the I3C bus is already powered up, or when the device is connected using hot insertion methods (the device is powered up when it is physically inserted onto the powered-up I3C bus). The hot join will wait for Bus Idle, and SCTRL.EVENT=HOT_JOIN_REQUEST must be set before the slave enable (SCONFIG.SLVENA)."]
    #[inline(always)]
    pub fn hot_join_request(self) -> &'a mut crate::W<REG> {
        self.variant(Event::HotJoinRequest)
    }
}
#[doc = "Field `IBIDATA` reader - In-Band Interrupt data"]
pub type IbidataR = crate::FieldReader;
#[doc = "Field `IBIDATA` writer - In-Band Interrupt data"]
pub type IbidataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PENDINT` reader - Pending interrupt"]
pub type PendintR = crate::FieldReader;
#[doc = "Field `PENDINT` writer - Pending interrupt"]
pub type PendintW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACTSTATE` reader - Activity state (of slave)"]
pub type ActstateR = crate::FieldReader;
#[doc = "Field `ACTSTATE` writer - Activity state (of slave)"]
pub type ActstateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VENDINFO` reader - Vendor information"]
pub type VendinfoR = crate::FieldReader;
#[doc = "Field `VENDINFO` writer - Vendor information"]
pub type VendinfoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - EVENT"]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - In-Band Interrupt data"]
    #[inline(always)]
    pub fn ibidata(&self) -> IbidataR {
        IbidataR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Pending interrupt"]
    #[inline(always)]
    pub fn pendint(&self) -> PendintR {
        PendintR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Activity state (of slave)"]
    #[inline(always)]
    pub fn actstate(&self) -> ActstateR {
        ActstateR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Vendor information"]
    #[inline(always)]
    pub fn vendinfo(&self) -> VendinfoR {
        VendinfoR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCTRL")
            .field("event", &self.event())
            .field("ibidata", &self.ibidata())
            .field("pendint", &self.pendint())
            .field("actstate", &self.actstate())
            .field("vendinfo", &self.vendinfo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - EVENT"]
    #[inline(always)]
    #[must_use]
    pub fn event(&mut self) -> EventW<SctrlSpec> {
        EventW::new(self, 0)
    }
    #[doc = "Bits 8:15 - In-Band Interrupt data"]
    #[inline(always)]
    #[must_use]
    pub fn ibidata(&mut self) -> IbidataW<SctrlSpec> {
        IbidataW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pendint(&mut self) -> PendintW<SctrlSpec> {
        PendintW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Activity state (of slave)"]
    #[inline(always)]
    #[must_use]
    pub fn actstate(&mut self) -> ActstateW<SctrlSpec> {
        ActstateW::new(self, 20)
    }
    #[doc = "Bits 24:31 - Vendor information"]
    #[inline(always)]
    #[must_use]
    pub fn vendinfo(&mut self) -> VendinfoW<SctrlSpec> {
        VendinfoW::new(self, 24)
    }
}
#[doc = "Slave Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SctrlSpec;
impl crate::RegisterSpec for SctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sctrl::R`](R) reader structure"]
impl crate::Readable for SctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sctrl::W`](W) writer structure"]
impl crate::Writable for SctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCTRL to value 0"]
impl crate::Resettable for SctrlSpec {
    const RESET_VALUE: u32 = 0;
}
