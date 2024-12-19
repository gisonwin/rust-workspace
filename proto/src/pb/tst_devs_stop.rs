///前端软件连接设备服务后，发送停止命令
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    ///20210915前端请求者的GUID（客户端代号）
    #[prost(string, tag="1")]
    pub str_ask_soft_guid: ::prost::alloc::string::String,
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="2")]
    pub water_id: ::prost::alloc::string::String,
    ///设备实例代号 添加by yanwh, 2022-07-19
    #[prost(string, repeated, tag="3")]
    pub str_dev_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///设备服务接收指令判读、计数减一，确定是否释放资源，向所连接的设备代理(代理再转给资源管理)、前端发送数据包
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Respond {
    ///设备服务名称
    #[prost(string, tag="1")]
    pub str_devs_name: ::prost::alloc::string::String,
    ///设备实例代号
    #[prost(string, tag="2")]
    pub str_dev_code: ::prost::alloc::string::String,
    ///true成功false失败
    #[prost(bool, tag="3")]
    pub b_is_ok: bool,
    ///失败原因
    #[prost(string, tag="4")]
    pub str_error: ::prost::alloc::string::String,
    ///引用计数
    #[prost(int32, tag="5")]
    pub i_reference_count: i32,
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="6")]
    pub water_id: ::prost::alloc::string::String,
}
