use super::BufferedJob;
use crate::jobs_buffer::BufferError;

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

    pub fn add_job(&mut self, job: BufferedJob) -> Result<(), BufferError> {
        if self.submission_exists(&job) {
            return Err(BufferError::from("A job with this submission_id already exists in the buffer"));
        }
        if self.job_exists(&job) {
            return Err(BufferError::from("A job with this id already exists in the buffer"));
        }

        self.jobs.push(job);
        Ok(())
    }

    pub fn remove_job(&mut self, id: &str) -> Result<(), BufferError>{
        let index = self.jobs.iter().position(|job| {
            match &job.id {
                Some(current_id) => current_id == id,
                None => false
            }
        });

        if let Some(i) = index {
            self.jobs.remove(i);
            return Ok(());
        }

        Err(BufferError::from("This job is not currently in the buffer"))
    }

    pub fn get_job_by_id(&mut self, id: &str) -> Option<&mut BufferedJob> {
        let index = self.jobs.iter().position(|job| {
            match &job.id {
                Some(current_id) => current_id == id,
                None => false
            }
        });

        match index {
            Some(i) => {
                self.jobs.get_mut(i)
            },
            None => {
                None
            }
        }
    }

    pub fn get_pending_jobs(&mut self) -> Option<Vec<&mut BufferedJob>>{
        let mut pending_jobs = Vec::new();

        for job in self.jobs.iter_mut(){
            if job.id.is_none() {
                pending_jobs.push(job)
            }
        }

        if pending_jobs.is_empty() {
            return None;
        }

        Some(pending_jobs)
    }

    pub fn submission_exists(&self, job: &BufferedJob) -> bool {
        let jobs = &self.jobs;
        jobs.iter().any(|j| {
            j.submission_id == job.submission_id
        })
    }

    pub fn job_exists(&self, job: &BufferedJob) -> bool {
        let jobs = &self.jobs;

        if job.id.is_none() {
            return false;
        }
        let job_id = job.id.clone().unwrap();

        jobs.iter().any(|j| {
            if let Some(id) = &j.id{
                id.clone() == job_id
            } else {
                false
            }
        })
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
    use std::time::SystemTime;

    #[test]
    pub fn test_new() {
        let buffer = JobsBuffer::new();
        assert_eq!(0, buffer.jobs.len());
    }

    #[test]
    pub fn test_add_job() {
        let mut buffer = JobsBuffer::new();

        assert_eq!(0, buffer.jobs.len());
        buffer.add_job(BufferedJob::new(&None, &Vec::new(), &1, SystemTime::now()));
        assert_eq!(1, buffer.jobs.len());
        buffer.add_job(BufferedJob::new(&None, &Vec::new(), &2, SystemTime::now()));
        assert_eq!(2, buffer.jobs.len());
        buffer.add_job(BufferedJob::new(&None, &Vec::new(), &3, SystemTime::now()));
        assert_eq!(3, buffer.jobs.len());
    }

    #[test]
    pub fn test_remove_job() {
        let mut buffer = JobsBuffer::new();
        let j1 = BufferedJob::new(&Some("azer"), &Vec::new(), &1, SystemTime::now());
        let j2 = BufferedJob::new(&Some("1234"), &Vec::new(), &2, SystemTime::now());

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

    /*
    #[test]
    pub fn test_get_job() {
        let mut buffer = JobsBuffer::new();
        let job = BufferedJob::new(&Some("azer"), &Vec::new(), &1, SystemTime::now());
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
    */

    #[test]
    pub fn test_has_buffered_jobs() {
        let mut buffer = JobsBuffer::new();
        let j1 = BufferedJob::new(&Some("azer"), &Vec::new(), &1, SystemTime::now());
        let j2 = BufferedJob::new(&Some("1234"), &Vec::new(), &2, SystemTime::now());

        assert_eq!(false, buffer.has_buffered_jobs());
        buffer.add_job(j1);
        assert_eq!(true, buffer.has_buffered_jobs());
        buffer.add_job(j2);
        assert_eq!(true, buffer.has_buffered_jobs());
    }

    #[test]
    pub fn test_submission_exists(){
        let mut buffer = JobsBuffer::new();
        let j1 = BufferedJob::new(&Some("azer"), &Vec::new(), &1, SystemTime::now());

        assert_eq!(false, buffer.submission_exists(&j1));
        buffer.add_job(j1);

        let j1 = BufferedJob::new(&Some("azer"), &Vec::new(), &1, SystemTime::now());
        assert_eq!(true, buffer.submission_exists(&j1));
    }

    #[test]
    pub fn test_job_exists(){
        let mut buffer = JobsBuffer::new();
        let j1 = BufferedJob::new(&Some("azer"), &Vec::new(), &1, SystemTime::now());

        assert_eq!(false, buffer.submission_exists(&j1));
        buffer.add_job(j1);
        let j1 = BufferedJob::new(&Some("azer"), &Vec::new(), &1, SystemTime::now());
        assert_eq!(true, buffer.submission_exists(&j1));
    }
}
