#[cfg(test)]
mod tests {
  #[path = "../../src/core/vcypher.rs"]
  mod vcypher;

  #[test]
  fn test_output() {
    assert_eq!(vcypher::vcypher("BFC"), "81156121701");
    assert_eq!(vcypher::vcypher("*()"), "8123614147318");
    println!("Finished tests.");
  }

  #[test]
  fn test_edge_cases_output() {
    assert_eq!(vcypher::vcypher(""), "");
  }

  #[test]
  fn test_non_english_letters_output() {
    assert_eq!(vcypher::vcypher("ľ"), "623");
    assert_eq!(vcypher::vcypher("š"), "415");
    assert_eq!(vcypher::vcypher("č"), "445");
    assert_eq!(vcypher::vcypher("ť"), "5134");
    assert_eq!(vcypher::vcypher("ž"), "712");
    assert_eq!(vcypher::vcypher("ý"), "71");
    assert_eq!(vcypher::vcypher("á"), "44");
    assert_eq!(vcypher::vcypher("í"), "612");
    assert_eq!(vcypher::vcypher("é"), "513");
  }
}
