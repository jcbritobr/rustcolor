use rustcolor::style::StyleBuilder;

fn main() {
    let custom_style = StyleBuilder::new()
        .csi()
        .foreground_8bit()
        .delimiter()
        .color(0)
        .delimiter()
        .background_8bit()
        .delimiter()
        .color(201)
        .end_sgr()
        .message("a custom style with 0fg and 201bg")
        .csi()
        .reset()
        .end_sgr()
        .build();

    println!("{}", custom_style);
}
