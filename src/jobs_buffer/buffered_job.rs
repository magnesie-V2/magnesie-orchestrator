use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;

/// Data structure representing a job in the orchestrator's buffer
pub struct BufferedJob{
    pub id: Option<String>,
    pub photos: Vec<String>,
    pub submission_id: String,
    pub submission_date: SystemTime,
}

impl BufferedJob{
    pub fn new(id: &Option<&str>, photos: &[&str], input_id: &str, submission_date: SystemTime) -> BufferedJob {
        let id = match id {
            None => None,
            Some(id) => Some(id.to_string())
        };

        let photos = photos.to_vec().iter().map(|p| p.to_string()).collect();
        let input_id = input_id.to_string();

        BufferedJob {
            id,
            photos,
            submission_id: input_id,
            submission_date
        }
    }
}

impl ToString for BufferedJob {
    fn to_string(&self) -> String {
        let datetime: DateTime<Utc> = self.submission_date.into();
        let formatted_buffered_time = format!("{}", datetime.format("%d/%m/%Y %T"));

        match &self.id {
            Some(id) => format!("BufferedJob(id: {}, #photos: {}, request_id: {}, buffered_time: {})", id, self.photos.len(), self.submission_id, formatted_buffered_time),
            None => format!("BufferedJob(id: None, #photos: {}, request_id: {}, buffered_time: {})", self.photos.len(), self.submission_id, formatted_buffered_time)
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    pub fn test_new() {
        let no_id = None;
        let id = Some("id");
        let no_photos = Vec::new();
        let mut photos = Vec::new();
        photos.push("photo1.jpeg");
        photos.push("photo2.jpeg");
        photos.push("photo3.jpeg");
        let input_id = "1";

        // no id, no photos
        let buffered_job = BufferedJob::new(&no_id, &no_photos, &input_id, SystemTime::now());
        assert_eq!(None, buffered_job.id);
        assert_eq!(0, buffered_job.photos.len());
        assert_eq!("1", buffered_job.submission_id);
        // id, no photos
        let buffered_job = BufferedJob::new(&id, &no_photos, &input_id, SystemTime::now());
        assert_eq!(Some("id".to_string()), buffered_job.id);
        assert_eq!(0, buffered_job.photos.len());
        assert_eq!("1", buffered_job.submission_id);
        // no id, photos
        let buffered_job = BufferedJob::new(&no_id, &photos, &input_id, SystemTime::now());
        assert_eq!(None, buffered_job.id);
        assert_eq!(3, buffered_job.photos.len());
        assert_eq!("1", buffered_job.submission_id);
        // id, photos
        let buffered_job = BufferedJob::new(&id, &photos, &input_id, SystemTime::now());
        assert_eq!(Some("id".to_string()), buffered_job.id);
        assert_eq!(3, buffered_job.photos.len());
        assert_eq!("1", buffered_job.submission_id);
    }

    #[test]
    pub fn test_to_string(){
        let no_id = None;
        let id = Some("id");
        let no_photos = Vec::new();
        let mut photos = Vec::new();
        photos.push("photo1.jpeg");
        photos.push("photo2.jpeg");
        photos.push("photo3.jpeg");
        let input_id = "1";

        // no id, no photos
        let buffered_job = BufferedJob::new(&no_id, &no_photos, &input_id, SystemTime::now());
        let datetime: DateTime<Utc> = buffered_job.submission_date.into();
        let formatted_buffered_time = format!("{}", datetime.format("%d/%m/%Y %T"));
        assert_eq!(format!("BufferedJob(id: None, #photos: 0, request_id: 1, buffered_time: {})", formatted_buffered_time).to_string(), buffered_job.to_string());
        // id, no photos
        let buffered_job = BufferedJob::new(&id, &no_photos, &input_id, SystemTime::now());
        let datetime: DateTime<Utc> = buffered_job.submission_date.into();
        let formatted_buffered_time = format!("{}", datetime.format("%d/%m/%Y %T"));
        assert_eq!(format!("BufferedJob(id: id, #photos: 0, request_id: 1, buffered_time: {})", formatted_buffered_time).to_string(), buffered_job.to_string());
        // no id, photos
        let buffered_job = BufferedJob::new(&no_id, &photos, &input_id, SystemTime::now());
        let datetime: DateTime<Utc> = buffered_job.submission_date.into();
        let formatted_buffered_time = format!("{}", datetime.format("%d/%m/%Y %T"));
        assert_eq!(format!("BufferedJob(id: None, #photos: 3, request_id: 1, buffered_time: {})", formatted_buffered_time).to_string(), buffered_job.to_string());
        // id, photos
        let buffered_job = BufferedJob::new(&id, &photos, &input_id, SystemTime::now());
        let datetime: DateTime<Utc> = buffered_job.submission_date.into();
        let formatted_buffered_time = format!("{}", datetime.format("%d/%m/%Y %T"));
        assert_eq!(format!("BufferedJob(id: id, #photos: 3, request_id: 1, buffered_time: {})", formatted_buffered_time).to_string(), buffered_job.to_string());
    }
}

