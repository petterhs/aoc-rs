{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = [ pkgs.git ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/processes/
  processes.cargo-watch.exec = "cargo-watch";

  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  git-hooks.hooks = {
    rustfmt.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
