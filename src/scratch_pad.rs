// This trait tells Rust exactly how each raw type should diff against a ScheduledTask
pub trait DiffCheck {
    fn check_diff(&self, task: &ScheduledTask) -> bool;
}

impl DiffCheck for u64 {
    fn check_diff(&self, task: &ScheduledTask) -> bool {
        match (&task.last_value, &task.limit) {
            (Some(AgentValue::Int(last)), Some(AgentValue::Int(limit))) => self.abs_diff(*last) >= *limit,
            (Some(AgentValue::Int(last)), None) => self != last,
            (None, _) => true,
            _ => true,
        }
    }
}

impl DiffCheck for f64 {
    fn check_diff(&self, task: &ScheduledTask) -> bool {
        match (&task.last_value, &task.limit) {
            (Some(AgentValue::Float(last)), Some(AgentValue::Float(limit))) => (self - last).abs() >= *limit,
            (Some(AgentValue::Float(last)), None) => self != last,
            (None, _) => true,
            _ => true,
        }
    }
}

impl DiffCheck for String {
    fn check_diff(&self, task: &ScheduledTask) -> bool {
        match &task.last_value {
            // Strings and JSON rarely use numeric 'limits', so we just check inequality
            Some(AgentValue::Text(last)) => self != last,
            None => true,
            _ => true,
        }
    }
}

impl DiffCheck for Value {
    fn check_diff(&self, task: &ScheduledTask) -> bool {
        match &task.last_value {
            Some(AgentValue::Json(last)) => self != last,
            None => true,
            _ => true,
        }
    }
}