pub fn can_parse(string_number: &std::string::String) -> bool {
  match string_number.parse::<i32>() {
      Ok(_n) => return true,
      Err(_e) => return false,
  }
}

pub fn can_parse_f32(string_number: &std::string::String) -> bool {
  match string_number.parse::<f32>() {
      Ok(_n) => return true,
      Err(_e) => return false,
  }
}

pub fn can_parse_u16(string_number: &std::string::String) -> bool {
  match string_number.parse::<u16>() {
      Ok(_n) => return true,
      Err(_e) => return false,
  }
}
