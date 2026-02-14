use leptos::prelude::*;

const REVERSE_SNIPPET: &str = r#"enum List<T> {
    Nil,
    Cons(T, List<T>)
}

fn reverse_impl(list : List<i32>, acc : List<i32>) -> List<i32> {
    match list {
        List::Nil => acc,
        List::Cons(x, xs) => reverse_impl(xs, List::Cons{x, acc})
    }
}"#;

const TREE_MIRROR_SNIPPET: &str = r#"enum Tree {
    Leaf(i32),
    Node(Tree, Tree)
}

fn mirror(t : Tree) -> Tree {
    match t {
        Tree::Leaf(x) => Tree::Leaf{x},
        Tree::Node(l, r) => Tree::Node{mirror(r), mirror(l)}
    }
}"#;

const FIBONACCI_SNIPPET: &str = r#"struct [value] Matrix<T : Num> {
    m00: T, m01: T, m10: T, m11: T
}

fn matmul<T : Num>(a : Matrix<T>, b : Matrix<T>) -> Matrix<T> { ... }

fn fibonacci_logarithmic_impl<T : Integral>(n: T, a: Matrix<T>, b: Matrix<T>) -> T {
    if n == 0 { a.m01 } else { ... }
}"#;
const REUSSIR_REPO_URL: &str = "https://github.com/schrodingerzhu/reussir";

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="relative min-h-screen overflow-x-hidden bg-slate-950 text-slate-100">
            <div class="science-grid absolute inset-0 opacity-50"></div>
            <div class="hero-aurora absolute left-[-8rem] top-[-8rem] h-[28rem] w-[28rem] rounded-full blur-3xl"></div>
            <div class="hero-aurora-2 absolute bottom-[-10rem] right-[-10rem] h-[30rem] w-[30rem] rounded-full blur-3xl"></div>

            <main class="relative mx-auto flex w-full max-w-6xl flex-col gap-12 px-5 pb-20 pt-8 md:px-8 md:pt-12">
                <section class="rounded-3xl border border-slate-700/70 bg-slate-900/70 p-7 backdrop-blur md:p-12">
                    <div class="grid gap-8 lg:grid-cols-[1.25fr_1fr]">
                        <div class="space-y-5">
                            <p class="inline-flex items-center gap-2 rounded-full border border-cyan-300/50 bg-cyan-400/10 px-4 py-1.5 font-mono text-xs uppercase tracking-[0.18em] text-cyan-200">
                                <i class="fa-solid fa-satellite-dish"></i>
                                "comming soon"
                            </p>
                            <h1 class="font-display text-4xl leading-tight text-white md:text-6xl">
                                "Reussir"
                                <span class="block text-2xl text-slate-300 md:text-4xl">
                                    "Compiler Research, Memory Reuse, and MLIR"
                                </span>
                            </h1>
                            <p class="max-w-2xl text-base text-slate-300 md:text-lg">
                                "Reussir is a programming language project centered on RC-based memory reuse analysis and region-based memory management. This site is a technical preview and the language is not officially released."
                            </p>
                            <div class="flex flex-wrap gap-3">
                                <a
                                    class="btn h-auto min-h-0 gap-2 rounded-2xl border-none bg-cyan-300 px-6 py-3 text-sm font-semibold text-slate-950 hover:bg-cyan-200"
                                    href="#observatory"
                                >
                                    <i class="fa-solid fa-diagram-project"></i>
                                    <span class="whitespace-nowrap leading-none">"See Pipeline"</span>
                                </a>
                                <a
                                    class="btn h-auto min-h-0 gap-2 rounded-2xl border border-slate-500 bg-transparent px-6 py-3 text-sm font-semibold text-slate-200 hover:bg-slate-800"
                                    href="#evidence"
                                >
                                    <i class="fa-solid fa-code"></i>
                                    <span class="whitespace-nowrap leading-none">"Read Test Evidence"</span>
                                </a>
                                <a
                                    class="btn h-auto min-h-0 gap-2 rounded-2xl border border-cyan-400/60 bg-slate-900 px-6 py-3 text-sm font-semibold text-cyan-200 hover:bg-slate-800"
                                    href="/reussir-design-slides.pdf"
                                    target="_blank"
                                    rel="noreferrer"
                                >
                                    <i class="fa-solid fa-file-pdf"></i>
                                    <span class="whitespace-nowrap leading-none">"Design Slides (PDF)"</span>
                                </a>
                            </div>
                        </div>

                        <div class="rounded-2xl border border-slate-700 bg-slate-950/70 p-6">
                            <p class="mb-4 flex items-center gap-2 font-mono text-xs uppercase tracking-[0.2em] text-slate-400">
                                <i class="fa-solid fa-chart-line"></i>
                                "System Snapshot"
                            </p>
                            <div class="space-y-4">
                                <div>
                                    <div class="mb-1 flex items-center justify-between text-xs">
                                        <span class="text-slate-300">"Frontend"</span>
                                        <span class="text-cyan-300">"Haskell"</span>
                                    </div>
                                    <progress class="progress progress-info w-full" value="84" max="100"></progress>
                                </div>
                                <div>
                                    <div class="mb-1 flex items-center justify-between text-xs">
                                        <span class="text-slate-300">"Backend"</span>
                                        <span class="text-cyan-300">"MLIR + LLVM"</span>
                                    </div>
                                    <progress class="progress progress-info w-full" value="92" max="100"></progress>
                                </div>
                                <div>
                                    <div class="mb-1 flex items-center justify-between text-xs">
                                        <span class="text-slate-300">"Runtime"</span>
                                        <span class="text-cyan-300">"Rust RC/Region"</span>
                                    </div>
                                    <progress class="progress progress-info w-full" value="88" max="100"></progress>
                                </div>
                            </div>
                            <div class="mt-5 grid grid-cols-2 gap-2 text-xs">
                                <span class="rounded-lg border border-slate-700 bg-slate-900 px-2 py-1 text-center text-slate-300">"token reuse"</span>
                                <span class="rounded-lg border border-slate-700 bg-slate-900 px-2 py-1 text-center text-slate-300">"drop expansion"</span>
                                <span class="rounded-lg border border-slate-700 bg-slate-900 px-2 py-1 text-center text-slate-300">"inc/dec fusion"</span>
                                <span class="rounded-lg border border-slate-700 bg-slate-900 px-2 py-1 text-center text-slate-300">"mixed MM"</span>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                    <article class="rounded-2xl border border-slate-700 bg-slate-900/70 p-6">
                        <p class="font-mono text-xs text-cyan-300">"01 / FP foundation"</p>
                        <h2 class="mt-2 flex items-center gap-2 font-display text-xl text-white">
                            <i class="fa-solid fa-shuffle text-cyan-300"></i>
                            "Intro-Elim Reuse"
                        </h2>
                        <p class="mt-2 text-sm text-slate-300">
                            "Eliminated structure and newly introduced structure can share storage when ownership allows."
                        </p>
                    </article>
                    <article class="rounded-2xl border border-slate-700 bg-slate-900/70 p-6">
                        <p class="font-mono text-xs text-cyan-300">"02 / memory model"</p>
                        <h2 class="mt-2 flex items-center gap-2 font-display text-xl text-white">
                            <i class="fa-solid fa-memory text-cyan-300"></i>
                            "FBIP Direction"
                        </h2>
                        <p class="mt-2 text-sm text-slate-300">
                            "Functional-But-In-Place: unique references unlock safe in-place mutation and reuse."
                        </p>
                    </article>
                    <article class="rounded-2xl border border-slate-700 bg-slate-900/70 p-6">
                        <p class="font-mono text-xs text-cyan-300">"03 / compilation"</p>
                        <h2 class="mt-2 flex items-center gap-2 font-display text-xl text-white">
                            <i class="fa-solid fa-cubes text-cyan-300"></i>
                            "Pass-Oriented IR"
                        </h2>
                        <p class="mt-2 text-sm text-slate-300">
                            "Token instantiation, infer-tag, and drop expansion expose optimization opportunities."
                        </p>
                    </article>
                    <article class="rounded-2xl border border-slate-700 bg-slate-900/70 p-6">
                        <p class="font-mono text-xs text-cyan-300">"04 / interop"</p>
                        <h2 class="mt-2 flex items-center gap-2 font-display text-xl text-white">
                            <i class="fa-solid fa-plug-circle-bolt text-cyan-300"></i>
                            "RC Runtime + FFI"
                        </h2>
                        <p class="mt-2 text-sm text-slate-300">
                            "A polymorphic FFI path connects managed memory logic to native ecosystems."
                        </p>
                    </article>
                </section>

                <section id="observatory" class="rounded-3xl border border-slate-700 bg-slate-900/70 p-7 md:p-10">
                    <div class="mb-5 flex flex-wrap items-end justify-between gap-2">
                        <h2 class="flex items-center gap-3 font-display text-2xl text-white md:text-3xl">
                            <i class="fa-solid fa-timeline text-cyan-300"></i>
                            "Compilation Observatory"
                        </h2>
                        <p class="font-mono text-xs text-slate-400">
                            "from "
                            <a class="underline decoration-cyan-400/60 underline-offset-2 hover:text-cyan-300" href=REUSSIR_REPO_URL target="_blank" rel="noreferrer">
                                "github.com/schrodingerzhu/reussir"
                            </a>
                        </p>
                    </div>
                    <div class="grid gap-3 md:grid-cols-2 xl:grid-cols-4">
                        <div class="rounded-xl border border-slate-700 bg-slate-950 p-5">
                            <p class="font-mono text-xs text-cyan-300">"phase A"</p>
                            <p class="mt-1 text-sm text-slate-200">"token instantiation"</p>
                            <p class="text-sm text-slate-200">"inc/dec cancellation (1)"</p>
                            <p class="text-sm text-slate-200">"rc decrement expansion"</p>
                        </div>
                        <div class="rounded-xl border border-slate-700 bg-slate-950 p-5">
                            <p class="font-mono text-xs text-cyan-300">"phase B"</p>
                            <p class="mt-1 text-sm text-slate-200">"infer variant tag"</p>
                            <p class="text-sm text-slate-200">"drop expansion"</p>
                            <p class="text-sm text-slate-200">"inc/dec cancellation (2)"</p>
                        </div>
                        <div class="rounded-xl border border-slate-700 bg-slate-950 p-5">
                            <p class="font-mono text-xs text-cyan-300">"phase C"</p>
                            <p class="mt-1 text-sm text-slate-200">"drop logic expansion"</p>
                            <p class="text-sm text-slate-200">"token reuse"</p>
                            <p class="text-sm text-slate-200">"scf lowering"</p>
                        </div>
                        <div class="rounded-xl border border-slate-700 bg-slate-950 p-5">
                            <p class="font-mono text-xs text-cyan-300">"phase D"</p>
                            <p class="mt-1 text-sm text-slate-200">"polyffi + cf lowering"</p>
                            <p class="text-sm text-slate-200">"cf -> llvm"</p>
                            <p class="text-sm text-slate-200">"cse / canonicalize"</p>
                        </div>
                    </div>
                </section>

                <section class="rounded-3xl border border-slate-700 bg-slate-900/70 p-7 md:p-10">
                    <div class="mb-5 flex flex-wrap items-end justify-between gap-2">
                        <h2 class="flex items-center gap-3 font-display text-2xl text-white md:text-3xl">
                            <i class="fa-solid fa-gears text-cyan-300"></i>
                            "Backend Optimization, Heavily"
                        </h2>
                        <p class="font-mono text-xs text-slate-400">"MLIR backend is the center of Reussir's value"</p>
                    </div>
                    <div class="grid gap-4 lg:grid-cols-3">
                        <article class="rounded-2xl border border-slate-700 bg-slate-950 p-5">
                            <h3 class="mb-3 font-display text-xl text-white">"Ownership Exposure"</h3>
                            <ul class="space-y-2 text-sm text-slate-300">
                                <li>"- token instantiation makes allocation sites explicit"</li>
                                <li>"- rc.dec expansion lowers uniqueness checks into structured control flow"</li>
                                <li>"- infer-variant-tag unlocks precise drop specialization"</li>
                            </ul>
                        </article>
                        <article class="rounded-2xl border border-slate-700 bg-slate-950 p-5">
                            <h3 class="mb-3 font-display text-xl text-white">"Cancellation + Reuse"</h3>
                            <ul class="space-y-2 text-sm text-slate-300">
                                <li>"- inc/dec cancellation (twice) removes redundant refcount traffic"</li>
                                <li>"- drop expansion surfaces hidden decrements for fusion"</li>
                                <li>"- token reuse dataflow rewrites fresh alloc to reuse paths"</li>
                            </ul>
                        </article>
                        <article class="rounded-2xl border border-slate-700 bg-slate-950 p-5">
                            <h3 class="mb-3 font-display text-xl text-white">"Lowering Pipeline"</h3>
                            <ul class="space-y-2 text-sm text-slate-300">
                                <li>"- SCF-level ownership logic is preserved until late"</li>
                                <li>"- SCF -> CF -> LLVM lowering after reuse/cancellation decisions"</li>
                                <li>"- final canonicalization and CSE clean residual overhead"</li>
                            </ul>
                        </article>
                    </div>
                </section>

                <section class="rounded-3xl border border-slate-700 bg-slate-900/70 p-7 md:p-10">
                    <div class="mb-5 flex flex-wrap items-end justify-between gap-2">
                        <h2 class="flex items-center gap-3 font-display text-2xl text-white md:text-3xl">
                            <i class="fa-solid fa-microchip text-cyan-300"></i>
                            "Codegen Quality Story"
                        </h2>
                        <p class="font-mono text-xs text-slate-400">"from Fibonacci and Tree Mirror case studies"</p>
                    </div>
                    <div class="grid gap-4 lg:grid-cols-2">
                        <article class="rounded-2xl border border-slate-700 bg-slate-950 p-5">
                            <h3 class="mb-3 font-display text-xl text-white">"Fibonacci (matrix exponentiation)"</h3>
                            <ul class="space-y-2 text-sm text-slate-300">
                                <li>"- optimization shapes recursion into efficient accumulation-loop codegen"</li>
                                <li>"- in matmul(A, B), A can be reused when uniqueness is proven"</li>
                                <li>"- one remaining allocation in matmul(B, B) tracks shared ownership constraints"</li>
                            </ul>
                            <p class="mt-4 rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-xs text-slate-300">
                                "Effect: reduced allocation churn and improved locality without breaking FP semantics."
                            </p>
                        </article>
                        <article class="rounded-2xl border border-slate-700 bg-slate-950 p-5">
                            <h3 class="mb-3 font-display text-xl text-white">"Tree Mirror (non-tail recursion)"</h3>
                            <ul class="space-y-2 text-sm text-slate-300">
                                <li>"- ownership destruction is fused with traversal instead of a separate deep-free pass"</li>
                                <li>"- RC updates are emitted only where aliasing requires"</li>
                                <li>"- recursive-call barriers prevent unsafe token carry-over and heap growth"</li>
                            </ul>
                            <p class="mt-4 rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-xs text-slate-300">
                                "Effect: practical low-level codegen while keeping functional source structure."
                            </p>
                        </article>
                    </div>
                    <div class="mt-4 rounded-2xl border border-cyan-400/30 bg-cyan-500/10 p-5">
                        <h3 class="mb-3 flex items-center gap-2 font-display text-xl text-white">
                            <i class="fa-solid fa-link text-cyan-200"></i>
                            "polyFFI + Multimodal Memory"
                        </h3>
                        <div class="grid gap-3 md:grid-cols-2">
                            <div class="rounded-xl border border-slate-700 bg-slate-950 p-4 text-sm text-slate-300">
                                "polyFFI: runtime ownership model is bridgeable to native code through explicit headers/templates, enabling mixed-language systems workflows."
                            </div>
                            <div class="rounded-xl border border-slate-700 bg-slate-950 p-4 text-sm text-slate-300">
                                "Multimodal memory model: [value] (inline/no RC), [shared] (immutable RC), [regional] (region-scoped mutability), resolved before ownership emission."
                            </div>
                        </div>
                    </div>
                </section>

                <section id="evidence" class="space-y-4">
                    <div class="flex flex-wrap items-end justify-between gap-2">
                        <h2 class="flex items-center gap-3 font-display text-2xl text-white md:text-3xl">
                            <i class="fa-solid fa-flask-vial text-cyan-300"></i>
                            "Ground-Truth Examples"
                        </h2>
                        <p class="font-mono text-xs text-slate-400">
                            "examples from "
                            <a class="underline decoration-cyan-400/60 underline-offset-2 hover:text-cyan-300" href=REUSSIR_REPO_URL target="_blank" rel="noreferrer">
                                "reussir/tests/integration"
                            </a>
                        </p>
                    </div>
                    <div class="grid gap-4 lg:grid-cols-3">
                        <article class="rounded-2xl border border-slate-700 bg-slate-900/70 p-5 md:p-6">
                            <p class="mb-2 font-mono text-xs uppercase tracking-[0.15em] text-cyan-300">"list_reverse.rr"</p>
                            <pre class="overflow-x-auto rounded-xl bg-slate-950 p-4 text-xs leading-relaxed text-slate-100"><code>{REVERSE_SNIPPET}</code></pre>
                        </article>
                        <article class="rounded-2xl border border-slate-700 bg-slate-900/70 p-5 md:p-6">
                            <p class="mb-2 font-mono text-xs uppercase tracking-[0.15em] text-cyan-300">"tree_mirror.rr"</p>
                            <pre class="overflow-x-auto rounded-xl bg-slate-950 p-4 text-xs leading-relaxed text-slate-100"><code>{TREE_MIRROR_SNIPPET}</code></pre>
                        </article>
                        <article class="rounded-2xl border border-slate-700 bg-slate-900/70 p-5 md:p-6">
                            <p class="mb-2 font-mono text-xs uppercase tracking-[0.15em] text-cyan-300">"fibonacci-generic.rr"</p>
                            <pre class="overflow-x-auto rounded-xl bg-slate-950 p-4 text-xs leading-relaxed text-slate-100"><code>{FIBONACCI_SNIPPET}</code></pre>
                        </article>
                    </div>
                </section>

                <section class="grid gap-4 lg:grid-cols-[1.1fr_1fr]">
                    <article class="rounded-2xl border border-cyan-400/30 bg-cyan-500/10 p-6 md:p-7">
                        <h2 class="flex items-center gap-3 font-display text-2xl text-white">
                            <i class="fa-solid fa-robot text-cyan-200"></i>
                            "AI View (Excerpt)"
                        </h2>
                        <p class="mt-1 font-mono text-xs text-cyan-200">
                            "from "
                            <a class="underline decoration-cyan-300/70 underline-offset-2 hover:text-cyan-100" href=REUSSIR_REPO_URL target="_blank" rel="noreferrer">
                                "AGENTS.md in public repo"
                            </a>
                        </p>
                        <blockquote class="mt-5 rounded-xl border-l-4 border-cyan-300 bg-slate-950/70 p-5 text-sm text-slate-200">
                            "Reussir is developed \"with AI, and for AI.\""
                        </blockquote>
                        <ul class="mt-4 space-y-2 text-sm text-slate-200">
                            <li>"- clear logic and architecture"</li>
                            <li>"- continuous testing for newly added features"</li>
                            <li>"- clear documentation and comments"</li>
                            <li>"- efficient implementation for complex problems"</li>
                        </ul>
                    </article>
                    <article class="rounded-2xl border border-slate-700 bg-slate-900/70 p-6 md:p-7">
                        <h2 class="flex items-center gap-3 font-display text-2xl text-white">
                            <i class="fa-solid fa-user-group text-cyan-300"></i>
                            "People Involved"
                        </h2>
                        <p class="mt-1 font-mono text-xs text-slate-400">
                            "human contributors only, based on "
                            <a class="underline decoration-cyan-400/60 underline-offset-2 hover:text-cyan-300" href=REUSSIR_REPO_URL target="_blank" rel="noreferrer">
                                "repository history"
                            </a>
                        </p>
                        <div class="mt-4 grid gap-2 text-sm">
                            <div class="rounded-lg border border-slate-700 bg-slate-950 px-3 py-2">"Schrodinger ZHU Yifan"</div>
                            <div class="rounded-lg border border-slate-700 bg-slate-950 px-3 py-2">"Yekai Pan"</div>
                            <div class="rounded-lg border border-slate-700 bg-slate-950 px-3 py-2">"Archaversine / Adam"</div>
                            <div class="rounded-lg border border-slate-700 bg-slate-950 px-3 py-2">"queclr"</div>
                        </div>
                        <p class="mt-4 text-xs text-amber-300">
                            "AI accounts removed: copilot-swe-agent[bot], google-labs-jules[bot], Copilot."
                        </p>
                    </article>
                </section>
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
