#![forbid(unsafe_code)]

pub mod templates;
pub mod entity;

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::scaffold::entity::{parse_entity_spec, EntitySpec};

pub fn create_project(name: &str, path: Option<&Path>, entities: &[String]) -> Result<()> {
    let entity_specs = parse_entities(entities)?;
    let project_dir = path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(name));

    if project_dir.exists() {
        bail!("target directory already exists: {}", project_dir.display());
    }

    fs::create_dir_all(project_dir.join("src/controllers"))
        .context("failed to create src/controllers")?;
    fs::create_dir_all(project_dir.join("src/domain"))
        .context("failed to create src/domain")?;
    fs::create_dir_all(project_dir.join("src/mappers"))
        .context("failed to create src/mappers")?;
    fs::create_dir_all(project_dir.join("src/repository/entities"))
        .context("failed to create src/repository/entities")?;
    fs::create_dir_all(project_dir.join("src/repository/repositories"))
        .context("failed to create src/repository/repositories")?;
    fs::create_dir_all(project_dir.join("src/services"))
        .context("failed to create src/services")?;

    write_file(
        project_dir.join("Cargo.toml"),
        &templates::render_cargo_toml(name),
    )?;
    write_file(
        project_dir.join("README.md"),
        &templates::render_readme(name),
    )?;
    write_file(project_dir.join("src/config.rs"), &templates::render_config_rs())?;
    write_file(
        project_dir.join("src/main.rs"),
        &templates::render_main_rs(&entity_specs),
    )?;
    write_file(
        project_dir.join("src/controllers/mod.rs"),
        &templates::render_controllers_mod(&entity_specs),
    )?;
    write_file(
        project_dir.join("src/controllers/health_controller.rs"),
        &templates::render_health_controller(),
    )?;
    write_file(
        project_dir.join("src/controllers/dto.rs"),
        &templates::render_controller_dto(&entity_specs),
    )?;
    write_file(
        project_dir.join("src/domain/mod.rs"),
        &templates::render_domain_mod(&entity_specs),
    )?;
    for entity in &entity_specs {
        write_file(
            project_dir.join(format!("src/domain/{}.rs", entity.snake_name())),
            &templates::render_domain_entity(entity),
        )?;
    }
    write_file(
        project_dir.join("src/mappers/mod.rs"),
        &templates::render_mappers_mod(&entity_specs),
    )?;
    write_file(
        project_dir.join("src/mappers/controller_mapper.rs"),
        &templates::render_controller_mapper(&entity_specs),
    )?;
    write_file(
        project_dir.join("src/mappers/repository_mapper.rs"),
        &templates::render_repository_mapper(&entity_specs),
    )?;
    write_file(
        project_dir.join("src/repository/mod.rs"),
        &templates::render_repository_mod(&entity_specs),
    )?;
    write_file(
        project_dir.join("src/repository/entities/mod.rs"),
        &templates::render_repository_entities_mod(&entity_specs),
    )?;
    for entity in &entity_specs {
        write_file(
            project_dir.join(format!(
                "src/repository/entities/{}_entity.rs",
                entity.snake_name()
            )),
            &templates::render_repository_entity(entity),
        )?;
    }
    write_file(
        project_dir.join("src/repository/repositories/mod.rs"),
        &templates::render_repository_repositories_mod(&entity_specs),
    )?;
    for entity in &entity_specs {
        write_file(
            project_dir.join(format!(
                "src/repository/repositories/{}_repository.rs",
                entity.snake_name()
            )),
            &templates::render_repository_repository(entity),
        )?;
    }
    write_file(
        project_dir.join("src/services/mod.rs"),
        &templates::render_services_mod(&entity_specs),
    )?;
    write_file(
        project_dir.join("src/services/system_info_service.rs"),
        &templates::render_system_info_service(),
    )?;
    for entity in &entity_specs {
        write_file(
            project_dir.join(format!("src/services/{}_service.rs", entity.snake_name())),
            &templates::render_service(entity),
        )?;
    }
    write_file(
        project_dir.join("application.yaml"),
        &templates::render_application_yaml(name),
    )?;

    for entity in &entity_specs {
        write_file(
            project_dir.join(format!(
                "src/controllers/{}_controller.rs",
                entity.snake_name()
            )),
            &templates::render_controller(entity),
        )?;
    }

    println!("Ember project created at: {}", project_dir.display());
    println!("Next steps:");
    println!("  cd {}", project_dir.display());
    println!("  cargo run");
    Ok(())
}

fn write_file(path: PathBuf, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory: {}", parent.display()))?;
    }
    fs::write(&path, contents)
        .with_context(|| format!("failed to write file: {}", path.display()))?;
    Ok(())
}

fn parse_entities(entities: &[String]) -> Result<Vec<EntitySpec>> {
    if entities.is_empty() {
        return Ok(Vec::new());
    }

    let mut specs = Vec::new();
    for entity in entities {
        let spec = parse_entity_spec(entity)
            .map_err(|err| anyhow::anyhow!("invalid --entity '{entity}': {err}"))?;
        specs.push(spec);
    }
    Ok(specs)
}
