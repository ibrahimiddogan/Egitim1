fn main() {
    let y: i32  = 2;

    let yazi:&str="Hayat";

    let tek_hane:char='a';
    let pi :f32=3.14;

    let x = 20;

    let y=32;
    let topla = x+y;
    let çarpma = x*y;
    let bölme = x/y;
    let çıkarma = x -y ;


    println!("toplanın cevabı ={} çarpmanın 
    cevabı= {}{}{}",topla,çarpma,bölme,çıkarma);

    if topla<50 {

        println!("Sayı 50 den küçük")
        
    }else {
        println!("Sayı 50 den büyük")
    }


    boş();
    calculator(60, 70);


    let günler: [&str; 7] = [
        "Pazartesi", "Salı", "Çarşamba", "Perşembe", "Cuma", "Cumartesi", "Pazar"
    ];

    println!("{:?} Haftanın bütün günleri ", günler);

    let number = 12;



    
    match number {
        2 => println!("Sayı 2"),
        3 => println!("Sayı 3"),
        12=> println!("SAyı 12 "),
        _ => println!("Diğer ihtimaller")
    }

    

    if palindrome("hayat") {
        println!(" kelimesi palindrome'dur.");
    } else {
        println!("kelimesi palindrome değildir");
    }



}


fn boş() {
    println!("Boş bir fonksiyon bu");
}

fn calculator(x:i32,y:i32) {

    let topla = x+y;
    let çarpma=x*y;
    let bölme =x/y;
    let çıkarma = x-y;

    println!("{},{},{},{}",topla,bölme,çıkarma,çarpma);

    let is_it_true = true;

    fn palindrome_mi(s: &str) -> bool {
        let ters: String = s.chars().rev().collect(); // String'i ters çeviriyoruz
        s == ters // Orijinal ve ters halini karşılaştırıyoruz
    };

    
}


fn palindrome(s: &str) -> bool {
    let ters: String = s.chars().rev().collect(); // String'i ters çeviriyoruz
    s == ters // Orijinal ve ters halini karşılaştırıyoruz
}


