//公共部分

///获取MTP资源消息-请求
///(客户端与[资源管理服务]建立TCP/IP连接后，客户端主动发送)
///数据包类型:message_id=1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestMtpResource {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///用户标志
    #[prost(string, tag="2")]
    pub user_code: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="3")]
    pub star_code: u64,
    ///终端类型  1=TCC  2=MON  3=设备前端
    #[prost(uint32, tag="4")]
    pub terminal_type: u32,
}
///获取MTP资源消息-响应
///数据包类型:message_id=2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRespondMtpResource {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///结果标志  0=成功,其他值失败
    #[prost(int32, tag="2")]
    pub result: i32,
    ///用户编号(请求包中信息，原样返回)
    #[prost(string, tag="3")]
    pub user_code: ::prost::alloc::string::String,
    ///产品(卫星)编号(请求包中信息，原样返回)
    #[prost(uint64, tag="4")]
    pub star_code: u64,
    ///MTP资源IP
    #[prost(string, tag="5")]
    pub mtp_resource_ip: ::prost::alloc::string::String,
    ///MTP资源监听端口
    #[prost(uint32, tag="6")]
    pub mtp_resource_port: u32,
    ///MTP资源code
    #[prost(string, tag="7")]
    pub mtp_resource_code: ::prost::alloc::string::String,
    ///MTP资源IP(内)
    #[prost(string, tag="8")]
    pub mtp_resource_ip_lan: ::prost::alloc::string::String,
}
///登录消息-请求
///(客户端与[MTP资源]建立TCP/IP连接后，客户端主动发送登录消息)
///数据包类型:message_id=3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestLogon {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///用户编号
    #[prost(string, tag="2")]
    pub user_code: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="3")]
    pub star_code: u64,
    ///使用查询MTP资源管理服务返回的MTPcode
    #[prost(string, tag="4")]
    pub mtp_code: ::prost::alloc::string::String,
    ///终端类型  1=TCC  2=MON  3=设备前端
    #[prost(uint32, tag="5")]
    pub terminal_type: u32,
    ///登录时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="6")]
    pub logon_time: ::prost::alloc::string::String,
}
///登录消息-响应
///数据包类型:message_id=4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRespondLogon {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///结果标志  0=成功,其他值失败
    #[prost(int32, tag="2")]
    pub result: i32,
    ///用户编号(请求包中信息，原样返回)
    #[prost(string, tag="3")]
    pub user_code: ::prost::alloc::string::String,
    ///产品(卫星)编号(请求包中信息，原样返回)
    #[prost(uint64, tag="4")]
    pub star_code: u64,
    ///描述信息
    #[prost(string, tag="5")]
    pub logon_desc: ::prost::alloc::string::String,
    ///登录响应时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="6")]
    pub respond_logon_time: ::prost::alloc::string::String,
}
///测试初始化-请求
///数据包类型:message_id=5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestInit {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///用户编号
    #[prost(string, tag="2")]
    pub user_code: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="3")]
    pub star_code: u64,
    ///测试初始化方式：1=初始化 2=重新初始化 3=测试退出
    #[prost(uint32, tag="4")]
    pub init_mode: u32,
    ///使用测试数据库方式：1=中心数据库     2=离线数据库
    #[prost(uint32, tag="5")]
    pub use_db_mode: u32,
    ///主控权限：1=有主控权限  0=无主控权限
    #[prost(uint32, tag="6")]
    pub master_control: u32,
    ///初始化时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="7")]
    pub init_time: ::prost::alloc::string::String,
    ///测试阶段
    #[prost(uint32, tag="8")]
    pub test_level: u32,
    ///测试工位（GUID）
    #[prost(string, tag="9")]
    pub test_position: ::prost::alloc::string::String,
    ///用户名称
    #[prost(string, tag="10")]
    pub user_name: ::prost::alloc::string::String,
    ///测试阶段(名称)
    #[prost(string, tag="11")]
    pub test_level_name: ::prost::alloc::string::String,
    ///测试工位名称
    #[prost(string, tag="12")]
    pub test_position_name: ::prost::alloc::string::String,
}
///测试初始化-响应
///数据包类型:message_id=6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRespondInit {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///结果标志  0=成功,其他值失败
    #[prost(int32, tag="3")]
    pub result: i32,
    ///描述信息
    #[prost(string, tag="4")]
    pub init_desc: ::prost::alloc::string::String,
    ///响应初始化时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="5")]
    pub respond_init_time: ::prost::alloc::string::String,
}
///发送指令-请求(指令节点或者手工指令)
///数据包类型:message_id=7
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendCommand {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///指令代号(一致性比对字段1)
    #[prost(string, tag="3")]
    pub command_code: ::prost::alloc::string::String,
    ///发送指令超时时间
    #[prost(uint32, tag="4")]
    pub send_conmand_outtime: u32,
    ///指令前判据代号(无判据时，填空)
    #[prost(string, tag="5")]
    pub pre_judge_code: ::prost::alloc::string::String,
    ///指令后判据代号(无判据时，填空)
    #[prost(string, tag="6")]
    pub post_judge_code: ::prost::alloc::string::String,
    ///后判超时时间
    #[prost(uint32, tag="7")]
    pub post_judge_outtime: u32,
    ///指令发送时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="8")]
    pub send_command_time: ::prost::alloc::string::String,
    ///产品(卫星)名称
    #[prost(string, tag="9")]
    pub star_name: ::prost::alloc::string::String,
    ///指令名称(一致性比对字段2)
    #[prost(string, tag="10")]
    pub command_name: ::prost::alloc::string::String,
    ///用户名称
    #[prost(string, tag="11")]
    pub user_name: ::prost::alloc::string::String,
    ///发送类型：手工指令| 快速序列 |测试序列
    #[prost(string, tag="12")]
    pub send_type: ::prost::alloc::string::String,
    ///指令类型(一致性比对字段3)
    #[prost(uint32, tag="13")]
    pub command_type: u32,
    ///指令长度(一致性比对字段4)
    #[prost(uint32, tag="14")]
    pub command_len: u32,
    ///指令码字(一致性比对字段5)
    #[prost(string, tag="15")]
    pub command_data_block: ::prost::alloc::string::String,
}
///发送指令-响应
///数据包类型:message_id=8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCommandCack {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///结果标志  0x30=成功,其他值失败
    #[prost(uint32, tag="3")]
    pub command_result: u32,
    ///描述信息
    #[prost(string, tag="4")]
    pub cack_desc: ::prost::alloc::string::String,
    ///响应指令时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="5")]
    pub cack_command_time: ::prost::alloc::string::String,
    ///指令code
    #[prost(string, tag="6")]
    pub command_code: ::prost::alloc::string::String,
    ///指令数据块
    #[prost(string, tag="7")]
    pub command_data_block: ::prost::alloc::string::String,
    ///公司设备资产编号(多设备时，设备之间用'|'分割)
    #[prost(string, tag="8")]
    pub dev_resource_code: ::prost::alloc::string::String,
    ///设备通道描述(多通道时，设备之间用'|'分割)
    #[prost(string, tag="9")]
    pub dev_channel_code: ::prost::alloc::string::String,
    ///设备名称(多设备时，设备之间用'|'分割)
    #[prost(string, tag="10")]
    pub dev_name: ::prost::alloc::string::String,
    ///测试阶段（由MTP返回，设备可以填空）
    #[prost(string, tag="11")]
    pub test_level: ::prost::alloc::string::String,
    ///后判失败指示标志  0x99=后判失败,其他值无意义，外部填0
    #[prost(uint32, tag="12")]
    pub post_judge_error_flag: u32,
}
///判据请求(无指令判据节点)
///数据包类型:message_id=9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJudgeRequest {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///判据代号(无指令判据)
    #[prost(string, tag="3")]
    pub judge_code: ::prost::alloc::string::String,
    ///0x30=不影响、0x31=影响
    #[prost(uint32, tag="4")]
    pub affect_sequence_flag: u32,
}
///判据结果
///数据包类型:message_id=10
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJudgeResult {
    ///业务流水号(请求指令请求包或者独立判据请求[无指令判读节点]包信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号(请求指令请求包或者独立判据请求[无指令判读节点]包信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///0x30=前判判据、0x31=后判判据、0x32=无指令判据
    #[prost(uint32, tag="3")]
    pub judge_type: u32,
    ///0x30=不影响、0x31=影响
    #[prost(uint32, tag="4")]
    pub affect_sequence_flag: u32,
    ///判据代号(请求指令请求包或者独立判据请求[无指令判读节点]包信息，原样返回)
    #[prost(string, tag="5")]
    pub judge_code: ::prost::alloc::string::String,
    ///判据结果 0x30=判据成功；其他值，表示异常。约定典型异常原因。
    #[prost(uint32, tag="6")]
    pub judge_result: u32,
    ///判据结果描述
    #[prost(string, tag="7")]
    pub judge_result_desc: ::prost::alloc::string::String,
}
///遥测原码帧
///数据包类型:message_id=11
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTmFrame {
    ///产品(卫星)编号
    #[prost(uint64, tag="1")]
    pub star_code: u64,
    ///原码帧数据
    #[prost(bytes="vec", tag="2")]
    pub tm_frame_data: ::prost::alloc::vec::Vec<u8>,
    ///0=正确   1=错误
    #[prost(uint32, tag="3")]
    pub crc_result: u32,
    ///测试阶段
    #[prost(string, tag="4")]
    pub test_level: ::prost::alloc::string::String,
    ///公司设备资产编号
    #[prost(string, tag="5")]
    pub dev_resource_code: ::prost::alloc::string::String,
    ///设备通道描述
    #[prost(string, tag="6")]
    pub dev_channel_code: ::prost::alloc::string::String,
    ///数据源
    #[prost(string, tag="7")]
    pub data_source: ::prost::alloc::string::String,
    ///星上时间
    #[prost(uint32, tag="8")]
    pub star_time: u32,
    ///终端采集时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="9")]
    pub gather_time: ::prost::alloc::string::String,
    ///数据类型(0x31=遥测数据，其他待定)
    #[prost(uint32, tag="10")]
    pub data_type: u32,
    ///数据标签(0x30=实时数据，0x31=回放数据，0x32=回灌数据)
    #[prost(uint32, tag="11")]
    pub data_tag: u32,
    ///数据存储标志(0x30=不存储，0x31=存储)
    #[prost(uint32, tag="12")]
    pub data_save_flag: u32,
    ///MTP内部存储（雪花ID），同步帧、分包和参数，外部填0
    #[prost(uint64, tag="13")]
    pub frame_guid: u64,
    ///帧计数,MTP内部存储,外部填0
    #[prost(uint32, tag="14")]
    pub frame_count: u32,
}
///遥测分包
///数据包类型:message_id=12
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTmPackage {
    ///产品(卫星)编号
    #[prost(uint64, tag="1")]
    pub star_code: u64,
    ///遥测分包标识
    #[prost(uint32, tag="2")]
    pub tm_pkg_id: u32,
    ///遥测分包代号
    #[prost(string, tag="3")]
    pub tm_pkg_code: ::prost::alloc::string::String,
    ///分包头数据(6字节)
    #[prost(bytes="vec", tag="4")]
    pub tm_pkg_head: ::prost::alloc::vec::Vec<u8>,
    ///分包遥测数据
    #[prost(bytes="vec", tag="5")]
    pub tm_pkg_data: ::prost::alloc::vec::Vec<u8>,
    ///延时实时标识(默认0x30实时)  0x30实时  0x31延时
    #[prost(uint32, tag="6")]
    pub tm_pkg_flag: u32,
    ///测试阶段
    #[prost(string, tag="7")]
    pub test_level: ::prost::alloc::string::String,
    ///公司设备资产编号
    #[prost(string, tag="8")]
    pub dev_resource_code: ::prost::alloc::string::String,
    ///设备通道描述
    #[prost(string, tag="9")]
    pub dev_channel_code: ::prost::alloc::string::String,
    ///数据源
    #[prost(string, tag="10")]
    pub data_source: ::prost::alloc::string::String,
    ///星上时间
    #[prost(uint32, tag="11")]
    pub star_time: u32,
    ///终端采集时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="12")]
    pub gather_time: ::prost::alloc::string::String,
    ///数据标签(0x30=实时数据，0x31=回放数据，0x32=回灌数据)
    #[prost(uint32, tag="13")]
    pub data_tag: u32,
    ///数据存储标志(0x30=不存储，0x31=存储)
    #[prost(uint32, tag="14")]
    pub data_save_flag: u32,
    ///MTP内部存储（雪花ID），同步帧、分包和参数，外部填0
    #[prost(uint64, tag="15")]
    pub frame_guid: u64,
    ///MTP内部存储，外部填0
    #[prost(int32, tag="16")]
    pub tm_id_result: i32,
}
///参数项
///数据包类型:message_id=13
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgParamItem {
    ///参数代号
    #[prost(string, tag="1")]
    pub param_code: ::prost::alloc::string::String,
    ///参数名称，可不填
    #[prost(string, tag="2")]
    pub param_name: ::prost::alloc::string::String,
    ///参数原码值
    #[prost(uint64, tag="3")]
    pub raw_value: u64,
    ///参数工程值数据类别
    #[prost(int32, tag="4")]
    pub eng_value_type: i32,
    ///状态值,默认填0
    #[prost(int32, tag="13")]
    pub status: i32,
    ///延时实时标识(默认0x30实时)  0x30实时  0x31延时
    #[prost(uint32, tag="14")]
    pub tm_pkg_flag: u32,
    ///参数id(唯一id)
    #[prost(string, tag="15")]
    pub param_id: ::prost::alloc::string::String,
    ///工程值描述(解码配置)
    #[prost(string, tag="16")]
    pub value_desc: ::prost::alloc::string::String,
    ///工程值判读(解码模块)
    #[prost(string, tag="17")]
    pub value_judge: ::prost::alloc::string::String,
    #[prost(oneof="msg_param_item::EngValue", tags="5, 6, 7, 8, 9, 10, 11, 12")]
    pub eng_value: ::core::option::Option<msg_param_item::EngValue>,
}
/// Nested message and enum types in `msgParamItem`.
pub mod msg_param_item {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EngValue {
        ///eng_value_type=1
        #[prost(int32, tag="5")]
        VInt32(i32),
        ///eng_value_type=2
        #[prost(uint32, tag="6")]
        VUint32(u32),
        ///eng_value_type=3
        #[prost(int64, tag="7")]
        VInt64(i64),
        ///eng_value_type=4
        #[prost(uint64, tag="8")]
        VUint64(u64),
        ///eng_value_type=5
        #[prost(float, tag="9")]
        VFloat(f32),
        ///eng_value_type=6
        #[prost(double, tag="10")]
        VDouble(f64),
        ///eng_value_type=7
        #[prost(string, tag="11")]
        VString(::prost::alloc::string::String),
        ///eng_value_type=8
        #[prost(bytes, tag="12")]
        VBinary(::prost::alloc::vec::Vec<u8>),
    }
}
///参数项集合(Map型)
///数据包类型:message_id=14
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgParamPackagesMap {
    ///产品(卫星)编号
    #[prost(uint64, tag="1")]
    pub star_code: u64,
    #[prost(map="string, message", tag="2")]
    pub map_param_packages: ::std::collections::HashMap<::prost::alloc::string::String, MsgParamItem>,
    ///测试阶段
    #[prost(string, tag="3")]
    pub test_level: ::prost::alloc::string::String,
    ///公司设备资产编号
    #[prost(string, tag="4")]
    pub dev_resource_code: ::prost::alloc::string::String,
    ///设备通道描述
    #[prost(string, tag="5")]
    pub dev_channel_code: ::prost::alloc::string::String,
    ///数据源
    #[prost(string, tag="6")]
    pub data_source: ::prost::alloc::string::String,
    ///星上时间
    #[prost(uint32, tag="7")]
    pub star_time: u32,
    ///终端采集时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="8")]
    pub gather_time: ::prost::alloc::string::String,
    ///数据标签(0x30=实时数据，0x31=回放数据，0x32=回灌数据)
    #[prost(uint32, tag="9")]
    pub data_tag: u32,
    ///数据存储标志(0x30=不存储，0x31=存储)
    #[prost(uint32, tag="10")]
    pub data_save_flag: u32,
    ///MTP内部存储（雪花ID），同步帧、分包和参数，外部填0
    #[prost(uint64, tag="11")]
    pub frame_guid: u64,
}
///参数项集合(数组型)
///数据包类型:message_id=15
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgParamPackagesArray {
    ///产品(卫星)编号
    #[prost(uint64, tag="1")]
    pub star_code: u64,
    #[prost(message, repeated, tag="2")]
    pub map_param_packages: ::prost::alloc::vec::Vec<MsgParamItem>,
    ///测试阶段
    #[prost(string, tag="3")]
    pub test_level: ::prost::alloc::string::String,
    ///公司设备资产编号
    #[prost(string, tag="4")]
    pub dev_resource_code: ::prost::alloc::string::String,
    ///设备通道描述
    #[prost(string, tag="5")]
    pub dev_channel_code: ::prost::alloc::string::String,
    ///数据源
    #[prost(string, tag="6")]
    pub data_source: ::prost::alloc::string::String,
    ///星上时间
    #[prost(uint32, tag="7")]
    pub star_time: u32,
    ///终端采集时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="8")]
    pub gather_time: ::prost::alloc::string::String,
    ///数据标签(0x30=实时数据，0x31=回放数据，0x32=回灌数据)
    #[prost(uint32, tag="9")]
    pub data_tag: u32,
    ///数据存储标志(0x30=不存储，0x31=存储)
    #[prost(uint32, tag="10")]
    pub data_save_flag: u32,
    ///MTP内部存储（雪花ID），同步帧、分包和参数，外部填0
    #[prost(uint64, tag="11")]
    pub frame_guid: u64,
}
///测试序列结果记录
///数据包类型:message_id=16
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTestSequenceResultRecord {
    ///产品(卫星)编号
    #[prost(uint64, tag="1")]
    pub star_code: u64,
    ///本次测试开始到终止为一次测试，这个过程中只有一个GUID就是本字段
    #[prost(uint64, tag="2")]
    pub test_ins_guid: u64,
    ///测试项目guid，填写名称，由uint64变为string
    #[prost(string, tag="3")]
    pub test_item_guid: ::prost::alloc::string::String,
    ///测试项目父guid，填写名称，由uint64变为string
    #[prost(string, tag="4")]
    pub item_parent_guid: ::prost::alloc::string::String,
    ///测试项目序列内顺序索引
    #[prost(uint32, tag="5")]
    pub test_item_index: u32,
    ///节点类型  1=指令;2=无指令判据,3延时，4交互，5记录参数，6跳转指令。填写名称，由int32变为string
    #[prost(string, tag="6")]
    pub node_type: ::prost::alloc::string::String,
    ///填写指令代号_指令名称，由uint64变为string
    #[prost(string, tag="7")]
    pub command_guid: ::prost::alloc::string::String,
    ///前判据号guid，填写前判据结果，由uint64变为string
    #[prost(string, tag="8")]
    pub pre_judage_guid: ::prost::alloc::string::String,
    ///后判据号guid，填写前判据结果，由uint64变为string
    #[prost(string, tag="9")]
    pub next_judage_guid: ::prost::alloc::string::String,
    ///0=成功(与节点类型有关，处理逻辑暂不变)  1或其他值=失败
    #[prost(int32, tag="10")]
    pub test_resukt: i32,
    ///测试结果内容
    #[prost(string, tag="11")]
    pub test_content: ::prost::alloc::string::String,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="12")]
    pub test_time: ::prost::alloc::string::String,
    ///用户名称
    #[prost(string, tag="13")]
    pub test_use_name: ::prost::alloc::string::String,
}
///参数结果请求
///数据包类型:message_id=17
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgParamResultRequest {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///参数列表,例如:SS001 SS002 SS003
    #[prost(string, repeated, tag="2")]
    pub param_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///产品(卫星)编号
    #[prost(uint64, tag="3")]
    pub star_code: u64,
}
///参数结果请求(响应)
///数据包类型:message_id=18
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgParamResultRepond {
    ///业务流水号（参数结果请求,原样返回）
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///参数结果列表列表  格式：参数代号=参数工程值,例如:SS001=5.8 SS002=42.5 SS003=0.66
    #[prost(string, repeated, tag="2")]
    pub param_result: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///产品(卫星)编号（参数结果请求,原样返回）
    #[prost(uint64, tag="3")]
    pub star_code: u64,
}
///指令日志
///数据包类型:message_id=19
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCommandLog {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///指令ID
    #[prost(string, tag="3")]
    pub command_id: ::prost::alloc::string::String,
    ///用户
    #[prost(string, tag="4")]
    pub user: ::prost::alloc::string::String,
    ///指令发送时间:格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="5")]
    pub send_time: ::prost::alloc::string::String,
    ///操作结果
    #[prost(uint32, tag="6")]
    pub command_result: u32,
    ///指令代号
    #[prost(string, tag="7")]
    pub command_code: ::prost::alloc::string::String,
    ///指令名称
    #[prost(string, tag="8")]
    pub command_name: ::prost::alloc::string::String,
    ///指令数据块
    #[prost(bytes="vec", tag="9")]
    pub command_data: ::prost::alloc::vec::Vec<u8>,
    ///指令类别
    #[prost(string, tag="10")]
    pub command_type: ::prost::alloc::string::String,
    ///所属类别
    #[prost(string, tag="11")]
    pub one_class: ::prost::alloc::string::String,
    ///所属分系统
    #[prost(string, tag="12")]
    pub two_class: ::prost::alloc::string::String,
    ///前判结果
    #[prost(string, tag="13")]
    pub pre_judge_result: ::prost::alloc::string::String,
    ///后判结果
    #[prost(string, tag="14")]
    pub post_judge_result: ::prost::alloc::string::String,
    ///测试阶段
    #[prost(string, tag="15")]
    pub test_level: ::prost::alloc::string::String,
    ///公司设备资产编号
    #[prost(string, tag="16")]
    pub dev_resource_code: ::prost::alloc::string::String,
    ///设备通道描述
    #[prost(string, tag="17")]
    pub dev_channel_code: ::prost::alloc::string::String,
    ///设备名称
    #[prost(string, tag="18")]
    pub dev_name: ::prost::alloc::string::String,
    ///发送类型：手工指令| 快速序列 |流程指令
    #[prost(string, tag="19")]
    pub send_type: ::prost::alloc::string::String,
    ///数据标签(0x30=实时数据，0x31=回放数据，0x32=回灌数据)
    #[prost(uint32, tag="20")]
    pub data_tag: u32,
    ///数据存储标志(0x30=不存储，0x31=存储)
    #[prost(uint32, tag="21")]
    pub data_save_flag: u32,
}
///常监视控制
///数据包类型:message_id=21
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConstMonitorControl {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///0x30=停止常监视功能，0x31启动常监视功能，0x32查询常监视功能状态
    #[prost(uint32, tag="3")]
    pub control_command: u32,
    ///模块ID（所属）
    #[prost(string, tag="4")]
    pub page_id: ::prost::alloc::string::String,
}
///常监视控制响应
///数据包类型:message_id=22
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConstMonitorControlCack {
    ///业务流水号（原样返回）
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号（原样返回）
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///（原样返回）
    #[prost(uint32, tag="3")]
    pub control_command: u32,
    ///0x30=停止常监视功能，0x31启动常监视功能,其他值失败
    #[prost(uint32, tag="4")]
    pub control_result: u32,
    ///模块ID（原样返回）
    #[prost(string, tag="5")]
    pub page_id: ::prost::alloc::string::String,
}
///常监视设置监视项
///数据包类型:message_id=23
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetConstMonitorItem {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///监视项code列表(一个或者多个),暂时使用数据库中GU_ID[2023.1.5]
    #[prost(string, repeated, tag="3")]
    pub monitor_item_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///模块ID（所属）
    #[prost(string, tag="4")]
    pub page_id: ::prost::alloc::string::String,
}
///常监视设置监视项响应
///数据包类型:message_id=24
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetConstMonitorItemCack {
    ///业务流水号（原样返回）
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号（原样返回）
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///0=成功,其他值失败
    #[prost(uint32, tag="3")]
    pub monitor_config_result: u32,
    ///模块ID（原样返回）
    #[prost(string, tag="4")]
    pub page_id: ::prost::alloc::string::String,
}
///常监视结果
///数据包类型:message_id=25
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConstMonitorResult {
    ///业务流水号(随机动态生成，一次报警一个)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///0x30仅进行报警信息显示，0x31=报警结果影响正在执行的序列，同时进行报警信息展示
    #[prost(uint32, tag="3")]
    pub monitor_item_flag: u32,
    ///监视项code
    #[prost(string, tag="4")]
    pub monitor_item_code: ::prost::alloc::string::String,
    ///常监视结果描述
    #[prost(string, tag="5")]
    pub monitor_item_desc: ::prost::alloc::string::String,
    ///模块ID（所属）
    #[prost(string, tag="6")]
    pub page_id: ::prost::alloc::string::String,
    ///产品(卫星)名称
    #[prost(string, tag="7")]
    pub star_name: ::prost::alloc::string::String,
    ///报警时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="8")]
    pub warn_time: ::prost::alloc::string::String,
    ///监视项名称
    #[prost(string, tag="9")]
    pub monitor_item_name: ::prost::alloc::string::String,
}
///常监视（删除监视页）
///数据包类型:message_id=26
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteConstMonitorPage {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///模块ID（所属）
    #[prost(string, tag="3")]
    pub page_id: ::prost::alloc::string::String,
}
///常监视（查询监视页）
///数据包类型:message_id=28
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgQueryMonitorPage {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
}
///常监视设置监视项(响应28号消息)
///数据包类型:message_id=29
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMonitorPageCack {
    ///业务流水号（原样返回）
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号（原样返回）
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///监视页列表page_id<--->page_desc
    #[prost(map="string, string", tag="3")]
    pub pages: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
///跳转指令
///数据包类型:message_id=31
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwitchCommand {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///跳转指令节点code
    #[prost(uint64, tag="3")]
    pub switch_node_code: u64,
}
///跳转指令响应
///数据包类型:message_id=32
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwitchCommandCack {
    ///业务流水号（原样返回）
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号（原样返回）
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///跳转指令结果  0=成功，其他值失败
    #[prost(uint32, tag="3")]
    pub switch_result: u32,
    ///跳转至其他节点索引(跳转指令结果=0有效)[废弃不用]
    #[prost(uint32, tag="4")]
    pub switch_to_index: u32,
    ///跳转信息描述
    #[prost(string, tag="5")]
    pub switch_desc: ::prost::alloc::string::String,
    ///跳转至其他节点GUID(跳转指令结果=0有效)
    #[prost(uint64, tag="6")]
    pub switch_to_node_code: u64,
}
///TCC用户发送标志
///数据包类型:message_id=41
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTccUserSendFlag {
    ///TCC主控用户code
    #[prost(string, tag="1")]
    pub tcc_user_code: ::prost::alloc::string::String,
    ///0x30=禁止发送  0x31=允许发送
    #[prost(uint32, tag="2")]
    pub send_flag: u32,
    ///主控权限：1=有主控权限  0=无主控权限
    #[prost(uint32, tag="3")]
    pub master_control: u32,
}
///TCC主控用户及TCC用户列表
///数据包类型:message_id=42
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTccUserList {
    ///产品(卫星)编号
    #[prost(uint64, tag="1")]
    pub star_code: u64,
    ///TCC主控用户code
    #[prost(string, tag="2")]
    pub master_control_user_code: ::prost::alloc::string::String,
    ///TCC用户状态列表
    #[prost(message, repeated, tag="3")]
    pub tcc_user_list: ::prost::alloc::vec::Vec<MsgTccUserSendFlag>,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="4")]
    pub modify_time: ::prost::alloc::string::String,
}
///设置TCC用户发送标志(只允许主控用户发送)
///数据包类型:message_id=43
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetTccUserSendFlag {
    ///产品(卫星)编号
    #[prost(uint64, tag="1")]
    pub star_code: u64,
    ///TCC主控用户code
    #[prost(string, tag="2")]
    pub master_control_user_code: ::prost::alloc::string::String,
    ///TCC用户code
    #[prost(string, tag="3")]
    pub tcc_user_code: ::prost::alloc::string::String,
    ///0x30=禁止发送  0x31=允许发送
    #[prost(uint32, tag="4")]
    pub send_flag: u32,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="5")]
    pub modify_time: ::prost::alloc::string::String,
}
///改变TCC主控用户(只允许(当前)主控用户发送)
///数据包类型:message_id=44
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChangeTccControlUser {
    ///产品(卫星)编号
    #[prost(uint64, tag="1")]
    pub star_code: u64,
    ///TCC主控用户code(当前)
    #[prost(string, tag="2")]
    pub current_control_user_code: ::prost::alloc::string::String,
    ///TCC主控用户code(转移到)
    #[prost(string, tag="3")]
    pub future_control_user_code: ::prost::alloc::string::String,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="4")]
    pub modify_time: ::prost::alloc::string::String,
}
///设置TMTC状态
///数据包类型:message_id=51
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetTmtcStatus {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///用户编号
    #[prost(string, tag="3")]
    pub user_code: ::prost::alloc::string::String,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="4")]
    pub set_time: ::prost::alloc::string::String,
    ///是否加密  1=加密  0=不加密
    #[prost(uint32, tag="5")]
    pub is_encryption: u32,
    ///是否加扰  1=加扰  0=不加扰
    #[prost(uint32, tag="6")]
    pub is_scramble: u32,
    ///是否优先加密  1=优先加密  0=不优先加密
    #[prost(uint32, tag="7")]
    pub is_priority_encryption: u32,
    ///是否解密  1=解密  0=不解密
    #[prost(uint32, tag="8")]
    pub is_decryption: u32,
    ///是否解扰  1=解扰  0=不解扰
    #[prost(uint32, tag="9")]
    pub is_descramble: u32,
    ///是否优先解扰  1=优先解扰  0=不优先解扰
    #[prost(uint32, tag="10")]
    pub is_priority_decryption: u32,
}
///设置TMTC状态CACK
///数据包类型:message_id=52
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetTmtcStatusCack {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///用户编号(请求包中信息，原样返回)
    #[prost(string, tag="3")]
    pub user_code: ::prost::alloc::string::String,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="4")]
    pub cack_time: ::prost::alloc::string::String,
    ///0=成功  其他值等于失败
    #[prost(uint32, tag="5")]
    pub cack_result: u32,
    ///返回标志  1=MTP返回  2=设备后台返回
    #[prost(uint32, tag="6")]
    pub cack_flag: u32,
}
///获取TMTC状态
///数据包类型:message_id=53
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGetTmtcStatus {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///用户编号
    #[prost(string, tag="3")]
    pub user_code: ::prost::alloc::string::String,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="4")]
    pub query_time: ::prost::alloc::string::String,
}
///获取TMTC状态CACK
///数据包类型:message_id=54
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGetTmtcStatusCack {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///用户编号(请求包中信息，原样返回)
    #[prost(string, tag="3")]
    pub user_code: ::prost::alloc::string::String,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="4")]
    pub cack_time: ::prost::alloc::string::String,
    ///0=成功  其他值等于失败
    #[prost(uint32, tag="5")]
    pub cack_result: u32,
    ///返回标志  1=MTP返回  2=设备后台返回
    #[prost(uint32, tag="6")]
    pub cack_flag: u32,
    ///是否加密  1=加密  0=不加密
    #[prost(uint32, tag="7")]
    pub is_encryption: u32,
    ///是否加扰  1=加扰  0=不加扰
    #[prost(uint32, tag="8")]
    pub is_scramble: u32,
    ///是否优先加密  1=优先加密  0=不优先加密
    #[prost(uint32, tag="9")]
    pub is_priority_encryption: u32,
    ///是否解密  1=解密  0=不解密
    #[prost(uint32, tag="10")]
    pub is_decryption: u32,
    ///是否解扰  1=解扰  0=不解扰
    #[prost(uint32, tag="11")]
    pub is_descramble: u32,
    ///是否优先解扰  1=优先解扰  0=不优先解扰
    #[prost(uint32, tag="12")]
    pub is_priority_decryption: u32,
}
///MTP资源测试退出通报(MTP接收到测试退出时，通报所有终端链路,然后关闭链路)
///终端链路暂不做逻辑处理，打印日志即可
///数据包类型:message_id=61
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpTestQuit {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="3")]
    pub test_quit_time: ::prost::alloc::string::String,
    ///描述信息
    #[prost(string, tag="4")]
    pub desc: ::prost::alloc::string::String,
}
///终端链路ping到MTP资源
///数据包类型:message_id=62
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPingMtp {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///终端信息(自定义，暂不要带汉字)
    #[prost(string, tag="3")]
    pub ping_information: ::prost::alloc::string::String,
    ///终端维护计数器
    #[prost(uint32, tag="4")]
    pub ping_count: u32,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="5")]
    pub ping_time: ::prost::alloc::string::String,
    ///备用设计，填空
    #[prost(string, tag="6")]
    pub command_line: ::prost::alloc::string::String,
}
///MTP资源响应终端链路ping
///数据包类型:message_id=63
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMtpPong {
    ///业务流水号(请求包中信息，原样返回)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///终端信息(自定义，暂不要带汉字)(请求包中信息，原样返回)
    #[prost(string, tag="3")]
    pub ping_information: ::prost::alloc::string::String,
    ///在终端维护计数器基础上加1
    #[prost(uint32, tag="4")]
    pub pong_count: u32,
    ///格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="5")]
    pub pong_time: ::prost::alloc::string::String,
    ///mtp信息(自定义，暂不要带汉字)
    #[prost(string, tag="6")]
    pub mtp_information: ::prost::alloc::string::String,
    ///备用设计，填空
    #[prost(string, tag="7")]
    pub command_result: ::prost::alloc::string::String,
}
///终止手工指令请求
///数据包类型:message_id=71
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTerminateManualCommand {
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///手工发送时的业务流水号
    #[prost(string, tag="3")]
    pub manual_water_id: ::prost::alloc::string::String,
    ///指令代号
    #[prost(string, tag="4")]
    pub command_code: ::prost::alloc::string::String,
    ///发送指令超时时间
    #[prost(uint32, tag="5")]
    pub send_conmand_outtime: u32,
    ///指令前判据代号(无判据时，填空)
    #[prost(string, tag="6")]
    pub pre_judge_code: ::prost::alloc::string::String,
    ///指令后判据代号(无判据时，填空)
    #[prost(string, tag="7")]
    pub post_judge_code: ::prost::alloc::string::String,
    ///后判超时时间
    #[prost(uint32, tag="8")]
    pub post_judge_outtime: u32,
    ///指令发送时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="9")]
    pub send_command_time: ::prost::alloc::string::String,
    ///产品(卫星)名称
    #[prost(string, tag="10")]
    pub star_name: ::prost::alloc::string::String,
    ///指令名称
    #[prost(string, tag="11")]
    pub command_name: ::prost::alloc::string::String,
    ///用户名称
    #[prost(string, tag="12")]
    pub user_name: ::prost::alloc::string::String,
    ///发送类型：手工指令
    #[prost(string, tag="13")]
    pub send_type: ::prost::alloc::string::String,
}
///终止手工指令响应
///数据包类型:message_id=72
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTerminateManualCommandCack {
    ///业务流水号((请求包中信息，原样返回))
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号(请求包中信息，原样返回)
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///结果标志  0x30=成功,其他值失败
    #[prost(uint32, tag="3")]
    pub result: u32,
    ///描述信息
    #[prost(string, tag="4")]
    pub desc: ::prost::alloc::string::String,
    ///终止手工指令响应时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="5")]
    pub cack_time: ::prost::alloc::string::String,
}
///设备指令响应
///数据包类型:message_id=78
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDevCommandCack {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///tcc业务流水号(请求包中信息[7号消息]，原样返回)
    #[prost(string, tag="2")]
    pub tcc_water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号(请求包中信息，原样返回)
    #[prost(uint64, tag="3")]
    pub star_code: u64,
    ///指令code
    #[prost(string, tag="4")]
    pub command_code: ::prost::alloc::string::String,
    ///设备响应指令时间[MTP接收前端服务返回时刻],格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="5")]
    pub dev_cack_time: ::prost::alloc::string::String,
    ///设备响应描述信息
    #[prost(string, tag="6")]
    pub dev_desc: ::prost::alloc::string::String,
}
///日志信息
///数据包类型:message_id=81
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgXLog {
    ///业务流水号
    #[prost(string, tag="1")]
    pub water_id: ::prost::alloc::string::String,
    ///产品(卫星)编号，无时可以填0
    #[prost(uint64, tag="2")]
    pub star_code: u64,
    ///目前填0
    #[prost(uint32, tag="3")]
    pub log_type: u32,
    ///描述信息
    #[prost(string, tag="4")]
    pub log: ::prost::alloc::string::String,
    ///日志时间,格式:YYYY-MM-DD hh:mm:ss.ms
    #[prost(string, tag="5")]
    pub log_time: ::prost::alloc::string::String,
}
