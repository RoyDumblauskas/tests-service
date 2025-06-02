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

      nativeBuildInputs = [ pkgs.cargo ]; # needed only if you actually run dx build inside the derivation

      installPhase = ''
        mkdir -p $out
        cp -r dist/* $out/
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

