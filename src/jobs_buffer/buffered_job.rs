use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;

/// Represents a job in the orchestrator's buffer
pub struct BufferedJob{
    /// The job's id. It's None by default but can be set to Some(String) to indicate it's currently being processed
    pub id: Option<String>,
    /// The list of photos of the submission
    pub photos: Vec<String>,
    /// The id of the original submission
    pub submission_id: i32,
    /// The time when this submission has been added to the buffer
    pub submission_date: SystemTime,
}

impl BufferedJob{
    /// Creates a BufferedJob struct
    pub fn new(id: &Option<&str>, photos: &[&str], submission_id: &i32, submission_date: SystemTime) -> BufferedJob {
        let id = match id {
            None => None,
            Some(id) => Some(id.to_string())
        };

        let photos = photos.to_vec().iter().map(|p| p.to_string()).collect();

        BufferedJob {
            id,
            photos,
            submission_id: submission_id.clone(),
            submission_date
        }
    }

    /// Returns the complexity of this job
    ///
    /// Currently simply returns the number of the photos
    pub fn get_complexity(&self) -> f32 {
        self.photos.len() as f32
    }
}

impl ToString for BufferedJob {
    fn to_string(&self) -> String {
        let datetime: DateTime<Utc> = self.submission_date.into();
        let formatted_buffered_time = format!("{}", datetime.format("%d/%m/%Y %T"));

        match &self.id {
            Some(id) => format!("BufferedJob(id: {}, #photos: {}, submission_id: {}, buffered_time: {})", id, self.photos.len(), self.submission_id, formatted_buffered_time),
            None => format!("BufferedJob(id: None, #photos: {}, submission_id: {}, buffered_time: {})", self.photos.len(), self.submission_id, formatted_buffered_time)
        }
    }
}
