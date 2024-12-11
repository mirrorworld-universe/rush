REPO_ROOT=$(git rev-parse --show-toplevel)
solana-test-validator --reset --bpf-program FXm4HiySCyKv3HrynYKY7yfanyH7dJGMuvxXsbnvtW5c $REPO_ROOT/target/deploy/rush_ecs_store.so
