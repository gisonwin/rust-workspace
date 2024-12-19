///公共部分
///卫星信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubMStarInfo {
    ///型号名称
    #[prost(string, tag="1")]
    pub s_style_name: ::prost::alloc::string::String,
    ///型号代号
    #[prost(string, tag="2")]
    pub s_style_code: ::prost::alloc::string::String,
    ///型号ID
    #[prost(int32, tag="3")]
    pub i_style_id: i32,
    ///该型号下的直属卫星，可以有多个，也可以是0
    #[prost(message, repeated, tag="4")]
    pub n_star: ::prost::alloc::vec::Vec<pub_m_star_info::SStar>,
    ///该型号下的批次，可以有多个，也可以是0
    #[prost(message, repeated, tag="5")]
    pub n_batch: ::prost::alloc::vec::Vec<pub_m_star_info::SBatch>,
}
/// Nested message and enum types in `PUB_MStarInfo`.
pub mod pub_m_star_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SStar {
        ///卫星名称  全局不唯一 型号内唯一
        #[prost(string, tag="1")]
        pub s_star_name: ::prost::alloc::string::String,
        ///卫星代号  全局不唯一 型号内唯一
        #[prost(string, tag="2")]
        pub s_star_code: ::prost::alloc::string::String,
        ///卫星ID
        #[prost(int32, tag="3")]
        pub i_star_id: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SBatch {
        ///批次名称  全局不唯一 型号内唯一
        #[prost(string, tag="1")]
        pub s_batch_name: ::prost::alloc::string::String,
        ///批次代号  全局不唯一 型号内唯一
        #[prost(string, tag="2")]
        pub s_batch_code: ::prost::alloc::string::String,
        ///批次ID
        #[prost(int32, tag="3")]
        pub i_batch_id: i32,
        ///该批次下的卫星
        #[prost(message, repeated, tag="4")]
        pub n_star: ::prost::alloc::vec::Vec<SStar>,
    }
}
///全局配置结构体定义
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubGlobalinfo {
    ///主键
    #[prost(string, tag="1")]
    pub str_key: ::prost::alloc::string::String,
    ///名称
    #[prost(string, tag="2")]
    pub str_name: ::prost::alloc::string::String,
    ///值
    #[prost(string, tag="3")]
    pub str_value: ::prost::alloc::string::String,
    ///备注
    #[prost(string, tag="4")]
    pub str_mark: ::prost::alloc::string::String,
}
//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////
///指令下参数携带的数据结构，属性结构体
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Param {
    ///数据类别
    #[prost(enumeration="PubEDataType", tag="1")]
    pub n_data_kind: i32,
    #[prost(oneof="param::Value", tags="2, 3, 4, 5, 6, 7, 8, 9")]
    pub value: ::core::option::Option<param::Value>,
}
/// Nested message and enum types in `Param`.
pub mod param {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(int32, tag="2")]
        VInt32(i32),
        #[prost(uint32, tag="3")]
        VUint32(u32),
        #[prost(int64, tag="4")]
        VInt64(i64),
        #[prost(uint64, tag="5")]
        VUint64(u64),
        #[prost(float, tag="6")]
        VFloat(f32),
        #[prost(double, tag="7")]
        VDouble(f64),
        #[prost(string, tag="8")]
        VString(::prost::alloc::string::String),
        #[prost(bytes, tag="9")]
        VBinary(::prost::alloc::vec::Vec<u8>),
    }
}
///参数map一个指令可以有多个参数
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapParam {
    ///参数代号，参数值
    #[prost(map="string, message", tag="1")]
    pub map_param: ::std::collections::HashMap<::prost::alloc::string::String, Param>,
}
///全局登录 任何软件作为客户端连接成功后，首先发送登录信息便于管理
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TstLogin {
    ///软件代号，每个软件都有自己的代号
    #[prost(string, tag="1")]
    pub str_soft_code: ::prost::alloc::string::String,
    ///软件实例的GUID，唯一标识
    #[prost(string, tag="2")]
    pub str_soft_guid: ::prost::alloc::string::String,
    ///登录账号  可以没有，例如服务类软件
    #[prost(string, tag="3")]
    pub str_user_id: ::prost::alloc::string::String,
    ///登录用户名 可以没有
    #[prost(string, tag="4")]
    pub str_user_name: ::prost::alloc::string::String,
    ///本机监听地址
    #[prost(string, tag="5")]
    pub str_soft_local_ip: ::prost::alloc::string::String,
    ///本机监听端口号
    #[prost(int32, tag="6")]
    pub i_soft_local_port: i32,
}
///全局退出 任何软件作为客户端连接成功后，正常退出时发送退出信息便于管理
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TstLogoff {
    ///软件代号，每个软件都有自己的代号
    #[prost(string, tag="1")]
    pub str_soft_code: ::prost::alloc::string::String,
    ///软件实例的GUID，唯一标识
    #[prost(string, tag="2")]
    pub str_soft_guid: ::prost::alloc::string::String,
    ///登录账号  可以没有，例如服务类软件
    #[prost(string, tag="3")]
    pub str_user_id: ::prost::alloc::string::String,
    ///登录用户名 可以没有
    #[prost(string, tag="4")]
    pub str_user_name: ::prost::alloc::string::String,
    ///本机监听地址
    #[prost(string, tag="5")]
    pub str_soft_local_ip: ::prost::alloc::string::String,
    ///本机监听端口号
    #[prost(int32, tag="6")]
    pub i_soft_local_port: i32,
}
///数据类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PubEDataType {
    TInt32 = 0,
    TUint32 = 1,
    TInt64 = 2,
    TUint64 = 3,
    TFloat = 4,
    TDouble = 5,
    TString = 6,
    TBytes = 7,
}
///设备实例运行状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PubEDevState {
    ///驱动已经启动
    Load = 0,
    ///驱动加载中......
    Loading = 1,
    ///驱动已经卸载(或停止)
    Unload = 2,
    ///加载失败(启动失败)
    Loadfail = 3,
    ///与硬件建立通信
    Devon = 4,
    ///与硬件未建立通信
    Devoff = 5,
    ///与硬件通信异常
    Devbreak = 6,
    ///与硬件建立通信(共享)
    Devshareon = 7,
    ///与硬件未建立通信(共享)
    Devshareoff = 8,
    ///与硬件通信异常(共享)
    Devsharebreak = 9,
    ///该设备不存在
    DevNone = 10,
}
///操作系统平台枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PubEplatform {
    ///windows操作系统
    Windows = 0,
    ///linux操作系统
    Linux = 1,
}
///语言枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PubElanguage {
    ///中文
    Chinese = 0,
    ///英文
    English = 1,
}
