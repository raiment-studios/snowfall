export async function upgrade_tools() {
  console.error(
    `
#!/bin/env bash
echo "Refreshing tools..."
pushd $MONOREPO_ROOT
asdf plugin list all > /dev/null

asdf plugin-add deno https://github.com/asdf-community/asdf-deno.git
asdf install deno latest
asdf local deno latest

deno install -Arf --global jsr:@deno/deployctl
asdf reshim deno    

asdf plugin-add rust https://github.com/asdf-community/asdf-rust.git
asdf install rust latest
asdf local rust latest

cargo binstall --root $MONOREPO_ROOT/bin/cargo mprocs -y

asdf plugin-add zig https://github.com/asdf-community/asdf-zig.git
asdf install zig latest
asdf local zig latest

asdf plugin add golang https://github.com/asdf-community/asdf-golang.git
asdf install golang latest
asdf local golang latest

asdf plugin add starship
asdf install starship latest
asdf local starship latest

asdf plugin-add zellij
asdf install zellij latest
asdf local zellij latest

asdf plugin-add bat
asdf install bat latest
asdf local bat latest

asdf plugin-add just
asdf install just latest
asdf local just latest

popd
`.trim() + "\n"
  );
}
