pub mod commands;

use clap::{Parser, Subcommand};

/// LumaStack Backend CLI
#[derive(Parser, Debug)]
#[command(name = "lumastack-backend")]
#[command(about = "LumaStack Backend - Git repository monitoring with Telegram integration", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Crear usuario administrador
    CreateAdmin {
        /// Email del administrador
        #[arg(short, long)]
        email: Option<String>,

        /// Username del administrador
        #[arg(short, long)]
        username: Option<String>,

        /// Password del administrador
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Iniciar el servidor HTTP
    Serve,
}
