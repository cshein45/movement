{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/a7abebc31a8f60011277437e000eebcc01702b9f";
    rust-overlay.url = "github:oxalica/rust-overlay/47beae969336c05e892e1e4a9dbaac9593de34ab";
    flake-utils.url = "github:numtide/flake-utils";
    foundry.url = "github:shazow/foundry.nix/36a5c5e448b4cdc85813e3b8205a8846a428d528"; 
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, foundry, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) foundry.overlay ];
        };

        toolchain = p: (p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = [ "rustfmt" "clippy" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain(toolchain);

        frameworks = pkgs.darwin.apple_sdk.frameworks;

        buildDependencies = with pkgs; [
          llvmPackages.bintools
          openssl
          openssl.dev
          libiconv 
          pkg-config
          libclang.lib
          libz
          clang
          pkg-config
          protobuf
          rustPlatform.bindgenHook
          lld
          mold
          coreutils
          gcc
          rust
          postgresql
          tesseract4
          ansible
          zlib
          fixDarwinDylibNames
        ];
        
        sysDependencies = with pkgs; [] 
        ++ lib.optionals stdenv.isDarwin [
          frameworks.Security
          frameworks.CoreServices
          frameworks.SystemConfiguration
          frameworks.AppKit
          libelf
        ] ++ lib.optionals stdenv.isLinux [
          udev
          systemd
          snappy
          bzip2
          elfutils
        ];

        testDependencies = with pkgs; [
          python311
          poetry
          just
          foundry-bin
          process-compose
          celestia-node
          celestia-app
          jq
          docker
          solc
          grpcurl
          grpcui
        ];

        # Specific version of toolchain
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust;
          rustc = rust;
        };

        celestia-app = pkgs.buildGoModule {
          pname = "celestia-app";
          version = "2.3.1";

          src = pkgs.fetchgit {
            url = "https://github.com/celestiaorg/celestia-app";
            rev = "v2.3.1";
            hash = "sha256-ui67KRaabQyZiV5QD4Qyaqobky++rAe9ppJ2yveoXOs=";
          };

          vendorHash = "sha256-zL3G+ml2bIcQlthHY6rovr2ykCGHqV51rQBkS3J9tGo=";
          subPackages = [ "cmd/celestia-appd" ];
        };

        celestia-node = pkgs.buildGoModule {
          pname = "celestia-node";
          version = "0.20.4";

          src = pkgs.fetchgit {
            url = "https://github.com/celestiaorg/celestia-node";
            rev = "v0.20.4";
            hash = "sha256-DWOKdbeRIlQycV0yvkfN/9eTlYt+qhwb9Ur7Y2SixFU=";
          };

          vendorHash = "sha256-yTN36N02fl9zf+YvliTREfEW1fnUnoRFdsLssx0xikk=";
          subPackages = [ "cmd/celestia" "cmd/cel-key" ];
        };
    
      in {
        packages = {
          inherit celestia-app celestia-node;
        };
        devShells = rec {
          default = docker-build;
          docker-build = pkgs.mkShell {
            ROCKSDB = pkgs.rocksdb;
            SNAPPY = if pkgs.stdenv.isLinux then pkgs.snappy else null;
            OPENSSL_DEV = pkgs.openssl.dev;

            hardeningDisable = ["fortify"];
         
            buildInputs = with pkgs; [
              # rust toolchain
              (toolchain pkgs)

              # build dependencies
              llvmPackages.bintools openssl openssl.dev libiconv pkg-config
              libclang.lib libz clang pkg-config protobuf rustPlatform.bindgenHook
              lld mold coreutils postgresql

              # test dependencies
              python311 poetry just foundry-bin process-compose jq docker solc
              grpcurl grpcui

              celestia-app celestia-node
            ] ++ lib.optionals stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
              Security CoreServices SystemConfiguration AppKit
            ]) ++ lib.optionals stdenv.isLinux (with pkgs; [
              udev systemd snappy bzip2 elfutils.dev
            ]);

            LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib/";

            shellHook = ''
              #!/usr/bin/env ${pkgs.bash}

              export DOT_MOVEMENT_PATH=$(pwd)/.movement
              mkdir -p $DOT_MOVEMENT_PATH

              # export PKG_CONFIG_PATH=$PKG_CONFIG_PATH_FOR_TARGET

              # Export linker flags if on Darwin (macOS)
              if [[ "${pkgs.stdenv.hostPlatform.system}" =~ "darwin" ]]; then
                export LDFLAGS="-L/opt/homebrew/opt/zlib/lib"
                export CPPFLAGS="-I/opt/homebrew/opt/zlib/include"
              fi

              echo "Monza Aptos path: $MONZA_APTOS_PATH"
              cat <<'EOF'
                 _  _   __   _  _  ____  _  _  ____  __ _  ____
                ( \/ ) /  \ / )( \(  __)( \/ )(  __)(  ( \(_  _)
                / \/ \(  O )\ \/ / ) _) / \/ \ ) _) /    /  )(
                \_)(_/ \__/  \__/ (____)\_)(_/(____)\_)__) (__)
              EOF

              echo "Develop with Move Anywhere"
            '';
          };
        };
      }
    );
}
