use primal::Primes;
use rug::{integer::IntegerExt64, Integer};

use crate::{Attack, AttackResult, Error, Parameters, PrivateKey};

/// Small prime attack
pub struct SmallPrimeAttack;

impl Attack for SmallPrimeAttack {
    fn name(&self) -> &'static str {
        "small_prime"
    }

    fn run(&self, params: &Parameters) -> AttackResult {
        let e = &params.e;
        let n = params.n.as_ref().ok_or(Error::MissingParameters)?;

        for p in Primes::all().take(10000000) {
            if p.ge(n) {
                break;
            }
            if n.is_divisible_u64(p as u64) {
                println!("find p = {}", p);
                let q = n.clone() / p;
                let p: Integer = p.into();

                let _d = e
                    .clone()
                    .invert(&((p.clone() - 1) * (q.clone() - 1)))
                    .map_err(|_| Error::NotFound)?;

                return Ok((Some(PrivateKey::from_p_q(p, q, e.clone())), None));
            }
        }

        Err(Error::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Attack, Parameters};

    use super::*;

    #[test]
    fn test_small_prime() {
        let params = Parameters {
            n: Some(Integer::from(5051846941u64)),
            ..Default::default()
        };

        let (priv_key, m) = SmallPrimeAttack.run(&params).unwrap();
        let priv_key = priv_key.unwrap();

        assert_eq!(priv_key.factors[0], Integer::from(54269));
        assert_eq!(priv_key.factors[1], Integer::from(93089));
        assert!(m.is_none());
    }
}
