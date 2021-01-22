use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;

pub struct BufferedJob{
    id: Option<String>,
    photos: Vec<String>,
    request_id: u32,
    buffered_time: SystemTime,
}

impl BufferedJob{
    pub fn new(id: Option<&str>, photos: &[&str], request_id: &u32) -> BufferedJob {
        let id = match id {
            None => None,
            Some(id) => Some(id.to_string())
        };

        let photos = photos.to_vec().iter().map(|p| p.to_string()).collect();

        BufferedJob {
            id,
            photos,
            request_id: *request_id,
            buffered_time: SystemTime::now()
        }
    }
}

impl ToString for BufferedJob {
    fn to_string(&self) -> String {
        let datetime: DateTime<Utc> = self.buffered_time.into();
        let formatted_buffered_time = format!("{}", datetime.format("%d/%m/%Y %T"));

        match &self.id {
            Some(id) => format!("BufferedJob(id: {}, #photos: {}, request_id: {}, buffered_time: {})", id, self.photos.len(), self.request_id, formatted_buffered_time),
            None => format!("BufferedJob(id: None, #photos: {}, request_id: {}, buffered_time: {})", self.photos.len(), self.request_id, formatted_buffered_time)
        }
    }
}

pub struct JobsBuffer {
    jobs: Vec<BufferedJob>,
}

impl JobsBuffer {
    pub fn new() -> JobsBuffer {
        JobsBuffer {
            jobs: Vec::new()
        }
    }

    fn add_job(&mut self, job: BufferedJob) {
        println!("[JobsBuffer] Adding job {}", job.to_string());
        self.jobs.push(job);
        println!("[JobsBuffer] --> OK");
    }

    fn remove_job(&mut self, id: &str){
        println!("[JobsBuffer] Removing job of id {}", id);
        let index = self.jobs.iter().position(|job| {
            match &job.id {
                Some(current_id) => current_id == id,
                None => false
            }
        });

        match index {
            Some(i) => {
                self.jobs.remove(i);
                println!("[JobsBuffer] --> OK");
            },
            None => {
                println!("[JobsBuffer] --> FAILED");
            }
        };
    }

    fn get_job(&self) -> &BufferedJob {
        &self.jobs[0]
    }

    /// Returns true if the buffer has jobs waiting to be processed
    fn has_buffered_jobs(&self) -> bool {
        self.jobs.len() > 0
    }
}

impl ToString for JobsBuffer{
    fn to_string(&self) -> String {
        format!("JobsBuffer(#jobs: {})", self.jobs.len())
    }
}

