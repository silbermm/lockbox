extern crate lockbox_lib;

use lockbox_lib::{encryption};

#[test]
fn test_encryption() -> Result<(), std::io::Error> {
    match encryption::generate_keys()  {
        _ => {
            let cryptobox = encryption::load_keys().expect("Unable to load encryption keys");
            let to_encrypt = String::from("somerandompassword");
            let edata = cryptobox.encrypt(&to_encrypt);

            let from_encrypted = encryption::load_from_encoded(edata.to_string())?;
            assert_eq!(cryptobox.decrypt(from_encrypted).unwrap(), to_encrypt);
            Ok(())
        }
    }

}
