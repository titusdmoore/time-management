pub struct Track {
    pub project: String,
    pub task: String,
    pub message: Option<String>,
    pub amount: u32,
}

impl Track {
    pub fn new(project_task: String, message: Option<String>, amount: String) -> Self {
        let project_task: Vec<&str> = project_task.split("/").collect();

        Self {
            project: project_task[0].to_string(),
            task: project_task[1].to_string(),
            message,
            amount: Track::parse_amount(amount),
        }
    }
    pub fn run(&self) {
        println!("Project: {}", self.project);
        println!("Task: {}", self.task);
        println!("Message: {}", self.message.as_ref().unwrap());
        println!("Amount: {}", self.amount);
    }
    pub fn parse_amount(amount: String) -> u32 {
        let amount: Vec<&str> = amount.split(":").collect();

        let mut total_minutes: u32 = 0;

        for (i, time) in amount.iter().enumerate() {
            let time: u32 = time.parse().unwrap();

            match i {
                0 => {
                    total_minutes += time * 60;
                }
                1 => {
                    total_minutes += time;
                }
                _ => {}
            }
        }

        total_minutes
    }
}
