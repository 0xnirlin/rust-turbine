fn main() {
    let s = String::from("Hello World Mofo");

    let len = s.len();

    let slice = &s[0..len];
    let slice2 = &s[..];
    let slice3 = &s[0..];
    let slice4 = &s[..len];
    let slice5 = &s[..len/2];
    println!("slice: {}", slice);
    println!("slice2: {}", slice2);
    println!("slice3: {}", slice3);
    println!("slice4: {}", slice4);
    println!("slice5: {}", slice5);
    
}