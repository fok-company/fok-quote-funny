{
  description = "A simple project. Just a fok (seal) with a quote cloud, saying some fun stuff (mostly ripped out of context, from my vc talks with Frends)";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    foklang.url = "github:fokohetman/foklang";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    foklang,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      config = import ./configuration.nix;
      lib = nixpkgs.lib;
    in {
      formatter = pkgs.alejandra;
      packages.default =
        pkgs.runCommand "fok-quote" {
          foklang = foklang.packages.${pkgs.stdenv.hostPlatform.system}.default;
          buildInputs = [pkgs.rustc pkgs.gcc];
          src = ./src;
          quotes =
            ["["]
            ++ (lib.lists.forEach config.quotes (x: "[\\\"" + toString (builtins.elemAt x 0) + "\\\" \\\"" + toString (builtins.elemAt x 1) + "\\\"]"))
            ++ ["]"];
          plush = "\\\"" + config.plush + "\\\"";
            #["["]
            #++ (lib.lists.forEach (lib.strings.splitString "\n" config.plush) (x: "\\\"" + toString x + "\\\""))
            #++ ["]"];
          #quotes = "[[\"test quote\" \"fokfok\"]]";
        } ''
          export CONFIG="{quotes=$quotes; plush=$plush}" #;$plush"
          mkdir -p "$out/bin"
          #rustc "$src/fok-quote.rs" -o "$out/bin/fok-quote";

          cp $src/fok-quote.fok $out/bin;
          cp $foklang/bin/foklang $out/bin; 
          echo "#! /usr/bin/env nix-shell" > $out/bin/fok-quote
          echo "#! nix-shell -i bash -p bash" >> $out/bin/fok-quote
          echo "export CONFIG=\"$CONFIG\"" >> $out/bin/fok-quote
          echo "$out/bin/foklang $out/bin/fok-quote.fok" >> $out/bin/fok-quote
          chmod +x $out/bin/fok-quote
        '';
    });
}
