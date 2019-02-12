use std::io;

type Threeple = [Vec<char>; 3];

pub fn parse_rgb(maybe_color: &mut str) -> Result<Threeple, &'static str> {
  match trim_rgb(maybe_color) {
    Ok((_, rgb)) => get_terms(rgb),
    Err(message) => Err(message),
  }
}

fn check_parentheses(maybe_color: &str) -> Result<(), &'static str> {
  let open_index = 0;
  let last_index = maybe_color.len() - 1;
  let opening_paren_index = maybe_color.find('(');
  let closing_paren_index = maybe_color.find(')');

  match (opening_paren_index, closing_paren_index) {
    (None, None) => Err("No parentheses found"),
    (None, _) => Err("No opening paren found"),
    (_, None) => Err("No closing paren found"),
    (_, Some(index)) if (index != last_index) => Err("Closing paren in wrong place"),
    (Some(index), _) if (index != open_index) => Err("Opening paren in wrong place"),
    (_, _) => Ok(()),
  }
}

fn get_terms(maybe_color: &str) -> Result<Threeple, &'static str> {
  match check_parentheses(maybe_color) {
    Err(message) => return Err(message),
    _ => (),
  }

  // gather the rest of the string
  let mut copy: String = maybe_color.to_string();
  let range_between_parenthesses = 4..maybe_color.len() - 1;
  let remainder: String = copy.drain(range_between_parenthesses).collect();

  // prep the threeple
  let mut results: Threeple = [Vec::new(), Vec::new(), Vec::new()];
  let mut counter = 0;

  for current_char in remainder.chars() {
    if counter < 3 {
      match current_char {
        ',' => counter += 1,
        ' ' => (),
        value => results[counter].push(value),
      }
    } else {
      return Err("Wrong number of terms.");
    }
  }

  return Ok(results);
}

fn trim_rgb(maybe_color: &str) -> Result<(&str, &str), &'static str> {
  let mut current_char = maybe_color.chars();
  let (first, second, third) = (
    current_char.next(),
    current_char.next(),
    current_char.next(),
  );

  match (first, second, third) {
    (Some('r'), Some('g'), Some('b')) => Ok(maybe_color.split_at(3)),
    _ => Err("String must start with rgb"),
  }
}

mod tests {
  use super::trim_rgb;

  #[test]
  fn trim_rgb_doesnt_fail() {
    let sample = r"rgb(23,44,123)";
    let result = trim_rgb(sample);
    assert!(result.is_ok());
  }

  #[test]
  fn trim_rgb_hasrgb() {
    let sample = r"rgb(23,44,123)";
    let result = trim_rgb(sample);
    let (hopefully_rgb, _) = result.unwrap();
    assert_eq!(hopefully_rgb, "rgb");
  }
}
