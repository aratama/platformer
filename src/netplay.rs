pub const NETPLAY: *const u8 = 0x0020 as *const u8;

pub fn get_my_net_player_index() -> u8 {
    unsafe {
        if is_netplay_active() {
            *NETPLAY & 0b011
        } else {
            0
        }
    }
}

pub fn is_netplay_active() -> bool {
    unsafe { *NETPLAY & 0b100 != 0 }
}
