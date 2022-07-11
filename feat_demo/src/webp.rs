// 特征
// Cargo“特性”提供了一种表达`条件编译`和`可选依赖`的机制。
// [features]一个包在的表中定义了一组命名特性Cargo.toml，每个特性都可以被启用或禁用。
// 可以在命令行上使用诸如 `--features`. 依赖关系的特性可以在依赖声明中启用Cargo.toml。
// https://course.rs/cargo/reference/features/intro.html
// #[cfg(feature = "feat")]
pub fn feat() {
    println!("this is a features");
}
