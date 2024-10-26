// src/main.rs
fn main() {
  let a = "type";
  let  typo_chec_result= typo_checker::check(a.to_string());

  println!("typo_chec_result: {:?}", typo_chec_result);
}
