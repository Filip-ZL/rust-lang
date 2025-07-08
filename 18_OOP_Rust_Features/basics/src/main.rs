
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
// We let the user communicate with the AveragedCollection only through the API
// That are methods that public (pub). This avoids unsync the values!
impl AveragedCollection {

    pub fn add(&mut self, value) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self){
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64
    }
}