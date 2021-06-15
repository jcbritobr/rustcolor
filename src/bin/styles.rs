use rustcolor::printer::ColorPrinter;

fn main() {
    println!("{}", "this is the info style".to_owned().info());
    println!("{}", "this is the primary style".to_owned().primary());
    println!("{}", "this is the warn style".to_owned().warn());
    println!("{}", "this is the danger style".to_owned().danger());
    println!("{}", "this is the error style".to_owned().error());
    println!("{}", "this is the underlined style".to_owned().underline());
}
