use rustix::io::Errno;
use rustix::net::bluetooth::{
    self, BdAddr, BdAddrType, BluetoothMode, HciChannel, HciDevice, L2capMode, SocketAddrHci,
    SocketAddrL2cap, SocketAddrRfcomm, SocketAddrSco,
};
use rustix::net::{AddressFamily, RawProtocol, SocketAddrAny, SocketAddrV4, SocketType};

#[allow(unused_unsafe)]
unsafe fn sockaddr_bytes(addr: &SocketAddrAny) -> &[u8] {
    unsafe { core::slice::from_raw_parts(addr.as_ptr().cast(), addr.addr_len() as usize) }
}

#[test]
fn protocols() {
    assert_eq!(bluetooth::L2CAP, None);
    assert_eq!(
        bluetooth::HCI.unwrap().as_raw(),
        RawProtocol::new(1).unwrap()
    );
}

#[test]
fn l2cap_options_modes_are_not_bluetooth_modes() {
    assert_eq!(L2capMode::ERTM.as_raw(), 0x03);
    assert_eq!(BluetoothMode::ERTM.as_raw(), 0x01);
    assert_ne!(L2capMode::ERTM.as_raw(), BluetoothMode::ERTM.as_raw());
}

#[cfg(all(linux_kernel, linux_raw_dep))]
#[test]
fn l2cap_mtu_sockopts_reject_non_bluetooth_sockets() {
    let socket = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();

    assert!(rustix::net::sockopt::l2cap_send_mtu(&socket).is_err());
    assert!(rustix::net::sockopt::set_l2cap_send_mtu(&socket, 64).is_err());
    assert!(rustix::net::sockopt::l2cap_recv_mtu(&socket).is_err());
    assert!(rustix::net::sockopt::set_l2cap_recv_mtu(&socket, 64).is_err());
}

#[test]
fn hci_encode_decode() {
    let orig = SocketAddrHci::new(HciDevice::from_raw(2), HciChannel::RAW);
    let encoded = SocketAddrAny::from(orig);
    assert_eq!(encoded.address_family(), AddressFamily::BLUETOOTH);
    let decoded = SocketAddrHci::try_from(encoded).unwrap();
    assert_eq!(decoded, orig);
}

#[test]
fn l2cap_encode_decode() {
    let addr = BdAddr::new([0x06, 0x05, 0x04, 0x03, 0x02, 0x01]);
    let orig = SocketAddrL2cap::new(addr, BdAddrType::LE_RANDOM, 0x1001, 0x0004);
    let encoded = SocketAddrAny::from(orig);
    assert_eq!(encoded.address_family(), AddressFamily::BLUETOOTH);
    assert_eq!(unsafe { sockaddr_bytes(&encoded) }.last(), Some(&0));
    let decoded = SocketAddrL2cap::try_from(encoded).unwrap();
    assert_eq!(decoded, orig);
}

#[test]
fn sco_encode_decode() {
    let orig = SocketAddrSco::new(BdAddr::new([0x06, 0x05, 0x04, 0x03, 0x02, 0x01]));
    let encoded = SocketAddrAny::from(orig);
    assert_eq!(encoded.address_family(), AddressFamily::BLUETOOTH);
    let decoded = SocketAddrSco::try_from(encoded).unwrap();
    assert_eq!(decoded, orig);
}

#[test]
fn rfcomm_encode_decode() {
    let orig = SocketAddrRfcomm::new(BdAddr::new([0x06, 0x05, 0x04, 0x03, 0x02, 0x01]), 7);
    let encoded = SocketAddrAny::from(orig);
    assert_eq!(encoded.address_family(), AddressFamily::BLUETOOTH);
    assert_eq!(unsafe { sockaddr_bytes(&encoded) }.last(), Some(&0));
    let decoded = SocketAddrRfcomm::try_from(encoded).unwrap();
    assert_eq!(decoded, orig);
}

#[test]
fn wrong_family() {
    let encoded = SocketAddrAny::from(SocketAddrV4::new([127, 0, 0, 1].into(), 0));
    assert_eq!(SocketAddrHci::try_from(encoded), Err(Errno::AFNOSUPPORT));
}
