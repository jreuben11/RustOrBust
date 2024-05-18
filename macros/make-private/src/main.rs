use make_private_macro::private; 
private!(
  struct Example {
    string_value: String, 
    number_value: i32, 
  }
); 
fn main() {
//   let e = Example {
//     string_value: "value".to_string(), 
//     number_value: 2, 
//   }; 
//   e.get_string_value();
//   e.get_number_value(); 
}