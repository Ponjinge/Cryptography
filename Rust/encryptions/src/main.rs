use num_bigint::BigInt;
use num_traits::{One, Zero};

//Backend
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, Filter};

#[derive(Debug)]
struct UserWeak {
    name: String,
    prime_p: BigInt,
    prime_q: BigInt,
    e: BigInt,
    n: BigInt,
    eulers_totient: BigInt,
    d: BigInt,
    inbox: Vec<BigInt>,
}

impl UserWeak {
    fn new(name: String, prime_p: BigInt, prime_q: BigInt, e: BigInt) -> Self {
        let n = &prime_p * &prime_q;
        let eulers_totient = (&prime_p - BigInt::one()) * (&prime_q - BigInt::one());
        let d = mod_inv(&e, &eulers_totient).expect("Modular inverse does not exist"); // Handles error

        UserWeak {
            name,
            prime_p,
            prime_q,
            e,
            n,
            eulers_totient,
            d,
            inbox: Vec::new(),
        }
    }

    fn encrypt_message(&self, message: &BigInt) -> BigInt {
        message.modpow(&self.e, &self.n)
    }

    fn decrypt_message(&self, cipher: &BigInt) -> BigInt {
        cipher.modpow(&self.d, &self.n)
    }

    fn send_message(&self, recipient: &mut UserWeak, message: &BigInt) -> () {
        let cipher = recipient.encrypt_message(message);
        recipient.inbox.push(cipher);
    }

    fn read_message(&self, index: usize) -> BigInt {
        self.decrypt_message(&self.inbox[index])
    }

    fn delete_message(&mut self, index: usize) -> () {
        self.inbox.remove(index);
    }
}

fn gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b.clone(), BigInt::zero(), BigInt::one())
    } else {
        let (g, x, y) = gcd(&(b % a).clone(), a);
        return (g, y.clone() - (b / a) * &x, x);
    }
}

fn mod_inv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = gcd(a, m); //We dont need y
    if g == BigInt::one() {
        Some((x + m) % m) //Extra code to ensure that we do not get a negative value
    } else {
        None // Modular inverse does not exist
    }
}

fn test_crud() {
    // Example prime numbers and exponent for RSA. In a real application, choose large primes.

    //John Doe Initialization
    let prime_p = BigInt::from(27647u32);
    let prime_q = BigInt::from(60041u32);
    let e = BigInt::from(1579u32);

    let mut user = UserWeak::new("John Doe".to_string(), prime_p, prime_q, e);

    //Alice Initialization
    let prime_p = BigInt::from(616445430637u128);
    let prime_q = BigInt::from(892411672643u128);
    let e = BigInt::from(1579u32);

    let mut user2 = UserWeak::new("Alice".to_string(), prime_p, prime_q, e);

    // Let us check is a user can be created
    println!("User created: {:?}", user);

    // Encrypt and Decrypt a message (here a BigInt)
    let mess = BigInt::from(123u32);
    let cipher = user.encrypt_message(&mess);
    println!("{}", cipher);
    let mess2 = user.decrypt_message(&cipher);
    println!("{}", mess2);

    // Send a message to another user
    let message_to_send = BigInt::from(30035u32);
    user.send_message(&mut user2, &message_to_send);

    // Read and delete the message
    println!("{:?}", user2.inbox);
    println!("{}", user2.read_message(0));
    user2.delete_message(0);
    println!("{:?}", user2.inbox); //Check the inbox is empty
}

// fn fermat_method(n: BigInt) -> (BigInt, BigInt) {
//     // An odd number can be represented as the difference of two squares
//     // n = a**2 - b**2
//     // We can then look for n + b**2 = a**2, and we can then use n = (`a+b)(a-b) to find the prime factors

//     for b in 1..=&n.sqrt() {
//         if (n + b * *BigInt::from(2u32)).sqrt().is_integer() {
//             let p = b + n + b * *BigInt::from(2u32).sqrt();
//             let q = n + b * *BigInt::from(2u32).sqrt() - b;
//             return (p, q);
//         }
//     }
//     return (BigInt::from(0u32), BigInt::from(0u32)); // No prime factors found
//                                                      //( int( b + math.sqrt(n + b**2) ), int(math.sqrt(n + b**2)-b))
// }

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
}

async fn create_user(user: User) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Creating user: {:?}", user.name);
    // Here you would insert logic to save the user in a database
    Ok(warp::reply::with_status(
        "User created",
        StatusCode::CREATED,
    ))
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type", "Authorization"])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]); // Ensure OPTIONS is allowed for preflight

    let user_route = warp::post()
        .and(warp::path("user"))
        .and(warp::body::json())
        .and_then(create_user)
        .with(cors);

    warp::serve(user_route).run(([127, 0, 0, 1], 3030)).await;
}
