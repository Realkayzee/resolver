use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ClapperArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

// ---------------
// Valid Commands: get | scaffold
// ---------------
#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Clones projects boilerplate for diamond standard (JavaScript, TypeScript and Foundry) and NestJs
    Get(GetCommand),
    /// Scaffolds projects for any development tool
    Scaffold(ScaffoldCommand),
}

// ----------------
// GetCommand Args
// ----------------
#[derive(Debug, Args)]
pub struct GetCommand{
    #[clap(subcommand)]
    pub command: GetSubCommand,
}


#[derive(Debug, Subcommand)]
pub enum GetSubCommand {
    /// Clones a diamond standard JavaScript project 
    Dhjs(GetDir),
    /// Clones a diamond standard TypeScript project
    Dhts(GetDir),
    /// Clones a diamond standard Foundry project
    Dfd(GetDir),
    /// Clones a NestJS project boilerplate
    Nestjs(GetDir)
}

// --------------------
// ScaffoldCommand Args
// --------------------
#[derive(Debug, Args)]
pub struct ScaffoldCommand{
    #[clap(subcommand)]
    pub command: ScaffoldSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ScaffoldSubCommand {
    /// Scaffolds a create-react-app JavaScript project
    Reactjs(GetDir),
    /// Scaffolds a create-react-app TypeScript project
    Reactts(GetDir),
    /// Scaffolds a Hardhat project
    Hardhat(GetDir),
    /// Scaffolds a NestJS project
    Nestjs(GetDir),
    /// Scaffolds a Laravel project
    Laravel(GetDir),
    /// Scaffolds a Next project
    Nextjs(GetDir),
}

// --------------------------------------
// GetDir: For passing the directory name
// --------------------------------------
#[derive(Debug, Args)]
pub struct GetDir {
    /// Specifies the name of the project directory to initialize
    pub dir_name: String
}