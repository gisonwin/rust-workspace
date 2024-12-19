///属性订阅 ，作为通用设计
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    ///设备code
    #[prost(string, tag="1")]
    pub str_dev_code: ::prost::alloc::string::String,
    ///false取消订阅true订阅
    #[prost(bool, tag="2")]
    pub b_sub_scribe: bool,
    ///订阅属性代号
    #[prost(string, repeated, tag="3")]
    pub str_param_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="4")]
    pub water_id: ::prost::alloc::string::String,
}
///属性订阅返回 ，成功后会不定期推送属性结构体TST_DEVS_PUSHPARAM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Respond {
    ///设备code
    #[prost(string, tag="1")]
    pub str_dev_code: ::prost::alloc::string::String,
    ///false取消订阅true订阅
    #[prost(bool, tag="2")]
    pub b_sub_scribe: bool,
    ///false失败true成功
    #[prost(bool, tag="3")]
    pub b_success: bool,
    ///失败返回的错误信息
    #[prost(string, tag="4")]
    pub str_error: ::prost::alloc::string::String,
    ///业务流水号(确保全局唯一，建议GUID字符串)
    #[prost(string, tag="5")]
    pub water_id: ::prost::alloc::string::String,
}
