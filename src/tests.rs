use ::segments;

#[test]
fn environment_data() {
  let env = segments::environment();
  match env.get("OSTYPE"){
    Some(ostype) => println!("OSTYPE={:?}", ostype),
    None => println!("OSTYPE=<not found>")
  }
}
