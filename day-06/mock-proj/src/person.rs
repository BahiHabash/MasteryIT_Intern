#[derive(Debug)]
pub struct Person {
  pub first_name: String,
  pub last_name: String,
  pub age: u8
}

impl Person {
  pub fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }
}