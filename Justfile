docker_image := "registry.gitlab.com/xfbs/restless"
docker_target_volume := "restless_target"
docker_registry_volume := "restless_registry"

# list commands
list:
    just --list

# pull docker image
docker-pull:
    docker pull {{docker_image}}

# build docker image
docker-build:
    docker build . -t {{docker_image}}

# run a raw command in docker
docker-run +command:
    docker volume create {{docker_target_volume}}
    docker volume create {{docker_registry_volume}}
    docker run -it --rm -v $(pwd):/crate:ro -v {{docker_target_volume}}:/crate/target -v {{docker_registry_volume}}:/usr/local/cargo/registry --workdir /crate {{docker_image}} {{command}}

# delete docker volumes
docker-clean:
    docker volume remove {{docker_target_volume}}
    docker volume remove {{docker_registry_volume}}

# run some just targets inside the docker container (try `just docker check`)
docker +target:
    just docker-run just {{target}}

# run tests (using cargo test)
test:
    cargo test --all-features --all

# test crate features (using cargo hack)
features:
    cargo hack --feature-powerset --depth 2 check --all

# check style (using clippy and rustfmt)
style:
    cargo clippy --all-features --all
    cargo fmt --check --all

# build crate docs (using rustdoc)
docs:
    cargo doc --all-features

# build book (using mdbook)
book:
    mdbook build

# build coverage (using cargo llvm-cov)
coverage:
    cargo llvm-cov --html --all-features --all
    cargo llvm-cov report

# run checks (runs test, style and features)
checks:
    just test style features
