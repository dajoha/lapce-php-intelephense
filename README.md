# lapce-php-intelephense

Lapce plugin for the Php language.

## Prerequisites

Install [Intelephense](https://github.com/bmewburn/intelephense-docs/blob/master/installation.md),
typically by running:

```sh
$ npm i intelephense -g
```

## Settings

### Settings: Intelephense server path

By default, the Intelephense server path is `/usr/bin/intelephense`. To ensure where Intelephense is
actually installed, you can run in a terminal:

```sh
$ whereis intelephense
```

In order to change this setting you can also edit the Lapce `settings.toml` file manually by adding:

```toml
[lapce-php-intelephense]
"lsp.serverPath" = "/absolute/path/to/intelephense/binary"
```

### Settings: licence key

If you have an Intelephense licence key, you can enter it in the plugin settings. Reload the plugin to make it
effective.

In order to change this setting you can also edit the Lapce `settings.toml` file manually by adding:

```toml
[lapce-php-intelephense]
licenceKey = "YOUR_LICENCE_KEY"
```
