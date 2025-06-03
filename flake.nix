{
  description = "tests-service for Roy's Homelab";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    dioxusWebApp = pkgs.stdenv.mkDerivation {
      pname = "tests-service";
      version = "0.1.0";
      src = ./.;

      nativeBuildInputs = with pkgs; [
        dioxus-cli
        rustc
        wasm-bindgen-cli
        cargo

        # tauri deps
        pkg-config
        gobject-introspection
        cargo-tauri
        nodejs

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
        lld
      ];

      buildPhase = ''
        export HOME=$TMPDIR
        dx bundle --platform web
        echo "Contents of target:"
        ls -R target
      '';

    installPhase = ''
        echo "Attempting to install from:"
        ls -R target/dx/tests-service/release/web/public || true
        mkdir -p $out
        cp -r target/dx/tests-service/release/web/public/* $out/
      ''; 
    };

    nixosModule = { config, lib, pkgs, ... }: {
      options.services.tests-service = {
        enable = lib.mkEnableOption "Enable Dioxus static web service";

        default-nginx = {
          enable = lib.mkEnableOption "Enable nginx reverse proxy to serve the static site";
          hostname = lib.mkOption {
            type = lib.types.str;
            default = "localhost";
            description = "Hostname to serve the app";
          };
        };
      };

      config = lib.mkIf config.services.tests-service.enable {
        # Serve the static site via nginx
        services.nginx = lib.mkIf config.services.tests-service.default-nginx.enable {
          enable = true;
          virtualHosts."${config.services.tests-service.default-nginx.hostname}" = {
            forceSSL = true;
            enableACME = true;
            acmeRoot = null;
            root = "${dioxusWebApp}";
            locations."/" = {
              tryFiles = "$uri $uri/ /index.html";
            };
          };
        };

        # Open necessary ports
        networking.firewall.allowedTCPPorts = lib.mkIf config.services.tests-service.default-nginx.enable [
          80 443
        ];
      };
    };

  in {
    packages.${system}.default = dioxusWebApp;

    nixosModules.default = nixosModule;
  };
}

