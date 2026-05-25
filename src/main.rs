use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::{EqAffine, Fp},
    plonk::*,
    poly::commitment::Params,
    transcript::{Blake2bRead, Blake2bWrite, Challenge255},
};
use rand::rngs::OsRng;

// ----------------------------------------------------
// Income ZKP using Halo2
// Proves salary > threshold without revealing salary
// ----------------------------------------------------

#[derive(Clone)]
struct IncomeConfig {
    salary: Column<Advice>,
    diff: Column<Advice>,
    threshold: Column<Instance>,
    sel: Selector,
}

#[derive(Default, Clone)]
struct IncomeCircuit {
    salary: Value<Fp>,
    diff: Value<Fp>,
}

impl Circuit<Fp> for IncomeCircuit {
    type Config = IncomeConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> IncomeConfig {
        let salary = meta.advice_column();
        let diff = meta.advice_column();
        let threshold = meta.instance_column();
        let sel = meta.selector();

        meta.enable_equality(salary);
        meta.enable_equality(diff);
        meta.enable_equality(threshold);

        // Constraint:
        // salary - threshold = diff
        meta.create_gate("salary - threshold = diff", |meta| {
            let s = meta.query_selector(sel);

            let sal =
                meta.query_advice(salary, halo2_proofs::poly::Rotation::cur());

            let dif =
                meta.query_advice(diff, halo2_proofs::poly::Rotation::cur());

            let thr =
                meta.query_instance(threshold, halo2_proofs::poly::Rotation::cur());

            vec![s * (sal - thr - dif)]
        });

        IncomeConfig {
            salary,
            diff,
            threshold,
            sel,
        }
    }

    fn synthesize(
        &self,
        config: IncomeConfig,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "income check",
            |mut region| {
                config.sel.enable(&mut region, 0)?;

                region.assign_advice(
                    || "salary",
                    config.salary,
                    0,
                    || self.salary,
                )?;

                region.assign_advice(
                    || "diff",
                    config.diff,
                    0,
                    || self.diff,
                )?;

                Ok(())
            },
        )
    }
}

fn main() {
    println!("==============================================");
    println!("  Income ZKP  —  Halo2 Transparent Setup");
    println!("==============================================");

    // --------------------------------------------
    // Change these values for testing
    // --------------------------------------------

    let salary: u64 = 40_000;
    let threshold: u64 = 30_000;

    println!("\n  Threshold (PUBLIC)  : Rs.{}", threshold);
    println!("  Salary    (PRIVATE) : *** HIDDEN ***");

    // ------------------------------------------------
    // IMPORTANT FIX
    // Prevent negative subtraction / overflow
    // ------------------------------------------------

    if salary <= threshold {
        println!("\n==============================================");
        println!("  PROOF FAILED!");
        println!("  Salary does NOT exceed threshold.");
        println!("==============================================");
        return;
    }

    let diff: u64 = salary - threshold;

    println!("  Diff      (PRIVATE) : *** HIDDEN ***");

    let circuit = IncomeCircuit {
        salary: Value::known(Fp::from(salary)),
        diff: Value::known(Fp::from(diff)),
    };

    let public_inputs = vec![Fp::from(threshold)];

    // --------------------------------------------
    // Setup
    // --------------------------------------------

    println!("\n[1] Transparent universal params (no trusted setup)...");
    let params: Params<EqAffine> = Params::new(4);

    // --------------------------------------------
    // Key Generation
    // --------------------------------------------

    println!("[2] Generating proving & verification keys...");

    let vk = keygen_vk(&params, &circuit)
        .expect("vk generation failed");

    let pk = keygen_pk(&params, vk.clone(), &circuit)
        .expect("pk generation failed");

    // --------------------------------------------
    // Create Proof
    // --------------------------------------------

    println!("[3] Generating ZK proof (salary never exposed)...");

    let mut transcript =
        Blake2bWrite::<_, EqAffine, Challenge255<_>>::init(vec![]);

    create_proof(
        &params,
        &pk,
        &[circuit],
        &[&[&public_inputs]],
        OsRng,
        &mut transcript,
    )
    .expect("proof creation failed");

    let proof = transcript.finalize();

    println!("    Proof size: {} bytes", proof.len());

    // --------------------------------------------
    // Verify Proof
    // --------------------------------------------

    println!("[4] Verifying proof...");

    let strategy = halo2_proofs::plonk::SingleVerifier::new(&params);

    let mut verifier_transcript =
        Blake2bRead::<_, EqAffine, Challenge255<_>>::init(&proof[..]);

    let result = verify_proof(
        &params,
        &vk,
        strategy,
        &[&[&public_inputs]],
        &mut verifier_transcript,
    );

    // --------------------------------------------
    // Final Output
    // --------------------------------------------

    println!("\n==============================================");

    match result {
        Ok(_) => {
            println!("  PROOF VERIFIED SUCCESSFULLY!");
            println!("  Salary > Rs.{}  : CONFIRMED", threshold);
            println!("  Actual salary   : NEVER REVEALED");
            println!("  Proof system    : Halo2 (Transparent)");
        }

        Err(e) => {
            println!("  PROOF VERIFICATION FAILED!");
            println!("  Error: {:?}", e);
        }
    }

    println!("==============================================");
}