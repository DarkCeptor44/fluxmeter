# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.1.0](https://github.com/DarkCeptor44/fluxmeter/compare/c6ba8fc84d8c377db85713474c9a4b9ab74b8614..v0.1.0) - 2026-09-22
#### Features
- (**backend**) add Dockerfile and a workflow to publish a multi-arch image to GHCR automatically - ([f5b9096](https://github.com/DarkCeptor44/fluxmeter/commit/f5b909624e65d4cc11c463c2f51681d4f4054852)) - DarkCeptor44
- (**backend**) make the header when cli runs server dynamic (overkill) - ([ac18f7d](https://github.com/DarkCeptor44/fluxmeter/commit/ac18f7d1b2f00142555f73e708bdd185ea08eaaf)) - DarkCeptor44
- (**backend**) trigger a backend rebuild if frontend changes - ([474c33f](https://github.com/DarkCeptor44/fluxmeter/commit/474c33f3e3fc0a55fcb3753b6375681971fce658)) - DarkCeptor44
- (**frontend**) add i18n - ([dcc7bbd](https://github.com/DarkCeptor44/fluxmeter/commit/dcc7bbd3eb86445053a707a13e9f38e0b13d2e32)) - DarkCeptor44
- (**frontend**) add toasts to frontend - ([f0d5faa](https://github.com/DarkCeptor44/fluxmeter/commit/f0d5faa1bc92bf8eb0ecd6f28fc69a3ac7b50aa0)) - DarkCeptor44
- (**frontend**) add settings page to customize payload length - ([3326a87](https://github.com/DarkCeptor44/fluxmeter/commit/3326a879b42ea8fd922805085261b4aa83ab5096)) - DarkCeptor44
- (**frontend**) initial frontend - ([54c7ec3](https://github.com/DarkCeptor44/fluxmeter/commit/54c7ec34dae854d13faeca6fccf977eea02f55ad)) - DarkCeptor44
- initial commit - ([c6ba8fc](https://github.com/DarkCeptor44/fluxmeter/commit/c6ba8fc84d8c377db85713474c9a4b9ab74b8614)) - DarkCeptor44
#### Bug Fixes
- (**build**) package pre-built 'dist' directory to crates.io so users dont need Bun to install from crates.io - ([0996908](https://github.com/DarkCeptor44/fluxmeter/commit/0996908837bc60ffed5a37089ce1db7e8291d5fa)) - DarkCeptor44
- (**docker**) make sure the frontend is installed before building the Rust binary - ([5cd544f](https://github.com/DarkCeptor44/fluxmeter/commit/5cd544f19a7663fca38a27f3eadebe9b4d845e1a)) - DarkCeptor44
- (**frontend**) do actual median in the utility function - ([7f9d44e](https://github.com/DarkCeptor44/fluxmeter/commit/7f9d44e89c7aec6f27181117d486dcf408ce8812)) - DarkCeptor44
- (**frontend**) use children snippet in Button component to allow any text inside the  button - ([ae45745](https://github.com/DarkCeptor44/fluxmeter/commit/ae457459ee3ef4e0994b7e7d75c6cbb1a0a10bf7)) - DarkCeptor44
- (**frontend**) labels in settings fields focus their input field when clicked - ([fce4a90](https://github.com/DarkCeptor44/fluxmeter/commit/fce4a908b8b7ab428c5301745d30be4237a90d2c)) - DarkCeptor44
- stop trying to package anything extra to crates.io, just dist and src - ([255fe76](https://github.com/DarkCeptor44/fluxmeter/commit/255fe7686dc19329071ab878652cf506698fa88e)) - DarkCeptor44
- hopefully fix node_modules getting package into the crates.io crate - ([77fdd1e](https://github.com/DarkCeptor44/fluxmeter/commit/77fdd1e7774526cd529e4027ccf4bd02b8f455e0)) - DarkCeptor44
#### Continuous Integration
- fix Dockerfile LABELs - ([a5a51cd](https://github.com/DarkCeptor44/fluxmeter/commit/a5a51cd9f3712faea1372ab3e0938e3af05ba83a)) - DarkCeptor44
#### Refactors
- (**backend**) raise MSRV due to time dep; remove unused parking_lot dep - ([aeb1726](https://github.com/DarkCeptor44/fluxmeter/commit/aeb1726ce14e863236f5aaa4327e8f686d936f54)) - DarkCeptor44
- (**backend**) implement ping, download, and upload endpoints; cleanup everything - ([3413640](https://github.com/DarkCeptor44/fluxmeter/commit/34136400b84e486f0348633da5e04f50455012ef)) - DarkCeptor44
- (**backend**) add server boilerplate + health endpoint - ([3bb0964](https://github.com/DarkCeptor44/fluxmeter/commit/3bb0964b18a5a4b4ec5c0d84d42d157f070134a4)) - DarkCeptor44
- (**frontend**) implement basic functionality for a speedtest - ([8dde80f](https://github.com/DarkCeptor44/fluxmeter/commit/8dde80f0e4e0e9cefc082c7d1a75d77812334c01)) - DarkCeptor44
- use port 7890 for the project - ([575f8ac](https://github.com/DarkCeptor44/fluxmeter/commit/575f8ac3031b45d94e88efdb7b9e9c2d0a984c11)) - DarkCeptor44
#### Style
- (**frontend**) add some color transitions to the stage bar - ([609d9e8](https://github.com/DarkCeptor44/fluxmeter/commit/609d9e8a739fde696d5d69da0be695fa51ae4da9)) - DarkCeptor44
- (**frontend**) adjust design on mobile screens and in general - ([2ae3330](https://github.com/DarkCeptor44/fluxmeter/commit/2ae3330c6fffb81c068a3151adced85e4810a114)) - DarkCeptor44
- (**frontend**) add icon/logo to project - ([8c0fa4d](https://github.com/DarkCeptor44/fluxmeter/commit/8c0fa4d8270839fbbb0ebf3a4eb037a5c29de64f)) - DarkCeptor44
- (**frontend**) adjust design for mobile screens - ([97c34ed](https://github.com/DarkCeptor44/fluxmeter/commit/97c34ed79bc4e9199715f3884789fbaf710cfc9d)) - DarkCeptor44
- (**frontend**) redesign of the main page - ([b0ec6dc](https://github.com/DarkCeptor44/fluxmeter/commit/b0ec6dca2edd5e8e2d943b9f8eaed9d34e1de201)) - DarkCeptor44

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).