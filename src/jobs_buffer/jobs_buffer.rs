use super::BufferedJob;
use crate::jobs_buffer::BufferError;
use std::time::SystemTime;

/// Keeps a list of jobs and submissions (which are jobs
pub struct JobsBuffer {
    /// The list of submissions and jobs (which are submissions sent to the photogrammetry service)
    jobs: Vec<BufferedJob>
}

impl JobsBuffer {
    /// Creates a JobsBuffer struct
    pub fn new() -> JobsBuffer {
        JobsBuffer {
            jobs: Vec::new()
        }
    }

    /// Adds a submission or a job to the buffer
    pub fn add_job_or_submission(&mut self, job: BufferedJob) -> Result<(), BufferError> {
        if self.submission_exists(&job) {
            return Err(BufferError::from("A job with this submission_id already exists in the buffer"));
        }
        if self.job_exists(&job) {
            return Err(BufferError::from("A job with this id already exists in the buffer"));
        }

        self.jobs.push(job);
        Ok(())
    }

    /// Removes a job based on its id
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

    /// Returns a job based on it's id
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

    /// Returns the list of jobs that have not been sent to be processed yet
    pub fn get_pending_submissions(&mut self) -> Option<Vec<&mut BufferedJob>>{
        let mut pending_submissions = Vec::new();

        for job in self.jobs.iter_mut(){
            if job.id.is_none() {
                pending_submissions.push(job)
            }
        }

        if pending_submissions.is_empty() {
            return None;
        }

        Some(pending_submissions)
    }

    /// Returns **true** if there's a submission in the buffer that has the same submission_id
    pub fn submission_exists(&self, job: &BufferedJob) -> bool {
        let jobs = &self.jobs;
        jobs.iter().any(|j| {
            j.submission_id == job.submission_id
        })
    }

    /// Returns **true** if there is a job in the buffer with the same id
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

    /// Checks whether there are jobs that were created more than 24h ago.
    ///
    /// If it finds any, the jobs will be set as a pending submission
    pub fn check_timeouts(&mut self) {
        for job in self.jobs.iter_mut(){
            if let Ok(added_since) = SystemTime::now().duration_since(job.submission_date){
                if job.id.is_some() && added_since.as_secs() >= 86400 {
                    job.id = None;
                }
            }
        }
    }
}

impl ToString for JobsBuffer{
    fn to_string(&self) -> String {
        format!("JobsBuffer(#jobs: {})", self.jobs.len())
    }
}
