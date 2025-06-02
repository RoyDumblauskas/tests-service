{
  description = "tests-service for Roy's Homelab";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }: let
    nixosModule = {
      config,
      lib,
      pkgs,
      ...
    }: {
      options.services.tests-service = {
        enable = lib.mkEnableOption "Status reports for all homelab services";

        port = lib.mkOption {
          type = lib.types.port;
          default = 8080;
          description = "Port to listen on";
        };

        default-nginx = {
          enable = lib.mkEnableOption "Enable nginx reverse proxy to direct to service";
          hostname = lib.mkOption {
            type = lib.types.str;
            default = "localhost";
            description = "Hostname to attach to service";
          };
        };
      };

      config = lib.mkIf config.services.tests-service.enable {
        systemd.services.tests-service = {
          description = "Status page for homelab services";
          wantedBy = ["multi-user.target"];
          after = ["network.target"];
          serviceConfig = {
            ExecStart = "${self.packages.${pkgs.system}.default}/bin/tests-service}";
            Restart = "always";
            Type = "simple";
            DynamicUser = "yes";
            WorkingDirectory = "${self.packages.${pkgs.system}.default}";
          };
          environment = {
            PORT = toString config.services.tests-service.port;
            IP = "0.0.0.0";
          };
        };

        services.nginx = lib.mkIf config.services.tests-service.default-nginx.enable {
          enable = true;
          virtualHosts."${config.services.tests-service.default-nginx.hostname}" = {
            forceSSL = true;
            enableACME = true;
            acmeRoot = null;
            locations."/" = {
              proxyPass = "http://localhost:${toString config.services.tests-service.port}";
            };
          };
        };

        networking.firewall.allowedTCPPorts = lib.mkMerge [
          (lib.mkIf config.services.tests-service.enable [ config.services.tests-service.port ])
          (lib.mkIf config.services.tests-service.default-nginx.enable [ 80 443 ])
        ];
      };
    };
  in
    (flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};

      rustPlatform = pkgs.makeRustPlatform {
        cargo = pkgs.cargo;
        rustc = pkgs.rustc;
      };

      dioxusApp = rustPlatform.buildRustPackage {
        pname = "tests-service";
        version = "0.1.0";
        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

          nativeBuildInputs = with pkgs; [
            dioxus-cli
            rustc
            cargo

            # tauri deps
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
          dx bundle --release 
        '';

        installPhase = ''
          mkdir -p $out
          cp -r dist/* $out/
        '';
      };
    in {
      packages.default = dioxusApp;

      apps.default = {
        type = "app";
        program = "${self.packages.${pkgs.system}.default}/bin/tests-service";
      };
    }))
    // {
      nixosModules.default = nixosModule;
    };
}
