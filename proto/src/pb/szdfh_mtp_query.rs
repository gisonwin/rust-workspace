///MTP查询基本信息
///数据包类型:message_id=101
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryBaseInformation {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
}
///MTP查询基本信息响应
///数据包类型:message_id=102
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryBaseInformationCack {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///型号代号
    #[prost(string, tag="3")]
    pub style_code: ::prost::alloc::string::String,
    ///型号名称
    #[prost(string, tag="4")]
    pub style_name: ::prost::alloc::string::String,
    ///型号序号
    #[prost(uint32, tag="5")]
    pub style_serial_num: u32,
    ///卫星代号
    #[prost(string, tag="9")]
    pub satelite_code: ::prost::alloc::string::String,
    ///卫星名称
    #[prost(string, tag="10")]
    pub satelite_name: ::prost::alloc::string::String,
    ///卫星过程库名称(主库)
    #[prost(string, tag="15")]
    pub satelite_f_name: ::prost::alloc::string::String,
    ///卫星过程库IP(主库)
    #[prost(string, tag="16")]
    pub satelite_f_ip: ::prost::alloc::string::String,
    ///卫星过程库端口(主库)
    #[prost(uint32, tag="17")]
    pub satelite_f_port: u32,
    ///卫星过程库用户(主库)
    #[prost(string, tag="18")]
    pub satelite_f_user: ::prost::alloc::string::String,
    ///卫星过程库pwd(主库)
    #[prost(string, tag="19")]
    pub satelite_f_pwd: ::prost::alloc::string::String,
    ///卫星过程库名称(备库)
    #[prost(string, tag="20")]
    pub satelite_b_name: ::prost::alloc::string::String,
    ///卫星过程库IP(备库)
    #[prost(string, tag="21")]
    pub satelite_b_ip: ::prost::alloc::string::String,
    ///卫星过程库端口(备库)
    #[prost(uint32, tag="22")]
    pub satelite_b_port: u32,
    ///卫星过程库用户(备库)
    #[prost(string, tag="23")]
    pub satelite_b_user: ::prost::alloc::string::String,
    ///卫星过程库pwd(备库)
    #[prost(string, tag="24")]
    pub satelite_b_pwd: ::prost::alloc::string::String,
}
///MTP指令信息
///数据包类型:message_id=111
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpCommandItem {
    ///指令guid(数据库中)
    #[prost(string, tag="1")]
    pub command_guid: ::prost::alloc::string::String,
    ///指令代号
    #[prost(string, tag="2")]
    pub command_code: ::prost::alloc::string::String,
    ///指令名称
    #[prost(string, tag="3")]
    pub command_name: ::prost::alloc::string::String,
    ///指令发送标志   0=禁止发送  1=允许发送
    #[prost(uint32, tag="4")]
    pub command_send_flag: u32,
}
///MTP查询指令
///数据包类型:message_id=112
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryCommands {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
}
///MTP查询指令响应
///数据包类型:message_id=113
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryCommandsCack {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///key为指令代号，指令列表
    #[prost(map="string, message", tag="3")]
    pub map_commands: ::std::collections::HashMap<::prost::alloc::string::String, MsgMtpCommandItem>,
}
///MTP参数信息
///数据包类型:message_id=114
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpParameterItem {
    ///参数guid(数据库中)
    #[prost(string, tag="1")]
    pub parameter_guid: ::prost::alloc::string::String,
    ///参数代号
    #[prost(string, tag="2")]
    pub parameter_code: ::prost::alloc::string::String,
    ///参数名称
    #[prost(string, tag="3")]
    pub parameter_name: ::prost::alloc::string::String,
}
///MTP查询参数
///数据包类型:message_id=115
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryParameters {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
}
///MTP查询参数响应
///数据包类型:message_id=116
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryParametersCack {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///key为参数代号，参数列表
    #[prost(map="string, message", tag="3")]
    pub map_parameters: ::std::collections::HashMap<::prost::alloc::string::String, MsgMtpParameterItem>,
    ///解码配置XML（由数据库信息抽取生成，XML格式）
    #[prost(string, tag="4")]
    pub config_xml: ::prost::alloc::string::String,
    ///解码配置ini（界面配置）
    #[prost(string, tag="5")]
    pub config_ini: ::prost::alloc::string::String,
}
///MTP常监视信息
///数据包类型:message_id=117
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpConstMonitorItem {
    ///常监视项guid(数据库中)
    #[prost(string, tag="1")]
    pub monitor_guid: ::prost::alloc::string::String,
    ///常监视项名称
    #[prost(string, tag="2")]
    pub monitor_name: ::prost::alloc::string::String,
    ///是否影响序列执行（0x30=不影响  0x31=影响）
    #[prost(uint32, tag="3")]
    pub affect_sequence_flag: u32,
    ///常监视项算法(例如:LNSRangeCheck)
    #[prost(string, tag="4")]
    pub monitor_function: ::prost::alloc::string::String,
    ///常监视项算法配置(key为V0,V1,V2...)例如（V0="XXX" V1="YYY" V2="ZZZ"）
    #[prost(map="string, string", tag="5")]
    pub map_function_config: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
///MTP查询常监视信息
///数据包类型:message_id=118
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryConstMonitors {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
}
///MTP查询常监视信息响应
///数据包类型:message_id=119
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryConstMonitorsCack {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///key为常监视项guid，常监视信息列表
    #[prost(map="string, message", tag="3")]
    pub map_const_monitors: ::std::collections::HashMap<::prost::alloc::string::String, MsgMtpConstMonitorItem>,
}
///MTP判据判读项信息
///数据包类型:message_id=120
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpJudgeItem {
    ///判据判读项guid(数据库中)
    #[prost(string, tag="1")]
    pub judge_item_guid: ::prost::alloc::string::String,
    ///判据判读项描述
    #[prost(string, tag="2")]
    pub judge_item_desc: ::prost::alloc::string::String,
    ///判据判读项算法(例如:LNSRangeCheck)
    #[prost(string, tag="3")]
    pub judge_item_function: ::prost::alloc::string::String,
    ///判据判读项算法配置(key为V0,V1,V2...)例如（V0="XXX" V1="YYY" V2="ZZZ"）
    #[prost(map="string, string", tag="4")]
    pub map_function_config: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
///MTP判据信息
///数据包类型:message_id=121
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpJudge {
    ///判据guid(数据库中)
    #[prost(string, tag="1")]
    pub judge_guid: ::prost::alloc::string::String,
    ///判据code
    #[prost(string, tag="2")]
    pub judge_code: ::prost::alloc::string::String,
    ///判据启动时间
    #[prost(uint32, tag="3")]
    pub judge_start_time: u32,
    ///判据超时时间
    #[prost(uint32, tag="4")]
    pub judge_out_time: u32,
    ///指令code
    #[prost(string, tag="5")]
    pub command_code: ::prost::alloc::string::String,
    ///是否影响序列执行（0x30=不影响  0x31=影响）
    #[prost(uint32, tag="6")]
    pub affect_sequence_flag: u32,
    ///判据类型(0=前判据 1=后判据)
    #[prost(uint32, tag="7")]
    pub judge_type: u32,
    ///判据判读项列表(key为判据判读项guid)
    #[prost(map="string, message", tag="8")]
    pub judge_item_config: ::std::collections::HashMap<::prost::alloc::string::String, MsgMtpJudgeItem>,
}
///MTP查询判据信息
///数据包类型:message_id=122
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryJudges {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
}
///MTP查询判据信息响应
///数据包类型:message_id=123
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryJudgesCack {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///key为判据guid，判据信息列表
    #[prost(map="string, message", tag="3")]
    pub map_judges: ::std::collections::HashMap<::prost::alloc::string::String, MsgMtpJudge>,
}
///MTP指令跳转项信息
///数据包类型:message_id=124
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpCommandSwitchItem {
    ///指令跳转项guid(数据库中)
    #[prost(string, tag="1")]
    pub switch_item_guid: ::prost::alloc::string::String,
    ///指令跳转项描述
    #[prost(string, tag="2")]
    pub switch_item_desc: ::prost::alloc::string::String,
    ///成功跳转至测试项目内索引
    #[prost(uint32, tag="3")]
    pub switch_to_index: u32,
    ///指令跳转项算法(例如:LNSRangeCheck)
    #[prost(string, tag="4")]
    pub switch_item_function: ::prost::alloc::string::String,
    ///指令跳转项算法配置(key为V0,V1,V2...)例如（V0="XXX" V1="YYY" V2="ZZZ"）
    #[prost(map="string, string", tag="5")]
    pub map_function_config: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
///MTP跳转指令信息
///数据包类型:message_id=125
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpSwitchCommand {
    ///跳转指令guid(数据库中)
    #[prost(string, tag="1")]
    pub switch_command_guid: ::prost::alloc::string::String,
    ///指令跳转节点描述
    #[prost(string, tag="2")]
    pub switch_command_desc: ::prost::alloc::string::String,
    ///指令跳转项(key为指令跳转项guid)
    #[prost(map="string, message", tag="3")]
    pub map_command_switch_items: ::std::collections::HashMap<::prost::alloc::string::String, MsgMtpCommandSwitchItem>,
}
///MTP查询跳转指令信息
///数据包类型:message_id=126
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQuerySwitchCommands {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
}
///MTP查询判据信息响应
///数据包类型:message_id=127
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQuerySwitchCommandsCack {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///key为跳转指令guid，跳转指令信息列表
    #[prost(map="string, message", tag="3")]
    pub map_switch_commands: ::std::collections::HashMap<::prost::alloc::string::String, MsgMtpSwitchCommand>,
}
///MTP无指令判据信息
///数据包类型:message_id=128
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpNoCommandJudgeItem {
    ///判据guid(数据库中)
    #[prost(string, tag="1")]
    pub judge_guid: ::prost::alloc::string::String,
    ///判据code
    #[prost(string, tag="2")]
    pub judge_code: ::prost::alloc::string::String,
    ///判据描述
    #[prost(string, tag="3")]
    pub judge_desc: ::prost::alloc::string::String,
    ///是否影响序列执行（0x30=不影响  0x31=影响）
    #[prost(uint32, tag="4")]
    pub affect_sequence_flag: u32,
    ///判据判读项算法(例如:LNSRangeCheck)
    #[prost(string, tag="5")]
    pub judge_function: ::prost::alloc::string::String,
    ///判据算法配置(key为V0,V1,V2...)例如（V0="XXX" V1="YYY" V2="ZZZ"）
    #[prost(map="string, string", tag="6")]
    pub map_function_config: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
///MTP查询无指令判据信息
///数据包类型:message_id=129
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryNoCommandJudges {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
}
///MTP查询无指令判据信息响应
///数据包类型:message_id=130
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpQueryNoCommandJudgesCack {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///对应卫星编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///无指令判据算法配置(key为无指令判据信息judge_guid)
    #[prost(map="string, message", tag="3")]
    pub map_usu_judges: ::std::collections::HashMap<::prost::alloc::string::String, MsgMtpNoCommandJudgeItem>,
}
