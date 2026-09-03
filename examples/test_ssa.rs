use atipicial_decompiler::Decompiler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <aef_file>", args[0]);
        std::process::exit(1);
    }

    let aef_path = &args[1];
    let manifest_path = format!(
        "{}.manifest.json",
        aef_path.strip_suffix(".aef").unwrap_or(aef_path)
    );

    let decompiler = Decompiler::new();
    let mut result = decompiler.decompile_file_with_manifest(
        aef_path,
        Some(&manifest_path),
        atipicial_decompiler::OutputFormat::HighLevel,
    )?;

    println!("=== Decompilation Results ===\n");
    println!("Instructions: {}", result.instructions.len());
    println!("CFG Blocks: {}", result.cfg.block_count());
    println!("Call Graph Edges: {}", result.call_graph.edges.len());

    // Compute SSA
    println!("\n=== SSA Transformation ===\n");
    result.compute_ssa();

    if let Some(ssa) = result.ssa() {
        println!("{}", ssa.stats());
        println!();
        println!("{}", ssa.render());
    } else {
        println!("No SSA form available (empty CFG)");
    }

    // Optimized + rendered SSA view (constant folding, copy propagation,
    // trivial-phi elimination, DCE — see Decompilation::optimize_ssa).
    println!("\n=== Optimized SSA (rendered) ===\n");
    println!("{}", result.render_optimized_ssa());

    // Structured IR view: control flow recovered from the CFG into typed
    // ir::ControlFlow (if / if-else) — the Phase-4 IR-spine path.
    println!("\n=== Structured IR (CFG → ir::ControlFlow) ===\n");
    println!("{}", result.render_structured_ir());

    Ok(())
}
