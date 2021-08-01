#[cfg(test)]
extern crate eggricesoy;

#[test]
fn create_app() {
  let app = eggricesoy::app();
  assert_eq!(app.get_name(), "eggricesoy");
}

#[test]
fn test_app() {
  let app = eggricesoy::app();
  let matches = app.get_matches_from(["eggricesoy", "--log4rs-config", "test.yaml"]);
  eggricesoy::init_logger(&matches);
}
