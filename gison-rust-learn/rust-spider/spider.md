爬虫就是 通过模拟HTTP请求获取网页数据。

实现步骤：
1. 解析URL:使用url库解析URL，提取出协议，主机名，路径等信息;
2. 发送请求:使用reqwest库发送HTTP请求，并设置相应的header，cookie;
3. 获取响应:获取HTTP响应，并检查响应状态码是否是200;
4. 解析内容:使用html5ever库解析HTML文本，提出出目标内容;
5. 存储数据:将抓取到的数据存储到数据库或文件中.