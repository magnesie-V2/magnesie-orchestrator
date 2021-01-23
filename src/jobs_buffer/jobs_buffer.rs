use super::BufferedJob;

/// Keeps a list of BufferedJob and handles adding to/retrieving from the list <br /> <br />
/// add_job() <br /> remove_job() <br /> get_job() <br /> has_buffered_jobs()
pub struct JobsBuffer {
    jobs: Vec<BufferedJob>
}

impl JobsBuffer {
    pub fn new() -> JobsBuffer {
        JobsBuffer {
            jobs: Vec::new()
        }
    }

    pub fn add_job(&mut self, job: BufferedJob) {
        println!("[JobsBuffer] Adding job {}", job.to_string());
        self.jobs.push(job);
        println!("[JobsBuffer] --> OK");
    }

    pub fn remove_job(&mut self, id: &str){
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

    pub fn get_job(&mut self) -> Option<&mut BufferedJob> {
        let jobs = &mut self.jobs;

        jobs.get_mut(0)
    }

    /// Returns true if the buffer has jobs waiting to be processed
    pub fn has_buffered_jobs(&self) -> bool {
        self.jobs.len() > 0
    }
}

impl ToString for JobsBuffer{
    fn to_string(&self) -> String {
        format!("JobsBuffer(#jobs: {})", self.jobs.len())
    }
}

#[cfg(test)]
pub mod tests{
    use super::*;

    #[test]
    pub fn test_new() {
        let buffer = JobsBuffer::new();
        assert_eq!(0, buffer.jobs.len());
    }

    #[test]
    pub fn test_add_job() {
        let mut buffer = JobsBuffer::new();

        assert_eq!(0, buffer.jobs.len());
        buffer.add_job(BufferedJob::new(&None, &Vec::new(), &1));
        assert_eq!(1, buffer.jobs.len());
        buffer.add_job(BufferedJob::new(&None, &Vec::new(), &1));
        assert_eq!(2, buffer.jobs.len());
        buffer.add_job(BufferedJob::new(&None, &Vec::new(), &1));
        assert_eq!(3, buffer.jobs.len());
    }

    #[test]
    pub fn test_remove_job() {
        let mut buffer = JobsBuffer::new();
        let j1 = BufferedJob::new(&Some("azer"), &Vec::new(), &1);
        let j2 = BufferedJob::new(&Some("1234"), &Vec::new(), &2);

        assert_eq!(0, buffer.jobs.len());
        buffer.add_job(j1);
        buffer.add_job(j2);
        assert_eq!(2, buffer.jobs.len());
        buffer.remove_job("azer");
        assert_eq!(1, buffer.jobs.len());

        match buffer.jobs.get(0){
            Some(j) => {
                assert_eq!(Some("1234".to_string()), j.id);
            },
            None => {} // should not happen
        }
    }

    #[test]
    pub fn test_get_job() {
        let mut buffer = JobsBuffer::new();
        let job = BufferedJob::new(&Some("azer"), &Vec::new(), &1);
        let job_string = job.to_string();

       match buffer.get_job(){
           None => {}
           Some(_) => {
               assert_eq!(true, false)
           }
       }
        buffer.add_job(job);

        match buffer.get_job(){
            Some(j) => {
                assert_eq!(job_string, j.to_string());
            }
            None => {
                assert_eq!(true, false)
            }
        }
    }

    #[test]
    pub fn test_has_buffered_jobs() {
        let mut buffer = JobsBuffer::new();
        let j1 = BufferedJob::new(&Some("azer"), &Vec::new(), &1);
        let j2 = BufferedJob::new(&Some("1234"), &Vec::new(), &2);

        assert_eq!(false, buffer.has_buffered_jobs());
        buffer.add_job(j1);
        assert_eq!(true, buffer.has_buffered_jobs());
        buffer.add_job(j2);
        assert_eq!(true, buffer.has_buffered_jobs());
    }
}
