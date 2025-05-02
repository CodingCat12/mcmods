module completions {

  # A CLI tool to manage mod versions for your project
  export extern mcmods [
    --verbose(-v)
    --help(-h)                # Print help
  ]

  def "nu-complete mcmods install loader" [] {
    [ "fabric" "neoforge" "quilt" "forge" "mod-loader" "lite-loader" "rift" "minecraft" "datapack" "folia" "paper" "purpur" "bukkit" "spigot" ]
  }

  def "nu-complete mcmods install version_type" [] {
    [ "release" "beta" "alpha" ]
  }

  # Install the latest compatible versions of specified mods
  export extern "mcmods install" [
    ...project_ids: string    # Project IDs of the mods to install
    --loader(-l): string@"nu-complete mcmods install loader" # Optional loader type to match against.
    --game-version(-g): string # Optional Minecraft version to match.
    --version-type(-v): string@"nu-complete mcmods install version_type" # Version channel to filter (default: release).
    --max-concurrent-tasks: string # Set maximum concurrent tasks (default: 3).
    --help(-h)                # Print help
  ]

  # Remove a mod by project ID or slug
  export extern "mcmods remove" [
    project_id: string        # ID or slug of the project to remove
    --help(-h)                # Print help
  ]

  export extern "mcmods sync" [
    --max-concurrent-tasks: string # Maximum number of concurrent tasks (default: 3)
    --help(-h)                # Print help
  ]

  def "nu-complete mcmods upgrade loader" [] {
    [ "fabric" "neoforge" "quilt" "forge" "mod-loader" "lite-loader" "rift" "minecraft" "datapack" "folia" "paper" "purpur" "bukkit" "spigot" ]
  }

  def "nu-complete mcmods upgrade version_type" [] {
    [ "release" "beta" "alpha" ]
  }

  # Upgrade mods to their latest compatible versions
  export extern "mcmods upgrade" [
    ...project_ids: string    # Project IDs to upgrade. Use `--all` to upgrade everything
    --all(-a)                 # Upgrade all installed mods
    --loader(-l): string@"nu-complete mcmods upgrade loader" # Mod loader to filter by (e.g., fabric, forge)
    --game-version(-g): string # Minecraft version to filter by (e.g., 1.20.4)
    --version-type(-v): string@"nu-complete mcmods upgrade version_type" # Release channel to filter by (e.g., release, beta, alpha)
    --max-concurrent-tasks: string # Maximum number of concurrent tasks (default: 3)
    --help(-h)                # Print help
  ]

  # List installed mods
  export extern "mcmods list" [
    --help(-h)                # Print help
  ]

  def "nu-complete mcmods complete shell" [] {
    [ "bash" "zsh" "elvish" "fish" "power-shell" "nushell" ]
  }

  # Show completions
  export extern "mcmods complete" [
    --shell: string@"nu-complete mcmods complete shell" # The shell to generate completions for
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "mcmods help" [
  ]

  # Install the latest compatible versions of specified mods
  export extern "mcmods help install" [
  ]

  # Remove a mod by project ID or slug
  export extern "mcmods help remove" [
  ]

  export extern "mcmods help sync" [
  ]

  # Upgrade mods to their latest compatible versions
  export extern "mcmods help upgrade" [
  ]

  # List installed mods
  export extern "mcmods help list" [
  ]

  # Show completions
  export extern "mcmods help complete" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "mcmods help help" [
  ]

}

export use completions *
