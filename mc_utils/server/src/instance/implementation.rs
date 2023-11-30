use std::time::Duration;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::BufRead,
    io::BufReader,
    io::{BufWriter, Write},
    process::Child,
};
use std::{fs::File, path::PathBuf};

use super::{run_server, InstanceError, Result};

pub const SERVER_PROPERTIES: &str = "server.properties";
pub const EULA_TXT: &str = "eula.txt";
pub const DEFAULT_WORLD_NAME: &str = "world";

/// A running minecraft server
///
/// When this object is dropped, the running minecraft server gets killed.
/// It is advised to 'stop()' the server before dropping it
#[derive(Debug)]
pub struct ServerInstance {
    pub dir: PathBuf,
    pub jar: PathBuf,
    pub world_name: String,
    pub properties: HashMap<String, String>,
    process: Child,
}

impl ServerInstance {
    /// Creates a new server instance builder at 'dir'
    pub fn builder(dir: impl Into<PathBuf>) -> ServerBuilder {
        ServerBuilder::new(dir)
    }

    /// Creates a new server instance builder with the server 'jar'
    pub fn with_jar(jar: impl Into<PathBuf>) -> ServerBuilder {
        let jar: PathBuf = jar.into();
        ServerBuilder::new(
            jar.parent()
                .expect("Could not find parent directory of jar"),
        )
        .server_path(jar)
    }

    /// Tries to stop the server gracefully
    ///
    /// Returns Err if that was not possible.
    pub fn try_stop(&mut self) -> Result<bool> {
        self.command("stop")?;
        Ok(self.process.wait().map(|code| code.success())?)
    }

    /// Kills the server, automatically run on drop
    pub fn kill(mut self) {
        self.process.kill().expect("Could not kill the server");
    }

    pub fn process(&self) -> &Child {
        &self.process
    }

    /// Executes a command at the server
    pub fn command(&mut self, command: &str) -> Result<()> {
        let mut stdin = self
            .process
            .stdin
            .take()
            .ok_or_else(|| InstanceError::Other("Cannot access stdin".to_string()))?;

        {
            let mut stdin_buf = BufWriter::new(&mut stdin);
            stdin_buf.write_all(command.trim_end().as_bytes())?;

            // Make sure to write a newline, but only if it is not in command
            if !command.ends_with('\n') {
                stdin_buf.write_all("\n".as_bytes())?;
            }
        }

        self.process.stdin.replace(stdin);

        Ok(())
    }

    /// Starts the server
    fn start(builder: ServerBuilder) -> Result<Self> {
        // initializes the properties file
        let properties = {
            let properties_path = builder.dir.join(SERVER_PROPERTIES);

            // If the properties file does not exist yet, the server has to generate them
            if !properties_path.exists() {
                let mut proc = run_server(&builder.server_path, &["--initSettings"])?;
                let status_code = proc.wait()?;
                if !status_code.success() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Server did not exit successfully: {}", status_code),
                    )
                    .into());
                }
            }

            let f = OpenOptions::new()
                .append(false)
                .write(true)
                .read(true)
                .open(properties_path)?;
            let mut properties: HashMap<String, String> =
                java_properties::read(BufReader::new(f.try_clone()?))?;

            // add the builder properties
            if !builder.properties.is_empty() {
                properties.extend(builder.properties);
                java_properties::write(f, &properties)?;
            }

            properties
        };

        // Write the eula.txt file
        {
            let mut eula_file = File::create(builder.dir.join(EULA_TXT))?;
            eula_file.write_all("eula=true".as_bytes())?;
        }

        // And finally start the server
        let mut process = run_server(
            &builder.server_path,
            &["--nogui", "--world", &builder.world_name],
        )?;

        // Wait for the server to load
        let stdout = process
            .stdout
            .take()
            .ok_or_else(|| InstanceError::Other("Can not access stdout".to_string()))?;
        let mut buffer = BufReader::new(stdout);

        loop {
            let mut line = String::new();
            buffer.read_line(&mut line)?;

            print!("[Server]: {}", line);
            if line.contains("[Server thread/INFO]: Done ") || line.is_empty() {
                break;
            }
        }

        // For some reason the next line breaks the server when using mcrcon
        // process.stdout.replace(buffer.into_inner());

        let instance = ServerInstance {
            dir: builder.dir,
            jar: builder.server_path,
            world_name: builder.world_name,
            properties,
            process,
        };

        // wait a bit to be sure that rcon can be accessed
        std::thread::sleep(Duration::from_secs(1));

        Ok(instance)
    }
}

impl Drop for ServerInstance {
    fn drop(&mut self) {
        match self.try_stop() {
            Ok(true) => (),
            _ => {
                self.process.kill().ok();
            }
        }
    }
}

/// A minecraft server builder
#[derive(Debug)]
pub struct ServerBuilder {
    dir: PathBuf,
    server_path: PathBuf,
    world_name: String,
    properties: HashMap<String, String>,
}

impl ServerBuilder {
    pub fn build(self) -> Result<ServerInstance> {
        ServerInstance::start(self)
    }

    /// Constructs a new server builder from a directory
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        let dir = dir.into();
        let server_path = dir.join("server.jar");
        ServerBuilder {
            dir,
            server_path,
            world_name: DEFAULT_WORLD_NAME.to_string(),
            properties: HashMap::new(),
        }
    }

    /// Sets the path of the server jar
    pub fn server_path(mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        if !path.starts_with(&self.server_path) {
            panic!("The server path must be withing the server directory")
        }
        self.server_path = path;
        self
    }

    /// Sets the name of the world that should be used
    pub fn world_name(mut self, name: impl Into<String>) -> Self {
        self.world_name = name.into();
        self
    }

    /// Sets a property for 'server.properties'
    pub fn property<T: Into<String>, U: Into<String>>(mut self, key: T, value: U) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}
