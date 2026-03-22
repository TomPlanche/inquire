use inquire::{
    tabular::{ColumnAlignment, ColumnConfig},
    Select,
};

fn main() {
    println!("Example 1: Server Selection with Tabular Formatting\n");
    example_server_selection();

    println!("\n\nExample 2: Package Manager Style Display\n");
    example_package_manager();

    println!("\n\nExample 3: Process Manager\n");
    example_process_manager();
}

/// Example demonstrating server selection with aligned columns
/// Shows server name, IP, port, and status in a readable tabular format
fn example_server_selection() {
    let servers = vec![
        "web-server-01: 192.168.1.10, 8080, Running",
        "api-gateway: 192.168.1.20, 3000, Running",
        "db-primary: 192.168.1.30, 5432, Stopped",
        "db-replica: 192.168.1.31, 5432, Running",
        "cache: 192.168.1.40, 6379, Running",
    ];

    let columns = vec![
        ColumnConfig::new_with_separator(": ", ColumnAlignment::Left), // Server name
        ColumnConfig::new_with_separator(", ", ColumnAlignment::Right), // IP
        ColumnConfig::new_with_separator(", ", ColumnAlignment::Right), // Port
        ColumnConfig::new(ColumnAlignment::Left),                      // Status
    ];

    let ans = Select::new("Connect to server:", servers)
        .with_tabular_columns(columns)
        .with_help_message("Arrow keys to navigate, enter to select, type to filter")
        .prompt();

    match ans {
        Ok(server) => {
            let name = server.split(':').next().unwrap_or("unknown");
            println!("\nConnecting to {}...", name);
        }
        Err(_) => println!("Cancelled"),
    }
}

/// Example showing package selection with name, version, size, and description
fn example_package_manager() {
    let packages = vec![
        "serde: 1.0.197 (50.2 KB), Serialization framework",
        "tokio: 1.36.0 (634.8 KB), Async runtime",
        "clap: 4.5.1 (55.3 KB), Command line argument parser",
        "reqwest: 0.11.24 (186.4 KB), HTTP client",
        "sqlx: 0.7.3 (321.7 KB), SQL toolkit and ORM",
        "axum: 0.7.4 (102.1 KB), Web application framework",
    ];

    let columns = vec![
        ColumnConfig::new_with_separator(": ", ColumnAlignment::Left), // Package name
        ColumnConfig::new(ColumnAlignment::Right), // Version (default " " separator)
        ColumnConfig::new_with_separator(", ", ColumnAlignment::Right), // Size
        ColumnConfig::new(ColumnAlignment::Left),  // Description
    ];

    let ans = Select::new("Add dependency:", packages)
        .with_tabular_columns(columns)
        .prompt();

    match ans {
        Ok(package) => {
            let name = package.split(':').next().unwrap_or("unknown");
            println!("\nAdding {}...", name);
        }
        Err(_) => println!("Cancelled"),
    }
}

/// Example showing a process manager with PID, CPU, memory, and command
fn example_process_manager() {
    let processes = vec![
        "1234: cargo (2.5%), 128 MB, cargo build --release",
        "5678: node (0.3%), 64 MB, node server.js",
        "9012: postgres (1.1%), 256 MB, postgres: checkpointer",
        "3456: redis-server (0.0%), 8 MB, redis-server *:6379",
        "7890: nginx (0.1%), 12 MB, nginx: worker process",
    ];

    let columns = vec![
        ColumnConfig::new_with_separator(": ", ColumnAlignment::Right), // PID
        ColumnConfig::new(ColumnAlignment::Left), // Process name (default " " separator)
        ColumnConfig::new_with_separator(", ", ColumnAlignment::Right), // CPU
        ColumnConfig::new_with_separator(", ", ColumnAlignment::Right), // Memory
        ColumnConfig::new(ColumnAlignment::Left), // Command
    ];

    let ans = Select::new("Kill process:", processes)
        .with_tabular_columns(columns)
        .with_page_size(5)
        .prompt();

    match ans {
        Ok(process) => {
            let pid = process.split(':').next().unwrap_or("unknown").trim();
            println!("\nSending SIGTERM to PID {}...", pid);
        }
        Err(_) => println!("Cancelled"),
    }
}
