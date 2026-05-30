//! Linux Bluetooth socket types and constants.

#![allow(unsafe_code)]

use crate::backend::c;
use crate::backend::net::read_sockaddr::{
    read_sockaddr_hci, read_sockaddr_l2cap, read_sockaddr_rfcomm, read_sockaddr_sco,
};
use crate::net::addr::{call_with_sockaddr, SocketAddrArg, SocketAddrLen, SocketAddrOpaque};
use crate::net::{Protocol, RawProtocol, SocketAddrAny};
use bitflags::bitflags;

const fn new_raw_protocol(u: u32) -> Protocol {
    match RawProtocol::new(u) {
        Some(p) => Protocol::from_raw(p),
        None => panic!("new_raw_protocol: protocol must be non-zero"),
    }
}

/// `BTPROTO_L2CAP`.
///
/// This is `None` because Linux defines `BTPROTO_L2CAP` as `0`, and rustix
/// represents protocol zero with `None`.
pub const L2CAP: Option<Protocol> = None;
/// `BTPROTO_HCI`.
pub const HCI: Option<Protocol> = Some(new_raw_protocol(c::BTPROTO_HCI as _));
/// `BTPROTO_SCO`.
pub const SCO: Option<Protocol> = Some(new_raw_protocol(c::BTPROTO_SCO as _));
/// `BTPROTO_RFCOMM`.
pub const RFCOMM: Option<Protocol> = Some(new_raw_protocol(c::BTPROTO_RFCOMM as _));
/// `BTPROTO_BNEP`.
pub const BNEP: Option<Protocol> = Some(new_raw_protocol(c::BTPROTO_BNEP as _));
/// `BTPROTO_CMTP`.
pub const CMTP: Option<Protocol> = Some(new_raw_protocol(c::BTPROTO_CMTP as _));
/// `BTPROTO_HIDP`.
pub const HIDP: Option<Protocol> = Some(new_raw_protocol(c::BTPROTO_HIDP as _));
/// `BTPROTO_AVDTP`.
pub const AVDTP: Option<Protocol> = Some(new_raw_protocol(c::BTPROTO_AVDTP as _));

/// A Bluetooth device address.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct BdAddr([u8; 6]);

impl BdAddr {
    /// The number of bytes in a Bluetooth device address.
    pub const LEN: usize = 6;

    /// Construct a Bluetooth device address from its kernel-order bytes.
    #[inline]
    pub const fn new(bytes: [u8; Self::LEN]) -> Self {
        Self(bytes)
    }

    /// Return the kernel-order bytes.
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; Self::LEN] {
        &self.0
    }

    /// Return the kernel-order bytes.
    #[inline]
    pub const fn into_bytes(self) -> [u8; Self::LEN] {
        self.0
    }

    #[inline]
    const fn encode(self) -> c::bdaddr_t {
        c::bdaddr_t { b: self.0 }
    }
}

impl From<[u8; 6]> for BdAddr {
    #[inline]
    fn from(bytes: [u8; 6]) -> Self {
        Self::new(bytes)
    }
}

impl From<BdAddr> for [u8; 6] {
    #[inline]
    fn from(addr: BdAddr) -> Self {
        addr.into_bytes()
    }
}

impl From<c::bdaddr_t> for BdAddr {
    #[inline]
    fn from(addr: c::bdaddr_t) -> Self {
        Self(addr.b)
    }
}

/// Bluetooth device address type.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct BdAddrType(u8);

impl BdAddrType {
    /// `BDADDR_BREDR`.
    pub const BREDR: Self = Self(c::BDADDR_BREDR as _);
    /// `BDADDR_LE_PUBLIC`.
    pub const LE_PUBLIC: Self = Self(c::BDADDR_LE_PUBLIC as _);
    /// `BDADDR_LE_RANDOM`.
    pub const LE_RANDOM: Self = Self(c::BDADDR_LE_RANDOM as _);

    /// Construct a `BdAddrType` from a raw integer.
    #[inline]
    pub const fn from_raw(raw: u8) -> Self {
        Self(raw)
    }

    /// Return the raw integer.
    #[inline]
    pub const fn as_raw(self) -> u8 {
        self.0
    }
}

/// HCI device id.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct HciDevice(u16);

impl HciDevice {
    /// `HCI_DEV_NONE`.
    pub const NONE: Self = Self(c::HCI_DEV_NONE as _);

    /// Construct an HCI device id from a raw integer.
    #[inline]
    pub const fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    /// Return the raw integer.
    #[inline]
    pub const fn as_raw(self) -> u16 {
        self.0
    }
}

/// HCI socket channel.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct HciChannel(u16);

impl HciChannel {
    /// `HCI_CHANNEL_RAW`.
    pub const RAW: Self = Self(c::HCI_CHANNEL_RAW as _);
    /// `HCI_CHANNEL_USER`.
    pub const USER: Self = Self(c::HCI_CHANNEL_USER as _);
    /// `HCI_CHANNEL_MONITOR`.
    pub const MONITOR: Self = Self(c::HCI_CHANNEL_MONITOR as _);
    /// `HCI_CHANNEL_CONTROL`.
    pub const CONTROL: Self = Self(c::HCI_CHANNEL_CONTROL as _);
    /// `HCI_CHANNEL_LOGGING`.
    pub const LOGGING: Self = Self(c::HCI_CHANNEL_LOGGING as _);

    /// Construct an HCI channel from a raw integer.
    #[inline]
    pub const fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    /// Return the raw integer.
    #[inline]
    pub const fn as_raw(self) -> u16 {
        self.0
    }
}

/// A Bluetooth HCI socket address.
///
/// Not ABI compatible with `struct sockaddr_hci`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SocketAddrHci {
    device: HciDevice,
    channel: HciChannel,
}

impl SocketAddrHci {
    /// Construct an HCI socket address.
    #[inline]
    pub const fn new(device: HciDevice, channel: HciChannel) -> Self {
        Self { device, channel }
    }

    /// Return the HCI device id.
    #[inline]
    pub const fn device(&self) -> HciDevice {
        self.device
    }

    /// Set the HCI device id.
    #[inline]
    pub fn set_device(&mut self, device: HciDevice) {
        self.device = device;
    }

    /// Return the HCI channel.
    #[inline]
    pub const fn channel(&self) -> HciChannel {
        self.channel
    }

    /// Set the HCI channel.
    #[inline]
    pub fn set_channel(&mut self, channel: HciChannel) {
        self.channel = channel;
    }
}

unsafe impl SocketAddrArg for SocketAddrHci {
    unsafe fn with_sockaddr<R>(
        &self,
        f: impl FnOnce(*const SocketAddrOpaque, SocketAddrLen) -> R,
    ) -> R {
        let addr = c::sockaddr_hci {
            hci_family: c::AF_BLUETOOTH as _,
            hci_dev: self.device.as_raw(),
            hci_channel: self.channel.as_raw(),
        };
        call_with_sockaddr(&addr, f)
    }
}

impl From<SocketAddrHci> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddrHci) -> Self {
        from.as_any()
    }
}

impl TryFrom<SocketAddrAny> for SocketAddrHci {
    type Error = crate::io::Errno;

    #[inline]
    fn try_from(addr: SocketAddrAny) -> Result<Self, Self::Error> {
        read_sockaddr_hci(&addr)
    }
}

/// A Bluetooth L2CAP socket address.
///
/// Not ABI compatible with `struct sockaddr_l2`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SocketAddrL2cap {
    addr: BdAddr,
    addr_type: BdAddrType,
    psm: u16,
    cid: u16,
}

impl SocketAddrL2cap {
    /// Construct an L2CAP socket address.
    #[inline]
    pub const fn new(addr: BdAddr, addr_type: BdAddrType, psm: u16, cid: u16) -> Self {
        Self {
            addr,
            addr_type,
            psm,
            cid,
        }
    }

    /// Return the Bluetooth device address.
    #[inline]
    pub const fn addr(&self) -> BdAddr {
        self.addr
    }

    /// Set the Bluetooth device address.
    #[inline]
    pub fn set_addr(&mut self, addr: BdAddr) {
        self.addr = addr;
    }

    /// Return the Bluetooth device address type.
    #[inline]
    pub const fn addr_type(&self) -> BdAddrType {
        self.addr_type
    }

    /// Set the Bluetooth device address type.
    #[inline]
    pub fn set_addr_type(&mut self, addr_type: BdAddrType) {
        self.addr_type = addr_type;
    }

    /// Return the L2CAP PSM in host byte order.
    #[inline]
    pub const fn psm(&self) -> u16 {
        self.psm
    }

    /// Set the L2CAP PSM in host byte order.
    #[inline]
    pub fn set_psm(&mut self, psm: u16) {
        self.psm = psm;
    }

    /// Return the L2CAP CID in host byte order.
    #[inline]
    pub const fn cid(&self) -> u16 {
        self.cid
    }

    /// Set the L2CAP CID in host byte order.
    #[inline]
    pub fn set_cid(&mut self, cid: u16) {
        self.cid = cid;
    }
}

unsafe impl SocketAddrArg for SocketAddrL2cap {
    #[allow(unused_unsafe)]
    unsafe fn with_sockaddr<R>(
        &self,
        f: impl FnOnce(*const SocketAddrOpaque, SocketAddrLen) -> R,
    ) -> R {
        let mut addr = unsafe { core::mem::MaybeUninit::<c::sockaddr_l2>::zeroed().assume_init() };
        addr.l2_family = c::AF_BLUETOOTH as _;
        addr.l2_psm = self.psm.to_le();
        addr.l2_bdaddr = self.addr.encode();
        addr.l2_cid = self.cid.to_le();
        addr.l2_bdaddr_type = self.addr_type.as_raw();
        call_with_sockaddr(&addr, f)
    }
}

impl From<SocketAddrL2cap> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddrL2cap) -> Self {
        from.as_any()
    }
}

impl TryFrom<SocketAddrAny> for SocketAddrL2cap {
    type Error = crate::io::Errno;

    #[inline]
    fn try_from(addr: SocketAddrAny) -> Result<Self, Self::Error> {
        read_sockaddr_l2cap(&addr)
    }
}

/// A Bluetooth SCO socket address.
///
/// Not ABI compatible with `struct sockaddr_sco`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SocketAddrSco {
    addr: BdAddr,
}

impl SocketAddrSco {
    /// Construct a SCO socket address.
    #[inline]
    pub const fn new(addr: BdAddr) -> Self {
        Self { addr }
    }

    /// Return the Bluetooth device address.
    #[inline]
    pub const fn addr(&self) -> BdAddr {
        self.addr
    }

    /// Set the Bluetooth device address.
    #[inline]
    pub fn set_addr(&mut self, addr: BdAddr) {
        self.addr = addr;
    }
}

unsafe impl SocketAddrArg for SocketAddrSco {
    unsafe fn with_sockaddr<R>(
        &self,
        f: impl FnOnce(*const SocketAddrOpaque, SocketAddrLen) -> R,
    ) -> R {
        let addr = c::sockaddr_sco {
            sco_family: c::AF_BLUETOOTH as _,
            sco_bdaddr: self.addr.encode(),
        };
        call_with_sockaddr(&addr, f)
    }
}

impl From<SocketAddrSco> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddrSco) -> Self {
        from.as_any()
    }
}

impl TryFrom<SocketAddrAny> for SocketAddrSco {
    type Error = crate::io::Errno;

    #[inline]
    fn try_from(addr: SocketAddrAny) -> Result<Self, Self::Error> {
        read_sockaddr_sco(&addr)
    }
}

/// A Bluetooth RFCOMM socket address.
///
/// Not ABI compatible with `struct sockaddr_rc`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SocketAddrRfcomm {
    addr: BdAddr,
    channel: u8,
}

impl SocketAddrRfcomm {
    /// Construct an RFCOMM socket address.
    #[inline]
    pub const fn new(addr: BdAddr, channel: u8) -> Self {
        Self { addr, channel }
    }

    /// Return the Bluetooth device address.
    #[inline]
    pub const fn addr(&self) -> BdAddr {
        self.addr
    }

    /// Set the Bluetooth device address.
    #[inline]
    pub fn set_addr(&mut self, addr: BdAddr) {
        self.addr = addr;
    }

    /// Return the RFCOMM channel.
    #[inline]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    /// Set the RFCOMM channel.
    #[inline]
    pub fn set_channel(&mut self, channel: u8) {
        self.channel = channel;
    }
}

unsafe impl SocketAddrArg for SocketAddrRfcomm {
    #[allow(unused_unsafe)]
    unsafe fn with_sockaddr<R>(
        &self,
        f: impl FnOnce(*const SocketAddrOpaque, SocketAddrLen) -> R,
    ) -> R {
        let mut addr = unsafe { core::mem::MaybeUninit::<c::sockaddr_rc>::zeroed().assume_init() };
        addr.rc_family = c::AF_BLUETOOTH as _;
        addr.rc_bdaddr = self.addr.encode();
        addr.rc_channel = self.channel;
        call_with_sockaddr(&addr, f)
    }
}

impl From<SocketAddrRfcomm> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddrRfcomm) -> Self {
        from.as_any()
    }
}

impl TryFrom<SocketAddrAny> for SocketAddrRfcomm {
    type Error = crate::io::Errno;

    #[inline]
    fn try_from(addr: SocketAddrAny) -> Result<Self, Self::Error> {
        read_sockaddr_rfcomm(&addr)
    }
}

/// The userspace `HCI_FILTER` socket option payload.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct HciFilter {
    /// Packet type mask.
    pub type_mask: u32,
    /// Event mask.
    pub event_mask: [u32; 2],
    /// Opcode in host byte order.
    pub opcode: u16,
}

impl HciFilter {
    #[inline]
    pub(crate) const fn from_raw(raw: c::hci_ufilter) -> Self {
        Self {
            type_mask: raw.type_mask,
            event_mask: raw.event_mask,
            opcode: u16::from_le(raw.opcode),
        }
    }

    #[inline]
    pub(crate) fn to_raw(self) -> core::mem::MaybeUninit<c::hci_ufilter> {
        let mut raw = core::mem::MaybeUninit::<c::hci_ufilter>::zeroed();
        unsafe {
            let raw = raw.as_mut_ptr();
            core::ptr::addr_of_mut!((*raw).type_mask).write(self.type_mask);
            core::ptr::addr_of_mut!((*raw).event_mask).write(self.event_mask);
            core::ptr::addr_of_mut!((*raw).opcode).write(self.opcode.to_le());
        }
        raw
    }
}

/// `struct bt_security`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct BluetoothSecurity {
    /// Security level.
    pub level: BluetoothSecurityLevel,
    /// Key size.
    pub key_size: u8,
}

/// `BT_SECURITY_*`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct BluetoothSecurityLevel(u8);

impl BluetoothSecurityLevel {
    /// `BT_SECURITY_SDP`.
    pub const SDP: Self = Self(c::BT_SECURITY_SDP as _);
    /// `BT_SECURITY_LOW`.
    pub const LOW: Self = Self(c::BT_SECURITY_LOW as _);
    /// `BT_SECURITY_MEDIUM`.
    pub const MEDIUM: Self = Self(c::BT_SECURITY_MEDIUM as _);
    /// `BT_SECURITY_HIGH`.
    pub const HIGH: Self = Self(c::BT_SECURITY_HIGH as _);
    /// `BT_SECURITY_FIPS`.
    pub const FIPS: Self = Self(c::BT_SECURITY_FIPS as _);

    /// Construct a `BluetoothSecurityLevel` from a raw integer.
    #[inline]
    pub const fn from_raw(raw: u8) -> Self {
        Self(raw)
    }

    /// Return the raw integer.
    #[inline]
    pub const fn as_raw(self) -> u8 {
        self.0
    }
}

/// `struct bt_power`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct BluetoothPower {
    /// Whether to force active mode.
    pub force_active: u8,
}

impl BluetoothPower {
    /// `BT_POWER_FORCE_ACTIVE_OFF`.
    pub const FORCE_ACTIVE_OFF: Self = Self {
        force_active: c::BT_POWER_FORCE_ACTIVE_OFF as _,
    };
    /// `BT_POWER_FORCE_ACTIVE_ON`.
    pub const FORCE_ACTIVE_ON: Self = Self {
        force_active: c::BT_POWER_FORCE_ACTIVE_ON as _,
    };
}

/// `BT_CHANNEL_POLICY_*`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct BluetoothChannelPolicy(u32);

impl BluetoothChannelPolicy {
    /// `BT_CHANNEL_POLICY_BREDR_ONLY`.
    pub const BREDR_ONLY: Self = Self(c::BT_CHANNEL_POLICY_BREDR_ONLY as _);
    /// `BT_CHANNEL_POLICY_BREDR_PREFERRED`.
    pub const BREDR_PREFERRED: Self = Self(c::BT_CHANNEL_POLICY_BREDR_PREFERRED as _);
    /// `BT_CHANNEL_POLICY_AMP_PREFERRED`.
    pub const AMP_PREFERRED: Self = Self(c::BT_CHANNEL_POLICY_AMP_PREFERRED as _);

    /// Construct a `BluetoothChannelPolicy` from a raw integer.
    #[inline]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Return the raw integer.
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

/// `struct bt_voice`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct BluetoothVoice {
    /// Voice setting.
    pub setting: BluetoothVoiceSetting,
}

impl BluetoothVoice {
    /// `BT_VOICE_TRANSPARENT`.
    pub const TRANSPARENT: Self = Self {
        setting: BluetoothVoiceSetting::TRANSPARENT,
    };
    /// `BT_VOICE_CVSD_16BIT`.
    pub const CVSD_16BIT: Self = Self {
        setting: BluetoothVoiceSetting::CVSD_16BIT,
    };
    /// `BT_VOICE_TRANSPARENT_16BIT`.
    pub const TRANSPARENT_16BIT: Self = Self {
        setting: BluetoothVoiceSetting::TRANSPARENT_16BIT,
    };
}

/// `BT_VOICE_*`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct BluetoothVoiceSetting(u16);

impl BluetoothVoiceSetting {
    /// `BT_VOICE_TRANSPARENT`.
    pub const TRANSPARENT: Self = Self(c::BT_VOICE_TRANSPARENT as _);
    /// `BT_VOICE_CVSD_16BIT`.
    pub const CVSD_16BIT: Self = Self(c::BT_VOICE_CVSD_16BIT as _);
    /// `BT_VOICE_TRANSPARENT_16BIT`.
    pub const TRANSPARENT_16BIT: Self = Self(c::BT_VOICE_TRANSPARENT_16BIT as _);

    /// Construct a `BluetoothVoiceSetting` from a raw integer.
    #[inline]
    pub const fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    /// Return the raw integer.
    #[inline]
    pub const fn as_raw(self) -> u16 {
        self.0
    }
}

bitflags! {
    /// `BT_PHY_*`.
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    #[repr(transparent)]
    pub struct BluetoothPhy: u32 {
        /// `BT_PHY_BR_1M_1SLOT`.
        const BR_1M_1SLOT = c::BT_PHY_BR_1M_1SLOT;
        /// `BT_PHY_BR_1M_3SLOT`.
        const BR_1M_3SLOT = c::BT_PHY_BR_1M_3SLOT;
        /// `BT_PHY_BR_1M_5SLOT`.
        const BR_1M_5SLOT = c::BT_PHY_BR_1M_5SLOT;
        /// `BT_PHY_EDR_2M_1SLOT`.
        const EDR_2M_1SLOT = c::BT_PHY_EDR_2M_1SLOT;
        /// `BT_PHY_EDR_2M_3SLOT`.
        const EDR_2M_3SLOT = c::BT_PHY_EDR_2M_3SLOT;
        /// `BT_PHY_EDR_2M_5SLOT`.
        const EDR_2M_5SLOT = c::BT_PHY_EDR_2M_5SLOT;
        /// `BT_PHY_EDR_3M_1SLOT`.
        const EDR_3M_1SLOT = c::BT_PHY_EDR_3M_1SLOT;
        /// `BT_PHY_EDR_3M_3SLOT`.
        const EDR_3M_3SLOT = c::BT_PHY_EDR_3M_3SLOT;
        /// `BT_PHY_EDR_3M_5SLOT`.
        const EDR_3M_5SLOT = c::BT_PHY_EDR_3M_5SLOT;
        /// `BT_PHY_LE_1M_TX`.
        const LE_1M_TX = c::BT_PHY_LE_1M_TX;
        /// `BT_PHY_LE_1M_RX`.
        const LE_1M_RX = c::BT_PHY_LE_1M_RX;
        /// `BT_PHY_LE_2M_TX`.
        const LE_2M_TX = c::BT_PHY_LE_2M_TX;
        /// `BT_PHY_LE_2M_RX`.
        const LE_2M_RX = c::BT_PHY_LE_2M_RX;
        /// `BT_PHY_LE_CODED_TX`.
        const LE_CODED_TX = c::BT_PHY_LE_CODED_TX;
        /// `BT_PHY_LE_CODED_RX`.
        const LE_CODED_RX = c::BT_PHY_LE_CODED_RX;
    }
}

/// `BT_MODE_*`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct BluetoothMode(u8);

impl BluetoothMode {
    /// `BT_MODE_BASIC`.
    pub const BASIC: Self = Self(c::BT_MODE_BASIC as _);
    /// `BT_MODE_ERTM`.
    pub const ERTM: Self = Self(c::BT_MODE_ERTM as _);
    /// `BT_MODE_STREAMING`.
    pub const STREAMING: Self = Self(c::BT_MODE_STREAMING as _);
    /// `BT_MODE_LE_FLOWCTL`.
    pub const LE_FLOWCTL: Self = Self(c::BT_MODE_LE_FLOWCTL as _);
    /// `BT_MODE_EXT_FLOWCTL`.
    pub const EXT_FLOWCTL: Self = Self(c::BT_MODE_EXT_FLOWCTL as _);

    /// Construct a `BluetoothMode` from a raw integer.
    #[inline]
    pub const fn from_raw(raw: u8) -> Self {
        Self(raw)
    }

    /// Return the raw integer.
    #[inline]
    pub const fn as_raw(self) -> u8 {
        self.0
    }
}

/// `L2CAP_MODE_*`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct L2capMode(u8);

impl L2capMode {
    /// `L2CAP_MODE_BASIC`.
    pub const BASIC: Self = Self(c::L2CAP_MODE_BASIC as _);
    /// `L2CAP_MODE_RETRANS`.
    pub const RETRANS: Self = Self(c::L2CAP_MODE_RETRANS as _);
    /// `L2CAP_MODE_FLOWCTL`.
    pub const FLOWCTL: Self = Self(c::L2CAP_MODE_FLOWCTL as _);
    /// `L2CAP_MODE_ERTM`.
    pub const ERTM: Self = Self(c::L2CAP_MODE_ERTM as _);
    /// `L2CAP_MODE_STREAMING`.
    pub const STREAMING: Self = Self(c::L2CAP_MODE_STREAMING as _);
    /// `L2CAP_MODE_LE_FLOWCTL`.
    pub const LE_FLOWCTL: Self = Self(c::L2CAP_MODE_LE_FLOWCTL as _);
    /// `L2CAP_MODE_EXT_FLOWCTL`.
    pub const EXT_FLOWCTL: Self = Self(c::L2CAP_MODE_EXT_FLOWCTL as _);

    /// Construct an `L2capMode` from a raw integer.
    #[inline]
    pub const fn from_raw(raw: u8) -> Self {
        Self(raw)
    }

    /// Return the raw integer.
    #[inline]
    pub const fn as_raw(self) -> u8 {
        self.0
    }
}

/// `struct l2cap_options`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct L2capOptions {
    /// Outgoing MTU.
    pub omtu: u16,
    /// Incoming MTU.
    pub imtu: u16,
    /// Flush timeout.
    pub flush_to: u16,
    /// Mode.
    pub mode: L2capMode,
    /// Frame check sequence mode.
    pub fcs: u8,
    /// Maximum transmissions.
    pub max_tx: u8,
    /// Transmission window size.
    pub txwin_size: u16,
}

impl L2capOptions {
    #[inline]
    pub(crate) const fn from_raw(raw: c::l2cap_options) -> Self {
        Self {
            omtu: raw.omtu,
            imtu: raw.imtu,
            flush_to: raw.flush_to,
            mode: L2capMode::from_raw(raw.mode),
            fcs: raw.fcs,
            max_tx: raw.max_tx,
            txwin_size: raw.txwin_size,
        }
    }

    #[inline]
    pub(crate) fn to_raw(self) -> core::mem::MaybeUninit<c::l2cap_options> {
        let mut raw = core::mem::MaybeUninit::<c::l2cap_options>::zeroed();
        unsafe {
            let raw = raw.as_mut_ptr();
            core::ptr::addr_of_mut!((*raw).omtu).write(self.omtu);
            core::ptr::addr_of_mut!((*raw).imtu).write(self.imtu);
            core::ptr::addr_of_mut!((*raw).flush_to).write(self.flush_to);
            core::ptr::addr_of_mut!((*raw).mode).write(self.mode.as_raw());
            core::ptr::addr_of_mut!((*raw).fcs).write(self.fcs);
            core::ptr::addr_of_mut!((*raw).max_tx).write(self.max_tx);
            core::ptr::addr_of_mut!((*raw).txwin_size).write(self.txwin_size);
        }
        raw
    }
}

/// `struct l2cap_conninfo`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct L2capConnInfo {
    /// HCI handle.
    pub hci_handle: u16,
    /// Device class.
    pub dev_class: [u8; 3],
}

bitflags! {
    /// `L2CAP_LM_*`.
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    #[repr(transparent)]
    pub struct L2capLinkMode: u32 {
        /// `L2CAP_LM_MASTER`.
        const MASTER = c::L2CAP_LM_MASTER;
        /// `L2CAP_LM_AUTH`.
        const AUTH = c::L2CAP_LM_AUTH;
        /// `L2CAP_LM_ENCRYPT`.
        const ENCRYPT = c::L2CAP_LM_ENCRYPT;
        /// `L2CAP_LM_TRUSTED`.
        const TRUSTED = c::L2CAP_LM_TRUSTED;
        /// `L2CAP_LM_RELIABLE`.
        const RELIABLE = c::L2CAP_LM_RELIABLE;
        /// `L2CAP_LM_SECURE`.
        const SECURE = c::L2CAP_LM_SECURE;
        /// `L2CAP_LM_FIPS`.
        const FIPS = c::L2CAP_LM_FIPS;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizes() {
        assert_eq_size!(BdAddr, c::bdaddr_t);
        assert_eq_size!(HciFilter, c::hci_ufilter);
        assert_eq_size!(BluetoothSecurity, c::bt_security);
        assert_eq_size!(BluetoothPower, c::bt_power);
        assert_eq_size!(BluetoothVoice, c::bt_voice);
        assert_eq_size!(L2capOptions, c::l2cap_options);
        assert_eq_size!(L2capConnInfo, c::l2cap_conninfo);

        assert_eq_align!(HciFilter, c::hci_ufilter);
        assert_eq_align!(BluetoothSecurity, c::bt_security);
        assert_eq_align!(BluetoothPower, c::bt_power);
        assert_eq_align!(BluetoothVoice, c::bt_voice);
        assert_eq_align!(L2capOptions, c::l2cap_options);
        assert_eq_align!(L2capConnInfo, c::l2cap_conninfo);
    }

    #[test]
    fn hci_filter_opcode_is_host_order() {
        let filter = HciFilter {
            type_mask: 1,
            event_mask: [2, 3],
            opcode: 0x1234,
        };

        let raw = filter.to_raw();
        let raw_bytes = unsafe {
            core::slice::from_raw_parts(raw.as_ptr().cast::<u8>(), core::mem::size_of_val(&raw))
        };
        assert_eq!(raw_bytes.last(), Some(&0));
        let raw = unsafe { raw.assume_init() };
        assert_eq!(raw.opcode, 0x1234_u16.to_le());
        assert_eq!(HciFilter::from_raw(raw), filter);
    }

    #[test]
    fn l2cap_options_raw_padding_is_zeroed() {
        let options = L2capOptions {
            omtu: 1,
            imtu: 2,
            flush_to: 3,
            mode: L2capMode::ERTM,
            fcs: 4,
            max_tx: 5,
            txwin_size: 6,
        };

        let raw = options.to_raw();
        let raw_bytes = unsafe {
            core::slice::from_raw_parts(raw.as_ptr().cast::<u8>(), core::mem::size_of_val(&raw))
        };
        assert_eq!(raw_bytes[9], 0);
        let raw = unsafe { raw.assume_init() };
        assert_eq!(L2capOptions::from_raw(raw), options);
    }
}
