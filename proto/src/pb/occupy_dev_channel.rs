//占有设备通道

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    ///是否占用
    #[prost(bool, tag="1")]
    pub b_occupy: bool,
    ///设备code
    #[prost(string, tag="2")]
    pub str_dev_code: ::prost::alloc::string::String,
    ///1.key表示该客户端准备占用的"通道号", value值忽略;
    ///a)如果没有通道概念:key(通道号)用"空字符串"表示
    ///b)如果通道有层级,可以用.连接表示,比如pxi的通道,2.3表示2号板卡的3号通道
    ///2.如果发现其中的某一通道被另一个客户端占用,那么即使其它通道未被占用,该客户端也不会占用该设备的任何一个通道
    ///3.收到指令请求时,只有在该客户端占用该通道时,才会去执行该指令
    ///4.客户端退出时,会退出它占用的所有设备的所有通道
    ///
    ///准备占用哪些通道
    #[prost(map="string, bool", tag="3")]
    pub map_channel: ::std::collections::HashMap<::prost::alloc::string::String, bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Respond {
    ///是否成功
    #[prost(bool, tag="1")]
    pub b_success: bool,
    ///设备code
    #[prost(string, tag="2")]
    pub str_dev_code: ::prost::alloc::string::String,
    ///报警或错误信息
    #[prost(string, tag="3")]
    pub str_errors: ::prost::alloc::string::String,
}
