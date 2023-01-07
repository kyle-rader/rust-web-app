# Automata Games

> *For some definition of simple*

This repo provides a modern web application template that uses [Rails] for your classic MVC server-client relationship and [SvelteJS] for interactive JS components.

## Dev Setup
Either Linux, WSL (Windows Subsystem for Linux), or Mac will work best.

### Tools

* [Git](https://git-scm.com/)
* Libraries:
  * `libyaml` - needed for Ruby (`brew install libyaml`)
  * `libpq` - needed for postgres (`brew install libpq`)
* [Ruby](https://www.ruby-lang.org/en/) (`3.2.0-preview1`): _Recommended_ via [rbenv](https://github.com/rbenv/rbenv)
* [NodeJS](https://nodejs.org/en/) (LTS `16.15.1`): _Recommended_ via [nvm](https://github.com/nvm-sh/nvm)
* [Docker](https://www.docker.com/products/docker-desktop/) (If using WSL on Windows, install will default to the WSL2 engine - use this)
* The [Heroku CLI](https://devcenter.heroku.com/articles/heroku-cli#install-the-heroku-cli) - used as our local process runner.

### Setup
1. Clone the repo.

    SSH
    ```
    git clone git@github.com:kyle-rader/automata.games
    ```

    HTTPS
    ```
    git clone https://github.com/kyle-rader/automata.games
    ```

1. Confirm you have `bundle` and `npm` on your path (installed with Ruby, and NodeJS).

1. Install the Rails gem globally.
    ```
    gem install rails
    ```

1. Install ruby dependencies
    ```
    bundle install
    ```
    _Note_ that you may need to install `libpq-dev` in order to install the `pg` (Postgres) gem.

    On Linux or WSL this can be done with
    ```
    sudo apt install libpq-dev
    ```

    On Mac, you can
    ```
    brew install libpq
    ```
    and note that it's a keg-only and doesn't link, it will output the export line you can append to a profile to add the libpq bin dir to your PATH so that the `pg` gem can be built with native extensions.

1. Install JS dependencies
    ```
    npm install
    ```

1. Ensure `docker-compose` is installed (it should come with docker).

1. Bring up a Postgres instance in the background with
    ```
    docker-compose up -d
    ```

1. Create and migrate the database
    ```
    rails db:reset
    ```

1. Launch the rails web server and the vite dev server
    ```
    npm start
    ```
    _Note_ this just calls `heroku local -f Procfile.dev` simulating a Heroku deployment. You could also use the `foreman` gem for this.

[Rails]:https://rubyonrails.org/
[PostgreSQL]:https://www.postgresql.org/
[SvelteJS]:https://svelte.dev/
[TailWindCSS]: https://tailwindcss.com/
[Vite]: https://vitejs.dev/
