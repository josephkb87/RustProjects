## Rust Projects: RFSystemsCookbook

This Rust Cookbook Projects include;
```
rf-sic/
├── Cargo.toml                    # Rust workspace
├── rust-backend/
│   ├── src/
│   │   ├── main.rs               # Ratatui TUI entry point
│   │   ├── ui/                   # Ratatui widgets, layouts, event loop
│   │   ├── rf/                   # RF simulation engine (tpt-dsp, rustradio)
│   │   ├── problems/             # 50 problem modules (Rust-native solvers)
│   │   └── python_bridge/        # PyO3 bindings → Python backend
│   └── Cargo.toml
├── python-backend/
│   ├── notebooks/                # Jupyter notebooks (scikit-rf, numpy, scipy)
│   ├── solvers/                  # Python RF solvers (link budget, S-params)
│   └── requirements.txt
├── matlab-octave/
│   ├── scripts/                  # .m files (Octave-compatible)
│   └── results/
├── docs/
│   ├── NOTES.md                  # Private reasoning / reference model
│   ├── REPORTS.md                # Research details & solution paths
│   ├── WHITEPAPER.md             # Funding / standards justification
│   ├── latex/
│   │   └── rf_sic_problems.tex   # Shareable LaTeX problem compendium
│   └── problems_index.md         # Tabular index of 50 problems
├── .github/workflows/
│   ├── ci.yml                    # Rust + Python CI (test, lint, deny)
│   └── cd.yml                    # Release artefacts, Docker, docs
└── README.md
```

## For other Rust Projects go to:

- RustWebM - A [Rust_WASM WebApp CrossPlatform Tool(Application) for WebSystems Management](../Projects/RustWebApp/README.md) 

- QMLApp- [A QT Rust App](../Projects/QMLAppl/README.md)
  
- BridgesBot- A [Rust Tool for generating Tor Bridges using IPFS](https://github.com/josephkb87/Bridgesbot).

- RustAPI - [A RESTful RustAPI](https://github.com/josephkb87/Rustful_API).

- ToDoEmail - A [Rust TO_DO_App](../Projects/ToDoEmail/README.md) that sends a to do list to an email.

- Rust_SCM - A [Rust SCM Tool](https://github.com/josephkb87/QRust).

- Fortran IDE - A [Fortran IDE](../Projects/Fortran IDE/README.md). made with Rust and Bazel.

- [**LogoGen**](https://github.com/josephkb87/LogoGen) - A Logo Generator tool(framework) made with Rust.
  
- [**DocsGen**](https://github.com/josephkb87/docsgen) - A Documentation Generator made with Rust and OCaml.

#### [Docs](..docs/)

* [About Me](https://github.com/josephkb87)
* [AUTHORS](../docs/AUTHORS.md)
* [Rust Worked Examples](https://github.com/josephkb87/Beginners/tree/main/RustWorkedExamples/README.md)
* [Rust Projects](https://github.com/josephkb87/RustProjects/)
* [About Rust Beginners](../README.md)
* [CHANGELOG](../docs/CHANGELOG.md) 
* [Contribute](../docs/CONTRIBUTING.md)
* [Pull Requests](../docs/blob/PRs.md)
* [SECURITY](../docs/SECURITY.md)
* [Attributions](..docs/Attributions.md)
* [References](../docs/References.md)
