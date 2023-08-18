use rand::Rng;

pub fn generate_cpf() -> String {
    let mut rng = rand::thread_rng();

    let mut cpf: Vec<u8> = (0..9).map(|_| rng.gen_range(0..=9)).collect();

    for _ in 0..2 {
        let val = (cpf.iter().enumerate().map(|(i, v)| (cpf.len() + 1 - i) as u32 * (*v as u32)).sum::<u32>() % 11) as u8;
        cpf.push(if val > 1 { 11 - val } else { 0 });
    }

    format!("{}.{}.{}-{}", 
        &cpf[0..3].iter().map(|n| n.to_string()).collect::<String>(), 
        &cpf[3..6].iter().map(|n| n.to_string()).collect::<String>(), 
        &cpf[6..9].iter().map(|n| n.to_string()).collect::<String>(), 
        &cpf[9..11].iter().map(|n| n.to_string()).collect::<String>())
}

fn main() {
    for _ in 0..10 {
        println!("{}", generate_cpf());
    }
}

