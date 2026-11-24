# Thesis-Project
My thesis project in a private repository. Will be turned public or another repository will be made that is public once the project is considered complete.

## How the project structure should look like
This structure should let the api have access to the things it needs. Which is all veryl files located in the "src" folder and any subfolders inside "src". For SyncRim I haven't yet decided what my api needs access to but realistically it would need some sort of access to the simulator and a way to generate a visual of the code that my api has made. As for how I will access the parser from the "verylup" files stored on the computer globally by running "cargo install verylup" is something that needs to be looked into futher.

The api will need to have a script of some sort that checks for all veryl files that exist within the project. Those files will then be sent to the main api parser that parses them into SyncRim ready components.
<pre>
📁 your-thesis-project/           
│
├── 📄 Cargo.toml                  <-- 1. THE MASTER WORKSPACE CONFIG
├── 📁 veryl_project/              
│   ├── 📄 Veryl.toml              <-- 3. UNTOUCHED (Veryl's original tool configuration)
│   └── 📁 src/
│       └── 📄 test.veryl
└── 📁 bridge_api/                 
    ├── 📄 Cargo.toml              <-- 4. API CONFIG
    └── 📁 src/
        └── 📄 main.rs
</pre>
Then from the main root of the project so in this case that would be "your-thesis-project" you could run different "cargo run" commands depending on what functions you'd like to use. Examples:

`cargo run -p bridge_api`<br>
`cargo run -p syncrim --example counter`<br>
and even<br>
`veryl test --manifest-path veryl_project/Veryl.toml`

Ofcourse the names of the folders have to change in order for these commands to work fine and feel fine for the user.
