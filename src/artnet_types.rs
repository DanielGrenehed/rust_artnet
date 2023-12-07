
pub enum OpCodes {
    OpPoll,
    OpPollReply,
    OpDiagData,
    OpCommand,
    OpDataRequest,
    OpDataReply,
    OpDmx,
    OpNzs,
    OpSync,
    OpAddress,
    OpInput,
    OpTodRequest,
    OpTodData,
    OpTodControl,
    OpRdm,
    OpRdmSub,
    OpVideoSetup,
    OpVideoPalette,
    OpVideoData,
    OpMacMaster,
    OpMacSlave,
    OpFirmwareMaster,
    OpFirmwareReply,
    OpFileTnMaster,
    OpFileFnMaster,
    OpFileFnReply,
    OpIpProg,
    OpIpProgReply,
    OpMedia,
    OpMediaPatch,
    OpMediaControl,
    OpMediaControlReply,
    OpTimeCode,
    OpTimeSync,
    OpTrigger,
    OpDirectory,
    OpDirectoryReply
}

pub fn to_opcode(code :u16) -> Option<OpCodes> {
    return match code {
        0x2000 => Some(OpCodes::OpPoll),
        0x2100 => Some(OpCodes::OpPollReply),
        0x2300 => Some(OpCodes::OpDiagData),
        0x2400 => Some(OpCodes::OpCommand),
        0x2700 => Some(OpCodes::OpDataRequest),
        0x2800 => Some(OpCodes::OpDataReply),
        0x5000 => Some(OpCodes::OpDmx),
        0x5100 => Some(OpCodes::OpNzs),
        0x5200 => Some(OpCodes::OpSync),
        0x6000 => Some(OpCodes::OpAddress),
        0x7000 => Some(OpCodes::OpInput),
        0x8000 => Some(OpCodes::OpTodRequest),
        0x8100 => Some(OpCodes::OpTodData),
        0x8200 => Some(OpCodes::OpTodControl),
        0x8300 => Some(OpCodes::OpRdm),
        0x8400 => Some(OpCodes::OpRdmSub),
        0xA010 => Some(OpCodes::OpVideoSetup),
        0xA020 => Some(OpCodes::OpVideoPalette),
        0xA040 => Some(OpCodes::OpVideoData),
        0xF000 => Some(OpCodes::OpMacMaster),
        0xF100 => Some(OpCodes::OpMacSlave),
        0xF200 => Some(OpCodes::OpFirmwareMaster),
        0xF300 => Some(OpCodes::OpFirmwareReply),
        0xF400 => Some(OpCodes::OpFileTnMaster),
        0xF500 => Some(OpCodes::OpFileFnMaster),
        0xF600 => Some(OpCodes::OpFileFnReply),
        0xF800 => Some(OpCodes::OpIpProg),
        0xF900 => Some(OpCodes::OpIpProgReply),
        0x9000 => Some(OpCodes::OpMedia),
        0x9100 => Some(OpCodes::OpMediaPatch),
        0x9200 => Some(OpCodes::OpMediaControl),
        0x9300 => Some(OpCodes::OpMediaControlReply),
        0x9700 => Some(OpCodes::OpTimeCode),
        0x9800 => Some(OpCodes::OpTimeSync),
        0x9900 => Some(OpCodes::OpTrigger),
        0x9A00 => Some(OpCodes::OpDirectory),
        0x9B00 => Some(OpCodes::OpDirectoryReply),
        _ => None
    }
}


pub fn from_opcode(code :OpCodes) -> u16 {
    return match code {
        OpCodes::OpPoll             => 0x2000,
        OpCodes::OpPollReply        => 0x2100,
        OpCodes::OpDiagData         => 0x2300,
        OpCodes::OpCommand          => 0x2400,
        OpCodes::OpDataRequest      => 0x2700,
        OpCodes::OpDataReply        => 0x2800,
        OpCodes::OpDmx              => 0x5000,
        OpCodes::OpNzs              => 0x5100,
        OpCodes::OpSync             => 0x5200,
        OpCodes::OpAddress          => 0x6000,
        OpCodes::OpInput            => 0x7000,
        OpCodes::OpTodRequest       => 0x8000,
        OpCodes::OpTodData          => 0x8100,
        OpCodes::OpTodControl       => 0x8200,
        OpCodes::OpRdm              => 0x8300,
        OpCodes::OpRdmSub           => 0x8400,
        OpCodes::OpVideoSetup       => 0xA010,
        OpCodes::OpVideoPalette     => 0xA020,
        OpCodes::OpVideoData        => 0xA040,
        OpCodes::OpMacMaster        => 0xF000,
        OpCodes::OpMacSlave         => 0xF100,
        OpCodes::OpFirmwareMaster   => 0xF200,
        OpCodes::OpFirmwareReply    => 0xF300,
        OpCodes::OpFileTnMaster     => 0xF400,
        OpCodes::OpFileFnMaster     => 0xF500,
        OpCodes::OpFileFnReply      => 0xF600,
        OpCodes::OpIpProg           => 0xF800,
        OpCodes::OpIpProgReply      => 0xF900,
        OpCodes::OpMedia            => 0x9000,
        OpCodes::OpMediaPatch       => 0x9100,
        OpCodes::OpMediaControl     => 0x9200,
        OpCodes::OpMediaControlReply=> 0x9300,
        OpCodes::OpTimeCode         => 0x9700,
        OpCodes::OpTimeSync         => 0x9800,
        OpCodes::OpTrigger          => 0x9900,
        OpCodes::OpDirectory        => 0x9A00,
        OpCodes::OpDirectoryReply   => 0x9B00,
    }
}

impl fmt::Display for OpCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpCodes::OpPoll             => write!(f, "OpPoll"),
            OpCodes::OpPollReply        => write!(f, "OpPollReply"),
            OpCodes::OpDiagData         => write!(f, "OpDiagData"),
            OpCodes::OpCommand          => write!(f, "OpCommand"),
            OpCodes::OpDataRequest      => write!(f, "OpDataRequest"),
            OpCodes::OpDataReply        => write!(f, "OpDataReply"),
            OpCodes::OpDmx              => write!(f, "OpDmx"),
            OpCodes::OpNzs              => write!(f, "OpNzs"),
            OpCodes::OpSync             => write!(f, "OpSync"),
            OpCodes::OpAddress          => write!(f, "OpAddress"),
            OpCodes::OpInput            => write!(f, "OpInput"),
            OpCodes::OpTodRequest       => write!(f, "OpTodRequest"),
            OpCodes::OpTodData          => write!(f, "OpDotData"),
            OpCodes::OpTodControl       => write!(f, "OpTodControl"),
            OpCodes::OpRdm              => write!(f, "OpRdm"),
            OpCodes::OpRdmSub           => write!(f, "OpRdmSub"),
            OpCodes::OpVideoSetup       => write!(f, "OpVideoSetup"),
            OpCodes::OpVideoPalette     => write!(f, "OpVideoPalette"),
            OpCodes::OpVideoData        => write!(f, "OpVideoData"),
            OpCodes::OpMacMaster        => write!(f, "OpMacMaster"),
            OpCodes::OpMacSlave         => write!(f, "OpMacSlave"),
            OpCodes::OpFirmwareMaster   => write!(f, "OpFirmwareMaster"),
            OpCodes::OpFirmwareReply    => write!(f, "OpFirmwareReply"),
            OpCodes::OpFileTnMaster     => write!(f, "OpFileTnMaster"),
            OpCodes::OpFileFnMaster     => write!(f, "OpFileFnMaster"),
            OpCodes::OpFileFnReply      => write!(f, "OpFileFnReply"),
            OpCodes::OpIpProg           => write!(f, "OpIpProg"),
            OpCodes::OpIpProgReply      => write!(f, "OpIpProgReply"),
            OpCodes::OpMedia            => write!(f, "OpMedia"),
            OpCodes::OpMediaPatch       => write!(f, "OpMediaPatch"),
            OpCodes::OpMediaControl     => write!(f, "OpMediaControl"),
            OpCodes::OpMediaControlReply=> write!(f, "OpMediaControlReply"),
            OpCodes::OpTimeCode         => write!(f, "OpTimeCode"),
            OpCodes::OpTimeSync         => write!(f, "OpTimeSync"),
            OpCodes::OpTrigger          => write!(f, "OpTrigger"),
            OpCodes::OpDirectory        => write!(f, "OpDirectory"),
            OpCodes::OpDirectoryReply   => write!(f, "OpDirectoryReply")
        }
    }
}

pub enum NodeReportCodes {
    RcDebug,
    RcPowerOk,
    RcPowerFail,
    RcSocketWr1,
    RcParseFail,
    RcUdpFail,
    RcShNameOk,
    RcLoNameOk,
    RcDmxError,
    RcDmxUdpFull,
    RcDmxRxFull,
    RcSwitchErr,
    RcConfigErr,
    RcDmxShort,
    RcFirmwareFail,
    RcUserFail,
    RcFactoryRes
}

pub fn to_node_report_code(code :u16) -> Option<NodeReportCodes> {
    return match code {
        0x0000 => Some(NodeReportCodes::RcDebug),
        0x0001 => Some(NodeReportCodes::RcPowerOk),
        0x0002 => Some(NodeReportCodes::RcPowerFail),
        0x0003 => Some(NodeReportCodes::RcSocketWr1),
        0x0004 => Some(NodeReportCodes::RcParseFail),
        0x0005 => Some(NodeReportCodes::RcUdpFail),
        0x0006 => Some(NodeReportCodes::RcShNameOk),
        0x0007 => Some(NodeReportCodes::RcLoNameOk),
        0x0008 => Some(NodeReportCodes::RcDmxError),
        0x0009 => Some(NodeReportCodes::RcDmxUdpFull),
        0x000A => Some(NodeReportCodes::RcDmxRxFull),
        0x000B => Some(NodeReportCodes::RcSwitchErr),
        0x000C => Some(NodeReportCodes::RcConfigErr),
        0x000D => Some(NodeReportCodes::RcDmxShort),
        0x000E => Some(NodeReportCodes::RcFirmwareFail),
        0x000F => Some(NodeReportCodes::RcUserFail),
        0x0010 => Some(NodeReportCodes::RcFactoryRes),
        _ => None
    }
}

pub fn from_node_report_code(code :NodeReportCodes) -> u16 {
    return match code {
        NodeReportCodes::RcDebug         => 0x0000,
        NodeReportCodes::RcPowerOk       => 0x0001,
        NodeReportCodes::RcPowerFail     => 0x0002,
        NodeReportCodes::RcSocketWr1     => 0x0003,
        NodeReportCodes::RcParseFail     => 0x0004,
        NodeReportCodes::RcUdpFail       => 0x0005,
        NodeReportCodes::RcShNameOk      => 0x0006,
        NodeReportCodes::RcLoNameOk      => 0x0007,
        NodeReportCodes::RcDmxError      => 0x0008,
        NodeReportCodes::RcDmxUdpFull    => 0x0009,
        NodeReportCodes::RcDmxRxFull     => 0x000A,
        NodeReportCodes::RcSwitchErr     => 0x000B,
        NodeReportCodes::RcConfigErr     => 0x000C,
        NodeReportCodes::RcDmxShort      => 0x000D,
        NodeReportCodes::RcFirmwareFail  => 0x000E,
        NodeReportCodes::RcUserFail      => 0x000F,
        NodeReportCodes::RcFactoryRes    => 0x0010
    }
}

impl fmt::Display for NodeReportCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeReportCodes::RcDebug         => write!(f, "RcDebug"),
            NodeReportCodes::RcPowerOk       => write!(f, "RcPowerOk"),
            NodeReportCodes::RcPowerFail     => write!(f, "RcPowerFail"),
            NodeReportCodes::RcSocketWr1     => write!(f, "RcSocketWr1"),
            NodeReportCodes::RcParseFail     => write!(f, "RcParseFail"),
            NodeReportCodes::RcUdpFail       => write!(f, "RcUdpFail"),
            NodeReportCodes::RcShNameOk      => write!(f, "RcShNameOk"),
            NodeReportCodes::RcLoNameOk      => write!(f, "RcLoNameOk"),
            NodeReportCodes::RcDmxError      => write!(f, "RcDmxError"),
            NodeReportCodes::RcDmxUdpFull    => write!(f, "RcDmxUdpFull"),
            NodeReportCodes::RcDmxRxFull     => write!(f, "RcDmxRxFull"),
            NodeReportCodes::RcSwitchErr     => write!(f, "RcSwitchErr"),
            NodeReportCodes::RcConfigErr     => write!(f, "RcConfigErr"),
            NodeReportCodes::RcDmxShort      => write!(f, "RcDmxShort"),
            NodeReportCodes::RcFirmwareFail  => write!(f, "RcFirmwareFail"),
            NodeReportCodes::RcUserFail      => write!(f, "RcUserFail"),
            NodeReportCodes::RcFactoryRes    => write!(f, "RcFactoryRes")
        } 
    }
}

pub enum StyleCodes {
    StNode,
    StController,
    StMedia,
    StRoute,
    StBackup,
    StConfig,
    StVisual
}

pub fn to_style_code(code :u8) -> Option<StyleCodes> {
    return match code {
        0x00 => Some(StyleCodes::StNode),
        0x01 => Some(StyleCodes::StController),
        0x02 => Some(StyleCodes::StMedia),
        0x03 => Some(StyleCodes::StRoute),
        0x04 => Some(StyleCodes::StBackup),
        0x05 => Some(StyleCodes::StConfig),
        0x06 => Some(StyleCodes::StVisual),
        _ => None
    }
}

pub fn from_style_code(code :StyleCodes) -> u8 {
    return match code {
        StyleCodes::StNode          => 0x00,
        StyleCodes::StController    => 0x01,
        StyleCodes::StMedia         => 0x02,
        StyleCodes::StRoute         => 0x03,
        StyleCodes::StBackup        => 0x04,
        StyleCodes::StConfig        => 0x05,
        StyleCodes::StVisual        => 0x06
    }
}

impl fmt::Display for NodeReportCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StyleCodes::StNode          => write!(f, "StNode"),
            StyleCodes::StController    => write!(f, "StController"),
            StyleCodes::StMedia         => write!(f, "StMedia"),
            StyleCodes::StRoute         => write!(f, "StRoute"),
            StyleCodes::StBackup        => write!(f, "StBackup"),
            StyleCodes::StConfig        => write!(f, "StConfig"),
            StyleCodes::StVisual        => write!(f, "StVisual")
        }
    }
}

pub enum NodeConfigurationCommands {
    AcNone,
    AcCancelMerge,
    AcLedNormal,
    AcLedMute,
    AcLedLocate,
    AcResetRxFlags,
    AcAnalysisOn,
    AcAnalysisOff,
    AcFailHold,
    AcFailZero,
    AcFailFull,
    AcFailScene,
    AcFailRecord,
    
    AcMergeLtp0,
    AcMergeLtp1,
    AcMergeLtp2,
    AcMergeLtp3,
    
    AcDirectionTx0,
    AcDirectionTx1,
    AcDirectionTx2,
    AcDirectionTx3,
    
    AcDirectionRx0,
    AcDirectionRx1,
    AcDirectionRx2,
    AcDirectionRx3,
    
    AcMergeHtp0,
    AcMergeHtp1,
    AcMergeHtp2,
    AcMergeHtp3,
    
    AcArtNetSel0,
    AcArtNetSel1,
    AcArtNetSel2,
    AcArtNetSel3,
    
    AcAcnSel0,
    AcAcnSel1,
    AcAcnSel2,
    AcAcnSel3,
    
    AcClearOp0,
    AcClearOp1,
    AcClearOp2,
    AcClearOp3,
    
    AcStyleDelta0,
    AcStyleDelta1,
    AcStyleDelta2,
    AcStyleDelta3,
    
    AcStyleConst0,
    AcStyleConst1,
    AcStyleConst2,
    AcStyleConst3,

    AcRdmEnable0,
    AcRdmEnable1,
    AcRdmEnable2,
    AcRdmEnable3,

    AcRdmDisable0,
    AcRdmDisable1,
    AcRdmDisable2,
    AcRdmDisable3
}

pub fn to_node_configuration_command(code :u8) -> Option<NodeConfigurationCommands> {
    match code {
        0x00 => Some(NodeConfigurationCommands::AcNone),
        0x01 => Some(NodeConfigurationCommands::AcCancelMerge),
        0x02 => Some(NodeConfigurationCommands::AcLedNormal),
        0x03 => Some(NodeConfigurationCommands::AcLedMute),
        0x04 => Some(NodeConfigurationCommands::AcLedLocate),
        0x05 => Some(NodeConfigurationCommands::AcResetRxFlags),
        0x06 => Some(NodeConfigurationCommands::AcAnalysisOn),
        0x07 => Some(NodeConfigurationCommands::AcAnalysisOff),
        0x08 => Some(NodeConfigurationCommands::AcFailHold),
        0x09 => Some(NodeConfigurationCommands::AcFailZero),
        0x0A => Some(NodeConfigurationCommands::AcFailFull),
        0x0B => Some(NodeConfigurationCommands::AcFailScene),
        0x0C => Some(NodeConfigurationCommands::AcFailRecord),
        
        0x10 => Some(NodeConfigurationCommands::AcMergeLtp0),
        0x11 => Some(NodeConfigurationCommands::AcMergeLtp1),
        0x12 => Some(NodeConfigurationCommands::AcMergeLtp2),
        0x13 => Some(NodeConfigurationCommands::AcMergeLtp3),
        
        0x20 => Some(NodeConfigurationCommands::AcDirectionTx0),
        0x21 => Some(NodeConfigurationCommands::AcDirectionTx1),
        0x22 => Some(NodeConfigurationCommands::AcDirectionTx2),
        0x23 => Some(NodeConfigurationCommands::AcDirectionTx3),
        
        0x30 => Some(NodeConfigurationCommands::AcDirectionRx0),
        0x31 => Some(NodeConfigurationCommands::AcDirectionRx1),
        0x32 => Some(NodeConfigurationCommands::AcDirectionRx2),
        0x33 => Some(NodeConfigurationCommands::AcDirectionRx3),
        
        0x50 => Some(NodeConfigurationCommands::AcMergeHtp0),
        0x51 => Some(NodeConfigurationCommands::AcMergeHtp1),
        0x52 => Some(NodeConfigurationCommands::AcMergeHtp2),
        0x53 => Some(NodeConfigurationCommands::AcMergeHtp3),
        
        0x60 => Some(NodeConfigurationCommands::AcArtNetSel0),
        0x61 => Some(NodeConfigurationCommands::AcArtNetSel1),
        0x62 => Some(NodeConfigurationCommands::AcArtNetSel2),
        0x63 => Some(NodeConfigurationCommands::AcArtNetSel3),
        
        0x70 => Some(NodeConfigurationCommands::AcAcnSel0),
        0x71 => Some(NodeConfigurationCommands::AcAcnSel1),
        0x72 => Some(NodeConfigurationCommands::AcAcnSel2),
        0x73 => Some(NodeConfigurationCommands::AcAcnSel3),
        
        0x90 => Some(NodeConfigurationCommands::AcClearOp0),
        0x91 => Some(NodeConfigurationCommands::AcClearOp1),
        0x92 => Some(NodeConfigurationCommands::AcClearOp2),
        0x93 => Some(NodeConfigurationCommands::AcClearOp3),
        
        0xA0 => Some(NodeConfigurationCommands::AcStyleDelta0),
        0xA1 => Some(NodeConfigurationCommands::AcStyleDelta1),
        0xA2 => Some(NodeConfigurationCommands::AcStyleDelta2),
        0xA3 => Some(NodeConfigurationCommands::AcStyleDelta3),
        
        0xB0 => Some(NodeConfigurationCommands::AcStyleConst0),
        0xB1 => Some(NodeConfigurationCommands::AcStyleConst1),
        0xB2 => Some(NodeConfigurationCommands::AcStyleConst2),
        0xB3 => Some(NodeConfigurationCommands::AcStyleConst3),

        0xC0 => Some(NodeConfigurationCommands::AcRdmEnable0),
        0xC1 => Some(NodeConfigurationCommands::AcRdmEnable1),
        0xC2 => Some(NodeConfigurationCommands::AcRdmEnable2),
        0xC3 => Some(NodeConfigurationCommands::AcRdmEnable3),

        0xD0 => Some(NodeConfigurationCommands::AcRdmDisable0),
        0xD1 => Some(NodeConfigurationCommands::AcRdmDisable1),
        0xD2 => Some(NodeConfigurationCommands::AcRdmDisable2),
        0xD3 => Some(NodeConfigurationCommands::AcRdmDisable3),
        _ => None
        } 
}

pub fn from_node_configuration_command(command :NodeConfigurationCommands) -> u8 {
    return match command {
        NodeConfigurationCommands::AcNone           => 0x00,
        NodeConfigurationCommands::AcCancelMerge    => 0x01,
        NodeConfigurationCommands::AcLedNormal      => 0x02,
        NodeConfigurationCommands::AcLedMute        => 0x03,
        NodeConfigurationCommands::AcLedLocate      => 0x04,
        NodeConfigurationCommands::AcResetRxFlags   => 0x05,
        NodeConfigurationCommands::AcAnalysisOn     => 0x06,
        NodeConfigurationCommands::AcAnalysisOff    => 0x07,
        NodeConfigurationCommands::AcFailHold       => 0x08,
        NodeConfigurationCommands::AcFailZero       => 0x09,
        NodeConfigurationCommands::AcFailFull       => 0x0A,
        NodeConfigurationCommands::AcFailScene      => 0x0B,
        NodeConfigurationCommands::AcFailRecord     => 0x0C,
        
        NodeConfigurationCommands::AcMergeLtp0      => 0x10,
        NodeConfigurationCommands::AcMergeLtp1      => 0x11,
        NodeConfigurationCommands::AcMergeLtp2      => 0x12,
        NodeConfigurationCommands::AcMergeLtp3      => 0x13,
        
        NodeConfigurationCommands::AcDirectionTx0   => 0x20,
        NodeConfigurationCommands::AcDirectionTx1   => 0x21,
        NodeConfigurationCommands::AcDirectionTx2   => 0x22,
        NodeConfigurationCommands::AcDirectionTx3   => 0x23,
        
        NodeConfigurationCommands::AcDirectionRx0   => 0x30,
        NodeConfigurationCommands::AcDirectionRx1   => 0x31,
        NodeConfigurationCommands::AcDirectionRx2   => 0x32,
        NodeConfigurationCommands::AcDirectionRx3   => 0x33,
        
        NodeConfigurationCommands::AcMergeHtp0      => 0x50,
        NodeConfigurationCommands::AcMergeHtp1      => 0x51,
        NodeConfigurationCommands::AcMergeHtp2      => 0x52,
        NodeConfigurationCommands::AcMergeHtp3      => 0x53,
        
        NodeConfigurationCommands::AcArtNetSel0     => 0x60,
        NodeConfigurationCommands::AcArtNetSel1     => 0x61,
        NodeConfigurationCommands::AcArtNetSel2     => 0x62,
        NodeConfigurationCommands::AcArtNetSel3     => 0x63,
        
        NodeConfigurationCommands::AcAcnSel0        => 0x70,
        NodeConfigurationCommands::AcAcnSel1        => 0x71,
        NodeConfigurationCommands::AcAcnSel2        => 0x72,
        NodeConfigurationCommands::AcAcnSel3        => 0x73,
        
        NodeConfigurationCommands::AcClearOp0       => 0x90,
        NodeConfigurationCommands::AcClearOp1       => 0x91,
        NodeConfigurationCommands::AcClearOp2       => 0x92,
        NodeConfigurationCommands::AcClearOp3       => 0x93,
        
        NodeConfigurationCommands::AcStyleDelta0    => 0xA0,
        NodeConfigurationCommands::AcStyleDelta1    => 0xA1,
        NodeConfigurationCommands::AcStyleDelta2    => 0xA2,
        NodeConfigurationCommands::AcStyleDelta3    => 0xA3,
        
        NodeConfigurationCommands::AcStyleConst0    => 0xB0,
        NodeConfigurationCommands::AcStyleConst1    => 0xB1,
        NodeConfigurationCommands::AcStyleConst2    => 0xB2,
        NodeConfigurationCommands::AcStyleConst3    => 0xB3,

        NodeConfigurationCommands::AcRdmEnable0     => 0xC0,
        NodeConfigurationCommands::AcRdmEnable1     => 0xC1,
        NodeConfigurationCommands::AcRdmEnable2     => 0xC2,
        NodeConfigurationCommands::AcRdmEnable3     => 0xC3,

        NodeConfigurationCommands::AcRdmDisable0    => 0xD0,
        NodeConfigurationCommands::AcRdmDisable1    => 0xD1,
        NodeConfigurationCommands::AcRdmDisable2    => 0xD2,
        NodeConfigurationCommands::AcRdmDisable3    => 0xD3
    }
}

impl fmt::Display for NodeConfigurationCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeConfigurationCommands::AcNone           => write!(f, "AcNone"),
            NodeConfigurationCommands::AcCancelMerge    => write!(f, "AcCancelMerge"),
            NodeConfigurationCommands::AcLedNormal      => write!(f, "AcLedNormal"),
            NodeConfigurationCommands::AcLedMute        => write!(f, "AcLedMute"),
            NodeConfigurationCommands::AcLedLocate      => write!(f, "AcLedLocate"),
            NodeConfigurationCommands::AcResetRxFlags   => write!(f, "AcResetRxFlags"),
            NodeConfigurationCommands::AcAnalysisOn     => write!(f, "AcAnalysisOn"),
            NodeConfigurationCommands::AcAnalysisOff    => write!(f, "AcAnalysisOff"),
            NodeConfigurationCommands::AcFailHold       => write!(f, "AcFailHold"),
            NodeConfigurationCommands::AcFailZero       => write!(f, "AcFailZero"),
            NodeConfigurationCommands::AcFailFull       => write!(f, "AcFailFull"),
            NodeConfigurationCommands::AcFailScene      => write!(f, "AcFailScene"),
            NodeConfigurationCommands::AcFailRecord     => write!(f, "AcFailRecord"),
            
            NodeConfigurationCommands::AcMergeLtp0      => write!(f, "AcMergeLtp0"),
            NodeConfigurationCommands::AcMergeLtp1      => write!(f, "AcMergeLtp1"),
            NodeConfigurationCommands::AcMergeLtp2      => write!(f, "AcMergeLtp2"),
            NodeConfigurationCommands::AcMergeLtp3      => write!(f, "AcMergeLtp3"),
            
            NodeConfigurationCommands::AcDirectionTx0   => write!(f, "AcDirectionTx0"),
            NodeConfigurationCommands::AcDirectionTx1   => write!(f, "AcDirectionTx1"),
            NodeConfigurationCommands::AcDirectionTx2   => write!(f, "AcDirectionTx2"),
            NodeConfigurationCommands::AcDirectionTx3   => write!(f, "AcDirectionTx3"),
            
            NodeConfigurationCommands::AcDirectionRx0   => write!(f, "AcDirectionRx0"),
            NodeConfigurationCommands::AcDirectionRx1   => write!(f, "AcDirectionRx1"),
            NodeConfigurationCommands::AcDirectionRx2   => write!(f, "AcDirectionRx2"),
            NodeConfigurationCommands::AcDirectionRx3   => write!(f, "AcDirectionRx3"),
            
            NodeConfigurationCommands::AcMergeHtp0      => write!(f, "AcMergeHtp0"),
            NodeConfigurationCommands::AcMergeHtp1      => write!(f, "AcMergeHtp1"),
            NodeConfigurationCommands::AcMergeHtp2      => write!(f, "AcMergeHtp2"),
            NodeConfigurationCommands::AcMergeHtp3      => write!(f, "AcMergeHtp3"),
            
            NodeConfigurationCommands::AcArtNetSel0     => write!(f, "AcArtNetSel0"),
            NodeConfigurationCommands::AcArtNetSel1     => write!(f, "AcArtNetSel1"),
            NodeConfigurationCommands::AcArtNetSel2     => write!(f, "AcArtNetSel2"),
            NodeConfigurationCommands::AcArtNetSel3     => write!(f, "AcArtNetSel3"),
            
            NodeConfigurationCommands::AcAcnSel0        => write!(f, "AcAcnSel0"),
            NodeConfigurationCommands::AcAcnSel1        => write!(f, "AcAcnSel1"),
            NodeConfigurationCommands::AcAcnSel2        => write!(f, "AcAcnSel2"),
            NodeConfigurationCommands::AcAcnSel3        => write!(f, "AcAcnSel3"),
            
            NodeConfigurationCommands::AcClearOp0       => write!(f, "AcClearOp0"),
            NodeConfigurationCommands::AcClearOp1       => write!(f, "AcClearOp1"),
            NodeConfigurationCommands::AcClearOp2       => write!(f, "AcClearOp2"),
            NodeConfigurationCommands::AcClearOp3       => write!(f, "AcClearOp3"),
            
            NodeConfigurationCommands::AcStyleDelta0    => write!(f, "AcStyleDelta0"),
            NodeConfigurationCommands::AcStyleDelta1    => write!(f, "AcStyleDelta1"),
            NodeConfigurationCommands::AcStyleDelta2    => write!(f, "AcStyleDelta2"),
            NodeConfigurationCommands::AcStyleDelta3    => write!(f, "AcStyleDelta3"),
            
            NodeConfigurationCommands::AcStyleConst0    => write!(f, "AcStyleConst0"),
            NodeConfigurationCommands::AcStyleConst1    => write!(f, "AcStyleConst1"),
            NodeConfigurationCommands::AcStyleConst2    => write!(f, "AcStyleConst2"),
            NodeConfigurationCommands::AcStyleConst3    => write!(f, "AcStyleConst3"),

            NodeConfigurationCommands::AcRdmEnable0     => write!(f, "AcRdmEnable0"),
            NodeConfigurationCommands::AcRdmEnable1     => write!(f, "AcRdmEnable1"),
            NodeConfigurationCommands::AcRdmEnable2     => write!(f, "AcRdmEnable2"),
            NodeConfigurationCommands::AcRdmEnable3     => write!(f, "AcRdmEnable3"),

            NodeConfigurationCommands::AcRdmDisable0    => write!(f, "AcRdmDisable0"),
            NodeConfigurationCommands::AcRdmDisable1    => write!(f, "AcRdmDisable1"),
            NodeConfigurationCommands::AcRdmDisable2    => write!(f, "AcRdmDisable2"),
            NodeConfigurationCommands::AcRdmDisable3    => write!(f, "AcRdmDisable3")
        }
    }
}

pub enum DataRequestCodes {
    DrPoll,
    DrUrlProduct,
    DrUrlUserGuide,
    DrUrlSupport,
    DrUrlPersUdr,
    DrUrlPersGdtf,
    DrManSpec
}

pub fn to_data_request_code(code :u16) -> Option<DataRequestCodes> {
    return match code {
        0x0000 => Some(DataRequestCodes::DrPoll),
        0x0001 => Some(DataRequestCodes::DrUrlProduct),
        0x0002 => Some(DataRequestCodes::DrUrlUserGuide),
        0x0003 => Some(DataRequestCodes::DrUrlSupport),
        0x0004 => Some(DataRequestCodes::DrUrlPersUdr),
        0x0005 => Some(DataRequestCodes::DrUrlPersGdtf),
        0x8000 ..=> Some(DataRequestCodes::DrManSpec),
        _ => None
    }
}

pub fn from_data_request_code(code :DataRequestCodes) -> u16 {
    return match code {
        DataRequestCodes::DrPoll        => 0x0000,
        DataRequestCodes::DrUrlProduct  => 0x0001,
        DataRequestCodes::DrUrlUserGuide=> 0x0002,
        DataRequestCodes::DrUrlSupport  => 0x0003,
        DataRequestCodes::DrUrlPersUdr  => 0x0004,
        DataRequestCodes::DrUrlPersGdtf => 0x0005,
        DataRequestCodes::DrManSpec     => 0x8000
    }
}

impl fmt::Display for DataRequestCodes {
    fn fmt(&self, f :&mut fmt::Formatter) -> std::Result {
        match self {
            DataRequestCodes::DrPoll        => write!(f, "DrPoll"),
            DataRequestCodes::DrUrlProduct  => write!(f, "DrUrlProduct"),
            DataRequestCodes::DrUrlUserGuide=> write!(f, "DrUrlUserGuide"),
            DataRequestCodes::DrUrlSupport  => write!(f, "DrUrlSupport"),
            DataRequestCodes::DrUrlPersUdr  => write!(f, "DrUrlPersUdr"),
            DataRequestCodes::DrUrlPersGdtf => write!(f, "DrUrlPersGdtf"),
            DataRequestCodes::DrManSpec     => write!(f, "DrManSpec")
        }
    }
}

pub enum PriorityCodes {
    DpLow,
    DpMed,
    DpHigh,
    DpCritical,
    DpVolatile
}

pub fn to_priority_code(code :u8) -> Option<PriorityCodes> {
    return match code {
            0x10 => PriorityCodes::DpLow,
            0x40 => PriorityCodes::DpMed,
            0x80 => PriorityCodes::DpHigh,
            0xE0 => PriorityCodes::DpCritical,
            0xF0 => PriorityCodes::DpVolatile,
            _ => None
    }
}

pub fn from_priority_code(code : PriorityCodes) -> u8 {
    return match code {
        PriorityCodes::DpLow => 0x10,
        PriorityCodes::DpMed => 0x40,
        PriorityCodes::DpHigh => 0x80,
        PriorityCodes::DpCritical => 0xE0,
        PriorityCodes::DpVolatile => 0xF0
    }
}

impl fmt::Display for PriorityCodes {
    fn fmt(&self, f:&mut fmt::Formatter) -> std::Result {
        match self {
            PriorityCodes::DpLow => write!(f, "DpLow"),
            PriorityCodes::DpMed => write!(f, "DpMed"),
            PriorityCodes::DpHigh => write!(f, "DpHigh"),
            PriorityCodes::DpCritical => write!(f, "DpCritical"),
            PriorityCodes::DpVolatile => write!(f, "DpVolatile")
        }
    }
}
