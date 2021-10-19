use crate::message::Message;
use crate::errors::{Error as AnalyticsError};
use failure::Error;
use std::time::Duration;

pub struct RudderAnalytics{
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::Client
}



impl RudderAnalytics {
    pub fn load(write_key: String, data_plane_url: String) -> RudderAnalytics {
        RudderAnalytics {
            write_key,
            data_plane_url,
            client:reqwest::Client::builder()
            .connect_timeout(Duration::new(10, 0))
            .build()
            .unwrap()
        }
    } 
    
    pub fn send(&self, msg:&Message)-> Result<(), Error>{

        let path = match msg {
            Message::Identify(_) => "/v1/identify",
            Message::Track(_) => "/v1/track",
            Message::Page(_) => "/v1/page",
            Message::Screen(_) => "/v1/screen",
            Message::Group(_) => "/v1/group",
            Message::Alias(_) => "/v1/alias",
            Message::Batch(_) => "/v1/batch",
        };

        
        
        let res = self.client
            .post(&format!("{}{}", self.data_plane_url, path))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(&msg)
            .send()?;
            // .error_for_status()?;

        // println!("Response: {:#?}",res);

        if res.status() == 200{
            return Ok(())
        }else {
            return Err(AnalyticsError::InvalidRequest(String::from(format!("status code: {}, message: Invalid request", res.status()))).into());
        }
        
    }
}
