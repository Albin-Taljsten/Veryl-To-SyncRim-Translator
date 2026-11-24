use veryl_analyzer::{Analyzer, Context, ir::Ir};
use veryl_metadata::Metadata;
use veryl_parser::veryl_grammar_trait;
use anyhow::{Result};

/**
 * Gets the IR (Intermediate Representation)<br>
 * The IR comes from the given root node i.e. the given source code
 */
pub fn get_ir(root_node: &veryl_grammar_trait::Veryl) -> Result<Ir> {
    
    // Analyzer configuration/project context.
    let metadata = Metadata::create_default("test")?;
    let analyzer = Analyzer::new(&metadata);
    let mut context = Context::default();
    let mut ir = Ir::default();

        let mut errors = Vec::new();
    errors.extend(analyzer.analyze_pass1("my_project", &root_node));
    errors.extend(Analyzer::analyze_post_pass1());
    errors.extend(analyzer.analyze_pass2(&root_node, &mut context, Some(&mut ir)));
    errors.extend(Analyzer::analyze_post_pass2(&ir));

    if !errors.is_empty() {
        for error in &errors {
            eprintln!("{error:?}");
        }

        anyhow::bail!("Veryl analysis reported {} error(s)", errors.len());
    }
    Ok(ir)
}