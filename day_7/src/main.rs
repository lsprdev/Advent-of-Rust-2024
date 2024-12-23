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

fn main() {
    let x = vec!["this", "is", "a", "log", "vector"].iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let logs = LogQuery::new(&x);
    println!("{:#?}", logs.search("vec"));

    // println!("{:#?}", logs)
}
