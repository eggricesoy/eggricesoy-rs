#[cfg(test)]
extern crate eggricesoy;

#[test]
fn create_app() {
  let app = eggricesoy::app!();
  assert_eq!(app.get_name(), "eggricesoy");
}

#[test]
fn test_app() {
  let app = eggricesoy::app!();
  let matches = eggricesoy::init_app(app).matches;
}
