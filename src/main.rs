use std::collections::HashSet;

fn is_unique(s: &str) -> bool {
    // Создаем HashSet для хранения уникальных символов
    let mut seen = HashSet::new();
    // Преобразуем строку в итератор символов и приводим все символы к нижнему регистру
    for c in s.to_lowercase().chars() {
        // Если символ уже есть в HashSet, значит, он не уникален
        if !seen.insert(c) {
            return false;
        }
    }
    // Если мы прошли по всем символам и не нашли повторяющихся, значит, все символы уникальны
    true
}

fn main() {
    let tests = ["abcd", "abCdefAaf", "aabcd", "3Fri1fn2"];

    for test in tests {
        println!("{}: {}", test, is_unique(test));
    }
}