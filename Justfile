docker_image := "registry.gitlab.com/xfbs/restless"
docker_target_volume := "restless_target"
docker_registry_volume := "restless_registry"

list:
    just --list

docker-pull:
    docker pull {{docker_image}}

docker-build:
    docker build . -t {{docker_image}}

docker-run +command:
    docker volume create {{docker_target_volume}}
    docker volume create {{docker_registry_volume}}
    docker run -it --rm -v $(pwd):/crate:ro -v {{docker_target_volume}}:/crate/target -v {{docker_registry_volume}}:/usr/local/cargo/registry --workdir /crate {{docker_image}} {{command}}

docker-clean:
    docker volume remove {{docker_target_volume}}
    docker volume remove {{docker_registry_volume}}

docker +target:
    just docker-run just {{target}}

test:
    cargo test --all-features --all

features:
    cargo hack --feature-powerset --depth 2 check --all

style:
    cargo clippy --all-features --all
    cargo fmt --check

docs:
    cargo doc --all-features

book:
    mdbook build

coverage:
    cargo llvm-cov --html --all-features --all
    cargo llvm-cov report

check:
    just test style features
