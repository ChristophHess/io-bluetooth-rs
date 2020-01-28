pub use libbluetooth::{
    bdaddr_t, BTPROTO_L2CAP, BTPROTO_RFCOMM, inquiry_info, IREQ_CACHE_FLUSH,
    hci_close_dev, hci_get_route, hci_inquiry, hci_open_dev, sockaddr_rc
};
pub use libc::{
    accept, bind, connect, getpeername, getsockname, shutdown, sockaddr, socket,
    socklen_t, AF_BLUETOOTH, SHUT_RDWR, SOCK_STREAM,
};
