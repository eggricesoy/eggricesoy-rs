#[cfg(test)]
extern crate eggricesoy;

#[test]
fn create_app() {
  let app = eggricesoy::app!();
  assert_eq!(app.get_name(), "eggricesoy");
}
