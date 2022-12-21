{ pkgs }: {
	deps = [
		pkgs.rustc
    pkgs.openssl
    pkgs.pkg-config
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
    pkgs.rust-analyzer
	];
}