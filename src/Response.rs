pub trait RespDataType {
    fn get_response(&self) -> String;
}

pub struct RespSimpleString {
    pub content: String
}

impl RespDataType for RespSimpleString {
    fn get_response(&self) -> String {
        format!("+{}\r\n", self.content)
    }
}

pub struct RespError {
    pub content: String
}

impl RespDataType for RespError {
    fn get_response(&self) -> String {
        format!("-{}\r\n", self.content)
    }
}

pub struct RespInteger {
    pub content: String
}

impl RespDataType for RespInteger {
    fn get_response(&self) -> String {
        format!(":{}\r\n", self.content)
    }
}

pub struct RespBulkString {
    pub content: String
}

impl RespDataType for RespBulkString {
    fn get_response(&self) -> String {
        format!("${}\r\n{}\r\n", self.content.len(), self.content)
    }
}

pub struct RespArray {
    pub content: Vec<String>
}

impl RespDataType for RespArray {
    fn get_response(&self) -> String {
        let mut response = String::new();
        response.push_str("*");
        response.push_str(&self.content.len().to_string());
        response.push_str("\r\n");
        response.push_str(&self.content.join("\r\n"));
        response
    }
}
