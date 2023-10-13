fn main() -> std::io::Result<()> {
    // build options corresponding to ./configure --ultimate
    let mut build = cc::Build::new();
    build
        .include("src")
        .warnings(true)
        .debug(false)
        .opt_level(3)
        .define("COMPACT", None)
        .define("NDEBUG", None)
        .define("NOPTIONS", None)
        .define("NPROOFS", None)
        .define("QUIET", None);

    let version = std::fs::read_to_string("kissat/VERSION");
    let version = version.expect("missing kissat submodule");
    let version = format!("\"{}\"", version.trim());
    build.define("VERSION", version.as_ref());

    let files = vec![
        "kissat/src/allocate.c",
        "kissat/src/analyze.c",
        "kissat/src/ands.c",
        "kissat/src/arena.c",
        "kissat/src/assign.c",
        "kissat/src/averages.c",
        "kissat/src/backbone.c",
        "kissat/src/backtrack.c",
        "kissat/src/build.c",
        "kissat/src/bump.c",
        "kissat/src/check.c",
        "kissat/src/clause.c",
        "kissat/src/collect.c",
        "kissat/src/colors.c",
        "kissat/src/compact.c",
        "kissat/src/config.c",
        "kissat/src/decide.c",
        "kissat/src/deduce.c",
        "kissat/src/definition.c",
        "kissat/src/dense.c",
        "kissat/src/dump.c",
        "kissat/src/eliminate.c",
        "kissat/src/equivalences.c",
        "kissat/src/error.c",
        "kissat/src/extend.c",
        "kissat/src/file.c",
        "kissat/src/flags.c",
        "kissat/src/format.c",
        "kissat/src/forward.c",
        "kissat/src/gates.c",
        "kissat/src/heap.c",
        "kissat/src/ifthenelse.c",
        "kissat/src/import.c",
        "kissat/src/internal.c",
        "kissat/src/kimits.c",
        "kissat/src/kitten.c",
        "kissat/src/learn.c",
        "kissat/src/logging.c",
        "kissat/src/minimize.c",
        "kissat/src/mode.c",
        "kissat/src/options.c",
        "kissat/src/phases.c",
        "kissat/src/print.c",
        "kissat/src/probe.c",
        "kissat/src/profile.c",
        "kissat/src/promote.c",
        "kissat/src/proof.c",
        "kissat/src/propbeyond.c",
        "kissat/src/propdense.c",
        "kissat/src/proprobe.c",
        "kissat/src/propsearch.c",
        "kissat/src/queue.c",
        "kissat/src/reduce.c",
        "kissat/src/reluctant.c",
        "kissat/src/rephase.c",
        "kissat/src/report.c",
        "kissat/src/resize.c",
        "kissat/src/resolve.c",
        "kissat/src/resources.c",
        "kissat/src/restart.c",
        "kissat/src/search.c",
        "kissat/src/shrink.c",
        "kissat/src/smooth.c",
        "kissat/src/sort.c",
        "kissat/src/stack.c",
        "kissat/src/statistics.c",
        "kissat/src/strengthen.c",
        "kissat/src/substitute.c",
        "kissat/src/sweep.c",
        "kissat/src/terminate.c",
        "kissat/src/trail.c",
        "kissat/src/transitive.c",
        "kissat/src/utilities.c",
        "kissat/src/vector.c",
        "kissat/src/vivify.c",
        "kissat/src/walk.c",
        "kissat/src/warmup.c",
        "kissat/src/watch.c",
        "kissat/src/weaken.c",
    ];

    if build.get_compiler().is_like_msvc() {
        build.include("src/msvc");
    }

    build.files(files.iter());
    for &file in files.iter() {
        println!("cargo:rerun-if-changed={}", file);
    }

    build.compile("kissat");
    Ok(())
}
