// avoids unsafe O(n)
fn encrypt(message: &str, rotate: u8) -> String {
    let mut encrypt = String::with_capacity(message.len());
    let mut temp: u8;
    let mut base: u8;
    for byte in message.as_bytes() {
        temp = byte.clone();
        if byte.is_ascii_alphabetic() {
            base = if byte.is_ascii_lowercase() { 97 } else { 65 };
            // original character position in alphabet (byte) - base (lower or upper a) + rotation mod total chars + base gives new position
            temp = base + ((temp - base + rotate) % 26);
        }
        encrypt.push(char::from(temp));
    }
    encrypt
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_not_rotate_cipher() {
        let cipher: String = String::from("middle-Outz");
        let actual: String = encrypt(&cipher, 0);
        assert_eq!(actual, "middle-Outz");
    }

    #[test]
    fn should_rotate_cipher_by_two() {
        let cipher: String = String::from("middle-Outz");
        let actual: String = encrypt(&cipher, 2);
        assert_eq!(actual, "okffng-Qwvb");
    }

    #[test]
    fn should_rotate_cipher_by_five() {
        let cipher: String = String::from("Always-Look-on-the-Bright-Side-of-Life");
        let actual: String = encrypt(&cipher, 5);
        assert_eq!(actual, "Fqbfdx-Qttp-ts-ymj-Gwnlmy-Xnij-tk-Qnkj");
    }

    #[test]
    fn should_rotate_cipher_by_four() {
        let cipher: String = String::from("Hello_World!");
        let actual: String = encrypt(&cipher, 4);
        assert_eq!(actual, "Lipps_Asvph!");
    }

    #[test]
    fn should_rotate_cipher_unicode() {
        let cipher: String = String::from("!m-rB`");
        let actual: String = encrypt(&cipher, 62);
        assert_eq!(actual, "!w-bL`");
    }

    #[test]
    fn should_rotate_cipher_long_unicode() {
        let cipher: String = String::from("!m-rB`-oN!.W`cLAcVbN/CqSoolII!SImji.!w/`Xu`uZa1TWPRq`uRBtok`xPT`lL-zPTc.BSRIhu..-!.!tcl!-U");
        let actual: String = encrypt(&cipher, 62);
        assert_eq!(actual, "!w-bL`-yX!.G`mVKmFlX/MaCyyvSS!CSwts.!g/`He`eJk1DGZBa`eBLdyu`hZD`vV-jZDm.LCBSre..-!.!dmv!-E");
    }
}
