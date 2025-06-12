
#[allow(dead_code)]
#[derive(Debug)]
struct Structure(i32);

#[cfg(test)]
mod tests {
    use crate::print_example::Structure;

    #[test]
    fn test() {
        println!("{} 天", 31);

        println!(
            "你的名字是{0}，我的名字是{1}, {0}和{1}是好朋友",
            "小明", "小红"
        );

        println!("你的名字是{name}", name = "小明");

        println!("十进制：{}", 100);
        println!("二进制：{:b}", 100);
        println!("八进制：{:o}", 100);
        println!("十六进制：{:x}", 100);

        println!("右对齐: {:>5}", 1);
        println!("右对齐补齐：{:0>5}", 1);

        println!("左对齐: {:<5}", 1);
        println!("左对齐补齐：{:0<5}", 1);

        println!("对齐命名参数: {number:0>width$}", number = 1, width = 5);

        let number = 10;
        println!("number is {number}");

        println!("{:#?}",Structure(3));
    }
}
