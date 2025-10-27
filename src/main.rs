// fn print_elements(elements: &Vec<String>) {}
fn print_elements(elements: &[String]) {
// for element in elements {
  //   println!("{}", element);
  // }
  elements
    .iter()
    .map(|f| format!("{} {}", f, f))
    .for_each(|f| println!("{}", f));
}

fn main() {
  let colors = vec![
    String::from("red"),
    String::from("blue"),
    String::from("green"),
  ];

  // let mut colors_iter = colors.iter();

  // println!("{:#?}", colors_iter.next());
  // println!("{:#?}", colors_iter.next());
  // println!("{:#?}", colors_iter.next());
  // println!("{:#?}", colors_iter.next());

  print_elements(&colors);
}
