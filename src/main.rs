
fn print_elements(elements: &[String]) {
  // for element in elements {
  //   println!("{}", element);
  // }
  elements
    .iter()
    .map(|f| format!("{} {}", f, f))
    .for_each(|f| println!("{}", f));
}

fn shorten_strings(elements: &mut [String]) {
  elements
    .iter_mut()
    .for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
  return elements
    .iter()
    .map(|el| el.to_uppercase())
    .collect::<Vec<String>>();
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
  vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn main() {
  let mut colors = vec![
    String::from("red"),
    String::from("blue"),
    String::from("green"),
  ];

  // let mut colors_iter = colors.iter();

  // println!("{:#?}", colors_iter.next());
  // println!("{:#?}", colors_iter.next());
  // println!("{:#?}", colors_iter.next());
  // println!("{:#?}", colors_iter.next());

  // shorten_strings(&mut colors);
  // let uppercased = to_uppercase(&colors);
  // print_elements(&uppercased);

  let mut destination = vec![];
  move_elements(colors, &mut destination);
  print_elements(&destination);
}
