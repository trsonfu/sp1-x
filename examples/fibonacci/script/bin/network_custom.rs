use sp1_sdk::{
    include_elf, network::Error, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin,
};
use std::time::Duration;

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_elf!("fibonacci-program");

fn main() {
    // Setup logging.
    utils::setup_logger();

    // Get input from command line argument or use default
    let n = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(1000);

    println!("Computing fibonacci for n = {} using network", n);

    // The input stream that the program will read from using `sp1_zkvm::io::read`.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Create a `ProverClient` for network proving
    let client = ProverClient::from_env();

    // Generate the proof for the given program and input with network options
    let (pk, vk) = client.setup(ELF);
    
    println!("Requesting proof from network...");
    let proof_result = client.prove(&pk, &stdin)
        .compressed()  // Use compressed proof
        .timeout(Duration::from_secs(300))  // 5 minute timeout
        .cycle_limit(1000000)  // Set cycle limit
        .run();

    // Handle possible prover network errors with detailed error messages
    let mut proof = match proof_result {
        Ok(proof) => {
            println!("✅ Proof generated successfully from network!");
            proof
        },
        Err(e) => {
            if let Some(network_error) = e.downcast_ref::<Error>() {
                match network_error {
                    Error::RequestUnexecutable { request_id } => {
                        eprintln!("❌ Program is unexecutable");
                        eprintln!("   Request ID: {:?}", request_id);
                        eprintln!("   This usually means there's an issue with the program or input");
                        std::process::exit(1);
                    }
                    Error::RequestUnfulfillable { request_id } => {
                        eprintln!("❌ Proof request cannot be fulfilled");
                        eprintln!("   Request ID: {:?}", request_id);
                        eprintln!("   This might be due to network capacity or pricing issues");
                        std::process::exit(1);
                    }
                    Error::RequestTimedOut { request_id } => {
                        eprintln!("❌ Request timed out");
                        eprintln!("   Request ID: {:?}", request_id);
                        eprintln!("   Try increasing the timeout or retry later");
                        std::process::exit(1);
                    }
                    _ => {
                        eprintln!("❌ Unexpected network error: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("❌ Unexpected error: {}", e);
                std::process::exit(1);
            }
        }
    };

    // Read and verify the output.
    let input_n = proof.public_values.read::<u32>();
    let a = proof.public_values.read::<u32>();
    let b = proof.public_values.read::<u32>();

    println!("📊 Results:");
    println!("   Input n: {}", input_n);
    println!("   Fibonacci(n-1): {}", a);
    println!("   Fibonacci(n): {}", b);

    // Verify proof and public values
    println!("🔍 Verifying proof...");
    client.verify(&proof, &vk).expect("verification failed");
    println!("✅ Proof verification successful!");

    // Test a round trip of proof serialization and deserialization.
    let proof_file = format!("fibonacci-network-proof-{}.bin", n);
    proof.save(&proof_file).expect("saving proof failed");
    println!("💾 Proof saved to: {}", proof_file);

    let deserialized_proof =
        SP1ProofWithPublicValues::load(&proof_file).expect("loading proof failed");

    // Verify the deserialized proof.
    client.verify(&deserialized_proof, &vk).expect("verification failed");
    println!("✅ Deserialized proof verification successful!");

    println!("🎉 Successfully generated and verified proof for fibonacci({}) using network!", n);
} 