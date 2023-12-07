
struct ArtPoll {
    id :&[u8, 8],
    op_code : u16,
    prot_ver_hi : u8,
    prot_ver_lo : u8,
    flags : u8,
    diag_prioriy : u8,
    target_port_address_top_hi : u8,
    target_port_address_top_lo : u8,
    target_port_address_bottom_hi : u8,
    target_port_address_bottom_lo : u8,
    esta_man_hi : u8,
    esta_man_lo : u8,
    oem_hi : u8,
    oem_lo : u8
}

struct ArtPollReply {
    id :&[u8, 8],
    op_code :u16,
    ip_address :&[u8, 4],
    port :u16,
    vers_info_h :u8,
    vers_info_l :u8,
    net_switch :u8,
    sub_switch :u8,
    oem_hi :u8,
    oem_lo :u8,
    ubea_version :u8,
    status_1 :u8,
    esta_man_lo :u8,
    esta_man_hi :u8,
    port_name :&[u8, 18],
    long_name :&[u8, 64],
    node_report :&[u8, 64],
    num_ports_hi :u8,
    num_ports_lo :u8,
    port_types :&[u8, 4],
    good_input :&[u8, 4],
    good_output_a :&[u8, 4],
    sw_in :&[u8, 4],
    sw_out :&[u8, 4],
    acn_priority :u8,
    sw_macro :u8,
    sw_remote :u8,
    spare_1 :u8,
    spare_2 :u8,
    spare_3 :u8,
    style :u8,
    mac :&[u8, 6],
    bind_ip :&[u8, 4],
    bind_index :u8,
    status_2 :u8,
    good_output_b :&[u8, 4],
    status_3 :u8,
    default_resp_uid :&[u8, 6],
    user_hi :u8,
    user_lo :u8,
    refresh_rate_hi :u8,
    refresh_rate_lo :u8,
    filler :&[u8, 11]
}

struct ArtIpProg {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    filler_1 :u8,
    filler_2 :u8,
    command :u8,
    filler_3 :u8,
    prog_ip_hi :u8,
    prog_ip_2 :u8,
    prog_ip_1 :u8,
    prog_ip_lo :u8,
    prog_sm_hi :u8,
    prog_sm_2 :u8,
    prog_sm_1 :u8,
    prog_sm_lo :u8,
    prog_port_hi :u8,
    prog_port_lo :u8,
    porg_dg_hi :u8,
    prog_dg_2 :u8,
    prog_dg_1 :u8,
    porg_dg_lo :u8,
    spare :&[u8, 4]
}

struct ArtIpProgReply {
    id :&[u8, 8],       //'A' 'r' 't' '-' 'N' 'e' 't' 0x00
    op_code :u16,       // OpIpProgReply, low byte first
    prot_ver_hi :u8,    // Art-Net protocol revision number
    prot_ver_lo :u8,
    filler_1 :u8,
    filler_2 :u8,
    filler_3 :u8,
    filler_4 :u8,
    prog_ip_hi :u8,     // node Ip Address
    prog_ip_2 :u8,
    prog_ip_1 :u8,
    prog_ip_lo :u8,
    prog_sm_hi :u8,     // Node Subnet mask
    prog_sm_2 :u8,
    prog_sm_1 :u8,
    prog_sm_lo :u8,
    prog_port_hi :u8,   // deprecated
    prog_port_lo :u8,
    status :u8,         // bit 6 DHCP
    spare_1 :u8,        // zero
    prog_dg_hi :u8,     // Default Gateway of Node
    prog_dg_2 :u8,
    prog_dg_1 :u8,
    prog_dg_lo :u8,
    spare_2 :u8,        // zero
    spare_3 :u8         // zero 
}

struct ArtAddress {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    net_switch :u8,
    bind_index :u8,
    port_name :&[u8, 18],
    long_name :&[u8, 64],
    sw_in :&[u8, 4],
    sw_out :&[u8, 4],
    sub_switch :u8,
    acn_priority :u8,
    command :u8 // Node Configuration Command
}

struct ArtDataRequest {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    esta_man_hi :u8,
    esta_man_lo :u8,
    oem_hi :u8,
    oem_lo :u8,
    request_hi :u8, // DataRequestCodes
    request_lo :u8,
    spare :&[u8, 22]
}

struct ArtDataReply {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    esta_man_hi :u8,
    esta_man_lo :u8,
    oem_hi :u8,
    oem_lo :u8,
    request_hi :u8,
    request_lo :u8,
    pay_len_hi :u8,
    pay_len_lo :u8,
    payload :Vec<u8> //max 512
}

struct ArtDiagData {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    filler_1 :u8,
    diag_priority :u8,
    logical_port :u8,
    filler_2 :u8,
    length_hi :u8,
    length_lo :u8,
    data :Vec<u8> //max 512

}

struct ArtTimeCode {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    filler_1 :u8,
    filler_2 :u8,
    frames :u8,
    seconds :u8,
    minutes :u8,
    hours :u8,
    _type :u8
}

struct ArtCommand {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    esta_man_hi :u8,
    esta_man_lo :u8,
    length_hi :u8,
    length_lo :u8,
    data :Vec<u8> // max 512
}

struct ArtTrigger {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    filler_1 :u8,
    filler_2 :u8,
    oem_hi :u8, // if 0xffff then follow table for function of keys
    oem_lo :u8,
    key :u8,
    sub_key :u8,
    data :&[u8, 512]
}

struct ArtDmx {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    sequence :u8 // 0x00 to disable resequencing of packets
    physical :u8,
    sub_uni :u8,
    net :u8,
    length_hi :u8,
    length_lo :u8,
    data :Vec<u8> // max 512
}

struct ArtSync {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    aux_1 :u8,  // zero
    aux_2 :u8   // zero
}

struct ArtNzs {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    sequence :u8,
    start_code :u8,
    sub_uni :u8,
    net :u8,
    length_hi :u8,
    length_lo :u8,
    data :Vec<u8> // max 512
}

struct ArtVlc {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    sequence :u8,
    start_code :u8,
    sub_uni :u8,
    net :u8,
    length_hi :u8,
    length_lo :u8,
    man_id_hi :u8 // 0x41
    man_id_lo :u8 // 0x4C
    sub_code :u8 // 0x45
    flags :u8,
    trans_hi :u8,
    trans_lo :u8,
    slot_addr_hi :u8,
    slot_addr_lo :u8,
    pay_count_hi :u8,
    pay_count_lo :u8,
    pay_check_hi :u8,
    pay_check_lo :u8,
    spare_1 :u8,
    vlc_depth :u8,
    vlc_freq_hi :u8,
    vlc_freq_lo :u8,
    vlc_mod_hi :u8,
    vlc_mod_lo :u8,
    pay_lang_hi :u8,
    pay_lang_lo :u8,
    beac_rep_hi :u8,
    beac_rep_lo :u8,
    payload :Vec<u8>
}

struct ArtInput {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,   
    filler :u8,
    bind_index :u8,
    num_ports_hi :u8,
    num_ports_lo :u8,
    input :&[u8, 4]
}

struct ArtFirmwareMaster {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    filler_1 :u8,
    filler_2 :u8,
    _type :u8, // 0: FirstFirm, 1: FirmCont, 2: FirmLast, 3: UbeaFirst, 4: UbeaCont, 5: UbeaLast
    block_id :u8,
    firmweare_length :u64 // (hi->lo) total words(int16) of file to be uploaded
    spare :&[u8, 20],
    data :&[u8, 512]
}

struct ArtFirmwareReply {
    id :&[u8, 8],
    op_code :u16,
    prot_ver_hi :u8,
    prot_ver_lo :u8,
    filler_1 :u8,
    filler_2 :u8,
    _type :u8, // 00 FirmBlockGood 01 FirmAllGood FF FirmFail
    spare :&[u8, 21]
}
