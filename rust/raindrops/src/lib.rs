pub fn raindrops(n : u32) -> String {
    let mut drops : String = String::new();
    if n % 3 == 0 {
         drops.push_str("Pling");
    }

    if n % 5 == 0 {
        drops.push_str("Plang");
    }
    
    if n % 7 == 0 {
        drops.push_str("Plong");
    }

    if drops.is_empty() {
        drops.push_str(n.to_string().as_str())
    }

    drops
}