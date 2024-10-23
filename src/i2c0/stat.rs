#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpending {
    #[doc = "0: In progress. Communication is in progress and the Master function is busy and cannot currently accept a command."]
    InProgress = 0,
    #[doc = "1: Pending. The Master function needs software service or is in the idle state. If the master is not in the idle state, it is waiting to receive or transmit data or the NACK bit."]
    Pending = 1,
}
impl From<Mstpending> for bool {
    #[inline(always)]
    fn from(variant: Mstpending) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPENDING` reader - Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt."]
pub type MstpendingR = crate::BitReader<Mstpending>;
impl MstpendingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpending {
        match self.bits {
            false => Mstpending::InProgress,
            true => Mstpending::Pending,
        }
    }
    #[doc = "In progress. Communication is in progress and the Master function is busy and cannot currently accept a command."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Mstpending::InProgress
    }
    #[doc = "Pending. The Master function needs software service or is in the idle state. If the master is not in the idle state, it is waiting to receive or transmit data or the NACK bit."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Mstpending::Pending
    }
}
#[doc = "Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved. See Table 400 for details of state values and appropriate responses.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mststate {
    #[doc = "0: Idle. The Master function is available to be used for a new transaction."]
    Idle = 0,
    #[doc = "1: Receive ready. Received data available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    ReceiveReady = 1,
    #[doc = "2: Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    TransmitReady = 2,
    #[doc = "3: NACK Address. Slave NACKed address."]
    NackAddress = 3,
    #[doc = "4: NACK Data. Slave NACKed transmitted data."]
    NackData = 4,
}
impl From<Mststate> for u8 {
    #[inline(always)]
    fn from(variant: Mststate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mststate {
    type Ux = u8;
}
impl crate::IsEnum for Mststate {}
#[doc = "Field `MSTSTATE` reader - Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved. See Table 400 for details of state values and appropriate responses."]
pub type MststateR = crate::FieldReader<Mststate>;
impl MststateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mststate> {
        match self.bits {
            0 => Some(Mststate::Idle),
            1 => Some(Mststate::ReceiveReady),
            2 => Some(Mststate::TransmitReady),
            3 => Some(Mststate::NackAddress),
            4 => Some(Mststate::NackData),
            _ => None,
        }
    }
    #[doc = "Idle. The Master function is available to be used for a new transaction."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Mststate::Idle
    }
    #[doc = "Receive ready. Received data available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    #[inline(always)]
    pub fn is_receive_ready(&self) -> bool {
        *self == Mststate::ReceiveReady
    }
    #[doc = "Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    #[inline(always)]
    pub fn is_transmit_ready(&self) -> bool {
        *self == Mststate::TransmitReady
    }
    #[doc = "NACK Address. Slave NACKed address."]
    #[inline(always)]
    pub fn is_nack_address(&self) -> bool {
        *self == Mststate::NackAddress
    }
    #[doc = "NACK Data. Slave NACKed transmitted data."]
    #[inline(always)]
    pub fn is_nack_data(&self) -> bool {
        *self == Mststate::NackData
    }
}
#[doc = "Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstarbloss {
    #[doc = "0: No Arbitration Loss has occurred."]
    NoLoss = 0,
    #[doc = "1: Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    ArbitrationLoss = 1,
}
impl From<Mstarbloss> for bool {
    #[inline(always)]
    fn from(variant: Mstarbloss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTARBLOSS` reader - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
pub type MstarblossR = crate::BitReader<Mstarbloss>;
impl MstarblossR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstarbloss {
        match self.bits {
            false => Mstarbloss::NoLoss,
            true => Mstarbloss::ArbitrationLoss,
        }
    }
    #[doc = "No Arbitration Loss has occurred."]
    #[inline(always)]
    pub fn is_no_loss(&self) -> bool {
        *self == Mstarbloss::NoLoss
    }
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    #[inline(always)]
    pub fn is_arbitration_loss(&self) -> bool {
        *self == Mstarbloss::ArbitrationLoss
    }
}
#[doc = "Field `MSTARBLOSS` writer - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
pub type MstarblossW<'a, REG> = crate::BitWriter<'a, REG, Mstarbloss>;
impl<'a, REG> MstarblossW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Arbitration Loss has occurred."]
    #[inline(always)]
    pub fn no_loss(self) -> &'a mut crate::W<REG> {
        self.variant(Mstarbloss::NoLoss)
    }
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    #[inline(always)]
    pub fn arbitration_loss(self) -> &'a mut crate::W<REG> {
        self.variant(Mstarbloss::ArbitrationLoss)
    }
}
#[doc = "Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstststperr {
    #[doc = "0: No Start/Stop Error has occurred."]
    NoError = 0,
    #[doc = "1: The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    Error = 1,
}
impl From<Mstststperr> for bool {
    #[inline(always)]
    fn from(variant: Mstststperr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTSTSTPERR` reader - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
pub type MstststperrR = crate::BitReader<Mstststperr>;
impl MstststperrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstststperr {
        match self.bits {
            false => Mstststperr::NoError,
            true => Mstststperr::Error,
        }
    }
    #[doc = "No Start/Stop Error has occurred."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Mstststperr::NoError
    }
    #[doc = "The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Mstststperr::Error
    }
}
#[doc = "Field `MSTSTSTPERR` writer - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
pub type MstststperrW<'a, REG> = crate::BitWriter<'a, REG, Mstststperr>;
impl<'a, REG> MstststperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Start/Stop Error has occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Mstststperr::NoError)
    }
    #[doc = "The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Mstststperr::Error)
    }
}
#[doc = "Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the SLVCTL register. The point in time when SlvPending is set depends on whether the I2C interface is in HSCAPABLE mode. See Section 25.7.2.2.2. When the I2C interface is configured to be HSCAPABLE, HS master codes are detected automatically. Due to the requirements of the HS I2C specification, slave addresses must also be detected automatically, since the address must be acknowledged before the clock can be stretched.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvpending {
    #[doc = "0: In progress. The Slave function does not currently need service."]
    InProgress = 0,
    #[doc = "1: Pending. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    Pending = 1,
}
impl From<Slvpending> for bool {
    #[inline(always)]
    fn from(variant: Slvpending) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVPENDING` reader - Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the SLVCTL register. The point in time when SlvPending is set depends on whether the I2C interface is in HSCAPABLE mode. See Section 25.7.2.2.2. When the I2C interface is configured to be HSCAPABLE, HS master codes are detected automatically. Due to the requirements of the HS I2C specification, slave addresses must also be detected automatically, since the address must be acknowledged before the clock can be stretched."]
pub type SlvpendingR = crate::BitReader<Slvpending>;
impl SlvpendingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvpending {
        match self.bits {
            false => Slvpending::InProgress,
            true => Slvpending::Pending,
        }
    }
    #[doc = "In progress. The Slave function does not currently need service."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Slvpending::InProgress
    }
    #[doc = "Pending. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Slvpending::Pending
    }
}
#[doc = "Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved. See Table 401 for state values and actions. note that the occurrence of some states and how they are handled are affected by DMA mode and Automatic Operation modes.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slvstate {
    #[doc = "0: Slave address. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    SlaveAddress = 0,
    #[doc = "1: Slave receive. Received data is available (Slave Receiver mode)."]
    SlaveReceive = 1,
    #[doc = "2: Slave transmit. Data can be transmitted (Slave Transmitter mode)."]
    SlaveTransmit = 2,
}
impl From<Slvstate> for u8 {
    #[inline(always)]
    fn from(variant: Slvstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slvstate {
    type Ux = u8;
}
impl crate::IsEnum for Slvstate {}
#[doc = "Field `SLVSTATE` reader - Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved. See Table 401 for state values and actions. note that the occurrence of some states and how they are handled are affected by DMA mode and Automatic Operation modes."]
pub type SlvstateR = crate::FieldReader<Slvstate>;
impl SlvstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Slvstate> {
        match self.bits {
            0 => Some(Slvstate::SlaveAddress),
            1 => Some(Slvstate::SlaveReceive),
            2 => Some(Slvstate::SlaveTransmit),
            _ => None,
        }
    }
    #[doc = "Slave address. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    #[inline(always)]
    pub fn is_slave_address(&self) -> bool {
        *self == Slvstate::SlaveAddress
    }
    #[doc = "Slave receive. Received data is available (Slave Receiver mode)."]
    #[inline(always)]
    pub fn is_slave_receive(&self) -> bool {
        *self == Slvstate::SlaveReceive
    }
    #[doc = "Slave transmit. Data can be transmitted (Slave Transmitter mode)."]
    #[inline(always)]
    pub fn is_slave_transmit(&self) -> bool {
        *self == Slvstate::SlaveTransmit
    }
}
#[doc = "Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvnotstr {
    #[doc = "0: Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    Stretching = 0,
    #[doc = "1: Not stretching. The slave function is not currently stretching the I 2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    NotStretching = 1,
}
impl From<Slvnotstr> for bool {
    #[inline(always)]
    fn from(variant: Slvnotstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVNOTSTR` reader - Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
pub type SlvnotstrR = crate::BitReader<Slvnotstr>;
impl SlvnotstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvnotstr {
        match self.bits {
            false => Slvnotstr::Stretching,
            true => Slvnotstr::NotStretching,
        }
    }
    #[doc = "Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    #[inline(always)]
    pub fn is_stretching(&self) -> bool {
        *self == Slvnotstr::Stretching
    }
    #[doc = "Not stretching. The slave function is not currently stretching the I 2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    #[inline(always)]
    pub fn is_not_stretching(&self) -> bool {
        *self == Slvnotstr::NotStretching
    }
}
#[doc = "Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slvidx {
    #[doc = "0: Address 0. Slave address 0 was matched."]
    Address0 = 0,
    #[doc = "1: Address 1. Slave address 1 was matched."]
    Address1 = 1,
    #[doc = "2: Address 2. Slave address 2 was matched."]
    Address2 = 2,
    #[doc = "3: Address 3. Slave address 3 was matched."]
    Address3 = 3,
}
impl From<Slvidx> for u8 {
    #[inline(always)]
    fn from(variant: Slvidx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slvidx {
    type Ux = u8;
}
impl crate::IsEnum for Slvidx {}
#[doc = "Field `SLVIDX` reader - Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
pub type SlvidxR = crate::FieldReader<Slvidx>;
impl SlvidxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvidx {
        match self.bits {
            0 => Slvidx::Address0,
            1 => Slvidx::Address1,
            2 => Slvidx::Address2,
            3 => Slvidx::Address3,
            _ => unreachable!(),
        }
    }
    #[doc = "Address 0. Slave address 0 was matched."]
    #[inline(always)]
    pub fn is_address0(&self) -> bool {
        *self == Slvidx::Address0
    }
    #[doc = "Address 1. Slave address 1 was matched."]
    #[inline(always)]
    pub fn is_address1(&self) -> bool {
        *self == Slvidx::Address1
    }
    #[doc = "Address 2. Slave address 2 was matched."]
    #[inline(always)]
    pub fn is_address2(&self) -> bool {
        *self == Slvidx::Address2
    }
    #[doc = "Address 3. Slave address 3 was matched."]
    #[inline(always)]
    pub fn is_address3(&self) -> bool {
        *self == Slvidx::Address3
    }
}
#[doc = "Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address, or when the address has been automatically acknowledged. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, when there is a Stop detected on the bus, when the master NACKs slave data, and in some combinations of Automatic Operation. SLVSEL is not cleared if software NACKs data.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvsel {
    #[doc = "0: Not selected. The Slave function is not currently selected."]
    NotSelected = 0,
    #[doc = "1: Selected. The Slave function is currently selected."]
    Selected = 1,
}
impl From<Slvsel> for bool {
    #[inline(always)]
    fn from(variant: Slvsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVSEL` reader - Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address, or when the address has been automatically acknowledged. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, when there is a Stop detected on the bus, when the master NACKs slave data, and in some combinations of Automatic Operation. SLVSEL is not cleared if software NACKs data."]
pub type SlvselR = crate::BitReader<Slvsel>;
impl SlvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvsel {
        match self.bits {
            false => Slvsel::NotSelected,
            true => Slvsel::Selected,
        }
    }
    #[doc = "Not selected. The Slave function is not currently selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == Slvsel::NotSelected
    }
    #[doc = "Selected. The Slave function is currently selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Slvsel::Selected
    }
}
#[doc = "Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvdesel {
    #[doc = "0: Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    NotDeselected = 0,
    #[doc = "1: Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    Deselected = 1,
}
impl From<Slvdesel> for bool {
    #[inline(always)]
    fn from(variant: Slvdesel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVDESEL` reader - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
pub type SlvdeselR = crate::BitReader<Slvdesel>;
impl SlvdeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvdesel {
        match self.bits {
            false => Slvdesel::NotDeselected,
            true => Slvdesel::Deselected,
        }
    }
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    #[inline(always)]
    pub fn is_not_deselected(&self) -> bool {
        *self == Slvdesel::NotDeselected
    }
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    #[inline(always)]
    pub fn is_deselected(&self) -> bool {
        *self == Slvdesel::Deselected
    }
}
#[doc = "Field `SLVDESEL` writer - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
pub type SlvdeselW<'a, REG> = crate::BitWriter<'a, REG, Slvdesel>;
impl<'a, REG> SlvdeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    #[inline(always)]
    pub fn not_deselected(self) -> &'a mut crate::W<REG> {
        self.variant(Slvdesel::NotDeselected)
    }
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    #[inline(always)]
    pub fn deselected(self) -> &'a mut crate::W<REG> {
        self.variant(Slvdesel::Deselected)
    }
}
#[doc = "Monitor Ready. This flag is cleared when the MONRXDAT register is read.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monrdy {
    #[doc = "0: No data. The Monitor function does not currently have data available."]
    NoData = 0,
    #[doc = "1: Data waiting. The Monitor function has data waiting to be read."]
    DataWaiting = 1,
}
impl From<Monrdy> for bool {
    #[inline(always)]
    fn from(variant: Monrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONRDY` reader - Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
pub type MonrdyR = crate::BitReader<Monrdy>;
impl MonrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monrdy {
        match self.bits {
            false => Monrdy::NoData,
            true => Monrdy::DataWaiting,
        }
    }
    #[doc = "No data. The Monitor function does not currently have data available."]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Monrdy::NoData
    }
    #[doc = "Data waiting. The Monitor function has data waiting to be read."]
    #[inline(always)]
    pub fn is_data_waiting(&self) -> bool {
        *self == Monrdy::DataWaiting
    }
}
#[doc = "Monitor Overflow flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monov {
    #[doc = "0: No overrun. Monitor data has not overrun."]
    NoOverrun = 0,
    #[doc = "1: Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    Overrun = 1,
}
impl From<Monov> for bool {
    #[inline(always)]
    fn from(variant: Monov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONOV` reader - Monitor Overflow flag."]
pub type MonovR = crate::BitReader<Monov>;
impl MonovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monov {
        match self.bits {
            false => Monov::NoOverrun,
            true => Monov::Overrun,
        }
    }
    #[doc = "No overrun. Monitor data has not overrun."]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == Monov::NoOverrun
    }
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == Monov::Overrun
    }
}
#[doc = "Field `MONOV` writer - Monitor Overflow flag."]
pub type MonovW<'a, REG> = crate::BitWriter<'a, REG, Monov>;
impl<'a, REG> MonovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun. Monitor data has not overrun."]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Monov::NoOverrun)
    }
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Monov::Overrun)
    }
}
#[doc = "Monitor Active flag. Indicates when the Monitor function considers the I 2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monactive {
    #[doc = "0: Inactive. The Monitor function considers the I2C bus to be inactive."]
    Inactive = 0,
    #[doc = "1: Active. The Monitor function considers the I2C bus to be active."]
    Active = 1,
}
impl From<Monactive> for bool {
    #[inline(always)]
    fn from(variant: Monactive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONACTIVE` reader - Monitor Active flag. Indicates when the Monitor function considers the I 2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
pub type MonactiveR = crate::BitReader<Monactive>;
impl MonactiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monactive {
        match self.bits {
            false => Monactive::Inactive,
            true => Monactive::Active,
        }
    }
    #[doc = "Inactive. The Monitor function considers the I2C bus to be inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Monactive::Inactive
    }
    #[doc = "Active. The Monitor function considers the I2C bus to be active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Monactive::Active
    }
}
#[doc = "Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monidle {
    #[doc = "0: Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    NotIdle = 0,
    #[doc = "1: Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    Idle = 1,
}
impl From<Monidle> for bool {
    #[inline(always)]
    fn from(variant: Monidle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONIDLE` reader - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
pub type MonidleR = crate::BitReader<Monidle>;
impl MonidleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monidle {
        match self.bits {
            false => Monidle::NotIdle,
            true => Monidle::Idle,
        }
    }
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    #[inline(always)]
    pub fn is_not_idle(&self) -> bool {
        *self == Monidle::NotIdle
    }
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Monidle::Idle
    }
}
#[doc = "Field `MONIDLE` writer - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
pub type MonidleW<'a, REG> = crate::BitWriter<'a, REG, Monidle>;
impl<'a, REG> MonidleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    #[inline(always)]
    pub fn not_idle(self) -> &'a mut crate::W<REG> {
        self.variant(Monidle::NotIdle)
    }
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Monidle::Idle)
    }
}
#[doc = "Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eventtimeout {
    #[doc = "0: No time-out. I2C bus events have not caused a time-out."]
    NoTimeout = 0,
    #[doc = "1: Event time-out. The time between I2C bus events has been longer than the time specified by the TIMEOUT register."]
    EvenTimeout = 1,
}
impl From<Eventtimeout> for bool {
    #[inline(always)]
    fn from(variant: Eventtimeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTTIMEOUT` reader - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
pub type EventtimeoutR = crate::BitReader<Eventtimeout>;
impl EventtimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eventtimeout {
        match self.bits {
            false => Eventtimeout::NoTimeout,
            true => Eventtimeout::EvenTimeout,
        }
    }
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == Eventtimeout::NoTimeout
    }
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the TIMEOUT register."]
    #[inline(always)]
    pub fn is_even_timeout(&self) -> bool {
        *self == Eventtimeout::EvenTimeout
    }
}
#[doc = "Field `EVENTTIMEOUT` writer - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
pub type EventtimeoutW<'a, REG> = crate::BitWriter<'a, REG, Eventtimeout>;
impl<'a, REG> EventtimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Eventtimeout::NoTimeout)
    }
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the TIMEOUT register."]
    #[inline(always)]
    pub fn even_timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Eventtimeout::EvenTimeout)
    }
}
#[doc = "SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scltimeout {
    #[doc = "0: No time-out. SCL low time has not caused a time-out."]
    NoTimeout = 0,
    #[doc = "1: Time-out. SCL low time has caused a time-out."]
    Timeout = 1,
}
impl From<Scltimeout> for bool {
    #[inline(always)]
    fn from(variant: Scltimeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLTIMEOUT` reader - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
pub type ScltimeoutR = crate::BitReader<Scltimeout>;
impl ScltimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scltimeout {
        match self.bits {
            false => Scltimeout::NoTimeout,
            true => Scltimeout::Timeout,
        }
    }
    #[doc = "No time-out. SCL low time has not caused a time-out."]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == Scltimeout::NoTimeout
    }
    #[doc = "Time-out. SCL low time has caused a time-out."]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == Scltimeout::Timeout
    }
}
#[doc = "Field `SCLTIMEOUT` writer - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
pub type ScltimeoutW<'a, REG> = crate::BitWriter<'a, REG, Scltimeout>;
impl<'a, REG> ScltimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No time-out. SCL low time has not caused a time-out."]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Scltimeout::NoTimeout)
    }
    #[doc = "Time-out. SCL low time has caused a time-out."]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Scltimeout::Timeout)
    }
}
impl R {
    #[doc = "Bit 0 - Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt."]
    #[inline(always)]
    pub fn mstpending(&self) -> MstpendingR {
        MstpendingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved. See Table 400 for details of state values and appropriate responses."]
    #[inline(always)]
    pub fn mststate(&self) -> MststateR {
        MststateR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn mstarbloss(&self) -> MstarblossR {
        MstarblossR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn mstststperr(&self) -> MstststperrR {
        MstststperrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the SLVCTL register. The point in time when SlvPending is set depends on whether the I2C interface is in HSCAPABLE mode. See Section 25.7.2.2.2. When the I2C interface is configured to be HSCAPABLE, HS master codes are detected automatically. Due to the requirements of the HS I2C specification, slave addresses must also be detected automatically, since the address must be acknowledged before the clock can be stretched."]
    #[inline(always)]
    pub fn slvpending(&self) -> SlvpendingR {
        SlvpendingR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved. See Table 401 for state values and actions. note that the occurrence of some states and how they are handled are affected by DMA mode and Automatic Operation modes."]
    #[inline(always)]
    pub fn slvstate(&self) -> SlvstateR {
        SlvstateR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[inline(always)]
    pub fn slvnotstr(&self) -> SlvnotstrR {
        SlvnotstrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[inline(always)]
    pub fn slvidx(&self) -> SlvidxR {
        SlvidxR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address, or when the address has been automatically acknowledged. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, when there is a Stop detected on the bus, when the master NACKs slave data, and in some combinations of Automatic Operation. SLVSEL is not cleared if software NACKs data."]
    #[inline(always)]
    pub fn slvsel(&self) -> SlvselR {
        SlvselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn slvdesel(&self) -> SlvdeselR {
        SlvdeselR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[inline(always)]
    pub fn monrdy(&self) -> MonrdyR {
        MonrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    pub fn monov(&self) -> MonovR {
        MonovR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Monitor Active flag. Indicates when the Monitor function considers the I 2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[inline(always)]
    pub fn monactive(&self) -> MonactiveR {
        MonactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn monidle(&self) -> MonidleR {
        MonidleR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline(always)]
    pub fn eventtimeout(&self) -> EventtimeoutR {
        EventtimeoutR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn scltimeout(&self) -> ScltimeoutR {
        ScltimeoutR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("mstpending", &self.mstpending())
            .field("mststate", &self.mststate())
            .field("mstarbloss", &self.mstarbloss())
            .field("mstststperr", &self.mstststperr())
            .field("slvpending", &self.slvpending())
            .field("slvstate", &self.slvstate())
            .field("slvnotstr", &self.slvnotstr())
            .field("slvidx", &self.slvidx())
            .field("slvsel", &self.slvsel())
            .field("slvdesel", &self.slvdesel())
            .field("monrdy", &self.monrdy())
            .field("monov", &self.monov())
            .field("monactive", &self.monactive())
            .field("monidle", &self.monidle())
            .field("eventtimeout", &self.eventtimeout())
            .field("scltimeout", &self.scltimeout())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    #[must_use]
    pub fn mstarbloss(&mut self) -> MstarblossW<StatSpec> {
        MstarblossW::new(self, 4)
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    #[must_use]
    pub fn mstststperr(&mut self) -> MstststperrW<StatSpec> {
        MstststperrW::new(self, 6)
    }
    #[doc = "Bit 15 - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn slvdesel(&mut self) -> SlvdeselW<StatSpec> {
        SlvdeselW::new(self, 15)
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    #[must_use]
    pub fn monov(&mut self) -> MonovW<StatSpec> {
        MonovW::new(self, 17)
    }
    #[doc = "Bit 19 - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn monidle(&mut self) -> MonidleW<StatSpec> {
        MonidleW::new(self, 19)
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline(always)]
    #[must_use]
    pub fn eventtimeout(&mut self) -> EventtimeoutW<StatSpec> {
        EventtimeoutW::new(self, 24)
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn scltimeout(&mut self) -> ScltimeoutW<StatSpec> {
        ScltimeoutW::new(self, 25)
    }
}
#[doc = "Status register for Master, Slave, and Monitor functions.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x0801"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x0801;
}
