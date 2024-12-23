pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

// 1. Finish the implementation of LogQuery
impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> LogQuery<'a> {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&str> {
        let mut res : Vec<&str> = Vec::new();
        for log in self.logs {
            if log.contains(keyword) {
                res.push(log);
            }
        }
        res 
    }
}