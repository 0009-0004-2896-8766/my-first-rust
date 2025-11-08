fn main() {
    println!("==========================================");
    println!("My First Rust Executable!");
    println!("Built automatically via GitHub Actions");
    println!("==========================================");
    println!("");
    println!("This program demonstrates:");
    println!("✓ Basic Rust syntax");
    println!("✓ String formatting");
    println!("✓ GitHub Actions compilation");
    println!("✓ Cross-platform executable generation");
    println!("");
    
    let name = "Rust Learner";
    let version = env!("CARGO_PKG_VERSION");
    println!("Program: {} v{}", env!("CARGO_PKG_NAME"), version);
    println!("Created by: {}", name);
    println!("");
    
    // Simple calculation example
    let numbers = [1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    let average = sum as f32 / numbers.len() as f32;
    
    println!("Math Example:");
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum);
    println!("Average: {:.2}", average);
    println!("");
    println!("🎉 Congratulations! You built a Rust executable! 🎉");
    
    // Wait for user input before closing (Windows)
    #[cfg(target_os = "windows")]
    {
        use std::io;
        println!("Press Enter to exit...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}
