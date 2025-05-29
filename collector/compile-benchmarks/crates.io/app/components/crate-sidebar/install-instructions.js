import Component from '@glimmer/component';

export default class InstallInstructions extends Component {
  get cargoInstallCommand() {
    return this.args.exactVersion
      ? `cargo install ${this.args.crate}@${this.args.version}`
      : `cargo install ${this.args.crate}`;
  }

  get cargoAddCommand() {
    return this.args.exactVersion
      ? `cargo add ${this.args.crate}@=${this.args.version}`
      : `cargo add ${this.args.crate}`;
  }

  get tomlSnippet() {
    let version = this.args.version.split('+')[0];
    let exact = this.args.exactVersion ? '=' : '';
    return `${this.args.crate} = "${exact}${version}"`;
  }
}
