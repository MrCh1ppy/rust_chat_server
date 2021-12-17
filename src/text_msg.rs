use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug,Clone)]
pub struct TextMessage{
    from:String,
    to:String,
    content:String,
    m_date:String,
    username:String
}

impl  TextMessage{
    pub fn from(&self) -> &str {
        &self.from
    }

    #[warn(dead_code)]
    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn m_date(&self)->&str{
        &self.m_date
    }

    pub fn username(&self) -> &str{&self.username}

    pub fn new(from: String, to:String, content:String, m_date:String, username:String) ->TextMessage{
        TextMessage{
            from,
            to,
            content,
            m_date,
            username
        }
    }
}

//实现了toString接口
impl ToString for TextMessage{
    fn to_string(&self) -> String {
        format!("^{}&{}&{}&{}&{}^", self.from, self.to, self.content, self.m_date, self.username)
    }
}


//实现了通过字符串解析的接口
impl FromStr for TextMessage {
    type Err = ParseIntError;

    //用str反向解析获取对象
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let msg:Vec<&str>=s.trim_matches(|p|p=='^'||p=='^').split("&").collect();
        let from=msg[0].to_string();
        let to=msg[1].to_string();
        let content=msg[2].to_string();
        let m_date=msg[3].to_string();
        let username=msg[4].to_string();
        Ok(TextMessage{
            from,
            to,
            content,
            m_date,
            username
        })
    }
}
