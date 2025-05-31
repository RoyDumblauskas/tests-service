{
  description = "Dioxus Web App with NixOS module";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };

      dioxusApp = pkgs.stdenv.mkDerivation {
          name = "dioxus-web-app";
          src = ./.;

          nativeBuildInputs = with pkgs; [
            dioxus-cli
            rustc
            cargo

            # tauri deps?
            pkg-config
            gobject-introspection
            cargo-tauri
            nodejs
          ];

          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
            wasm-pack
            wasm-bindgen-cli
            lld
          ];

          buildPhase = ''
            dx build --release
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

            # Nginx reverse proxy setup
            services.nginx = lib.mkIf cfg.nginx.enable {
              enable = true;
              virtualHosts."${cfg.nginx.hostname}" = {
                forceSSL = true;
                enableAMCE = true;
                acmeRoot = null;
                locations."/" = {
                  proxyPass = "http://localhost:${toString cfg.port}";
                  extraConfig = ''
                    proxy_set_header Host $host;
                    proxy_set_header X-Real-IP $remote_addr;
                    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                    proxy_set_header X-Forwarded-Proto $scheme;
                  '';
                };
              };
            };
          };

        networking.firewall.allowedTCPPorts = lib.mkMerge [
          (lib.mkIf cfg.enable [ cfg.port ])
          (lib.mkIf cfg.nginx.enable [ 80 ])
        ];
      };
    };
  });
}

