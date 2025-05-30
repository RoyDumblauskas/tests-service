{
  description = "Dioxus Web App with NixOS module";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        dioxusApp = pkgs.stdenv.mkDerivation {
          name = "dioxus-web-app";
          src = ./.;

          nativeBuildInputs = [ pkgs.dioxus-cli pkgs.rustc pkgs.cargo ];

          buildPhase = ''
            dioxus build --release
          '';

          installPhase = ''
            mkdir -p $out
            cp -r dist/* $out/
          '';
        };
      in {
        packages.default = dioxusApp;

        # NixOS Module
        nixosModules.default = {
          config,
          lib,
          pkgs,
          ...
        }: let
          cfg = config.services.dioxus-web-app;
        in {
          options.services.dioxus-web-app = {
            enable = lib.mkEnableOption "Enable the Dioxus Web App";
            port = lib.mkOption {
              type = lib.types.port;
              default = 8080;
              description = "Port to serve the Dioxus web app on";
            };
          };

          config = lib.mkIf cfg.enable {
            systemd.services.dioxus-web-app = {
              description = "Dioxus Web App Server";
              wantedBy = [ "multi-user.target" ];
              serviceConfig = {
                ExecStart = "${pkgs.simple-http-server}/bin/simple-http-server ${dioxusApp} --port ${toString cfg.port}";
                WorkingDirectory = dioxusApp;
              };
            };

            networking.firewall.allowedTCPPorts = [ cfg.port ];
          };
        };
      });
}

