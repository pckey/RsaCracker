#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms)]
#![warn(missing_docs)]
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use key::PrivateKey;
use rug::integer::IsPrime;
use rug::Integer;
#[cfg(feature = "parallel")]
use std::sync::mpsc;
#[cfg(feature = "parallel")]
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

mod attack;
mod key;
mod ntheory;
mod params;
mod utils;

pub use attack::*;
pub use params::*;

/// Convert a `rug::Integer` to a byte vector.
pub fn integer_to_bytes(i: &Integer) -> Vec<u8> {
    base_x::decode("0123456789", &i.to_string()).unwrap()
}

/// Convert a `rug::Integer` to a string.
pub fn integer_to_string(i: &Integer) -> Option<String> {
    String::from_utf8(integer_to_bytes(i)).ok()
}

/// Run a single attack.
#[allow(clippy::borrowed_box)]
pub fn run_attack(
    attack: &Box<dyn Attack + Sync>,
    params: &Parameters,
    pb: Option<&ProgressBar>,
) -> Result<SolvedRsa, Error> {
    if let Some(pb) = pb {
        pb.set_style(
            ProgressStyle::with_template(
                "{prefix:>12.bold} [{elapsed_precise}] [{wide_bar}] {percent:>3}% ({eta:^4}) ",
            )
            .unwrap(),
        );
        pb.set_prefix(attack.name());
    }
    let res = attack.run(params, pb);
    // pb.finish_and_clear();

    let (private_key, m) = res?;
    // If we have a private key and a cipher message, decrypt it
    let m = if let (Some(private_key), Some(c), None) = (&private_key, &params.c, &m) {
        Some(private_key.decrypt(c))
    } else {
        m
    };
    Ok((private_key, m))
}

/// Run all attacks.
///
/// When the `parallel` feature is enabled, this function will run all attacks in parallel using all available CPU cores.
/// Else, it will run all attacks in sequence (single-threaded).
pub fn run_attacks(params: &Parameters) -> Option<SolvedRsa> {
    #[cfg(feature = "parallel")]
    return run_parallel_attacks(params, num_cpus::get());
    #[cfg(not(feature = "parallel"))]
    run_sequence_attacks(params)
}

fn check_n_prime(n: &Option<Integer>) -> Option<()> {
    if let Some(n) = &n {
        match n.is_probably_prime(30) {
            IsPrime::Yes => {
                eprintln!("N is prime, no attacks possible");
                return None;
            }
            IsPrime::Probably => {
                eprintln!("Warning: n is probably prime, but not certain");
            }
            _ => {}
        }
    }
    Some(())
}

fn given_p_q(params: &Parameters) -> Option<SolvedRsa> {
    if let (Some(p), Some(q)) = (&params.p, &params.q) {
        // If we have p and q, we can directly compute the private key
        let private_key = PrivateKey::from_p_q(p.clone(), q.clone(), params.e.clone()).ok()?;
        let m = params.c.as_ref().map(|c| private_key.decrypt(c));

        return Some((Some(private_key), m));
    }
    None
}

/// Run all attacks in sequence (single-threaded)
pub fn run_sequence_attacks(params: &Parameters) -> Option<SolvedRsa> {
    if let Some(solve) = given_p_q(params) {
        return Some(solve);
    }
    check_n_prime(&params.n)?;

    for attack in ATTACKS.iter() {
        match run_attack(attack, params, None) {
            Ok(solved) => return Some(solved),
            _ => continue,
        }
    }
    None
}

#[cfg(feature = "parallel")]
async fn _run_parallel_attacks(params: Arc<Parameters>, sender: mpsc::Sender<SolvedRsa>) {
    let mp = Arc::new(MultiProgress::new());
    let pb_main = Arc::new(mp.add(ProgressBar::new(ATTACKS.len() as u64)));
    pb_main.set_style(
        ProgressStyle::with_template(
            "{prefix:>12.green.bold} [{elapsed_precise}] [{wide_bar}] {pos}/{len}",
        )
        .unwrap()
        .progress_chars("=> "),
    );
    pb_main.set_prefix("Running");

    for attack in ATTACKS.iter() {
        let params = Arc::clone(&params);
        let sender = sender.clone();
        let mp = Arc::clone(&mp);
        let pb_main = Arc::clone(&pb_main);
        let pb = Arc::new(mp.insert(0, ProgressBar::new(1)));
        tokio::task::spawn(async move {
            let res = run_attack(attack, &params, Some(&pb));
            pb_main.inc(1);
            if let Ok(solved) = res {
                mp.suspend(|| {
                    sender.send(solved).expect("Failed to send result");
                    sleep(Duration::from_millis(100));
                });
            }
        });
    }
}

/// Run all attacks in parallel (multi-threaded)
#[cfg(feature = "parallel")]
pub fn run_parallel_attacks(params: &Parameters, threads: usize) -> Option<SolvedRsa> {
    if threads <= 1 {
        return run_sequence_attacks(params);
    }

    if let Some(solve) = given_p_q(params) {
        return Some(solve);
    }
    check_n_prime(&params.n)?;

    // Create channel for sending result
    let (sender, receiver) = mpsc::channel();

    // Create runtime
    let params = Arc::new(params.clone());
    let r = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(threads)
        .enable_all()
        .build()
        .unwrap();

    // Spawn attacks in background
    r.spawn(async { _run_parallel_attacks(params, sender).await });

    // Wait for result
    let res = receiver.recv().ok();

    // Shut down runtime
    r.shutdown_background();

    res
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn small_n_prime() {
        let params = Parameters {
            n: Some(17.into()),
            ..Default::default()
        };

        assert!(run_attacks(&params).is_none());
    }

    #[test]
    fn huge_n_prime() {
        let params = Parameters {
            n: Some(Integer::from_str("150950816111585055950436869236123284160990231413373647521828062928627753386032919782972055383166559635310664285162066372523610577993375279955113527486550958102770506832902500192425306025328135063076003572633672638348517524292873178557112480133897718407035981216896328771653270839246863855405457570499").unwrap()),  
            ..Default::default()
        };

        assert!(run_attacks(&params).is_none());
    }
}
