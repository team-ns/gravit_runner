use crate::config::Config;
use crate::jre::Jre;
use crate::util::OsType;
use anyhow::Result;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::result::ZipError;

pub struct LibericaJre {
    os_type: OsType,
    jre_version: String,
}

impl LibericaJre {
    pub fn new(os_type: OsType, config: &Config) -> Self {
        LibericaJre {
            os_type,
            jre_version: config.jre_version.clone(),
        }
    }
}

impl Jre for LibericaJre {
    fn check_jre_archive<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let os_type = &self.os_type;
        let get_sha = format!("https://api.bell-sw.com/v1/liberica/releases?version={}&version-feature=8&fx=true&bitness={}&os={}&arch=x86&installation-type=archive&bundle-type=jre&output=text&fields=sha1", self.jre_version, os_type.get_bitness(), os_type.get_os_type());
        let sha = minreq::get(get_sha).send()?.as_str()?.to_string();
        let mut hasher: Sha1 = Sha1::new();
        hasher.update(fs::read(path)?);
        let result = hasher.finalize();
        if format!("{:x}", result) == sha {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Files don't equal"))
        }
    }

    fn check_jre_folder<P: AsRef<Path>>(&self, folder: P, zip: P) -> Result<()> {
        let file = fs::File::open(zip)?;
        let mut archive = zip::ZipArchive::new(file)?;
        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            if file.is_file() {
                if let Some(path) = file.enclosed_name() {
                    let path = path.iter().skip(1).collect::<PathBuf>();

                    let mut hasher = crc32fast::Hasher::new();
                    hasher.update(&fs::read(folder.as_ref().join(&path))?);
                    let crc = hasher.finalize();
                    if crc != file.crc32() {
                        return Err(anyhow::anyhow!("File {:?} has invalid hash", &path));
                    }
                }
            }
        }
        Ok(())
    }

    fn extract_jre<P: AsRef<Path>>(&self, folder: P, zip: P) -> Result<()> {
        if !folder.as_ref().exists() {
            fs::create_dir_all(&folder)?;
        }
        let file = fs::File::open(&zip)?;
        let mut archive = zip::ZipArchive::new(file)?;

        use std::io;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let filepath = file
                .enclosed_name()
                .ok_or(ZipError::InvalidArchive("Invalid file path"))?;

            let filepath = filepath.iter().skip(1).collect::<PathBuf>();

            let outpath = folder.as_ref().join(filepath);

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }
            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }
        fs::remove_file(zip.as_ref())?;
        Ok(())
    }

    fn download_jre<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let os_type = &self.os_type;
        let get_download_url = format!("https://api.bell-sw.com/v1/liberica/releases?version={}&version-feature=8&fx=true&bitness={}&os={}&arch=x86&installation-type=archive&bundle-type=jre&output=text&fields=downloadUrl", self.jre_version, os_type.get_bitness(), os_type.get_os_type());
        let download_url = minreq::get(get_download_url).send()?.as_str()?.to_string();
        let response = minreq::get(download_url).send()?.as_bytes().to_vec();
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::OpenOptions::new().create(true).write(true).open(path)?;
        file.write(&response)?;
        Ok(())
    }
}
