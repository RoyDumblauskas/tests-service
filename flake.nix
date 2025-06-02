{
  description = "tests-service for Roy's Homelab";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
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
          hostname = {
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
            ExecStart = "${pkgs.simple-http-server}/bin/tests-service  --port ${ toString config.services.tests-service.port }";
            Restart = "always";
            Type = "simple";
            DynamicUser = "yes";
            WorkingDirectory = "${self.packages.${pkgs.system}.default}";
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
              extraConfig = ''
                proxy_set_header Host $host;
                proxy_set_header X-Real-IP $remote_addr;
                proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                proxy_set_header X-Forwarded-Proto $scheme;
              '';
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

      dioxusApp = pkgs.stdenv.mkDerivation {
        pname = "tests-service";
        version = "0.1.0";
        src = ./.;

        buildInputs = with pkgs; [
            dioxus-cli
            rustc
            cargo

            # tauri deps?
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

      apps.default = {
        type = "app";
        program = "${dioxusApp}/index.html";
      };
    }))
    // {
      nixosModules.default = nixosModule;
    };
}
