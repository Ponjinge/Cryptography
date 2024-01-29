use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Debug)]
struct UserWeak {
    name: String,
    prime_p: BigInt,
    prime_q: BigInt,
    e : BigInt,
    n : BigInt,
    eulers_totient : BigInt,
    d : BigInt,
    inbox : Vec<BigInt>,
}

impl UserWeak {
    fn new( name: String, prime_p: BigInt, prime_q: BigInt, e : BigInt)-> Self {
        let n = &prime_p * &prime_q;
        let eulers_totient = (&prime_p - BigInt::one()) * (&prime_q - BigInt::one());
        let d = mod_inv(&e, &eulers_totient).expect("Modular inverse does not exist"); // Handles error

        UserWeak { name, prime_p, prime_q, e, n, eulers_totient, d, inbox: Vec::new(), }
    }

    fn encrypt_message(&self, message: &BigInt) -> BigInt {
        message.modpow(&self.e, &self.n)
    }

    fn decrypt_message(&self, cipher: &BigInt) -> BigInt {
        cipher.modpow(&self.d, &self.n)
    }

    fn send_message(&self, recipient: UserWeak, message: &i32)-> () {

    }


}

fn gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt){
    if a.is_zero() {
        (b.clone(), BigInt::zero(), BigInt::one())
    }else {

        let (g,x,y) = gcd(&(b % a).clone() ,a);
        return (g, y.clone() - (b/a) * &x ,  x)
    }
}

fn mod_inv(a: &BigInt, m: &BigInt) ->Option<BigInt>{
    let (g, x, _) = gcd(a,m); //We dont need y
    if g == BigInt::one() {
        Some((x + m) % m) //Extra code to ensure that we do not get a negative value
    } else {
        None // Modular inverse does not exist
    }

}





fn main() {
    // Example prime numbers and exponent for RSA. In a real application, choose large primes.
    //John = User_weak("John", 27647, 60041, 1579)
    let prime_p = BigInt::from(27647u32);
    let prime_q = BigInt::from(60041u32);
    let e = BigInt::from(1579u32);

    let user = UserWeak::new("John Doe".to_string(), prime_p, prime_q, e);


    // Let us check is a user can be created
    println!("User created: {:?}", user);

    // Encrypt and Decrypt a message (here a BigInt)
    let mess = BigInt::from(123u32);
    let cipher = user.encrypt_message(&mess);
    println!("{}", cipher);
    let mess2 = user.decrypt_message(&cipher);
    println!("{}", mess2);
}



