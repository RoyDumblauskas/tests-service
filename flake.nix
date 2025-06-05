{
  description = "tests-service for Roy's Homelab";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    
    dioxusApp = pkgs.rustPlatform.buildRustPackage {
      pname = "tests-service";
      version = "0.1.0";
      src = ./.;

      cargoVendorDir = ./vendor;

      nativeBuildInputs = with pkgs; [
        dioxus-cli
        rustc
        wasm-bindgen-cli
        cargo

        pango
        cairo
        lld

        # tauri deps
        pkg-config
        gobject-introspection
        cargo-tauri
        nodejs
      ];

      buildInputs = with pkgs; [
        at-spi2-atk
        gdk-pixbuf
        glib
        gtk3
        harfbuzz
        librsvg
        libsoup_3
        webkitgtk_4_1
        openssl
        wasm-pack      
        xdotool

        # debug tools
        tree
      ];

      installPhase = ''
        dx clean
        dx bundle --platform web
      '';

    };

    nixosModule = { config, lib, pkgs, ... }: {
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
            ExecStart = "${dioxusApp}/bin/tests-service}";
            Restart = "always";
            Type = "simple";
            DynamicUser = "yes";
            WorkingDirectory = "${dioxusApp}";
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
    in {
      packages.${system}.default = dioxusApp;

      apps.${system}.default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/tests-service";
      };
    }
    // {
      nixosModules.default = nixosModule;
  };
}
