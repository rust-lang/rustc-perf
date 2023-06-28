#!/usr/bin/env perl
use Archive::Zip;

my $version = '0.10.1';

sub add_regulars {
    my $zip = shift;

    $zip->addFile('completions/completions.bash', 'completions/exa.bash');
    $zip->addFile('completions/completions.zsh', 'completions/exa.zsh');
    $zip->addFile('completions/completions.fish', 'completions/exa.fish');

    $zip->addFile('target/man/exa.1', 'man/exa.1');
    $zip->addFile('target/man/exa_colors.5', 'man/exa_colors.5');
}

sub acc {
    my $zip = Archive::Zip->new();
    add_regulars($zip);
    $zip->writeToFileNamed("exa-accoutrements-v$version.zip") == AZ_OK || die 'Zip write error!';
}

sub src {
    my $zip = Archive::Zip->new();
    add_regulars($zip);
    $zip->addFile('Cargo.lock', 'Cargo.lock');
    $zip->addFile('Cargo.toml', 'Cargo.toml');
    $zip->addFile('LICENCE', 'LICENCE');
    $zip->addFile('build.rs', 'build.rs');
    $zip->addFile('Justfile', 'Justfile');
    $zip->addFile('README.md', 'README.md');
    $zip->addTree('vendor', 'vendor');
    $zip->writeToFileNamed("exa-vendored-source-v$version.zip") == AZ_OK || die 'Zip write error!';
}

acc();
src();
