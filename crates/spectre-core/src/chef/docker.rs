//! Docker container management for CyberChef-MCP

use bollard::container::{
    Config as ContainerConfig, CreateContainerOptions, ListContainersOptions,
    StartContainerOptions, StopContainerOptions,
};
use bollard::image::CreateImageOptions;
use bollard::Docker;
use futures::StreamExt;
use tracing::{debug, info, instrument, warn};

use crate::config::Config;
use crate::error::ChefError;

/// Docker manager for CyberChef-MCP container
pub struct DockerManager {
    docker: Docker,
    image: String,
    container_name: String,
}

impl DockerManager {
    /// Create a new Docker manager
    #[instrument(skip(config))]
    pub async fn new(config: &Config) -> crate::Result<Self> {
        info!("Connecting to Docker");

        let docker = Docker::connect_with_local_defaults().map_err(|e| {
            crate::SpectreError::Chef(ChefError::DockerError(format!(
                "Failed to connect to Docker: {}",
                e
            )))
        })?;

        // Verify connection
        docker.ping().await.map_err(|e| {
            crate::SpectreError::Chef(ChefError::DockerError(format!(
                "Failed to ping Docker: {}",
                e
            )))
        })?;

        debug!("Docker connection established");

        Ok(Self {
            docker,
            image: config.chef.docker_image.clone(),
            container_name: config.chef.container_name.clone(),
        })
    }

    /// Pull the CyberChef-MCP image
    #[instrument(skip(self))]
    pub async fn pull_image(&self) -> crate::Result<()> {
        info!(image = %self.image, "Pulling image");

        let options = Some(CreateImageOptions {
            from_image: self.image.clone(),
            ..Default::default()
        });

        let mut stream = self.docker.create_image(options, None, None);

        while let Some(result) = stream.next().await {
            match result {
                Ok(info) => {
                    if let Some(status) = info.status {
                        debug!(status = %status, "Pull progress");
                    }
                },
                Err(e) => {
                    return Err(crate::SpectreError::Chef(ChefError::DockerError(format!(
                        "Failed to pull image: {}",
                        e
                    ))));
                },
            }
        }

        info!("Image pulled successfully");
        Ok(())
    }

    /// Start the CyberChef-MCP container
    #[instrument(skip(self))]
    pub async fn start_container(&self) -> crate::Result<()> {
        info!("Starting CyberChef-MCP container");

        // Check if container already exists
        let existing = self.find_container().await?;

        if let Some(container) = existing {
            if container.state == Some("running".to_string()) {
                info!("Container already running");
                return Ok(());
            }

            // Start existing container
            debug!("Starting existing container");
            self.docker
                .start_container(&self.container_name, None::<StartContainerOptions<String>>)
                .await
                .map_err(|e| {
                    crate::SpectreError::Chef(ChefError::DockerError(format!(
                        "Failed to start container: {}",
                        e
                    )))
                })?;
        } else {
            // Create and start new container
            debug!("Creating new container");

            let config = ContainerConfig {
                image: Some(self.image.clone()),
                exposed_ports: Some(
                    vec![("3001/tcp".to_string(), Default::default())]
                        .into_iter()
                        .collect(),
                ),
                host_config: Some(bollard::service::HostConfig {
                    port_bindings: Some(
                        vec![(
                            "3001/tcp".to_string(),
                            Some(vec![bollard::service::PortBinding {
                                host_ip: Some("127.0.0.1".to_string()),
                                host_port: Some("3001".to_string()),
                            }]),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            };

            let options = Some(CreateContainerOptions {
                name: &self.container_name,
                platform: None,
            });

            self.docker
                .create_container(options, config)
                .await
                .map_err(|e| {
                    crate::SpectreError::Chef(ChefError::DockerError(format!(
                        "Failed to create container: {}",
                        e
                    )))
                })?;

            self.docker
                .start_container(&self.container_name, None::<StartContainerOptions<String>>)
                .await
                .map_err(|e| {
                    crate::SpectreError::Chef(ChefError::DockerError(format!(
                        "Failed to start container: {}",
                        e
                    )))
                })?;
        }

        info!("Container started");
        Ok(())
    }

    /// Stop the CyberChef-MCP container
    #[instrument(skip(self))]
    pub async fn stop_container(&self) -> crate::Result<()> {
        info!("Stopping CyberChef-MCP container");

        self.docker
            .stop_container(&self.container_name, Some(StopContainerOptions { t: 10 }))
            .await
            .map_err(|e| {
                // Ignore "container not running" errors
                if e.to_string().contains("not running") {
                    warn!("Container was not running");
                    return crate::SpectreError::Chef(ChefError::DockerError(
                        "Container was not running".to_string(),
                    ));
                }
                crate::SpectreError::Chef(ChefError::DockerError(format!(
                    "Failed to stop container: {}",
                    e
                )))
            })?;

        info!("Container stopped");
        Ok(())
    }

    /// Remove the CyberChef-MCP container
    #[instrument(skip(self))]
    pub async fn remove_container(&self) -> crate::Result<()> {
        info!("Removing CyberChef-MCP container");

        // Try to stop first (ignore errors)
        let _ = self.stop_container().await;

        self.docker
            .remove_container(&self.container_name, None)
            .await
            .map_err(|e| {
                crate::SpectreError::Chef(ChefError::DockerError(format!(
                    "Failed to remove container: {}",
                    e
                )))
            })?;

        info!("Container removed");
        Ok(())
    }

    /// Get container status
    #[instrument(skip(self))]
    pub async fn container_status(&self) -> crate::Result<String> {
        if let Some(container) = self.find_container().await? {
            Ok(container.state.unwrap_or_else(|| "unknown".to_string()))
        } else {
            Ok("not found".to_string())
        }
    }

    /// Find the CyberChef-MCP container
    async fn find_container(&self) -> crate::Result<Option<bollard::models::ContainerSummary>> {
        let options = Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        });

        let containers = self.docker.list_containers(options).await.map_err(|e| {
            crate::SpectreError::Chef(ChefError::DockerError(format!(
                "Failed to list containers: {}",
                e
            )))
        })?;

        let name_filter = format!("/{}", self.container_name);
        Ok(containers.into_iter().find(|c| {
            c.names
                .as_ref()
                .is_some_and(|names| names.contains(&name_filter))
        }))
    }
}

#[cfg(test)]
mod tests {
    // Docker tests require actual Docker daemon, so they're integration tests
    // Unit tests would require mocking the Docker client
}
