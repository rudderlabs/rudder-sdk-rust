use crate::methods::Methods;
use crate::message::{Identify, Track, Page, Screen, Group, Alias, Batch};
use failure::Error;

pub struct Rudderelement{
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::Client
}


impl Rudderelement {
    pub fn load(write_key: String, data_plane_url: String) -> Rudderelement {
        Rudderelement {
            write_key,
            data_plane_url,
            client:reqwest::Client::new()
        }
    }
}

impl Methods for Rudderelement{

    fn identify(&self, msg:&Identify)-> Result<(), Error>{
        self.client
            .post(&format!("{}{}", self.data_plane_url, "/v1/identify"))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(&msg)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    fn track(&self, msg:&Track)-> Result<(), Error>{
        self.client
            .post(&format!("{}{}", self.data_plane_url, "/v1/track"))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(&msg)
            .send()?
            .error_for_status()?;
        Ok(())
    }
    fn page(&self, msg:&Page)-> Result<(), Error>{
        self.client
            .post(&format!("{}{}", self.data_plane_url, "/v1/page"))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(&msg)
            .send()?
            .error_for_status()?;
        Ok(())
    }
    fn screen(&self, msg:&Screen)-> Result<(), Error>{
        self.client
            .post(&format!("{}{}", self.data_plane_url, "/v1/screen"))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(&msg)
            .send()?
            .error_for_status()?;
        Ok(())
    }
    fn group(&self, msg:&Group)-> Result<(), Error>{
        self.client
            .post(&format!("{}{}", self.data_plane_url, "/v1/group"))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(&msg)
            .send()?
            .error_for_status()?;
        Ok(())
    }
    fn alias(&self, msg:&Alias)-> Result<(), Error>{
        self.client
            .post(&format!("{}{}", self.data_plane_url, "/v1/alias"))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(&msg)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    fn batch(&self, msg:&Batch)-> Result<(), Error>{
        self.client
            .post(&format!("{}{}", self.data_plane_url, "/v1/batch"))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(&msg)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}