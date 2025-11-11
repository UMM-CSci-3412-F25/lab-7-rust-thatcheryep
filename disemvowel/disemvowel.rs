//TODO: Return the input string without vowels.
pub fn disemvowel(s: &str) -> String {
    let mut result = String::new();

    for cur_char in s.chars() {
        if !matches!(cur_char, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U') { // If char is NOT a vowel
            result.push(cur_char); // Add char to result
        }
    }

    result
}


/* C version of disemvowel function for reference
void addChar(char *str, char c, int *length) {
  str[(*length)++] = c; // Add character at the end
  str[*length] = '\0'; // Replace null pointer
}

char *disemvowel(char *str) {
  char *tmp = (char*)malloc(strlen(str) + 1);  // Temporary buffer (I use malloc because the test file checks long strings, and this ensure we won't have issues with that)
  int length = 0; // Track length of tmp
  tmp[0] = '\0'; // Initialize as empty string

  for(int i = 0; str[i] != '\0'; i++) { // Go until null pointer
    if (!(str[i] == 'a' || str[i] == 'e' || str[i] == 'i' || str[i] == 'o' || str[i] == 'u' ||
       str[i] == 'A' || str[i] == 'E' || str[i] == 'I' || str[i] == 'O' || str[i] == 'U')) { // Check input for vowels, one index at a time
      // If char is NOT a vowel, then:
      addChar(tmp, str[i], &length); // Add char to tmp
    }
  }

  char *result = (char*)malloc(length + 1); // Allocate space for result
  for(int j = 0; tmp[j] != '\0'; j++){ // Copy tmp to result
    result[j] = tmp[j];
  }
  result[length] = '\0'; // Add null pointer
  
  free(tmp); // Free tmp
  tmp = NULL;

  return result;
}*/

// Everything from here down is Rust test code. You shouldn't need to
// change any of this.
//
// Use `cargo test` to run all these tests. All the tests will initially
// fail because there's no definition for the `disemvowel` function. Add
// that up above and work to get the tests to pass. See the lab write-up
// for some tips.

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn hello_world() {
        let input = "Hello, world!";
        let expected = "Hll, wrld!";

        assert_eq!(expected, disemvowel(input));
    }

    #[test]
    fn empty() {
        assert_eq!("", disemvowel(""));
    }

    #[test]
    fn no_vowels() {
        assert_eq!("pqrst", disemvowel("pqrst"));
    }

    #[test]
    fn all_vowels() {
        assert_eq!("", disemvowel("aeiouAEIOUOIEAuoiea"));
    }

    #[test]
    fn morris_minnesota() {
        assert_eq!("Mrrs, Mnnst", disemvowel("Morris, Minnesota"));
    }

    #[test]
    fn handle_punctuation() {
        assert_eq!(
            "n (nxplnd) lphnt!",
            disemvowel("An (Unexplained) Elephant!")
        );
    }

    #[test]
    fn handle_unicode() {
        assert_eq!(
            "Sm hrglyphs: 𒐁	𒐌	𒐥	𒑳",
            disemvowel("Some hieroglyphs: 𒐁	𒐌	𒐥	𒑳")
        );
        assert_eq!("Sm Lnr B: 	𐂀	𐂚	𐃃	𐃺", disemvowel("Some Linear B: 	𐂀	𐂚	𐃃	𐃺"));
        assert_eq!(
            " lttl Phncn: 𐤀	𐤈	𐤔	𐤕",
            disemvowel("A little Phoenician: 𐤀	𐤈	𐤔	𐤕")
        );
        assert_eq!(
            "W cn hndl mj s wll! 🤣😃👍",
            disemvowel("We can handle emoji as well! 🤣😃👍")
        )
    }
}
