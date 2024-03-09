<div align="center">

# cloup

![Downloads][downloads-badge]
![License][license-badge]

cloup is a **template manager** that delivers the files you desire when you need them, written in rust

you no longer need to locate your previous project and copy and paste files over

[How it works](#how-it-works) •
[Installation](#installation)

</div>

![Example image][image]

## Motivation

I always find myself having to go search for certain files or folders, usually config setups, that are the same across every project. These could be virtually anything, but for me it's usually `.prettierrc`, `.eslintrc`, `tsconfig.json`, `tests/`, etc.

The idea is to be able to retrieve these files at the speed of light in any project I need them. So.. instead of locating an old project and copying the files to then paste in the new project, I wanted a way to save them as templates and be able to apply them into the current directory with a command, but also to be able to create my own version of base templates with already-configured settings.

No longer do you need to use the same `create-app-*` and then change it to your liking. Run it once, create a base template that you like, and use `cloup apply <your-template-name>` and it inserts all the folders and files recursively into the current directory.

## How it works

Say you have a folder with some files:

```
project
│   .prettierrc
│   .eslintrc
│   index.ts
│   package.json
│   tsconfig.json
└───src
    │   index.ts
    │   utils.ts
    └───folder
        │   another_file.ts
        │   ...
```

and you want to re-use the `.prettierrc` and `.eslintrc` for every project, you can create a cloup of those files by running the following command:

```sh
$ cloup create react-dotfiles -f .prettierrc .eslintrc
```

We've now created a cloup called `react-dotfiles`.

Now, say you create a new project:

```
new-project
│   index.ts
│   package.json
│   tsconfig.json
```

To apply the cloup in this directory, we simply run this command in the same directory:

```sh
$ cloup apply react-dotfiles
```

And the file tree now looks like this:

```
new-project
│   .prettierrc
│   .eslintrc
│   index.ts
│   package.json
│   tsconfig.json
```

It's that simple.

However, cloups are not limited to single files or folders. In the above examples, we used the `-f` flag to specify individual files and folders we wish to include in our cloup, but we can also create cloups of entire folders, with all their files recursively included.

Take this example:

```
project
│   .prettierrc
│   .eslintrc
│   index.ts
│   package.json
│   tsconfig.json
└───src
    │   index.ts
    │   ...
────tests
    │   ...
```

If we wanted this to be our "base template" for every new project we start, we can cloup the entire file by only giving the cloup a name and no extra arguments:

```sh
$ cloup create base-template
```

Now you can apply this cloup to any new empty folder with `$ cloup apply base-template` and have the entire folder structure with already-configured files and configs in moments.

## Installation

Cloup is currently only available through Homebrew and Cargo. Other methods of installation are on the way!

### Homebrew

```
$ brew tap benja/tap
$ brew install cloup
```

### Cargo

```
$ cargo install cloup
```

## Setup

After a successful install, create a new directory somewhere on your computer and run `$ cloup init` in that folder to initialise it as the template directory.

This is where all your cloups will be stored. Feel free to keep this folder versioned so you never lose your cloups if you switch computers.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

[downloads-badge]: https://img.shields.io/github/downloads/benja/cloup/total?color=bright-green&style=flat-square
[license-badge]: https://img.shields.io/github/license/benja/cloup?style=flat-square
[image]: contrib/banner.png
