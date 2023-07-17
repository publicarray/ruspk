use crate::models::*;
use anyhow::{Result};
use s3::Region;
// use async_std::path::PathBuf;
use std::path::PathBuf;
extern crate serde_derive;
extern crate serde_qs as qs;
use crate::*;
use async_std::io;
use async_std::path::Path;
use s3::bucket::Bucket;
use s3::creds::Credentials;

// Info: package info struct
// filepath: temp file path
// icon256path: iconfile
pub async fn store_file(info: &Info, filepath: PathBuf, icon256path: PathBuf) -> Result<()> {

    let new_filename = format!(
        "{}.v{}.f{}[{}].spk",
        info.package,
        info.version.split('-').collect::<Vec<&str>>()[1], // package revision
        info.os_min_ver.split('-').collect::<Vec<&str>>()[1], // firmware build
        info.arch.replace(' ', "-")
    );


    if *STORAGE_TYPE == "filesystem" && !STORAGE_PATH.is_empty() {
        // path / package name / package revision
        trace!("Using filesystem");
        let file_path_str = format!(
            "{}/{}/{}",
            &*STORAGE_PATH,
            info.package,
            info.version.split('-').collect::<Vec<&str>>()[1]
        );
        let file_path = Path::new(&file_path_str);
        if let Err(e) = async_std::fs::create_dir_all(file_path).await {
            if e.kind() != io::ErrorKind::AlreadyExists {
                panic!("{:?}", e)
            }
        }

        let new_filepath = file_path.join(new_filename);

        debug!("rename: {:?}->{:?}", filepath, new_filepath);
        //async_std::fs::rename(filepath, new_filepath).await?; // /tmp is in memory (tmpfs) and therefore a different filesystem
        async_std::fs::copy(filepath, new_filepath).await?;
        async_std::fs::copy(icon256path, file_path.join("icon_256.png")).await?;
    // S3 API
    } else if *STORAGE_TYPE == "s3" && !STORAGE_S3_API.is_empty() && STORAGE_S3_ID.is_empty() && !STORAGE_S3_REGION.is_empty() && !STORAGE_S3_SECRET_KEY.is_empty()  && !STORAGE_S3_BUCKET.is_empty() {
        trace!("Using s3 api");
        let bucket_name = &**STORAGE_S3_BUCKET;
        let region_name = (*STORAGE_S3_REGION).clone();
        let endpoint = (*STORAGE_S3_API).clone();
        let region = Region::Custom { region: region_name, endpoint };
        let credentials = Credentials::new(Some(&**STORAGE_S3_ID), Some(&**STORAGE_S3_SECRET_KEY), None, None, None)?;
        //let credentials = Credentials::default()?;
        let bucket = Bucket::new(bucket_name, region, credentials)?;
        
        let contents = async_std::fs::read(filepath.clone()).await?;
        let content = contents.as_slice();
        //let content = "I want to go to S3".as_bytes();
        let new_filepath = format!(
            "{}/{}/{}",
            info.package,
            info.version.split('-').collect::<Vec<&str>>()[1],
            new_filename
        );

        // let s3_response = bucket.put_object_with_content_type(new_filepath, content, "application/zip").await?;
        let s3_response = bucket.put_object(new_filepath, content).await?;
        // debug!("s3: api response: {}", s3_response.to_string());
        debug!("s3: api response code: {}", s3_response.status_code());
        if s3_response.status_code() == 200 {
            //
        }

    }
    Ok(())
}
