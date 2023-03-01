use basic_mutator::EmptyDatabase;
use basic_mutator::Mutator;
use felt::Felt;
fn main() {
    // Create a mutator for 128-byte ASCII printable inputs
    let mut mutator = Mutator::new()
        .seed(1337)
        .max_input_size(9000)
        .printable(true);

    for _ in 0..128 {
        // Update the input
        mutator.input.clear();
        mutator.input.extend([
            Felt::from(984232342),
            Felt::from(234234),
            Felt::from(234251241),
        ]);

        // Corrupt it with 4 mutation passes
        mutator.mutate(4, &EmptyDatabase);

        // Just print the string
        println!("simple: {:#?}", mutator.input);
    }
}
