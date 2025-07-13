# RustWebApp

Welcome to Juras... I mean RustWebApp! A multiplayer puzzle games platform.

# Developer Setup

RustWebApp can be developed locally on your host of choice, or, entirely on Docker. We minimally use Docker to run local development databases.

> 🪟 **Windows** - I highly recommend using WSL - the Windows Subsystem for Linux, which Docker can also use for it's backend.
> 
> See [setup WSL](https://learn.microsoft.com/en-us/windows/wsl/install)

## Install Tools

Install the following for your platform first.

* [Git](https://git-scm.com/downloads) - for version control
* [Docker Desktop](https://www.docker.com/products/docker-desktop/) - for running containers locally
* [VS Code](https://code.visualstudio.com/) - <abbr title="Integrated Developer Experience">IDE</abbr> for editing code

### Host Development Tools

These tools are already installed in our [base docker image](./docker/rustwebapp.base.dockerfile).

* [Rust](https://rustup.rs/) - the language used for the back end server
* NodeJS + TypeScript - for the front end client
   
   Recommend using an installation manager
   * MacOS/Linux/WSL: [mise](https://github.com/jdx/mise)
   * Windows: [nvm](https://github.com/coreybutler/nvm-windows) or [NodeJS](https://nodejs.org/en) directly.

## Get the code

### Authentication

I recommend using SSH auth with GitHub.

See [GitHub SSH Docs](https://docs.github.com/en/authentication/connecting-to-github-with-ssh) for details.

TL;DR

```shell
~> ssh-keygen -t RSA -C YOUR_EMAIL -b 4096
# enter
# enter

# Keys created in ~/.ssh/id_rsa and ~/.ssh/id_rsa.pub

# Copy ~/.ssh/id_rsa.pub
# Go to https://github.com/settings/keys
# Add New SSH Key
# Paste Key

# Test auth by running
~> ssh git@github.com -T

# If it works you should see
Hi YOUR_USER_NAME! You've successfully authenticated, but GitHub does not provide shell access.
```

### Clone the repo

```shell
git clone git@github.com:kyle-rader/rustwebapp.git
```

## Develop

### On Docker

```shell
# Launch with all logs attached in the foreground
docker-compose up

# Launch in the background and then attach to logs
docker-compose up -d
```

Go to
```
http://localhost:3000
```

You should be redirected to `http://localhost:5173`.
In development we do not bundle assets and run a separate front-end live reload server with Vite.

### Host Development
